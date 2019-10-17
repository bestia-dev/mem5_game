//region: lmake_readme insert "readme.md"
//! **mem5_server - html and WebSocket server for the mem5 game**
//! 
//! version: 19.9.9  
//! Look also at the workspace readme https://github.com/LucianoBestia/mem5_game  
//! 
//! ## mem5_server
//! Primarily made for learning to code Rust for a http + WebSocket server on the same port  
//! Using Warp for a simple memory game for kids - mem5.  
//! On the local public IP address on port 80 listens to http and WebSocket.  
//! Route for http `/` serves static files from folder `/mem5/`  
//! Route `/mem5ws/` broadcast all WebSocket msg to all connected clients except sender  
//! 
//! ## Google vm
//! One working server is installed on google vm.  
//! You can play the game here (hosted on google cloud platform):  
//! http://bestia.dev/mem5  
//! 
//! 
//! 

//endregion: lmake_readme insert "readme.md"

//region: Clippy
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    //variable shadowing is idiomatic to Rust, but unnatural to me.
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::shadow_unrelated,
)]
#![allow(
    //library from dependencies have this clippy warnings. Not my code.
    clippy::cargo_common_metadata,
    clippy::multiple_crate_versions,
    clippy::wildcard_dependencies,
    //Rust is more idiomatic without return statement
    clippy::implicit_return,
    //I have private function inside a function. Self does not work there.
    //clippy::use_self,
    //Cannot add #[inline] to the start function with #[wasm_bindgen(start)]
    //because then wasm-pack build --target no-modules returns an error: export `run` not found 
    //clippy::missing_inline_in_public_items
    //Why is this bad : Doc is good. rustc has a MISSING_DOCS allowed-by-default lint for public members, but has no way to enforce documentation of private items. This lint fixes that.
    clippy::doc_markdown,
)]
//endregion

//region: use statements
use unwrap::unwrap;
use clap::{App, Arg};
use env_logger::Env;
use futures::sync::mpsc;
use futures::{Future, Stream};
use mem5_common::{ WsMessage};
use regex::Regex;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::process::Command;
use std::sync::{Arc, Mutex};
use warp::ws::{Message, WebSocket};
use warp::Filter;
use log::info;
//endregion

//region: enum, structs, const,...
/// Our status of currently connected users.
/// - Key is their id
/// - Value is a sender of `warp::ws::Message`
type Users = Arc<Mutex<HashMap<usize, mpsc::UnboundedSender<Message>>>>;

//endregion

///main function of the binary
fn main() {
    //region: ansi terminal color output (for log also)
    //only windows need this line
    enable_ansi_support();
    /*
    //region: examples
    eprintln!(
        "This is in red: {}",
        ansi_term::Colour::Red.paint("a red string")
    );

    eprintln!(
        "How about some {} and {}?",
        ansi_term::Style::new().bold().paint("bold"),
        ansi_term::Style::new().underline().paint("underline")
    );
    //endregion
    */
    //endregion

    //region: env_logger log text to stdout depend on ENV variable
    //in Linux : RUST_LOG=info ./mem5_server.exe
    //in Windows I don't know yet.
    //default for env variable info
    let mut builder = env_logger::from_env(Env::default().default_filter_or("info"));
    //nanoseconds in the logger
    builder.default_format_timestamp_nanos(true);
    builder.init();
    //endregion

    //region: cmdline parameters
    let matches = App::new("mem5_server")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("prm_ip")
                .value_name("ip")
                .default_value("0.0.0.0")
                .help("ip address for listening"),
        )
        .arg(
            Arg::with_name("prm_port")
                .value_name("port")
                .default_value("8085")
                .help("port for listening"),
        )
        .get_matches();

    //from string parameters to strong types
    let mut fnl_prm_ip = matches
        .value_of("prm_ip")
        .expect("error on prm_ip")
        .to_lowercase();
    let mut fnl_prm_port = matches
        .value_of("prm_port")
        .expect("error on prm_port")
        .to_lowercase();

    //if not cmd parameters, then use default local address
    //let's try the new defaults that work good with docker 0.0.0.0:8085
    if fnl_prm_ip == "" {
        let df_local_ip = local_ip_get().expect("cannot get local ip");
        fnl_prm_ip.push_str(&df_local_ip.to_string());
    }
    if fnl_prm_port == "" {
        let df_local_port = 80;
        fnl_prm_port.push_str(&df_local_port.to_string());
    }

    let local_ip = IpAddr::V4(fnl_prm_ip.parse::<Ipv4Addr>().expect("not an ip address"));
    let local_port = u16::from_str_radix(&fnl_prm_port, 10).expect("not a number");
    let local_addr = SocketAddr::new(local_ip, local_port);

    info!(
        "mem5 http server listening on {} and WebSocket on /mem5ws/",
        ansi_term::Colour::Red.paint(local_addr.to_string())
    );
    //endregion

    // Keep track of all connected users, key is usize, value
    // is a WebSocket sender.
    let users = Arc::new(Mutex::new(HashMap::new()));
    // Turn our "state" into a new Filter...
    //let users = warp::any().map(move || users.clone());
    //Clippy recommends this craziness instead of just users.clone()
    let users = warp::any().map(move || {
        Arc::<
            std::sync::Mutex<
                std::collections::HashMap<
                    usize,
                    futures::sync::mpsc::UnboundedSender<warp::ws::Message>,
                >,
            >,
        >::clone(&users)
    });

    //WebSocket server
    // GET from route /mem5ws/ -> WebSocket upgrade
    let websocket = warp::path("mem5ws")
        // The `ws2()` filter will prepare WebSocket handshake...
        .and(warp::ws2())
        .and(users)
        // Match `/mem5ws/url_param` it can be any string.
        .and(warp::path::param::<String>())
        .map(|ws: warp::ws::Ws2, users, url_param| {
            // This will call our function if the handshake succeeds.
            ws.on_upgrade(move |socket| user_connected(socket, users, url_param))
        });

    //static file server
    // GET files of route / -> are from folder /mem5/
    let fileserver = warp::fs::dir("./mem5/");

    let routes = fileserver.or(websocket);
    warp::serve(routes).run(local_addr);
}

