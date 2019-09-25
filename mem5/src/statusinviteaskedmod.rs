//! statusinviteaskedmod.rs - code flow from this status

//region: use
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::websocketcommunicationmod;
use crate::gamedatamod;

use unwrap::unwrap;
use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use mem5_common::{GameStatus, Player, WsMessage};
use typed_html::dodrio;
//endregion

///render asked
pub fn div_invite_asked<'a, 'bump>(
    rrc: &'a RootRenderingComponent,
    bump: &'bump Bump,
) -> Node<'bump>
where
    'a: 'bump,
{
    //return Click here to Accept play
    dodrio!(bump,
    <div class="div_clickable" onclick={move |root, vdom, _event| {
                let rrc = root.unwrap_mut::<RootRenderingComponent>();
                div_invite_asked_on_click(rrc);
                vdom.schedule_render();
            }}>
        <h2 id= "ws_elem" style= "color:green;">
                {vec![text(
                    //show Ask Player2 to Play!
                    bumpalo::format!(in bump, "{}, click here to Accept {} from {}!",
                    rrc.game_data.my_nickname,
                    rrc.game_data.asked_folder_name,
                    unwrap!(rrc.game_data.players.get(0)).nickname
                    )
                        .into_bump_str(),
                )]}
        </h2>
    </div>
    )
}

/// on click
pub fn div_invite_asked_on_click(rrc: &mut RootRenderingComponent) {
    rrc.game_data.game_status = GameStatus::StatusPlayAccepted;
    //logmod::debug_write(&format!("StatusPlayAccepted send {}",rrc.game_data.players_ws_uid));
    websocketcommunicationmod::ws_send_msg(
        &rrc.game_data.ws,
        &WsMessage::MsgPlayAccept {
            my_ws_uid: rrc.game_data.my_ws_uid,
            players_ws_uid: rrc.game_data.players_ws_uid.to_string(),
            my_nickname: rrc.game_data.my_nickname.clone(),
        },
    );
}

///msg accept play
pub fn on_msg_play_accept(
    rrc: &mut RootRenderingComponent,
    his_ws_uid: usize,
    his_nickname: String,
) {
    //logmod::debug_write(&format!("on_msg_play_accept {}",his_ws_uid));
    if rrc.game_data.my_player_number == 1 {
        rrc.game_data.players.push(Player {
            ws_uid: his_ws_uid,
            nickname: his_nickname,
            points: 0,
        });
        rrc.game_data.players_ws_uid = gamedatamod::prepare_players_ws_uid(&rrc.game_data.players);
        rrc.check_invalidate_for_all_components();
    }
}

///render play accepted
pub fn div_play_accepted<'a, 'bump>(
    rrc: &'a RootRenderingComponent,
    bump: &'bump Bump,
) -> Node<'bump>
where
    'a: 'bump,
{
    dodrio!(bump,
    <h2 id= "ws_elem" style= "color:red;">
        {vec![text(bumpalo::format!(in bump, "Game {} accepted.", rrc.game_data.asked_folder_name).into_bump_str(),)]}
    </h2>
    )
}
