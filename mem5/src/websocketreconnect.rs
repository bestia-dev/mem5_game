//! websocketreconnect.rs - reconnection for websocket must be part of the application.

//region: use
use crate::rootrenderingcomponent::RootRenderingComponent;
use crate::websocketcommunication;
use crate::logmod;

use unwrap::unwrap;
use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use typed_html::dodrio;

///render reconnect
pub fn div_reconnect<'a, 'bump>(_rrc: &'a RootRenderingComponent, bump: &'bump Bump) -> Node<'bump>
where
    'a: 'bump,
{
    dodrio!(bump,
    <div>
        <div class="div_clickable" onclick={
            move |root, vdom, _event| {
            let rrc = root.unwrap_mut::<RootRenderingComponent>();
            //the old ws and closures are now a memory leak, but small
            let window = unwrap!(web_sys::window(), "error: web_sys::window");
            let href = rrc.game_data.href.clone();
            //usize is Copy(), so I don't need clone()
            let my_ws_uid = rrc.game_data.my_ws_uid;
            logmod::debug_write(&format!(
                "href {}  my_ws_uid {}",
                href,
                my_ws_uid,
            ));
            logmod::debug_write(&"before reconnect");
            let ws = websocketcommunication::setup_ws_connection(href, my_ws_uid);
            websocketcommunication::setup_all_ws_events(&ws,vdom.clone());

            rrc.game_data.ws=ws;
            logmod::debug_write(&"before game_data.is_reconnect = false and schedule_render");
            rrc.game_data.is_reconnect = false;
            vdom.schedule_render();
        }}>
            <h2 id= "ws_elem" style= "color:green;">
                {vec![text(
                //StatusReconnect?
                bumpalo::format!(in bump, "StatusReconnect?{}", "").into_bump_str(),
                )]}
            </h2>
        </div>
        <h2 style= "color:red;">
            {vec![text(
                //connection lost
                bumpalo::format!(in bump, "Connection lost.{}", "").into_bump_str(),
            )]}
        </h2>
    </div>
    )
}
