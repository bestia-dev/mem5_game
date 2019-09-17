//! statusinviteaskbegin.rs - code flow from this status

//region: use
use crate::rootrenderingcomponent::RootRenderingComponent;
use crate::websocketcommunication;
use crate::logmod;
use crate::fetchgameconfig;
use crate::localstoragemod;

use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use mem5_common::{GameStatus, Player, WsMessage};
use typed_html::dodrio;
//endregion

///render invite ask begin, ask to play for multiple contents/folders
pub fn div_invite_ask_begin<'a, 'bump>(
    rrc: &'a RootRenderingComponent,
    bump: &'bump Bump,
) -> Node<'bump>
where
    'a: 'bump,
{
    logmod::log1_str("GameStatus::InviteAskBegin");
    let mut vec_of_nodes = Vec::new();
    //I don't know how to solve the lifetime problems. So I just clone the small data.
    let ff = rrc.game_data.content_folders.clone();
    for folder_name in ff {
        let folder_name_clone2 = folder_name.clone();
        vec_of_nodes.push(dodrio!(bump,
        <div class="div_clickable" onclick={move |root, vdom, _event| {
                let rrc = root.unwrap_mut::<RootRenderingComponent>();
                let v2= vdom.clone();
                div_invite_ask_begin_on_click(rrc, &folder_name,v2);

                vdom.schedule_render();
                }}>
            <h2 id= "ws_elem" style= "color:green;">
                {vec![text(
                //show Ask Player2 to Play!
                bumpalo::format!(in bump, "Invite for {} !", folder_name_clone2)
                    .into_bump_str(),
                )]}
            </h2>
        </div>
        ));
    }
    dodrio!(bump,
    <div>
        {vec_of_nodes}
        {vec![div_nickname_input(rrc,bump)]}
    </div>
    )
}

///render the nickname input
pub fn div_nickname_input<'a, 'bump>(
    rrc: &'a RootRenderingComponent,
    bump: &'bump Bump,
) -> Node<'bump>
where
    'a: 'bump,
{
dodrio!(bump,
<div>
    <p>
        {vec![text(
            bumpalo::format!(in bump, "{}",
            "Save your nickname in the local browser:")
            .into_bump_str()
        )]}
    </p>
    <input id="nickname" name="nickname" value={bumpalo::format!(in bump, "{}",
            rrc.game_data.my_nickname)
            .into_bump_str()
            } >
    </input>
    <a href="#" 
    onclick={move |root, vdom_weak, _event| {
                let v2 = vdom_weak.clone();
                localstoragemod::save_nickname_to_localstorage(&v2);
            }}>
        {vec![text(
            bumpalo::format!(in bump, "{}",
            "save")
            .into_bump_str()
        )]}
    </a>
</div>
)
}

/// on click updates some data and sends msgs
/// msgs will be asynchronously received and processed
pub fn div_invite_ask_begin_on_click(
    rrc: &mut RootRenderingComponent,
    folder_name: &str,
    vdom_weak: dodrio::VdomWeak,
) {
    rrc.game_data.my_player_number = 1;
    rrc.game_data.players.clear();
    rrc.game_data.players.push(Player {
        ws_uid: rrc.game_data.my_ws_uid,
        nickname: rrc.game_data.my_nickname.clone(),
        points: 0,
    });
    rrc.game_data.game_status = GameStatus::InviteAsking;
    rrc.game_data.asked_folder_name = folder_name.to_string();

    //async fetch_response() for gameconfig.json
    fetchgameconfig::fetch_game_config_request(rrc, vdom_weak);
    //send the msg Invite
    websocketcommunication::ws_send_msg(
        &rrc.game_data.ws,
        &WsMessage::Invite {
            my_ws_uid: rrc.game_data.my_ws_uid,
            my_nickname: rrc.game_data.my_nickname.clone(),
            asked_folder_name: folder_name.to_string(),
        },
    );
}

///msg invite
pub fn on_msg_invite(
    rrc: &mut RootRenderingComponent,
    his_ws_uid: usize,
    his_nickname: String,
    asked_folder_name: String,
) {
    logmod::log1_str("rcv invite");
    rrc.reset();
    rrc.game_data.game_status = GameStatus::InviteAsked;
    //the first player is the initiator
    rrc.game_data.players.push(Player {
        ws_uid: his_ws_uid,
        nickname: his_nickname,
        points: 0,
    });
    rrc.game_data.players.push(Player {
        ws_uid: rrc.game_data.my_ws_uid,
        nickname: rrc.game_data.my_nickname.clone(),
        points: 0,
    });
    rrc.game_data.my_player_number = 2; //temporary number
    rrc.game_data.asked_folder_name = asked_folder_name;
}
