//region: lmake_readme insert "readme.md"
//! # mem5
//! 
//! mem5 is a simple memory game made primarily for learning the Rust programming language and Wasm/WebAssembly with Virtual Dom Dodrio and WebSocket communication  
//! 
//! version: 19.9.23  
//! Look also at the workspace readme on <https://github.com/LucianoBestia/mem5_game>  
//! 
//! ## Idea
//! 
//! Playing the memory game alone is boring.  
//! Playing it with friends is better.  
//! But if all friends just stare in their smartphones, it is still boring.  
//! What makes memory games (and other board games) entertaining is the company of friends.  
//! There must be many friends around the table watching one another and stealing moves and laughing and screaming at each other.  
//! Today I assume everybody has a decent smartphone. If all friends open the mem5 game and put their smartphones on the center of the table one near the other so that everybody can see them and touch them, this is the closest it gets to a classic board game.  
//! All the phones will have a small card grid (ex. 3x3). But the combined card grid from all these phones together is not so small anymore. It is now much more interesting to play for the players.  
//! It can be played with as many friends as there are: 3,4,5,...  
//! More friends - more fun.  
//! 
//! ## Rust and Wasm/WebAssembly
//! 
//! Rust is a pretty new language created by Mozilla for really low level programming.  
//! It is a step forward from the C language with functionality and features that are best practice today.  
//! It is pretty hard to learn. Some concepts are so different from other languages it makes it
//! hard for beginners. Lifetimes are the strangest and most confusing concept.  
//! The Rust language has been made from the ground up with an ecosystem that makes it productive.  
//! The language and most of the libraries are Open Source. That is good and bad, but mostly good.  
//! Rust is the best language today to compile into Wasm/WebAssembly.  
//! That compiled code works inside a browser directly with the JavaScript engine.  
//! So finally no need for JavaScript to make cross-platform applications inside browsers.  
//! I have a lot of hope here.  
//! 
//! ## Virtual DOM
//! 
//! Constructing a HTML page with Virtual DOM (vdom) is easier  
//! because it is rendered completely on every tick (animation frame).  
//! Sometimes is hard for the developer to think what should change in the UI when some data changes.  
//! The data can change from many different events and very chaotically (asynchronously).  
//! It is easier to think how to render the complete DOM for the given data.  
//! The Rust Dodrio library has ticks, time intervals when it do something.  
//! If a rendering is scheduled, it will be done on the next tick.  
//! If a rendering is not scheduled I believe nothing happens.  
//! This enables asynchronous changing of data and rendering. They cannot happen theoretically in the
//! same exact moment. So, no data race here.  
//! When GameData change and we know it will affect the DOM, then rendering must be scheduled.  
//! The main component of the Dodrio Virtual Dom is the root rendering component.  
//! It is the component that renders the complete user interface (HTML).  
//! The root rendering component is easily splitted  into sub-components.  
//! ![subcomponents](https://github.com/LucianoBestia/mem5_game/raw/master/docs/img/subcomponents.png)  
//! Some subcomponents don't need any extra data and can be coded as simple functions.  
//! The subcomponent "players and scores" has its own data. This data is cached from the GameData.  
//! When this data does not match, invalidation is called to cache them.
//! That also schedules the rendering of the subcomponent.  
//! If no data has changed, the cached subcomponent Node is used. This is more efficient and performant.  
//! 
//! ## GameData
//! 
//! All the game data are in this simple struct.  
//! 
//! ## WebSocket communication
//! 
//! HTML5 has finally bring a true stateful bidirectional communication.  
//! Most of the programming problems are more easily and effectively solved this way.  
//! The old unidirectional stateless communication is very good for static html pages,  
//! but is terrible for any dynamic page. The WebSocket is very rudimental and often the  
//! communication breaks for many different reasons. The programmer must deal with it inside the application.  
//! The protocol has nothing that can be used to deal with reconnections.  
//! I send simple structs text messages in json format between the players.  
//! They are all in the WsMsg enum and therefore interchangeable.  
//! The WebSocket server is coded especially for this game and recognizes 3 types of msg:
//! 
//! - msg to broadcast to every other player
//! - msg to send only to the actual game players
//! 
//! ## WS reconnect
//! 
//! TODO: It looks that plain web sockets have often connection problems and they disconnect here and there. Creating a good reconnect is pretty challenging.  
//! 
//! ## The game flow
//! 
//! In a few words: Status1 - User action - Status2, Status1 - WsMessage - Status2
//! In one moment the game is in a certain Game Status. The user then makes an action.
//! This action changes the GameData and the GameStatus.  
//! Then a message is sent to other players so they can also change their local GameData and GameStatus.  
//! The rendering is scheduled and it will happen shortly (async).  
//! 
//! | Game Status1       | Render                     | User action                                 | Condition                            | GameStatus2 t.p.   | Sends Msg          | On rcv Msg o.p.              | GameStatus2 o.p.                   |
//! | ------------------ | -------------------------- | ------------------------------------------- | ------------------------------------ | ----------------   | ----------------   | --------------------------   | --------------------------------   |
//! | StatusInviteAskBegin     | div_invite_ask_begin       | div_invite_ask_begin_on_click               | -                                    | StatusInviteAsking       | MsgInvite             | on_msg_invite                | StatusInviteAsked                        |
//! | StatusInviteAsked        | div_invite_asked, div_play_accepted | div_invite_asked_on_click          | -                                    | StatusPlayAccepted       | MsgPlayAccept         | on_msg_play_accept           | -                                  |
//! | StatusInviteAsking       | div_invite_asking          | game_data_init                              | -                                    | StatusPlayBefore1stCard  | MsgGameDataInit       | on_msg_game_data_init        | StatusPlayBefore1stCard                  |
//! | StatusPlayBefore1stCard  | div_grid_container         | div_grid_item_on_click, on_click_1st_card();| -                                    | StatusPlayBefore2ndCard  | MsgPlayerClick1stCard | on_msg_player_click_1st_card | StatusPlayBefore2ndCard                  |
//! | StatusPlayBefore2ndCard  | div_grid_container         | div_grid_item_on_click, on_click_2nd_card();| If card match and points<all point   | StatusPlayBefore1stCard  | MsgPlayerClick2ndCardPoint | on_msg_player_click_2nd_card | StatusPlayBefore1stCard                  |
//! | -II-               | -II-                       | -II-                                        | If card match and points=>all points | StatusGameOverPlayAgainBegin | StatusGameOverPlayAgainBegin  | on_msg_play_again   | StatusGameOverPlayAgainBegin             |
//! | -II-               | -II-                       | -II-                                        | else                                 | MsgPlayerClick2ndCardTakeTurnBegin      | MsgPlayerClick2ndCardTakeTurnBegin      | on_msg_take_turn             | MsgPlayerClick2ndCardTakeTurnBegin                      |
//! | MsgPlayerClick2ndCardTakeTurnBegin      | div_take_turn_begin        | div_take_turn_begin_on_click                | -                                    | StatusPlayBefore1stCard  | MsgTakeTurnEnd        | on_msg_take_turn_end         | StatusPlayBefore1stCard, the next player |
//! | StatusGameOverPlayAgainBegin | div_play_again         | window.location().reload()                  | -                                    | -                  | -                  | -                            | -                                  |
//! |  |  |  |  |  |  |  |  |
//! 
//! t.p. = this player,   o.p. = other players,  rrc = rrc, rcv = receive
//! 
//! 1. Some actions can have different results. For example the condition card match or card don’t match.  
//! 2. one action must be only for one status1. This action changes Status for this player and sends Msg to other players.  
//! 3. on receive msg can produce only one status2.  
//! 4. in this table I ignore msgs for the server like GetConfig  
//! 
//! ## Futures and Promises, Rust and JavaScript
//! 
//! JavaScript is all asynchronous. Wasm is nothing else then a shortcut to the JavaScript engine.  
//! So everything is asynchronous too. This is pretty hard to grasp. Everything is Promises and Futures.  
//! There is a constant jumping from thinking in Rust to thinking is JavaScript and back. That is pretty confusing.  
//! JavaScript does not have a good idea of Rust datatypes. All there is is a generic JSValue type.  
//! The library `wasm-bindgen` has made a fantastic job of giving Rust the ability to call
//! anything JavaScript can call, but the way of doing it is sometimes very hard to understand.  
//! 
//! ## Typed html
//! 
//! Writing html inside Rust code is much easier with the macro `html!` from the `crate typed-html`  
//! <https://github.com/bodil/typed-html>  
//! It has also a macro `dodrio!` created exclusively for the dodrio vdom.  
//! Everything is done in compile time, so the runtime is nothing slower.
//! 
//! ## Browser console
//! 
//! At least in modern browsers (Firefox and Chrome) we have the developer tools F12 and there is a
//! console we can output to. So we can debug what is going on with our Wasm program.
//! But not on smartphones that are the only target for this app.  
//! 
//! ## Safari on iOS and FullScreen
//! 
//! Apple is very restrictive and does not allow fullscreen Safari on iPhones.  
//! The workaround is to make a shortcut for the webapp on the homescreen.  
//! 
//! ## mem5 as webapp on HomeScreen
//! 
//! On both android and iPhone is possible to "Add to homescreen" the webapp.  
//! Then it will open in fullscreen and be beautiful.  
//! In safari the share icon (a square with arrow up) has "Add to home screen".
//! <https://developer.apple.com/library/archive/documentation/AppleApplications/Reference/SafariWebContent/ConfiguringWebApplications/ConfiguringWebApplications.html>  
//! 
//! ## Modules
//! 
//! Rust code is splitted into modules. They are not exactly like classes, but can be similar.  
//! Rust has much more freedom to group code in different ways. So that is best suits the problem.  
//! I splitted the rendering into sub-components.  
//! And then I splitted the User Actions by the Status1 to easy follow the flow of the game.  
//! 
//! ## Clippy
//! 
//! Clippy is very useful to teach us how to program in a better way.  
//! These are not syntax errors, but hints how to do it in a more Rusty way (idiomatic).  
//! Some lints are problematic and they are explicitly allowed here.
//! 
//! ## Cargo make
//! 
//! I prepared some flows and tasks for Cargo make.  
//! `cargo make` - lists the possible available/public flows/tasks  
//! `cargo make dev` - builds the development version and runs the server and the browser  
//! `cargo make release` - builds the release version and runs the server and the browser  
//! `cargo make doc` - build the `/target/doc` folder and copy to the `../docs` folder.  

