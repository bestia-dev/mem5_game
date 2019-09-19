//! divfordebugging.rs - information for debugging

//region: use, const
use crate::rootrenderingcomponent::RootRenderingComponent;

use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use typed_html::dodrio;
//endregion

///information for debugging
#[allow(dead_code)]
pub fn div_for_debugging<'a>(rrc: &'a RootRenderingComponent, bump: &'a Bump) -> Node<'a> {
    //for debugging only
    
    let text2 = bumpalo::format!(in bump, "debug: status: {}, ws_uid: {} times : {}",
    rrc.game_data.game_status,
    rrc.game_data.my_ws_uid,
    rrc.debug_text
    ).into_bump_str();

    dodrio!(bump,
    <div >
        <h4>
            {vec![text(text2)]}
        </h4>
    </div>
    )
    
}