//the url_param is not consumed in this function and Clippy wants a reference instead a value
#[allow(clippy::needless_pass_by_value)]
//region: WebSocket callbacks: connect, msg, disconnect
///new user connects
fn user_connected(
    ws: WebSocket,
    users: Users,
    url_param: String,
) -> impl Future<Item = (), Error = ()> {
    //the client sends his ws_uid in url_param. it is a random number.
    info!("user_connect() url_param: {}", url_param);
    //convert string to usize
    //hahahahaha syntax 'turbofish' ::<>
    let my_id = unwrap!(url_param.parse::<usize>());
    //if uid already exists, it is an error
    let mut user_exist=false;
    for (&uid, ..) in users.lock().expect("error users.lock()").iter() {
        if uid == my_id {
            user_exist=true;
            break;
        }
    }

    if user_exist{
        //disconnect the old user
        info!("user_disconnected for reconnect: {}", my_id);
        user_disconnected(my_id, &users);
    }

    // Split the socket into a sender and receive of messages.
    let (user_ws_tx, user_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages
    // to the WebSocket...
    let (tx, rx) = mpsc::unbounded();
    warp::spawn(
        rx.map_err(|()| -> warp::Error { unreachable!("unbounded rx never errors") })
            .forward(user_ws_tx)
            .map(|_tx_rx| ())
            .map_err(|ws_err| info!("WebSocket send error: {}", ws_err)),
    );

    // Save the sender in our list of connected users.
    info!("users.insert: {}", my_id);
    users.lock().expect("error uses.lock()").insert(my_id, tx);

    // Return a `Future` that is basically a state machine managing
    // this specific user's connection.
    // Make an extra clone to give to our disconnection handler...
    //let users2 = users.clone();
    //Clippy recommends this craziness instead of users.clone()
    let users2 = Arc::<
        std::sync::Mutex<
            std::collections::HashMap<
                usize,
                futures::sync::mpsc::UnboundedSender<warp::ws::Message>,
            >,
        >,
    >::clone(&users);

    user_ws_rx
        // Every time the user sends a message, call receive message
        .for_each(move |msg| {
            receive_message(my_id, &msg, &users);
            Ok(())
        })
        // for_each will keep processing as long as the user stays
        // connected. Once they disconnect, then...
        .then(move |result| {
            user_disconnected(my_id, &users2);
            result
        })
        // If at any time, there was a WebSocket error, log here...
        .map_err(move |e| {
            info!("WebSocket error(uid={}): {}", my_id, e);
        })
}

///on receive WebSocket message
fn receive_message(ws_uid_of_message: usize, messg: &Message, users: &Users) {
    // Skip any non-Text messages...
    let msg = if let Ok(s) = messg.to_str() {
        s
    } else {
        return;
    };

    let new_msg = msg.to_string();
    //info!("msg: {}", new_msg);

    //There are different messages coming from the mem5 wasm app
    //MsgInvite must be broadcasted to all users
    //all others must be forwarded to exactly the other player.

    let msg: WsMessage = serde_json::from_str(&new_msg).unwrap_or_else(|_x| WsMessage::MsgDummy {
        dummy: String::from("error"),
    });

    match msg {
        WsMessage::MsgDummy { dummy } => info!("MsgDummy: {}", dummy),
        WsMessage::MsgRequestWsUid {my_ws_uid, players_ws_uid } => {
            info!("MsgRequestWsUid: {} {}", my_ws_uid, players_ws_uid);
            let j = serde_json::to_string(
                &WsMessage::MsgResponseWsUid { 
                    your_ws_uid: ws_uid_of_message,
                    server_version: env!("CARGO_PKG_VERSION").to_string(),
                     })
                .expect("serde_json::to_string(&WsMessage::MsgResponseWsUid { your_ws_uid: ws_uid_of_message })");
            info!("send MsgResponseWsUid: {}", j);
            match users
                .lock()
                .expect("error users.lock()")
                .get(&ws_uid_of_message)
                .unwrap()
                .unbounded_send(Message::text(j))
            {
                Ok(()) => (),
                Err(_disconnected) => {}
            }
            //send to other users for reconnect. Do nothing if there is not yet other users.
            send_to_other_players(users, ws_uid_of_message, &new_msg, &players_ws_uid)
        },
        /* obsolete, but keep it as an example how to return a text file over websocket
        WsMessage::RequestGameConfig { filename } => {
            info!("RequestGameConfig: {}", filename);
            // read the file
            let mut pathbuf = env::current_dir().expect("env::current_dir()");
            pathbuf.push("mem5");
            pathbuf.push(filename);
            let filename =
                String::from(pathbuf.as_path().to_str().expect("path.as_path().to_str()"));
            info!("filename: {}", filename);
            let mut file = File::open(filename).expect("Unable to open the file");
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Unable to read the file");
            info!("read file : {}", contents);
            let j = serde_json::to_string(&WsMessage::ResponseGameConfigJson { json: contents })
                .expect(
                    "serde_json::to_string(&WsMessage::ResponseGameConfigJson { json: contents })",
                );
            info!("send ResponseGameConfigJson: {}", j);
            match users
                .lock()
                .expect("error users.lock()")
                .get(&ws_uid_of_message)
                .unwrap()
                .unbounded_send(Message::text(j))
            {
                Ok(()) => (),
                Err(_disconnected) => {}
            }
        }
        */
        WsMessage::MsgInvite { .. } => broadcast(users, ws_uid_of_message, &new_msg),
        WsMessage::MsgResponseWsUid { .. } => info!("MsgResponseWsUid: {}", ""),
        WsMessage::MsgPlayAccept { players_ws_uid, .. }
        | WsMessage::MsgGameDataInit { players_ws_uid, .. }
        | WsMessage::MsgPlayerClick1stCard { players_ws_uid, .. }
        | WsMessage::MsgPlayerClick2ndCardPoint { players_ws_uid, .. }
        | WsMessage::MsgPlayerClick2ndCardTakeTurnBegin { players_ws_uid, .. }
        | WsMessage::MsgTakeTurnEnd { players_ws_uid, .. }
        | WsMessage::MsgPlayerClick2ndCardGameOverPlayAgainBegin { players_ws_uid, .. } 
        | WsMessage::MsgAllGameData { players_ws_uid, .. }
        | WsMessage::MsgAckTakeTurnEnd{players_ws_uid, ..}
        | WsMessage::MsgAckPlayerClick1stCard{players_ws_uid, ..}
        => {
            send_to_other_players(users, ws_uid_of_message, &new_msg, &players_ws_uid)
        }
    }
}

///New message from this user send to all other players except sender.
fn send_to_other_players(
    users: &Users,
    ws_uid_of_message: usize,
    new_msg: &str,
    players_ws_uid: &str,
) {
    //info!("send_to_other_players: {}", new_msg);

    let vec_players_ws_uid: Vec<usize> = unwrap!(serde_json::from_str(players_ws_uid));

    for (&uid, tx) in users.lock().expect("error users.lock()").iter() {
        let mut is_player;
        is_player = false;
        for &pl_ws_uid in &vec_players_ws_uid {
            if pl_ws_uid == uid {
                is_player = true;
            }
        }
        if ws_uid_of_message != uid && is_player {
            match tx.unbounded_send(Message::text(String::from(new_msg))) {
                Ok(()) => (),
                Err(_disconnected) => {
                    // The tx is disconnected, our `user_disconnected` code
                    // should be happening in another task, nothing more to
                    // do here.
                }
            }
        }
    }
}

///broadcast is the simplest
fn broadcast(users: &Users, ws_uid_of_message: usize, new_msg: &str) {
    // New message from this user, send it to everyone else (except same uid)...
    // We use `retain` instead of a for loop so that we can reap any user that
    // appears to have disconnected.
    info!("broadcast: {}", new_msg);
    for (&uid, tx) in users.lock().expect("error users.lock()").iter() {
        if ws_uid_of_message != uid {
            match tx.unbounded_send(Message::text(String::from(new_msg))) {
                Ok(()) => (),
                Err(_disconnected) => {
                    // The tx is disconnected, our `user_disconnected` code
                    // should be happening in another task, nothing more to
                    // do here.
                }
            }
        }
    }
}

///disconnect user
fn user_disconnected(my_id: usize, users: &Users) {
    info!("good bye user: {}", my_id);

    // Stream closed up, so remove from the user list
    users.lock().expect("users.lock").remove(&my_id);
}
//endregion

//region: local ip (Linux and windows distinct versions)
#[cfg(target_family = "unix")]
///get local ip for Unix with ifconfig
pub fn local_ip_get() -> Option<IpAddr> {
    info!("local_ip_get for unix{}", "");
    let output = Command::new("ifconfig")
        .output()
        .expect("failed to execute `ifconfig`");

    let stdout = unwrap!(String::from_utf8(output.stdout));

    let re = unwrap!(Regex::new(r#"(?m)^.*inet (addr:)?(([0-9]*\.){3}[0-9]*).*$"#));
    for cap in re.captures_iter(&stdout) {
        let host = cap.get(2).map_or("", |m| m.as_str());
        if host != "127.0.0.1" {
            if let Ok(addr) = host.parse::<Ipv4Addr>() {
                return Some(IpAddr::V4(addr));
            }
            if let Ok(addr) = host.parse::<Ipv6Addr>() {
                return Some(IpAddr::V6(addr));
            }
        }
    }
    //return
    None
}

#[cfg(target_family = "windows")]
///get local ip for windows with ipconfig
pub fn local_ip_get() -> Option<IpAddr> {
    info!("local_ip_get for windows{}", "");
    let output = Command::new("ipconfig")
        .output()
        .expect("failed to execute `ipconfig`");

    let stdout = String::from_utf8(output.stdout).expect("failed stdout");
    //variables are block scope and will not interfere with the other block
    {
        let re =
            Regex::new(r"(?m)^   IPv4 Address\. \. \. \. \. \. \. \. \. \. \. : ([0-9\.]*)\s*$")
                .expect("failed regex");
        let cap = re.captures(&stdout).expect("failed capture");
        let host = cap.get(1).map_or("", |m| m.as_str());
        if let Ok(addr) = host.parse::<Ipv4Addr>() {
            return Some(IpAddr::V4(addr));
        }
    }
    //variables are block scope and will not interfere with the other block
    {
        let re =
            Regex::new(r"(?m)^   Link-local IPv6 Address \. \. \. \. \. : ([:%a-f0-9\.]*)\s*$")
                .expect("failed regex");
        let cap = re.captures(&stdout).expect("capture");
        let host = cap.get(1).map_or("", |m| m.as_str());
        if let Ok(addr) = host.parse::<Ipv6Addr>() {
            return Some(IpAddr::V6(addr));
        }
    }
    None
}
//endregion

//region: only windows need enable ansi support
#[cfg(target_family = "windows")]
///ansi support
pub fn enable_ansi_support() {
    let _enabled = ansi_term::enable_ansi_support();
}
#[cfg(target_family = "unix")]
///ansi support
pub const fn enable_ansi_support() {
    //do nothing
}
