//! **localstoragemod.rs - the player nickname is saved in the LocalStorage of the browser**  

//region: use
use crate::logmod;
use crate::rootrenderingcomponent::RootRenderingComponent;

use unwrap::unwrap;
use wasm_bindgen::JsCast;
use futures::Future;
//endregion

///save nickname from html input elements to local storage and rrc
pub fn save_nickname_to_localstorage(vdom_weak: &dodrio::VdomWeak) {
    let window = unwrap!(web_sys::window(), "window");
    let document = unwrap!(window.document(), "document");

    logmod::debug_write("before get_element_by_id");
    let input_nickname = unwrap!(document.get_element_by_id("nickname"));
    logmod::debug_write("before dyn_into");
    let input_html_element_nickname = unwrap!(
        input_nickname.dyn_into::<web_sys::HtmlInputElement>(),
        "dyn_into"
    );
    logmod::debug_write("before value()");
    let nickname_string = input_html_element_nickname.value();
    logmod::debug_write("before as_str");
    let nickname = nickname_string.as_str();
    logmod::debug_write(nickname);

    let ls = unwrap!(unwrap!(window.local_storage()));
    let _x = ls.set_item("nickname", nickname);

    //To change the data in rrc I must use the future `vdom.with_component`
    //it will be executed at the next tick to avoid concurrent data races.
    wasm_bindgen_futures::spawn_local(
        vdom_weak
            .with_component({
                move |root| {
                    let rrc = root.unwrap_mut::<RootRenderingComponent>();
                    rrc.game_data.my_nickname = nickname_string;
                }
            })
            .map_err(|_| ()),
    );
}

///load nickname from local storage
pub fn load_nickname() -> String {
    let window = unwrap!(web_sys::window(), "window");
    let ls = unwrap!(unwrap!(window.local_storage()));
    let empty1 = "Player".to_string();
    //return nickname
    unwrap!(ls.get_item("nickname")).unwrap_or(empty1)
}