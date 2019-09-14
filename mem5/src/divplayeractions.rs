//! divplayeractions.rs - renders the div to inform player what to do next
//! and get a click action from the user

//region: use
use crate::rootrenderingcomponent::RootRenderingComponent;
use crate::statusplayagain;
use crate::statusplaybefore1stcard;
use crate::statusplaybefore2ndcard;
use crate::statustaketurnbegin;
use crate::statusinviteaskbegin;
use crate::statusinviteasked;
use crate::statusinviteasking;
use crate::websocketreconnect;

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
    if !rrc.game_data.is_status_invite_ask_begin()
        && (rrc.game_data.is_reconnect || rrc.game_data.ws.ready_state() != 1)
    {
        //ready_state: 0	CONNECTING, 1	OPEN, 2	CLOSING, 3	CLOSED
        websocketreconnect::div_reconnect(rrc, bump)
    } else if let GameStatus::InviteAskBegin = rrc.game_data.game_status {
        statusinviteaskbegin::div_invite_ask_begin(rrc, bump)
    } else if let GameStatus::InviteAsked = rrc.game_data.game_status {
        statusinviteasked::div_invite_asked(rrc, bump)
    } else if let GameStatus::InviteAsking = rrc.game_data.game_status {
        statusinviteasking::div_invite_asking(rrc, bump)
    } else if let GameStatus::PlayAccepted = rrc.game_data.game_status {
        statusinviteasked::div_play_accepted(rrc, bump)
    } else if let GameStatus::PlayBefore1stCard = rrc.game_data.game_status {
        statusplaybefore1stcard::div_click_1st_card(rrc, bump)
    } else if let GameStatus::PlayBefore2ndCard = rrc.game_data.game_status {
        statusplaybefore2ndcard::div_click_2nd_card(rrc, bump)
    } else if let GameStatus::TakeTurnBegin = rrc.game_data.game_status {
        statustaketurnbegin::div_take_turn_begin(rrc, bump)
    } else if let GameStatus::GameOverPlayAgainBegin = rrc.game_data.game_status {
        statusplayagain::div_play_again(rrc, bump)
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
    <h2 id= "ws_elem">
        {vec![text(bumpalo::format!(in bump, "gamestatus: {} player {}", rrc.game_data.game_status.as_ref(),rrc.game_data.my_player_number).into_bump_str())]}
    </h2>
    )
}
