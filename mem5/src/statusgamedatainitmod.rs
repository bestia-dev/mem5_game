// statusgamedatainitmod.rs
//! code flow from this status

//region: use
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::fetchallimgsforcachemod;
use crate::gamedatamod;

use unwrap::unwrap;
use mem5_common::GameStatus;
//endregion

///prepares the game data
pub fn game_data_init(rrc: &mut RootRenderingComponent) {
    rrc.game_data.content_folder_name = rrc.game_data.asked_folder_name.clone();
    rrc.game_data.prepare_random_data();
    rrc.game_data.game_status = GameStatus::StatusPlayBefore1stCard;
    rrc.game_data.player_turn = 1;
}

///on game data init
pub fn on_msg_game_data_init(
    rrc: &mut RootRenderingComponent,
    v2: dodrio::VdomWeak,
    card_grid_data: &str,
    game_config: &str,
    players: &str,
) {
    //logmod::debug_write(&format!("on_msg_game_data_init {}", players));
    rrc.game_data.content_folder_name = rrc.game_data.asked_folder_name.clone();
    rrc.game_data.game_status = GameStatus::StatusPlayBefore1stCard;
    rrc.game_data.player_turn = 1;

    rrc.game_data.game_config = unwrap!(
        serde_json::from_str(game_config),
        "error serde_json::from_str(game_config)"
    );

    rrc.game_data.card_grid_data = unwrap!(
        serde_json::from_str(card_grid_data),
        "error serde_json::from_str(card_grid_data)"
    );

    //async fetch all imgs and put them in service worker cache
    fetchallimgsforcachemod::fetch_all_img_for_cache_request(rrc, v2);

    rrc.game_data.players = unwrap!(
        serde_json::from_str(players),
        "error serde_json::from_str(players)"
    );

    rrc.game_data.players_ws_uid = gamedatamod::prepare_players_ws_uid(&rrc.game_data.players);

    //find my player number
    for index in 0..rrc.game_data.players.len() {
        if unwrap!(
            rrc.game_data.players.get_mut(index),
            "rrc.game_data.players.get_mut(index)"
        )
        .ws_uid
            == rrc.game_data.my_ws_uid
        {
            rrc.game_data.my_player_number = unwrap!(index.checked_add(1));
        }
    }
    rrc.check_invalidate_for_all_components();
}