//endregion: lmake_readme insert "readme.md"

//needed for dodrio! macro (typed-html)
#![recursion_limit = "512"]
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
    //Why is this bad: It will be more difficult for users to discover the purpose of the crate, 
    //and key information related to it.
    clippy::cargo_common_metadata,
    //Why is this bad : This bloats the size of targets, and can lead to confusing error messages when 
    //structs or traits are used interchangeably between different versions of a crate.
    clippy::multiple_crate_versions,
    //Why is this bad : As the edition guide says, it is highly unlikely that you work with any possible 
    //version of your dependency, and wildcard dependencies would cause unnecessary 
    //breakage in the ecosystem.
    clippy::wildcard_dependencies,
    //Rust is more idiomatic without return statement
    //Why is this bad : Actually omitting the return keyword is idiomatic Rust code. 
    //Programmers coming from other languages might prefer the expressiveness of return. 
    //It’s possible to miss the last returning statement because the only difference 
    //is a missing ;. Especially in bigger code with multiple return paths having a 
    //return keyword makes it easier to find the corresponding statements.
    clippy::implicit_return,
    //I have private function inside a function. Self does not work there.
    //Why is this bad: Unnecessary repetition. Mixed use of Self and struct name feels inconsistent.
    clippy::use_self,
    //Cannot add #[inline] to the start function with #[wasm_bindgen(start)]
    //because then wasm-pack build --target web returns an error: export run not found 
    //Why is this bad: In general, it is not. Functions can be inlined across crates when that’s profitable 
    //as long as any form of LTO is used. When LTO is disabled, functions that are not #[inline] 
    //cannot be inlined across crates. Certain types of crates might intend for most of the 
    //methods in their public API to be able to be inlined across crates even when LTO is disabled. 
    //For these types of crates, enabling this lint might make sense. It allows the crate to 
    //require all exported methods to be #[inline] by default, and then opt out for specific 
    //methods where this might not make sense.
    clippy::missing_inline_in_public_items,
    //Why is this bad: This is only checked against overflow in debug builds. In some applications one wants explicitly checked, wrapping or saturating arithmetic.
    //clippy::integer_arithmetic,
    //Why is this bad: For some embedded systems or kernel development, it can be useful to rule out floating-point numbers.
    clippy::float_arithmetic,
    //Why is this bad : Doc is good. rustc has a MISSING_DOCS allowed-by-default lint for public members, but has no way to enforce documentation of private items. This lint fixes that.
    clippy::doc_markdown,
    //Why is this bad : Splitting the implementation of a type makes the code harder to navigate.
    clippy::multiple_inherent_impl,
)]
//endregion

