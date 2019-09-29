//! divfullscreenmod.rs - instruction and button for fullscreen

//region: use, const
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::javascriptimportmod;
use crate::logmod;

use unwrap::unwrap;
use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use typed_html::dodrio;
use web_sys;
use conv::*;
//endregion

///information for fullscreen
pub fn div_for_fullscreen<'a>(_rrc: &'a RootRenderingComponent, bump: &'a Bump) -> Node<'a> {
    dodrio!(bump,
    <div >
        {button_for_fullscreen(bump)}
    </div>
    )
}

/*
///detect iphone
fn is_iphone() -> bool {
    let window = unwrap!(web_sys::window());
    let navigator = window.navigator();
    let user_agent = unwrap!(navigator.user_agent());
    user_agent.to_ascii_lowercase().contains("iphone")
}
*/

///render a fullscreen button on android, but not for iphone
fn button_for_fullscreen(bump: &Bump) -> Vec<Node> {
    let mut ret_val = vec![dodrio!(bump,
        <div>
        </div>
    )];

    if !is_fullscreen() {
        ret_val.push(dodrio!(bump,
        <h4>
            {vec![text(bumpalo::format!(in bump,
            "The best user experience of the game is in fullscreen.
            The best options on iPhone and Android is to 'Add to Home Screen' this webapp.{}", "")
            .into_bump_str(),)]}
        </h4>
        ));
    }
    //return
    ret_val
}

///check the fullscreen_element. works only on android
pub fn is_fullscreen() -> bool {
    let window = unwrap!(web_sys::window());
    let screen = unwrap!(window.screen());
    let document = unwrap!(window.document());

    //return
    if document.fullscreen_element().is_some() {
        true
    } else {
        false
    }
}

///render the div for fullscreen
pub fn div_fullscreen<'a, 'bump>(
    _rrc: &'a RootRenderingComponent,
    bump: &'bump Bump,
) -> Vec<Node<'bump>>
where
    'a: 'bump,
{
    let mut ret_val = vec![dodrio!(bump,
        <div>
        </div>
    )];

    if !is_fullscreen() {
        ret_val.push(dodrio!(bump,
        <div class="div_clickable" style="background-color: yellow;" onclick={move |root, vdom, _event| {
            javascriptimportmod::do_fullscreen();
            //async call
            /* TODO:
            let window = unwrap!(web_sys::window());
            let screen = unwrap!(window.screen());
            let document = unwrap!(window.document());
            let element = unwrap!(document.document_element());
            async{
                let promise = element.request_fullscreen();
                let js_fut = JsFuture::from(promise);
                js_fut.await.unwrap();
                vdom.schedule_render();
            }
            */
            }}>
            <h2 id= "ws_elem" style= "color:green;">
                {vec![text(
                        bumpalo::format!(in bump, "Click for FullScreen{}!", "")
                    .into_bump_str(),
                )]}
            </h2>
        </div>
        ));
    }
    //return
    ret_val
}
