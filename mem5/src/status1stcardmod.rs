// status1stcardmod.rs
//! code flow from this status

//region: use
use crate::gamedatamod::CardStatusCardFace;
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::logmod;
use crate::ackmsgmod;
use crate::divgridcontainermod;
use crate::utilsmod;
use crate::status2ndcardmod;

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
    //it happens that 2 smartphones send the msg simultaneosly.
    //They send it like it is 1st click.
    //If one receives the 1st click in the status 2nd click it is an exception
    if let GameStatus::Status2ndCard = rrc.game_data.game_status {
        //I should return ack message with ah error.
        // the original sender should than execute the code for 2nd click
        ackmsgmod::send_ack_with_error(
            rrc,
            msg_sender_ws_uid,
            msg_id,
            MsgAckKind::MsgClick1stCard,
            format!("Err:resend as 2nd click: {}", card_index_of_first_click),
        );
    } else {
        ackmsgmod::send_ack(rrc, msg_sender_ws_uid, msg_id, MsgAckKind::MsgClick1stCard);
        rrc.game_data.card_index_of_first_click = card_index_of_first_click;
        update_on_1st_card(rrc);
    }
}

///on msg ack
pub fn on_msg_ack_click_1st_card(
    rrc: &mut RootRenderingComponent,
    player_ws_uid: usize,
    msg_id: usize,
) {
    if ackmsgmod::remove_ack_msg_from_queue(rrc, player_ws_uid, msg_id) {
        logmod::debug_write("update_on_1st_card (rrc)");
        update_on_1st_card(rrc);
    }
    //TODO: timer if after 3 seconds the ack is not received resend the msg
    //do this 3 times and then hard error
}

///on msg ack with error
pub fn on_msg_ack_err_click_1st_card(
    rrc: &mut RootRenderingComponent,
    _player_ws_uid: usize,
    msg_id: usize,
    err_msg: String,
    vdom: &dodrio::VdomWeak,
) {
    let str_err_begin = "Err:resend as 2nd click:";
    let len_str_err_begin = str_err_begin.len();
    if &err_msg[..len_str_err_begin] == str_err_begin {
        logmod::debug_write(str_err_begin);
        let str_this_click = &err_msg[(len_str_err_begin + 1)..];
        let usize_this_click: usize = unwrap!(str_this_click.parse());
        logmod::debug_write(&format!("usize_click {}", usize_this_click));
        //remove all the waiting msgs from the queue because they are wrong
        //I use the oposite method "retain" because there is not a method "remove"
        rrc.game_data
            .msgs_waiting_ack
            .retain(|x| !(x.msg_id == msg_id));
        //begin the complete procedure for 2nd click
        status2ndcardmod::on_click_2nd_card(rrc, &vdom, usize_this_click);
    } else {
        logmod::debug_write("the slice is not equal");
        logmod::debug_write(&err_msg[..len_str_err_begin]);
    }
}

///update game data
pub fn update_on_1st_card(rrc: &mut RootRenderingComponent) {
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
