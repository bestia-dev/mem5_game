//! statusinviteaskingmod.rs - code flow from this status

//region: use
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::websocketcommunicationmod;
use crate::logmod;

use unwrap::unwrap;
use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use mem5_common::WsMessage;
use typed_html::dodrio;
//endregion

///render
pub fn div_invite_asking<'a, 'bump>(
    rrc: &'a RootRenderingComponent,
    bump: &'bump Bump,
) -> Node<'bump>
where
    'a: 'bump,
{
    dodrio!(bump,
    <div>
        <div class="div_clickable" onclick={move |root, vdom, _event| {
                    let rrc =
                        root.unwrap_mut::<RootRenderingComponent>();
                    //region: send WsMessage over WebSocket
                    rrc.game_data_init();
                    logmod::debug_write(&format!("MsgGameDataInit send {}",rrc.game_data.players_ws_uid));
                    websocketcommunicationmod::ws_send_msg(
                        &rrc.game_data.ws,
                        &WsMessage::MsgGameDataInit {
                            my_ws_uid: rrc.game_data.my_ws_uid,
                            players_ws_uid: rrc.game_data.players_ws_uid.to_string(),
                            players: unwrap!(serde_json::to_string(&rrc.game_data.players)),
                            card_grid_data: unwrap!(serde_json::to_string(&rrc.game_data.card_grid_data)),
                            game_config: unwrap!(serde_json::to_string(&rrc.game_data.game_config)),
                        },
                    );
        //endregion
        vdom.schedule_render();
        }}>
            <h2 id="ws_elem" style= "color:green;">
                {vec![
                    text(bumpalo::format!(in bump, "Start Game?{}", "").into_bump_str()),
                ]}
            </h2>
        </div>
        <div>
            <h2 style= "color:red;">
                {vec![
                    text(bumpalo::format!(in bump, "Players accepted: {}.", rrc.game_data.players.len()-1).into_bump_str()),
                ]}
            </h2>
        </div>
    </div>
    )
}
