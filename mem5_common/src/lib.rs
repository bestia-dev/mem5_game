//region: lmake_readme insert "readme.md"
//! # mem5_common
//! 
//! version: 19.10.18-17.47  
//! 
//! **commons for mem5 wasm and server**  
//! Learning to code Rust for a http + WebSocket.  
//! Here are just the structures, that are in common between frontend and backend.  
//! Mostly because of the Messages.  

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
    clippy::missing_inline_in_public_items,
    //Why is this bad : Doc is good. rustc has a MISSING_DOCS allowed-by-default lint for public members, but has no way to enforce documentation of private items. This lint fixes that.
    clippy::doc_markdown,
)]
//endregion

//region: use statements
use strum_macros::{Display, AsRefStr};
use serde_derive::{Serialize, Deserialize};
//endregion

///`WsMessage` enum for WebSocket
#[allow(clippy::pub_enum_variant_names)]
#[derive(Serialize, Deserialize, Clone)]
pub enum WsMessage {
    ///MsgDummy
    MsgDummy {
        ///anything
        dummy: String,
    },
    ///Request WebSocket Uid - first message to WebSocket server
    MsgRequestWsUid {
        ///ws client instance unique id. To not listen the echo to yourself.
        my_ws_uid: usize,
        ///json of vector of players for the server to know whom to send msg
        players_ws_uid: String,
    },
    ///response from WebSocket server for first message
    MsgResponseWsUid {
        ///WebSocket Uid
        your_ws_uid: usize,
        ///server version
        server_version: String,
    },
    ///invite
    MsgInvite {
        ///ws client instance unique id. To not listen the echo to yourself.
        my_ws_uid: usize,
        /// my nickname
        my_nickname: String,
        ///content folder name
        asked_folder_name: String,
    },
    /// accept play
    MsgPlayAccept {
        ///ws client instance unique id. To not listen the echo to yourself.
        my_ws_uid: usize,
        ///json of vector of players for the server to know whom to send msg
        players_ws_uid: String,
        /// my nickname
        my_nickname: String,
    },
    /// player1 initialize the game data and sends it to all players
    /// I will send json string to not confuse the server with vectors
    MsgGameDataInit {
        ///ws client instance unique id. To not listen the echo to yourself.
        my_ws_uid: usize,
        ///json of vector of players for the server to know whom to send msg
        players_ws_uid: String,
        ///json of vector of players with nicknames and order data
        players: String,
        ///vector of cards status
        card_grid_data: String,
        ///json of game_config
        game_config: String,
    },
    ///player click
    MsgPlayerClick1stCard {
        ///this identifies the smartphone, but not the player-in-turn
        my_ws_uid: usize,
        ///all players for the server to know whom to send msg
        players_ws_uid: String,
        ///have to send all the state of the game
        card_index_of_first_click: usize,
        ///msg id (random)
        msg_id: usize,
    },
    ///player click success
    MsgPlayerClick2ndCardPoint {
        ///this identifies the smartphone, but not the player-in-turn
        my_ws_uid: usize,
        ///all players for the server to know whom to send msg
        players_ws_uid: String,
        ///have to send all the state of the game
        card_index_of_second_click: usize,
        ///msg id (random)
        msg_id: usize,
    },
    ///take turn begin
    MsgPlayerClick2ndCardTakeTurnBegin {
        ///this identifies the smartphone, but not the player-in-turn
        my_ws_uid: usize,
        ///all players for the server to know whom to send msg
        players_ws_uid: String,
        ///have to send all the state of the game
        card_index_of_second_click: usize,
        ///msg id (random)
        msg_id: usize,
    },
    ///Play Again
    MsgPlayerClick2ndCardGameOverPlayAgainBegin {
        ///this identifies the smartphone, but not the player-in-turn
        my_ws_uid: usize,
        ///all players for the server to know whom to send msg
        players_ws_uid: String,
    },
    ///player change
    MsgTakeTurnEnd {
        ///ws client instance unique id. To not listen the echo to yourself.
        my_ws_uid: usize,
        ///all players for the server to know whom to send msg
        players_ws_uid: String,
        ///msg id (random)
        msg_id: usize,
    },
    ///all game data
    MsgAllGameData {
        ///ws client instance unique id. To not listen the echo to yourself.
        my_ws_uid: usize,
        ///only the players that recconected
        players_ws_uid: String,
        ///json of vector of players with nicknames and order data
        players: String,
        ///vector of cards status
        card_grid_data: String,
        ///have to send all the state of the game
        card_index_of_first_click: usize,
        ///have to send all the state of the game
        card_index_of_second_click: usize,
        ///player turn
        player_turn: usize,
        ///game status
        game_status: GameStatus,
    },
    ///acknowledge msg, that the receiver received the message
    MsgAck {
        ///msg sender uid
        my_ws_uid: usize,
        ///send msg to this players
        players_ws_uid: String,
        ///msg id (random)
        msg_id: usize,
        ///kind of ack msg
        msg_ack_kind: MsgAckKind,
    },
}

///the game can be in various statuses and that differentiate the UI and actions
/// all players have the same game status
#[derive(Display, AsRefStr, Serialize, Deserialize, Clone)]
#[allow(clippy::pub_enum_variant_names)]
pub enum GameStatus {
    /// invite ask begin
    StatusInviteAskBegin,
    ///Player1 MsgInvite Asking
    StatusInviteAsking,
    ///Player2 MsgInvite Asked
    StatusInviteAsked,
    ///StatusPlayAccepted
    StatusPlayAccepted,
    ///Play before first card
    StatusPlayBefore1stCard,
    ///Play before second card
    StatusPlayBefore2ndCard,
    ///take turn begin
    StatusTakeTurnBegin,
    ///take turn end
    StatusTakeTurnEnd,
    ///end game
    StatusGameOverPlayAgainBegin,
    ///StatusReconnect after a lost connection
    StatusReconnect,
    ///waiting ack msg
    StatusWaitingAckMsg,
}

///data for one player
#[derive(Serialize, Deserialize)]
pub struct Player {
    ///ws_uid
    pub ws_uid: usize,
    ///nickname
    pub nickname: String,
    ///field for src attribute for HTML element image and filename of card image
    pub points: usize,
}

#[derive(Display, AsRefStr, Serialize, Deserialize, Clone)]
#[allow(clippy::pub_enum_variant_names)]
///msg ack kind
pub enum MsgAckKind {
    ///ack for MsgTakeTurnEnd
    MsgTakeTurnEnd,
    ///ack for MsgPlayerClick1stCard
    MsgPlayerClick1stCard,
    ///ack for MsgPlayerClick2ndCardPoint
    MsgPlayerClick2ndCardPoint,
    ///
    MsgPlayerClick2ndCardTakeTurnBegin,
}

//endregion
