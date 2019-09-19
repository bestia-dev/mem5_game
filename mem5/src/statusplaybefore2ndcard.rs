//! statusplaybefore2ndcard.rs - code flow from this status

//region: use
use crate::gamedata::CardStatusCardFace;
use crate::rootrenderingcomponent::RootRenderingComponent;
use crate::websocketcommunication;
use crate::logmod;
use mem5_common::{GameStatus, WsMessage};

use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use typed_html::dodrio;
//endregion

///render Play or Wait
#[allow(clippy::integer_arithmetic)]
pub fn div_click_2nd_card<'a, 'bump>(
    rrc: &'a RootRenderingComponent,
    bump: &'bump Bump,
) -> Node<'bump>
where
    'a: 'bump,
{
    if rrc.game_data.my_player_number
        == rrc.game_data.player_turn
    {
        dodrio!(bump,
        <div >
            <h2 id= "ws_elem" style= "color:orange;">
                {vec![text(bumpalo::format!(in bump, "Play {} (p.{}) !", 
                unwrap!(rrc.game_data.players.get(rrc.game_data.player_turn-1)).nickname,
                rrc.game_data.player_turn
                ).into_bump_str())]}
            </h2>
        </div>
        )
    } else {
        //return wait for the other player
        dodrio!(bump,
        <h2 id="ws_elem" style= "color:red;">
            {vec![text(bumpalo::format!(in bump, "Wait for {} (p.{}) !", 
            unwrap!(rrc.game_data.players.get(rrc.game_data.player_turn-1)).nickname,
            rrc.game_data.player_turn
            ).into_bump_str())]}
        </h2>
        )
    }
}

//div_grid_container() is in divgridcontainer.rs

///on click
pub fn on_click_2nd_card(rrc: &mut RootRenderingComponent, this_click_card_index: usize) {
    logmod::debug_write("on_click_2nd_card");
    rrc.game_data.card_index_of_second_click = this_click_card_index;
    card_click_2nd_card(rrc);
}

