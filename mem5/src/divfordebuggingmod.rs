// divfordebuggingmod.rs
//! information for debugging

//region: use, const
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::sessionstoragemod;
use crate::websocketreconnectmod;

use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use typed_html::dodrio;
//endregion

///information for debugging
#[allow(dead_code)]
pub fn div_for_debugging<'a>(rrc: &'a RootRenderingComponent, bump: &'a Bump) -> Node<'a> {
    //for debugging only

    let text2 = bumpalo::format!(in bump, "debug info:\n{}",
    sessionstoragemod::get_debug_text()
    )
    .into_bump_str();

    dodrio!(bump,
    <div >
        <pre style="color: white; white-space: pre-wrap; word-break: break-all;">
            {vec![text(text2)]}
        </pre>
        {vec![websocketreconnectmod::div_reconnect(rrc, bump)]}
    </div>
    )
}
