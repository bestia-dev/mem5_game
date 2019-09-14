//! WebSocketcommunication.rs  - module that cares about WebSocket communication

//region: use
use crate::rootrenderingcomponent::RootRenderingComponent;
use crate::statusinviteasked;
use crate::statusinviteaskbegin;
use crate::statusplaybefore1stcard;
use crate::statusplaybefore2ndcard;
use crate::statustaketurnbegin;
use crate::logmod;

use futures::Future;
use js_sys::Reflect;
use mem5_common::GameStatus;
use mem5_common::WsMessage;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, ErrorEvent, WebSocket};
//endregion

//the location_href is not consumed in this function and Clippy wants a reference instead a value
//but I don't want references, because they have the lifetime problem.
#[allow(clippy::needless_pass_by_value)]
///setup WebSocket connection
pub fn setup_ws_connection(location_href: String, client_ws_id: usize) -> WebSocket {
    //web-sys has WebSocket for Rust exactly like JavaScript has¸
    //location_href comes in this format  http://localhost:4000/
    let mut loc_href = location_href.replace("http://", "ws://");
    //Only for debugging in the development environment
    //let mut loc_href = String::from("ws://192.168.1.57:80/");
    loc_href.push_str("mem5ws/");

    //send the client ws id as url_param for the first connect
    //and reconnect on lost connection
    loc_href.push_str(client_ws_id.to_string().as_str());
    logmod::log1_str(&format!(
        "location_href {}  loc_href {} client_ws_id {}",
        location_href, loc_href, client_ws_id
    ));

    //same server address and port as http server
    //for reconnect the old ws id will be an url param
    let ws = unwrap!(WebSocket::new(&loc_href), "WebSocket failed to connect.");

    //I don't know why is clone needed
    let ws_c = ws.clone();
    //It looks that the first send is in some way a handshake and is part of the connection
    //it will be execute on open as a closure
    let open_handler = Box::new(move || {
        console::log_1(&"Connection opened, sending RequestWsUid to server".into());
        unwrap!(
            ws_c.send_with_str(
                &serde_json::to_string(&WsMessage::RequestWsUid {
                    test: String::from("test"),
                })
                .expect("error sending test"),
            ),
            "Failed to send 'test' to server"
        );
    });

    let cb_oh: Closure<dyn Fn()> = Closure::wrap(open_handler);
    ws.set_onopen(Some(cb_oh.as_ref().unchecked_ref()));
    //don't drop the open_handler memory
    cb_oh.forget();

    ws
}

