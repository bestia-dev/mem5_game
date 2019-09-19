//! divfordebugging.rs - information for debugging

//region: use, const
use crate::rootrenderingcomponent::RootRenderingComponent;
use crate::sessionstoragemod;

use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use typed_html::dodrio;
//endregion

///information for debugging
#[allow(dead_code)]
pub fn div_for_debugging<'a>(_rrc: &'a RootRenderingComponent, bump: &'a Bump) -> Node<'a> {
    //for debugging only
    
    let text2 = bumpalo::format!(in bump, "debug info:\n{}",
    sessionstoragemod::get_debug_text()
    ).into_bump_str();

    dodrio!(bump,
    <div >
        <pre style="color:white;">
            {vec![text(text2)]}
        </pre>
    </div>
    )    
}