//region: mod is used only in lib file. All the rest use use crate
mod divcardmonikermod;
mod divfordebuggingmod;
mod divfullscreenmod;
mod divgridcontainermod;
mod divplayeractionsmod;
mod divplayersandscoresmod;
mod divrulesanddescriptionmod;
mod fetchmod;
mod fetchgamesmetadatamod;
mod fetchgameconfigmod;
mod gamedatamod;
mod javascriptimportmod;
mod localstoragemod;
mod logmod;
mod rootrenderingcomponentmod;
mod sessionstoragemod;
mod statusinviteaskbeginmod;
mod statusinviteaskedmod;
mod statusinviteaskingmod;
mod statusplayagainmod;
mod statusplaybefore1stcardmod;
mod statusplaybefore2ndcardmod;
mod statustaketurnbeginmod;
mod websocketcommunicationmod;
mod websocketreconnectmod;
//endregion

//region: use statements
use unwrap::unwrap;
use rand::rngs::SmallRng;
use rand::FromEntropy;
use rand::Rng;
use wasm_bindgen::prelude::*;
//endregion

//region: wasm_bindgen(start) is where everything starts
#[wasm_bindgen(start)]
///To start the Wasm application, wasm_bindgen runs this functions
pub fn run() -> Result<(), JsValue> {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();

    // Get the document's container to render the virtual Dom component.
    let window = unwrap!(web_sys::window(), "error: web_sys::window");
    let document = unwrap!(window.document(), "error: window.document");
    let div_for_virtual_dom = unwrap!(
        document.get_element_by_id("div_for_virtual_dom"),
        "No #div_for_virtual_dom"
    );

    let mut rng = SmallRng::from_entropy();
    //my_ws_uid is random generated on the client and sent to
    //WebSocket server with an url param
    let my_ws_uid: usize = rng.gen_range(1, 9999);

    //find out URL
    let mut location_href = unwrap!(window.location().href(), "href not known");
    //without /index.html
    location_href=location_href.to_lowercase().replace("index.html","");
    //logmod::debug_write(&format!("location_href: {}", &location_href));

    //WebSocket connection
    let players_ws_uid="[]".to_string(); //empty vector in json
    let ws = websocketcommunicationmod::setup_ws_connection(location_href.clone(), my_ws_uid,players_ws_uid);
    //I don't know why is needed to clone the WebSocket connection
    let ws_c = ws.clone();

    // Construct a new RootRenderingComponent.
    //I added ws_c so that I can send messages on WebSocket

    let mut rrc =
        rootrenderingcomponentmod::RootRenderingComponent::new(ws_c, my_ws_uid);
    rrc.game_data.href = location_href.to_string();

    // Mount the component to the `<div id="div_for_virtual_dom">`.
    let vdom = dodrio::Vdom::new(&div_for_virtual_dom, rrc);

    websocketcommunicationmod::setup_all_ws_events(&ws, vdom.weak());

    //async fetch_response() for gamesmetadata.json
    let v2 = vdom.weak();
    fetchgamesmetadatamod::fetch_games_metadata_request(location_href, v2);

    // Run the component forever. Forget to drop the memory.
    vdom.forget();

    Ok(())
}
//endregion

/// Get the top-level window's session storage.
/// TODO: to save user preferences maybe?
pub fn session_storage() -> web_sys::Storage {
    let window = unwrap!(web_sys::window(), "error: web_sys::window");
    window.session_storage().unwrap_throw().unwrap_throw()
}
//endregion

///format ordinal numbers as string 1st, 2nd,3rd,...
#[allow(
    clippy::indexing_slicing,  
    clippy::integer_arithmetic
    )]
pub fn ordinal_numbers(number:usize)->String{
    //these are only ascii characters, so no problem with utf_8
    let mut ord_str = format!("{}",number);
    let places = ord_str.len();

    let slice = &ord_str[places-1..];
    if slice=="1"{
        ord_str.push_str("st");
    }
    else if slice=="2"{
        ord_str.push_str("nd");
    }
    else if slice=="3"{
        ord_str.push_str("rd");
    }
    else {
        ord_str.push_str("th");
    }
    //return
    ord_str
}