/// receive WebSocket msg callback. I don't understand this much. Too much future and promises.
pub fn setup_ws_msg_recv(ws: &WebSocket, weak: dodrio::VdomWeak) {
    //Player1 on machine1 have a button Ask player to play! before he starts to play.
    //Click and it sends the WsMessage invite. Player1 waits for the reply and cannot play.
    //Player2 on machine2 see the WsMessage and Accepts it.
    //It sends a WsMessage with the vector of cards. Both will need the same vector.
    //The vector of cards is copied.
    //Player1 click a card. It opens locally and sends WsMessage with index of the card.
    //Machine2 receives the WsMessage and runs the same code as the player would click. The RootRenderingComponent is blocked.
    //The method with_component() needs a future (promise) It will be executed on the next vdom tick.
    //This is the only way I found to write to RootRenderingComponent fields.
    let msg_recv_handler = Box::new(move |msg: JsValue| {
        let data: JsValue = unwrap!(
            Reflect::get(&msg, &"data".into()),
            "No 'data' field in WebSocket message!"
        );

        //serde_json can find out the variant of WsMessage
        //parse json and put data in the enum
        let msg: WsMessage =
            serde_json::from_str(&data.as_string().expect("Field 'data' is not string"))
                .unwrap_or_else(|_x| WsMessage::Dummy {
                    dummy: String::from("error"),
                });

        //match enum by variant and prepares the future that will be executed on the next tick
        //in this big enum I put only boilerplate code that don't change any data.
        //for changing data I put code in separate functions for easy reading.
        match msg {
            //I don't know why I need a dummy, but is entertaining to have one.
            WsMessage::Dummy { dummy } => console::log_1(&dummy.into()),
            //this RequestWsUid is only for the WebSocket server
            WsMessage::RequestWsUid { test } => console::log_1(&test.into()),
            WsMessage::ResponseWsUid { your_ws_uid } => {
                wasm_bindgen_futures::spawn_local(
                    weak.with_component({
                        move |root| {
                            logmod::log1_str(&format!("ResponseWsUid: {}  ", your_ws_uid));
                            let rrc =
                                root.unwrap_mut::<RootRenderingComponent>();
                            rrc.on_response_ws_uid(your_ws_uid);
                        }
                    })
                    .map_err(|_| ()),
                );
            }

            WsMessage::Invite {
                my_ws_uid,
                my_nickname,
                asked_folder_name,
            } => {
                wasm_bindgen_futures::spawn_local(
                    weak.with_component({
                        let v2 = weak.clone();
                        move |root| {
                            let rrc =
                                root.unwrap_mut::<RootRenderingComponent>();

                            if let GameStatus::GameOverPlayAgainBegin
                            | GameStatus::InviteAskBegin
                            | GameStatus::InviteAsked =
                                rrc.game_data.game_status
                            {
                                statusinviteaskbegin::on_msg_invite(
                                    rrc,
                                    my_ws_uid,
                                    my_nickname,
                                    asked_folder_name,
                                );
                                v2.schedule_render();
                            }
                        }
                    })
                    .map_err(|_| ()),
                );
            }
            
            WsMessage::PlayAccept { my_ws_uid, my_nickname, .. } => {
                wasm_bindgen_futures::spawn_local(
                    weak.with_component({
                        let v2 = weak.clone();
                        move |root| {
                            console::log_1(&"rcv PlayAccept".into());
                            let rrc =
                                root.unwrap_mut::<RootRenderingComponent>();
                            statusinviteasked::on_msg_play_accept(
                                rrc,
                                my_ws_uid,
                                my_nickname
                            );
                            v2.schedule_render();
                        }
                    })
                    .map_err(|_| ()),
                );
            }
            WsMessage::GameDataInit {
                card_grid_data,
                game_config,
                players,
            } => {
                wasm_bindgen_futures::spawn_local(
                    weak.with_component({
                        let v2 = weak.clone();
                        move |root| {
                            let rrc =
                                root.unwrap_mut::<RootRenderingComponent>();

                            if let GameStatus::PlayAccepted =
                                rrc.game_data.game_status
                            {
                                rrc.on_msg_game_data_init(
                                    &card_grid_data,
                                    &game_config,
                                    &players,
                                );
                                v2.schedule_render();
                            }
                        }
                    })
                    .map_err(|_| ()),
                );
            }
            WsMessage::PlayerClick1stCard {
                card_grid_data,
                game_status,
                card_index_of_first_click,
                card_index_of_second_click,
                ..
            } => {
                wasm_bindgen_futures::spawn_local(
                    weak.with_component({
                        let v2 = weak.clone();
                        console::log_1(&"player_click".into());
                        move |root| {
                            let rrc =
                                root.unwrap_mut::<RootRenderingComponent>();
                            console::log_1(&"players".into());
                            statusplaybefore1stcard::on_msg_player_click_1st_card(
                                rrc,
                                game_status,
                                card_grid_data.as_str(),
                                card_index_of_first_click,
                                card_index_of_second_click,
                            );
                            v2.schedule_render();
                        }
                    })
                    .map_err(|_| ()),
                );
            }
            WsMessage::PlayerClick2ndCard {
                players,
                card_grid_data,
                game_status,
                card_index_of_first_click,
                card_index_of_second_click,
                ..
            } => {
                wasm_bindgen_futures::spawn_local(
                    weak.with_component({
                        let v2 = weak.clone();
                        console::log_1(&"player_click".into());
                        move |root| {
                            let rrc =
                                root.unwrap_mut::<RootRenderingComponent>();
                            console::log_1(&"players".into());
                            statusplaybefore2ndcard::on_msg_player_click_2nd_card(
                                rrc,
                                players.as_str(),
                                game_status,
                                card_grid_data.as_str(),
                                card_index_of_first_click,
                                card_index_of_second_click,
                            );
                            v2.schedule_render();
                        }
                    })
                    .map_err(|_| ()),
                );
            }
            WsMessage::TakeTurnBegin {
                card_grid_data,
                game_status,
                card_index_of_first_click,
                card_index_of_second_click,
                ..
            } => {
                wasm_bindgen_futures::spawn_local(
                    weak.with_component({
                        let v2 = weak.clone();
                        console::log_1(&"take turn begin".into());
                        move |root| {
                            let rrc =
                                root.unwrap_mut::<RootRenderingComponent>();
                            console::log_1(&"players".into());
                            statustaketurnbegin::on_msg_take_turn_begin(
                                rrc,
                                game_status,
                                card_grid_data.as_str(),
                                card_index_of_first_click,
                                card_index_of_second_click,
                            );
                            v2.schedule_render();
                        }
                    })
                    .map_err(|_| ()),
                );
            }
            WsMessage::TakeTurnEnd { .. } => {
                wasm_bindgen_futures::spawn_local(
                    weak.with_component({
                        let v2 = weak.clone();
                        move |root| {
                            let rrc =
                                root.unwrap_mut::<RootRenderingComponent>();
                            console::log_1(&"TakeTurnEnd".into());
                            statustaketurnbegin::on_msg_take_turn_end(rrc);
                            v2.schedule_render();
                        }
                    })
                    .map_err(|_| ()),
                );
            }
            WsMessage::GameOverPlayAgainBegin {
                players,
                card_grid_data,
                game_status,
                card_index_of_first_click,
                card_index_of_second_click,
                ..
            } => {
                wasm_bindgen_futures::spawn_local(
                    weak.with_component({
                        let v2 = weak.clone();
                        console::log_1(&"play again".into());
                        move |root| {
                            let rrc =
                                root.unwrap_mut::<RootRenderingComponent>();
                            console::log_1(&"players".into());
                            statusplaybefore2ndcard::on_msg_play_again(
                                rrc,
                                players.as_str(),
                                game_status,
                                card_grid_data.as_str(),
                                card_index_of_first_click,
                                card_index_of_second_click,
                            );
                            v2.schedule_render();
                        }
                    })
                    .map_err(|_| ()),
                );
            }
        }
    });

    //magic ??
    let cb_mrh: Closure<dyn Fn(JsValue)> = Closure::wrap(msg_recv_handler);
    ws.set_onmessage(Some(cb_mrh.as_ref().unchecked_ref()));

    //don't drop the event listener from memory
    cb_mrh.forget();
}
/// on error write it on the screen for debugging
pub fn setup_ws_onerror(ws: &WebSocket, weak: dodrio::VdomWeak) {
    let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
        let err_text = format!("error event {:?}", e);
        logmod::log1_str(&err_text);
        {
            wasm_bindgen_futures::spawn_local(
                weak.with_component({
                    let v2 = weak.clone();
                    move |root| {
                        console::log_1(&"error text".into());
                        let rrc = root.unwrap_mut::<RootRenderingComponent>();
                        rrc.game_data.error_text = err_text;
                        v2.schedule_render();
                    }
                })
                .map_err(|_| ()),
            );
        }
    }) as Box<dyn FnMut(ErrorEvent)>);
    ws.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
    onerror_callback.forget();
}
/// on close WebSocket connection
pub fn setup_ws_onclose(ws: &WebSocket, weak: dodrio::VdomWeak) {
    let onclose_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
        let err_text = format!("ws_onclose {:?}", e);
        logmod::log1_str(&err_text);
        {
            wasm_bindgen_futures::spawn_local(
                weak.with_component({
                    let v2 = weak.clone();
                    move |root| {
                        console::log_1(&"spawn_local because of vdom".into());
                        let rrc = root.unwrap_mut::<RootRenderingComponent>();
                        //I want to show a reconnect button to the user
                        rrc.game_data.is_reconnect = true;
                        v2.schedule_render();
                    }
                })
                .map_err(|_| ()),
            );
        }
    }) as Box<dyn FnMut(ErrorEvent)>);
    ws.set_onclose(Some(onclose_callback.as_ref().unchecked_ref()));
    onclose_callback.forget();
}
///setup all ws events
pub fn setup_all_ws_events(ws: &WebSocket, weak: dodrio::VdomWeak) {
    //WebSocket on receive message callback
    setup_ws_msg_recv(ws, weak.clone());

    //WebSocket on error message callback
    setup_ws_onerror(ws, weak.clone());

    //WebSocket on close message callback
    setup_ws_onclose(ws, weak);
}

///generic send ws message
pub fn ws_send_msg(ws: &WebSocket, ws_message: &WsMessage) {
    unwrap!(
        ws.send_with_str(&unwrap!(
            serde_json::to_string(ws_message),
            "error serde_json to_string WsMessage"
        ),),
        "Failed to send"
    );
}
