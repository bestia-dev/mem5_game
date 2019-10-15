//! divplayeractionsmod.rs - renders the div to inform player what to do next
//! and get a click action from the user

//region: use
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::statusplayagainmod;
use crate::statusplaybefore1stcardmod;
use crate::statusplaybefore2ndcardmod;
use crate::statustaketurnbeginmod;
use crate::statusinviteaskbeginmod;
use crate::statusinviteaskedmod;
use crate::statusinviteaskingmod;
use crate::statuswaitingackmsgmod;
//use crate::websocketreconnectmod;

use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use mem5_common::{GameStatus};
use typed_html::dodrio;
//endregion

///render html element to inform player what to do and get a click action from user
pub fn div_player_actions_from_game_status<'a, 'bump>(
    rrc: &'a RootRenderingComponent,
    bump: &'bump Bump,
) -> Node<'bump>
where
    'a: 'bump,
{
    
    //if rrc.game_data.is_status_invite_ask_begin() {
    /*
        && (rrc.game_data.is_reconnect || rrc.game_data.ws.ready_state() != 1)
    {
        //ready_state: 0	CONNECTING, 1	OPEN, 2	CLOSING, 3	CLOSED
        websocketreconnectmod::div_reconnect(rrc, bump)
    */
     if let GameStatus::StatusInviteAskBegin = rrc.game_data.game_status {
        statusinviteaskbeginmod::div_invite_ask_begin(rrc, bump)
    } else if let GameStatus::StatusInviteAsked = rrc.game_data.game_status {
        statusinviteaskedmod::div_invite_asked(rrc, bump)
    } else if let GameStatus::StatusInviteAsking = rrc.game_data.game_status {
        statusinviteaskingmod::div_invite_asking(rrc, bump)
    } else if let GameStatus::StatusPlayAccepted = rrc.game_data.game_status {
        statusinviteaskedmod::div_play_accepted(rrc, bump)
    } else if let GameStatus::StatusPlayBefore1stCard = rrc.game_data.game_status {
        statusplaybefore1stcardmod::div_click_1st_card(rrc, bump)
    } else if let GameStatus::StatusPlayBefore2ndCard = rrc.game_data.game_status {
        statusplaybefore2ndcardmod::div_click_2nd_card(rrc, bump)
    } else if let GameStatus::StatusTakeTurnBegin = rrc.game_data.game_status {
        statustaketurnbeginmod::div_take_turn_begin(rrc, bump)
    } else if let GameStatus::StatusGameOverPlayAgainBegin = rrc.game_data.game_status {
        statusplayagainmod::div_play_again(rrc, bump)
    } else if let GameStatus::StatusWaitingAckMsg = rrc.game_data.game_status {
        statuswaitingackmsgmod::div_waiting_ack_msg(rrc, bump)
    } else {
        div_unpredicted(rrc, bump)
    }
}

///render unpredicted
fn div_unpredicted<'a, 'bump>(
    rrc: &'a RootRenderingComponent,
    bump: &'bump Bump,
) -> Node<'bump>
where
    'a: 'bump,
{
    //unpredictable situation
    //return
    dodrio!(bump,
    <h2  >
        {vec![text(bumpalo::format!(in bump, "gamestatus: {} player {}", rrc.game_data.game_status.as_ref(),rrc.game_data.my_player_number).into_bump_str())]}
    </h2>
    )
}
