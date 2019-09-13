//! statustaketurnbegin.rs - code flow from this status

//region: use
use crate::rootrenderingcomponent::RootRenderingComponent;
//use crate::websocketcommunication;
use mem5_common::{GameStatus, WsMessage};
use crate::gamedata::{CardStatusCardFace};
use crate::logmod;

use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use typed_html::dodrio;
//endregion

///render take turn
pub fn div_take_turn_begin<'a, 'bump>(
    rrc: &'a RootRenderingComponent,
    bump: &'bump Bump,
) -> Node<'bump>
where
    'a: 'bump,
{
    logmod::log1_str(&format!(
        "my_player_number {}",
        &rrc.game_data.my_player_number
    ));
    logmod::log1_str(&format!("player_turn {}", &rrc.game_data.player_turn));
    let next_player = if rrc.game_data.player_turn < rrc.game_data.players.len() {
        unwrap!(rrc.game_data.player_turn.checked_add(1))
    } else {
        1
    };
    if rrc.game_data.my_player_number == next_player {
        dodrio!(bump,
        <div class="div_clickable" onclick={move |root, vdom, _event| {
                    let rrc =
                        root.unwrap_mut::<RootRenderingComponent>();
                    //this game_data mutable reference is dropped on the end of the function
                    //region: send WsMessage over WebSocket
                    unwrap!(rrc
                        .game_data
                        .ws
                        .send_with_str(
                            &serde_json::to_string(&WsMessage::TakeTurnEnd {
                                my_ws_uid: rrc.game_data.my_ws_uid,
                                players: unwrap!(serde_json::to_string(
                                    &rrc.game_data.players,
                                )
                                ,"serde_json::to_string(&rrc.game_data.players)"),
                            })
                            .expect("error sending TakeTurnEnd"),
                        )
                        ,"Failed to send TakeTurnEnd");
                    //endregion
                    take_turn_end(rrc);
                    // Finally, re-render the component on the next animation frame.
                    vdom.schedule_render();
                }}>
            <h2 id= "ws_elem" style= "color:green;">
                {vec![text(
                    bumpalo::format!(in bump, "Click here to take your turn !{}", "")
                        .into_bump_str(),
                )]}
            </h2>
        </div>
        )
    } else {
        //return wait for the other player
        dodrio!(bump,
        <h2 id="ws_elem" style= "color:red;">
            {vec![text(bumpalo::format!(in bump, "Wait for player{} !", next_player).into_bump_str())]}
        </h2>
        )
    }
}

///fn on change for both click and we msg.
pub fn take_turn_end(rrc: &mut RootRenderingComponent) {
    rrc.game_data.player_turn = if rrc.game_data.player_turn < rrc.game_data.players.len() {
        unwrap!(rrc.game_data.player_turn.checked_add(1))
    } else {
        1
    };

    //click on Change button closes first and second card
    let x1 = rrc.game_data.card_index_of_first_click;
    let x2 = rrc.game_data.card_index_of_second_click;
    unwrap!(
        rrc.game_data.card_grid_data.get_mut(x1),
        "error game_data.card_index_of_first_click "
    )
    .status = CardStatusCardFace::Down;
    unwrap!(
        rrc.game_data.card_grid_data.get_mut(x2),
        "error game_data.card_index_of_second_click"
    )
    .status = CardStatusCardFace::Down;
    rrc.game_data.card_index_of_first_click = 0;
    rrc.game_data.card_index_of_second_click = 0;
    rrc.game_data.game_status = GameStatus::PlayBefore1stCard;

    rrc.check_invalidate_for_all_components();
}

///on msg take turn begin
pub fn on_msg_take_turn_begin(
    rrc: &mut RootRenderingComponent,
    game_status: GameStatus,
    card_grid_data: &str,
    card_index_of_first_click: usize,
    card_index_of_second_click: usize,
) {
    logmod::log1_str("on_msg_take_turn_begin");
    rrc.game_data.game_status = game_status;
    rrc.game_data.card_grid_data = unwrap!(serde_json::from_str(card_grid_data));
    rrc.game_data.card_index_of_first_click = card_index_of_first_click;
    rrc.game_data.card_index_of_second_click = card_index_of_second_click;
    rrc.check_invalidate_for_all_components();
}

///msg player change
pub fn on_msg_take_turn_end(rrc: &mut RootRenderingComponent) {
    take_turn_end(rrc);
}
