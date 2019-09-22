//! statusplaybefore1stcard.rs - code flow from this status

//region: use
use crate::gamedata::CardStatusCardFace;
use crate::rootrenderingcomponent::RootRenderingComponent;
use crate::websocketcommunication;
use crate::logmod;
use mem5_common::{GameStatus, WsMessage};

use unwrap::unwrap;
use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use typed_html::dodrio;
//endregion

#[allow(clippy::integer_arithmetic)]
///render Play or Wait
pub fn div_click_1st_card<'a, 'bump>(
    rrc: &'a RootRenderingComponent,
    bump: &'bump Bump,
) -> Node<'bump>
where
    'a: 'bump,
{
    if rrc.game_data.my_player_number == rrc.game_data.player_turn {
        dodrio!(bump,
        <div >
            <h2 id= "ws_elem" style= "color:orange;">
                {vec![text(bumpalo::format!(in bump, "Play {} {} !",
                unwrap!(rrc.game_data.players.get(rrc.game_data.player_turn-1)).nickname,
                crate::ordinal_numbers(rrc.game_data.player_turn))
                .into_bump_str())]}
            </h2>
        </div>
        )
    } else {
        //return wait for the other player
        dodrio!(bump,
        <h2 id="ws_elem" style= "color:red;">
            {vec![text(bumpalo::format!(in bump, "Wait for {} {} !",
            unwrap!(rrc.game_data.players.get(rrc.game_data.player_turn-1)).nickname,
            crate::ordinal_numbers(rrc.game_data.player_turn)
            ).into_bump_str())]}
        </h2>
        )
    }
}

//div_grid_container() is in divgridcontainer.rs

/// on click
pub fn on_click_1st_card(rrc: &mut RootRenderingComponent, this_click_card_index: usize) {
    //change card status and game status
    on_msg_player_click_1st_card(rrc,this_click_card_index);
    //region: send WsMessage over WebSocket
    websocketcommunication::ws_send_msg(
        &rrc.game_data.ws,
        &WsMessage::MsgPlayerClick1stCard {
            my_ws_uid: rrc.game_data.my_ws_uid,
            players_ws_uid: rrc.game_data.players_ws_uid.to_string(),
            card_index_of_first_click: rrc.game_data.card_index_of_first_click,
        },
    );
    //endregion
}

///msg player click
pub fn on_msg_player_click_1st_card(
    rrc: &mut RootRenderingComponent,
    card_index_of_first_click: usize,
) {
    logmod::debug_write("on_msg_player_click_1st_card");
    rrc.game_data.card_index_of_first_click = card_index_of_first_click;
    //flip the card up
    unwrap!(rrc
        .game_data
        .card_grid_data
        .get_mut(rrc.game_data.card_index_of_first_click))
    .status = CardStatusCardFace::UpTemporary;
    rrc.game_data.game_status = GameStatus::StatusPlayBefore2ndCard;
    rrc.check_invalidate_for_all_components();
}