///on second click
///The on click event passed by JavaScript executes all the logic
///and changes only the fields of the Card Grid struct.
///That struct is the only permanent data storage for later render the virtual dom.
pub fn card_click_2nd_card(rrc: &mut RootRenderingComponent) {
    //3 possible outcomes: 1) same player, 2) Next Player 3) end game/play again
    //flip the card up
    unwrap!(
        rrc.game_data
            .card_grid_data
            .get_mut(rrc.game_data.card_index_of_second_click),
        "error this_click_card_index"
    )
    .status = CardStatusCardFace::UpTemporary;

    //if the cards match, player get one point and continues another turn
    if unwrap!(
        rrc.game_data
            .card_grid_data
            .get(rrc.game_data.card_index_of_first_click),
        "error game_data.card_index_of_first_click"
    )
    .card_number_and_img_src
        == unwrap!(
            rrc.game_data
                .card_grid_data
                .get(rrc.game_data.card_index_of_second_click),
            "error game_data.card_index_of_second_click"
        )
        .card_number_and_img_src
    {
        //give points
        unwrap!(
            rrc.game_data
                .players
                .get_mut(unwrap!(rrc.game_data.player_turn.checked_sub(1))),
            "rrc.game_data.players.get_mu(rrc.game_data.player_turn - 1)"
        )
        .points += 1;

        // the two cards matches. make them permanent FaceUp
        let x1 = rrc.game_data.card_index_of_first_click;
        let x2 = rrc.game_data.card_index_of_second_click;
        unwrap!(
            rrc.game_data.card_grid_data.get_mut(x1),
            "error game_data.card_index_of_first_click"
        )
        .status = CardStatusCardFace::UpPermanently;
        unwrap!(
            rrc.game_data.card_grid_data.get_mut(x2),
            "error game_data.card_index_of_second_click"
        )
        .status = CardStatusCardFace::UpPermanently;
        //if the sum of points is number of card/2, the game is over
        let mut point_sum = 0;
        for x in &rrc.game_data.players {
            point_sum += x.points;
        }
        logmod::debug_write(
            format!(
                "card_grid len {}  point_sum {}",
                rrc.game_data.card_grid_data.len(),
                point_sum
            )
            .as_str(),
        );
        if unwrap!(rrc.game_data.card_grid_data.len().checked_div(2)) == point_sum {
            //The game is over and the question Play again?
            rrc.game_data.game_status = GameStatus::GameOverPlayAgainBegin;
            //send message
            unwrap!(
                rrc.game_data.ws.send_with_str(
                    &serde_json::to_string(&WsMessage::GameOverPlayAgainBegin {
                        my_ws_uid: rrc.game_data.my_ws_uid,
                        players: unwrap!(serde_json::to_string(&rrc.game_data.players)),
                        card_grid_data: unwrap!(serde_json::to_string(
                            &rrc.game_data.card_grid_data
                        )),
                        game_status: rrc.game_data.game_status.clone(),
                        card_index_of_first_click: rrc.game_data.card_index_of_first_click,
                        card_index_of_second_click: rrc.game_data.card_index_of_second_click,
                    })
                    .expect("error sending GameOverPlayAgainBegin"),
                ),
                "Failed to send GameOverPlayAgainBegin"
            );
        } else {
            //the same player continues to play
            rrc.game_data.game_status = GameStatus::PlayBefore1stCard;
            //region: send WsMessage over WebSocket
            websocketcommunication::ws_send_msg(
                &rrc.game_data.ws,
                &WsMessage::PlayerClick2ndCard {
                    my_ws_uid: rrc.game_data.my_ws_uid,
                    players: unwrap!(serde_json::to_string(&rrc.game_data.players)),
                    card_grid_data: unwrap!(serde_json::to_string(&rrc.game_data.card_grid_data)),
                    game_status: rrc.game_data.game_status.clone(),
                    card_index_of_first_click: rrc.game_data.card_index_of_first_click,
                    card_index_of_second_click: rrc.game_data.card_index_of_second_click,
                },
            );
            //endregion
        }
    } else {
        //if cards don't match
        rrc.game_data.game_status = GameStatus::TakeTurnBegin;
        //region: send WsMessage over WebSocket
        websocketcommunication::ws_send_msg(
            &rrc.game_data.ws,
            &WsMessage::TakeTurnBegin {
                my_ws_uid: rrc.game_data.my_ws_uid,
                players: unwrap!(serde_json::to_string(&rrc.game_data.players)),
                card_grid_data: unwrap!(serde_json::to_string(&rrc.game_data.card_grid_data)),
                game_status: rrc.game_data.game_status.clone(),
                card_index_of_first_click: rrc.game_data.card_index_of_first_click,
                card_index_of_second_click: rrc.game_data.card_index_of_second_click,
            },
        );
        //endregion
    }
    rrc.check_invalidate_for_all_components();
}
///msg player click
pub fn on_msg_player_click_2nd_card(
    rrc: &mut RootRenderingComponent,
    players: &str,
    game_status: GameStatus,
    card_grid_data: &str,
    card_index_of_first_click: usize,
    card_index_of_second_click: usize,
) {
    logmod::debug_write("on_msg_player_click_2nd_card");
    //player point has changed
    rrc.game_data.players = unwrap!(serde_json::from_str(players));
    rrc.game_data.game_status = game_status;
    rrc.game_data.card_grid_data = unwrap!(serde_json::from_str(card_grid_data));
    rrc.game_data.card_index_of_first_click = card_index_of_first_click;
    rrc.game_data.card_index_of_second_click = card_index_of_second_click;
    rrc.check_invalidate_for_all_components();
}

///msg player click
pub fn on_msg_play_again(
    rrc: &mut RootRenderingComponent,
    players: &str,
    game_status: GameStatus,
    card_grid_data: &str,
    card_index_of_first_click: usize,
    card_index_of_second_click: usize,
) {
    logmod::debug_write("on_msg_play_again");
    //player point has changed
    rrc.game_data.players = unwrap!(serde_json::from_str(players));
    rrc.game_data.game_status = game_status;
    rrc.game_data.card_grid_data = unwrap!(serde_json::from_str(card_grid_data));
    rrc.game_data.card_index_of_first_click = card_index_of_first_click;
    rrc.game_data.card_index_of_second_click = card_index_of_second_click;
    rrc.check_invalidate_for_all_components();
}
