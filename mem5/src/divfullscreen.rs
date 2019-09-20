//! divfullscreen.rs - instruction and button for fullscreen

//region: use, const
use crate::rootrenderingcomponent::RootRenderingComponent;
use crate::javascriptimportmod;

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
///detect iphone
fn is_iphone() -> bool {
    let window = unwrap!(web_sys::window());
    let navigator = window.navigator();
    let user_agent = unwrap!(navigator.user_agent());
    user_agent.to_ascii_lowercase().contains("iphone")
}

///render a fullscreen button on android, but not for iphone
fn button_for_fullscreen(bump: &Bump) -> Vec<Node> {
    //check the height if is fullscreen (not working now)
    let window = unwrap!(web_sys::window());
    let screen = unwrap!(window.screen());
    let _document = unwrap!(window.document());

    let mut ret_val = vec![dodrio!(bump,
        <div>
        </div>
    )];

    /* for debuggong
    if document.fullscreen_element().is_some() {
        ret_val.push(dodrio!(bump,
        <h6>
            {vec![text(bumpalo::format!(in bump,
            "fullscreen_element is some{}", "")
            .into_bump_str(),)]}
        </h6>
        ));
    } else {
        ret_val.push(dodrio!(bump,
        <h6>
            {vec![text(bumpalo::format!(in bump,
            "fullscreen_element is not some{}", "")
            .into_bump_str(),)]}
        </h6>
        ));
    }
    */

    let jsvalue_inner_height = unwrap!(window.inner_height(), "window.inner_height");

    let f64_inner_height = unwrap!(
        jsvalue_inner_height.as_f64(),
        "jsValue_inner_height.as_string()"
    );
    let usize_inner_height: usize = unwrap!(f64_inner_height.approx());
    let usize_height: usize = unwrap!(unwrap!(screen.height()).approx());

    /* for debugging
    ret_val.push(dodrio!(bump,
        <h6>
            {vec![text(bumpalo::format!(in bump,
            "usize_inner_height={},  usize_height={}", usize_inner_height,usize_height)
            .into_bump_str(),)]}
        </h6>
    ));
    */

    if usize_inner_height == usize_height {
        // browser is already fullscreen
        ret_val.push(dodrio!(bump,
        <h6>
            {vec![text(bumpalo::format!(in bump,
            "browser is already fullscreen{}", "")
            .into_bump_str(),)]}
        </h6>
        ));
    } else {
        ret_val.push(dodrio!(bump,
        <h4>
            {vec![text(bumpalo::format!(in bump,
            "The best user experience of the game is in fullscreen.
            The best options on iPhone and Android is to 'Add to Home Screen' this webapp.{}", "")
            .into_bump_str(),)]}
        </h4>
        ));

        if is_iphone() {
            //iPhone safari cannot go fullscreen in browser, only as Add to HomeScreen
            ret_val.push(dodrio!(bump,
                <h6>
                    {vec![text(bumpalo::format!(in bump,
                    "iPhone detected.{}", "")
                    .into_bump_str(),)]}
                </h6>
            ));
        } else {
            //probably android
            ret_val.push(dodrio!(bump,
                <h6>
                    {vec![text(bumpalo::format!(in bump,
                    "On android you can simply click the button FullScreen.{}", "")
                    .into_bump_str(),)]}
                </h6>
            ));
            ret_val.push(
            dodrio!(bump,
                <button id="view-fullscreen" style= "margin:auto;display:block;" onclick={move |root, vdom, _event| {
                    javascriptimportmod::do_fullscreen();
                    vdom.schedule_render();
                    }}>
                    "Fullscreen"
                </button>
            ));
        }
    }
    //return
    ret_val
}
