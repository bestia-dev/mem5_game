//! ackmsgmod.rs - ack msg

//region: use
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::websocketcommunicationmod;
use mem5_common::{GameStatus, WsMessage};
use crate::gamedatamod;

//use unwrap::unwrap;
use rand::Rng;
use rand::rngs::SmallRng;
use rand::FromEntropy;
//endregion

///remove ack msg from queue - return true if there are no more msgs
pub fn remove_ack_msg_from_queue(rrc: &mut RootRenderingComponent, player_ws_uid:usize, msg_id:usize) -> bool{
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
    //return
    ! has_msg_id
}

///prepare for ack msg waiting - return random msg_id
pub fn prepare_for_ack_msg_waiting(rrc:&mut RootRenderingComponent, vdom:dodrio::VdomWeak)->usize{
let mut rng = SmallRng::from_entropy();
let msg_id =rng.gen_range(1, 4294967295);
rrc.game_data.game_status=GameStatus::StatusWaitingAckMsg;
vdom.schedule_render();
//return
msg_id
}

pub fn send_msg_and_write_in_queue(rrc:&mut RootRenderingComponent,msg:&WsMessage,msg_id:usize){
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
}