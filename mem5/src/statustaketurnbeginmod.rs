//! statustaketurnbeginmod.rs - code flow from this status

//region: use
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::websocketcommunicationmod;
use mem5_common::{GameStatus, WsMessage};
use crate::gamedatamod::{CardStatusCardFace};
use crate::logmod;
use crate::gamedatamod;

use unwrap::unwrap;
use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use typed_html::dodrio;
use rand::Rng;
use rand::rngs::SmallRng;
use rand::FromEntropy;
//endregion

///render take turn
#[allow(clippy::integer_arithmetic)]
pub fn div_take_turn_begin<'a, 'bump>(
    rrc: &'a RootRenderingComponent,
    bump: &'bump Bump,
) -> Node<'bump>
where
    'a: 'bump,
{
    logmod::debug_write(&format!(
        "div_take_turn_begin: player_turn {}  my_player_number {}",
        &rrc.game_data.player_turn, &rrc.game_data.my_player_number
    ));
    let next_player = if rrc.game_data.player_turn < rrc.game_data.players.len() {
        unwrap!(rrc.game_data.player_turn.checked_add(1))
    } else {
        1
    };
    if rrc.game_data.my_player_number == next_player {
        dodrio!(bump,
        <div class="div_clickable" onclick={move |root, vdom, _event| {
                    let rrc = root.unwrap_mut::<RootRenderingComponent>();
                    //this game_data mutable reference is dropped on the end of the function
                    //region: send WsMessage over WebSocket
                    logmod::debug_write(&format!("ws_send_msg: MsgTakeTurnEnd {}", ""));
                    rrc.game_data.game_status=GameStatus::StatusWaitingAckMsg;
                     vdom.schedule_render();
                     
                    let mut rng = SmallRng::from_entropy();
                    let msg_id =rng.gen_range(1, 4294967295);
                    let msg = WsMessage::MsgTakeTurnEnd {
                        my_ws_uid: rrc.game_data.my_ws_uid,
                        players_ws_uid: rrc.game_data.players_ws_uid.to_string(),
                        msg_id,
                    };
                    websocketcommunicationmod::ws_send_msg(&rrc.game_data.ws,&msg);
                    //write the msgs in the queue
                    for player in rrc.game_data.players.iter(){
                        if player.ws_uid != rrc.game_data.my_ws_uid{
                            let msg_for_loop = msg.clone();
                            rrc.game_data.msgs_waiting_ack.push(
                                gamedatamod::MsgInQueue{
                                    player_ws_uid: player.ws_uid,
                                    msg_id,
                                    msg: msg_for_loop,
                                }
                            );
                        }
                    }
                    //endregion
                    //Here I wait for on_MsgAckTakeTurnEnd from 
                    //every player before call take_turn_end(rrc);
                }}>
            <h2 class="h2_user_must_click">
                {vec![text(
                    bumpalo::format!(in bump, "{} {}, click here to take your turn !",
                        unwrap!(rrc.game_data.players.get(rrc.game_data.my_player_number-1)).nickname,
                        crate::ordinal_numbers(rrc.game_data.my_player_number)
                    )
                        .into_bump_str(),
                )]}
            </h2>
        </div>
        )
    } else {
        //return wait for the other player
        dodrio!(bump,
        <h2 class="h2_user_must_wait">
            {vec![text(bumpalo::format!(in bump, "Wait for {} {} !",
            unwrap!(rrc.game_data.players.get(next_player-1)).nickname,
            crate::ordinal_numbers(next_player)
            ).into_bump_str())]}
        </h2>
        )
    }
}

///fn on change for both click and we msg.
pub fn take_turn_end(rrc: &mut RootRenderingComponent) {
    logmod::debug_write(&format!(
        "take_turn_end: player_turn {}  my_player_number {}",
        &rrc.game_data.player_turn, &rrc.game_data.my_player_number
    ));

    rrc.game_data.player_turn = if rrc.game_data.player_turn < rrc.game_data.players.len() {
        unwrap!(rrc.game_data.player_turn.checked_add(1))
    } else {
        1
    };

    //click on Change button closes first and second card
    let x1 = rrc.game_data.card_index_of_first_click;
    let x2 = rrc.game_data.card_index_of_second_click;
    unwrap!(
        rrc.game_data.card_grid_data.get_mut(x1),
        "error game_data.card_index_of_first_click "
    )
    .status = CardStatusCardFace::Down;
    unwrap!(
        rrc.game_data.card_grid_data.get_mut(x2),
        "error game_data.card_index_of_second_click"
    )
    .status = CardStatusCardFace::Down;
    rrc.game_data.card_index_of_first_click = 0;
    rrc.game_data.card_index_of_second_click = 0;
    rrc.game_data.game_status = GameStatus::StatusPlayBefore1stCard;

    rrc.check_invalidate_for_all_components();
}

///on msg take turn begin
pub fn on_msg_take_turn_begin(rrc: &mut RootRenderingComponent, card_index_of_second_click: usize) {
    logmod::debug_write("on_msg_take_turn_begin");
    rrc.game_data.card_index_of_second_click = card_index_of_second_click;
    //flip the card up
    unwrap!(
        rrc.game_data
            .card_grid_data
            .get_mut(rrc.game_data.card_index_of_second_click),
        "error this_click_card_index"
    )
    .status = CardStatusCardFace::UpTemporary;
    rrc.game_data.game_status = GameStatus::StatusTakeTurnBegin;
    rrc.check_invalidate_for_all_components();
}

///msg player change
pub fn on_msg_take_turn_end(rrc: &mut RootRenderingComponent,msg_sender_ws_uid:usize, msg_id:usize) {
    //send back the ACK msg to the sender
     websocketcommunicationmod::ws_send_msg(
        &rrc.game_data.ws,
        &WsMessage::MsgAckTakeTurnEnd {
            my_ws_uid: rrc.game_data.my_ws_uid,
            players_ws_uid: serde_json::to_string(&vec![msg_sender_ws_uid]).unwrap(),
            msg_id
        }
    );

    take_turn_end(rrc);
}


///all the players must acknowledge that they received the msg
#[allow(clippy::needless_pass_by_value)]
pub fn on_msg_ack_take_turn_end(
    rrc: &mut RootRenderingComponent,
    player_ws_uid: usize,
    msg_id: usize
) {
    //remove the waiting msg from the queue
    //I use the oposite method "retain" because there is not a method "remove"
    rrc.game_data.msgs_waiting_ack.retain(|x| !(x.player_ws_uid == player_ws_uid && x.msg_id==msg_id));

    //if there is no more items with this msg_id, then proceed
    let mut has_msg_id = false;
    for x in rrc.game_data.msgs_waiting_ack.iter(){
        if x.msg_id==msg_id {
            has_msg_id=true;
            break;
        }
    }
    if !has_msg_id{
        logmod::debug_write("take_turn_end(rrc)");
        take_turn_end(rrc);
    }
    //TODO: timer if after 3 seconds the ack is not received resend the msg
    //do this 3 times and then hard error
}
