//! statusinviteasking.rs - code flow from this status

//region: use
use crate::rootrenderingcomponent::RootRenderingComponent;

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

                    unwrap!(rrc
                        .game_data
                        .ws
                        .send_with_str(
                            &serde_json::to_string(&WsMessage::GameDataInit {
        card_grid_data: unwrap!(serde_json::to_string(&rrc.game_data.card_grid_data)
                    ,"serde_json::to_string(&self.game_data.card_grid_data)"),
        players: unwrap!(serde_json::to_string(&rrc.game_data.players)
                    ,"serde_json::to_string(&self.game_data.players)"),
        game_config: unwrap!(serde_json::to_string(&rrc.game_data.game_config)
                    ,"serde_json::to_string(&self.game_data.game_config)"),
                })
                .expect("error sending Invite"),
            )
            ,"Failed to send Invite");

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
