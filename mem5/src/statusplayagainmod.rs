//! statusplayagainmod.rs - code flow from this status

//region: use
use mem5_common::GameStatus;
use crate::rootrenderingcomponentmod::RootRenderingComponent;

use unwrap::unwrap;
use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use typed_html::dodrio;
//endregion

///play again
pub fn div_play_again<'a, 'bump>(_rrc: &'a RootRenderingComponent, bump: &'bump Bump) -> Node<'bump>
where
    'a: 'bump,
{
    //end game ,Play again?  reload webpage
    dodrio!(bump,
    <div class="div_clickable" onclick={
                move |root, vdom, _event| {
                //reload the webpage
                let window = unwrap!(web_sys::window(), "error: web_sys::window");
                let x = window.location().reload();
            }}>
        <h2 id= "ws_elem" style= "color:green;">
                {vec![text(
                    //Play again?
                    bumpalo::format!(in bump, "Game Over! Play again{}?", "").into_bump_str(),
                )]}
        </h2>
    </div>
    )
}

///msg player click
pub fn on_msg_play_again(rrc: &mut RootRenderingComponent) {
    //logmod::debug_write("on_msg_play_again");
    //The game is over and the question Play again?
    rrc.game_data.game_status = GameStatus::StatusGameOverPlayAgainBegin;
    rrc.check_invalidate_for_all_components();
}
