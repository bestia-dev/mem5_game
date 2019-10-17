//! statusplaybefore1stcardmod.rs - code flow from this status

//region: use
use crate::gamedatamod::CardStatusCardFace;
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::websocketcommunicationmod;
use mem5_common::{GameStatus, WsMessage};
use crate::gamedatamod;
use crate::logmod;

use unwrap::unwrap;
use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use typed_html::dodrio;
use rand::Rng;
use rand::rngs::SmallRng;
use rand::FromEntropy;
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
            <h2 class="h2_must_do_something">
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
        <h2 class="h2_user_must_wait">
            {vec![text(bumpalo::format!(in bump, "Wait for {} {} !",
            unwrap!(rrc.game_data.players.get(rrc.game_data.player_turn-1)).nickname,
            crate::ordinal_numbers(rrc.game_data.player_turn)
            ).into_bump_str())]}
        </h2>
        )
    }
}

//div_grid_container() is in divgridcontainermod.rs

/// on click
pub fn on_click_1st_card(rrc: &mut RootRenderingComponent, this_click_card_index: usize) {
    //change card status and game status
    rrc.game_data.card_index_of_first_click = this_click_card_index;

    let mut rng = SmallRng::from_entropy();
    let msg_id = rng.gen_range(1, 4294967295);
    let msg = WsMessage::MsgPlayerClick1stCard {
        my_ws_uid: rrc.game_data.my_ws_uid,
        players_ws_uid: rrc.game_data.players_ws_uid.to_string(),
        card_index_of_first_click: this_click_card_index,
        msg_id,
    };

    //region: send WsMessage over WebSocket
    rrc.game_data.game_status = GameStatus::StatusWaitingAckMsg;
    websocketcommunicationmod::ws_send_msg(&rrc.game_data.ws, &msg);
    //write the msgs in the queue
    for player in rrc.game_data.players.iter() {
        if player.ws_uid != rrc.game_data.my_ws_uid {
            let msg_for_loop = msg.clone();
            rrc.game_data
                .msgs_waiting_ack
                .push(gamedatamod::MsgInQueue {
                    player_ws_uid: player.ws_uid,
                    msg_id,
                    msg: msg_for_loop,
                });
        }
    }
    //after ack for this message call on_msg_player_click_1st_card(rrc, this_click_card_index);

    //endregion
}

///msg player click
pub fn on_msg_player_click_1st_card(
    rrc: &mut RootRenderingComponent,
    msg_sender_ws_uid:usize,
    card_index_of_first_click: usize,
    msg_id: usize,
) {
    //logmod::debug_write("on_msg_player_click_1st_card");
    //send ack
    //send back the ACK msg to the sender
    websocketcommunicationmod::ws_send_msg(
        &rrc.game_data.ws,
        &WsMessage::MsgAckPlayerClick1stCard
         {
            my_ws_uid: rrc.game_data.my_ws_uid,
            players_ws_uid: unwrap!(serde_json::to_string(&vec![msg_sender_ws_uid])),
            msg_id,
        },
    );

    rrc.game_data.card_index_of_first_click = card_index_of_first_click;
    update(rrc);
}

pub fn update(rrc: &mut RootRenderingComponent) {
    //flip the card up
    unwrap!(rrc
        .game_data
        .card_grid_data
        .get_mut(rrc.game_data.card_index_of_first_click))
    .status = CardStatusCardFace::UpTemporary;
    rrc.game_data.game_status = GameStatus::StatusPlayBefore2ndCard;
    rrc.check_invalidate_for_all_components();
}

pub fn on_msg_ack_player_click1st_card(
    rrc: &mut RootRenderingComponent,
    player_ws_uid: usize,
    msg_id: usize,
) {
    //remove the waiting msg from the queue
    //I use the oposite method "retain" because there is not a method "remove"
    rrc.game_data
        .msgs_waiting_ack
        .retain(|x| !(x.player_ws_uid == player_ws_uid && x.msg_id == msg_id));

    //if there is no more items with this msg_id, then proceed
    let mut has_msg_id = false;
    for x in &rrc.game_data.msgs_waiting_ack {
        if x.msg_id == msg_id {
            has_msg_id = true;
            break;
        }
    }
    if !has_msg_id {
        logmod::debug_write("update player_click_1st_card(rrc)");
        update(rrc);
    }
    //TODO: timer if after 3 seconds the ack is not received resend the msg
    //do this 3 times and then hard error
}
