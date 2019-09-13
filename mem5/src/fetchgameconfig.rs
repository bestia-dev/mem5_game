//! statusinviteaskbegin.rs - code flow from this status

//region: use
use crate::rootrenderingcomponent::RootRenderingComponent;
use crate::logmod;
use crate::fetchmod;

use web_sys::{Request, RequestInit};
//endregion

///async fetch_response() for gameconfig.json
pub fn fetch_game_config_request(rrc: &mut RootRenderingComponent, vdom_weak: dodrio::VdomWeak) {
    let url_config = format!(
        "{}/content/{}/game_config.json",
        rrc.game_data.href, rrc.game_data.asked_folder_name
    );
    logmod::log1_str(url_config.as_str());
    let webrequest = create_webrequest(url_config.as_str());
    fetchmod::fetch_response(vdom_weak, &webrequest, &set_game_config_from_json);
}

///create web request from string
pub fn create_webrequest(url: &str) -> web_sys::Request {
    let mut opts = RequestInit::new();
    opts.method("GET");

    let w_webrequest = unwrap!(Request::new_with_str_and_init(url, &opts));

    logmod::log1_str("let w_webrequest =");
    //return
    w_webrequest
}

#[allow(clippy::needless_pass_by_value)]
/// update a field in the struct
pub fn set_game_config_from_json(rrc: &mut RootRenderingComponent, respbody: String) {
    //respbody is json.
    logmod::log1_str(format!("respbody {}", respbody).as_str());
    rrc.game_data.game_config = unwrap!(serde_json::from_str(respbody.as_str()));
}
