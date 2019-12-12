// status1stcardmod.rs
//! code flow from this status

//region: use
use crate::gamedatamod::CardStatusCardFace;
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::logmod;
use crate::ackmsgmod;
use crate::divgridcontainermod;
use crate::utilsmod;
use crate::websocketreconnectmod;
use crate::websocketcommunicationmod;

use mem5_common::{GameStatus, WsMessage, MsgAckKind};

use unwrap::unwrap;
use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use typed_html::dodrio;
//endregion

/// on click
pub fn on_click_1st_card(
    rrc: &mut RootRenderingComponent,
    vdom: &dodrio::VdomWeak,
    this_click_card_index: usize,
) {
    logmod::debug_write("on_click_1st_card");
    //change card status and game status
    rrc.game_data.card_index_of_first_click = this_click_card_index;

    let msg_id = ackmsgmod::prepare_for_ack_msg_waiting(rrc, vdom);
    let msg = WsMessage::MsgClick1stCard {
        my_ws_uid: rrc.game_data.my_ws_uid,
        players_ws_uid: rrc.game_data.players_ws_uid.to_string(),
        card_index_of_first_click: this_click_card_index,
        msg_id,
    };
    ackmsgmod::send_msg_and_write_in_queue(rrc, &msg, msg_id);
    divgridcontainermod::play_sound(rrc, this_click_card_index);
    //after ack for this message call on_msg_click_1st_card(rrc, this_click_card_index);
}

///on msg
pub fn on_msg_click_1st_card(
    rrc: &mut RootRenderingComponent,
    msg_sender_ws_uid: usize,
    card_index_of_first_click: usize,
    msg_id: usize,
) {
    logmod::debug_write("on_msg_click_1st_card");
    //it happens that 2 smartphones send the msg simultaneosly.
    //They send it like it is 1st click.
    //Only is status1St is ok to receive 1st click. All else is a conflict.
    if let GameStatus::Status1stCard = rrc.game_data.game_status {
        ackmsgmod::send_ack(rrc, msg_sender_ws_uid, msg_id, MsgAckKind::MsgClick1stCard);
        rrc.game_data.card_index_of_first_click = card_index_of_first_click;
        update_on_1st_card(rrc);
    } else {
        //This is a conflict and only the Player1 must resolve it so to send his data.
        //If this is Player1 than just send GameData
        if rrc.game_data.my_player_number == 1 {
            websocketreconnectmod::send_msg_for_resync(rrc, rrc.game_data.my_ws_uid);
        } else {
            websocketcommunicationmod::ws_send_msg(
                &rrc.game_data.ws,
                &WsMessage::MsgAskPlayer1ForResync {
                    my_ws_uid: rrc.game_data.my_ws_uid,
                    players_ws_uid: unwrap!(serde_json::to_string(&vec![
                        rrc.game_data.players[0].ws_uid
                    ])),
                },
            );
        }
    }
}

///on msg ack
pub fn on_msg_ack_click_1st_card(
    rrc: &mut RootRenderingComponent,
    player_ws_uid: usize,
    msg_id: usize,
) {
    logmod::debug_write("on_msg_ack_click_1st_card");
    if ackmsgmod::remove_ack_msg_from_queue(rrc, player_ws_uid, msg_id) {
        logmod::debug_write("update_on_1st_card (rrc)");
        update_on_1st_card(rrc);
    }
    //TODO: timer if after 3 seconds the ack is not received resend the msg
    //do this 3 times and then hard error
}

///update game data
pub fn update_on_1st_card(rrc: &mut RootRenderingComponent) {
    logmod::debug_write("update_on_1st_card");
    //flip the card up
    unwrap!(rrc
        .game_data
        .card_grid_data
        .get_mut(rrc.game_data.card_index_of_first_click))
    .status = CardStatusCardFace::UpTemporary;
    rrc.game_data.game_status = GameStatus::Status2ndCard;
    rrc.check_invalidate_for_all_components();
}

///render div
#[allow(clippy::integer_arithmetic)]
pub fn div_on_1st_card<'b>(rrc: &RootRenderingComponent, bump: &'b Bump) -> Node<'b> {
    if rrc.game_data.my_player_number == rrc.game_data.player_turn {
        dodrio!(bump,
        <div >
            <h2 class="h2_must_do_something">
                {vec![text(bumpalo::format!(in bump, "Play {} {}",
                unwrap!(rrc.game_data.players.get(rrc.game_data.player_turn-1)).nickname,
                utilsmod::ordinal_numbers(rrc.game_data.player_turn))
                .into_bump_str())]}
            </h2>
        </div>
        )
    } else {
        //return wait for the other player
        dodrio!(bump,
        <h2 class="h2_user_must_wait">
            {vec![text(bumpalo::format!(in bump, "Wait for {} {}",
            unwrap!(rrc.game_data.players.get(rrc.game_data.player_turn-1)).nickname,
            utilsmod::ordinal_numbers(rrc.game_data.player_turn)
            ).into_bump_str())]}
        </h2>
        )
    }
}

//div_grid_container() is in divgridcontainermod.rs
