//! statustaketurnbeginmod.rs - code flow from this status

//region: use
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::websocketcommunicationmod;
use mem5_common::{GameStatus, WsMessage};
use crate::gamedatamod::{CardStatusCardFace};
use crate::logmod;

use unwrap::unwrap;
use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use typed_html::dodrio;
//endregion

///render take turn
#[allow(clippy::integer_arithmetic)]
pub fn div_take_turn_begin<'a, 'bump>(
    rrc: &'a RootRenderingComponent,
    bump: &'bump Bump,
) -> Node<'bump>
where
    'a: 'bump,
{
    logmod::debug_write(&format!(
        "div_take_turn_begin: player_turn {}  my_player_number {}",
        &rrc.game_data.player_turn, &rrc.game_data.my_player_number
    ));
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
                        logmod::debug_write(&format!("ws_send_msg: MsgTakeTurnEnd {}", ""));

                    websocketcommunicationmod::ws_send_msg(
                        &rrc.game_data.ws,
                        &WsMessage::MsgTakeTurnEnd {
                            my_ws_uid: rrc.game_data.my_ws_uid,
                            players_ws_uid: rrc.game_data.players_ws_uid.to_string(),
                        }
                    );
                    //endregion
                    take_turn_end(rrc);
                    // Finally, re-render the component on the next animation frame.
                    vdom.schedule_render();
                }}>
            <h2 class="h2_user_must_click">
                {vec![text(
                    bumpalo::format!(in bump, "{} {}, click here to take your turn !",
                        unwrap!(rrc.game_data.players.get(rrc.game_data.my_player_number-1)).nickname,
                        crate::ordinal_numbers(rrc.game_data.my_player_number)
                    )
                        .into_bump_str(),
                )]}
            </h2>
        </div>
        )
    } else {
        //return wait for the other player
        dodrio!(bump,
        <h2 class="h2_user_must_wait">
            {vec![text(bumpalo::format!(in bump, "Wait for {} {} !",
            unwrap!(rrc.game_data.players.get(next_player-1)).nickname,
            crate::ordinal_numbers(next_player)
            ).into_bump_str())]}
        </h2>
        )
    }
}

///fn on change for both click and we msg.
pub fn take_turn_end(rrc: &mut RootRenderingComponent) {
    logmod::debug_write(&format!(
        "take_turn_end: player_turn {}  my_player_number {}",
        &rrc.game_data.player_turn, &rrc.game_data.my_player_number
    ));

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
    rrc.game_data.game_status = GameStatus::StatusPlayBefore1stCard;

    rrc.check_invalidate_for_all_components();
}

///on msg take turn begin
pub fn on_msg_take_turn_begin(rrc: &mut RootRenderingComponent, card_index_of_second_click: usize) {
    logmod::debug_write("on_msg_take_turn_begin");
    rrc.game_data.card_index_of_second_click = card_index_of_second_click;
    //flip the card up
    unwrap!(
        rrc.game_data
            .card_grid_data
            .get_mut(rrc.game_data.card_index_of_second_click),
        "error this_click_card_index"
    )
    .status = CardStatusCardFace::UpTemporary;
    rrc.game_data.game_status = GameStatus::StatusTakeTurnBegin;
    rrc.check_invalidate_for_all_components();
}

///msg player change
pub fn on_msg_take_turn_end(rrc: &mut RootRenderingComponent) {
    take_turn_end(rrc);
}
