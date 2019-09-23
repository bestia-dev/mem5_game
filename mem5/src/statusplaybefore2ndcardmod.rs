//! statusplaybefore2ndcardmod.rs - code flow from this status

//region: use
use crate::gamedatamod::CardStatusCardFace;
use crate::rootrenderingcomponentmod::RootRenderingComponent;
use crate::websocketcommunicationmod;
use crate::statustaketurnbeginmod;
use crate::statusplayagainmod;

use mem5_common::{GameStatus, WsMessage};

use unwrap::unwrap;
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
    if rrc.game_data.my_player_number == rrc.game_data.player_turn {
        dodrio!(bump,
        <div >
            <h2 id= "ws_elem" style= "color:orange;">
                {vec![text(bumpalo::format!(in bump, "Play {} {} !",
                unwrap!(rrc.game_data.players.get(rrc.game_data.player_turn-1)).nickname,
                crate::ordinal_numbers(rrc.game_data.player_turn)
                ).into_bump_str())]}
            </h2>
        </div>
        )
    } else {
        //return wait for the other player
        dodrio!(bump,
        <h2 id="ws_elem" style= "color:red;">
            {vec![text(bumpalo::format!(in bump, "Wait for {} {} !",
            unwrap!(rrc.game_data.players.get(rrc.game_data.player_turn-1)).nickname,
            crate::ordinal_numbers(rrc.game_data.player_turn)
            ).into_bump_str())]}
        </h2>
        )
    }
}

//div_grid_container() is in divgridcontainermod.rs

///on click
pub fn on_click_2nd_card(rrc: &mut RootRenderingComponent, this_click_card_index: usize) {
    card_click_2nd_card(rrc, this_click_card_index);
}

///on second click
///The on click event passed by JavaScript executes all the logic
///and changes only the fields of the Card Grid struct.
///That struct is the only permanent data storage for later render the virtual dom.
pub fn card_click_2nd_card(rrc: &mut RootRenderingComponent, this_click_card_index: usize) {
    rrc.game_data.card_index_of_second_click = this_click_card_index;

    //3 possible outcomes: 1) same player, 2) Next Player 3) end game/play again
    //that changes: game status,CardStatusCardFace, points or/and player_turn
    //if the cards match, player get one point and continues another turn
    if unwrap!(rrc
        .game_data
        .card_grid_data
        .get(rrc.game_data.card_index_of_first_click))
    .card_number_and_img_src
        == unwrap!(rrc
            .game_data
            .card_grid_data
            .get(rrc.game_data.card_index_of_second_click))
        .card_number_and_img_src
    {
        on_msg_player_click_2nd_card_point(rrc, this_click_card_index);
        //region: send WsMessage over WebSocket
        websocketcommunicationmod::ws_send_msg(
            &rrc.game_data.ws,
            &WsMessage::MsgPlayerClick2ndCardPoint {
                my_ws_uid: rrc.game_data.my_ws_uid,
                players_ws_uid: rrc.game_data.players_ws_uid.to_string(),
                card_index_of_second_click: rrc.game_data.card_index_of_second_click,
            },
        );
        //endregion

        //if all the cards are permanenty up, this is the end of the game
        let mut all_permanently = true;
        //the zero element is exceptional, but the iterator uses it
        unwrap!(rrc.game_data.card_grid_data.get_mut(0)).status = CardStatusCardFace::UpPermanently;

        for x in &rrc.game_data.card_grid_data {
            match x.status {
                CardStatusCardFace::UpPermanently => {}
                _ => {
                    all_permanently = false;
                    break;
                }
            }
        }
        if all_permanently == true {
            statusplayagainmod::on_msg_play_again(rrc);
            //send message
            websocketcommunicationmod::ws_send_msg(
                &rrc.game_data.ws,
                &WsMessage::MsgPlayerClick2ndCardGameOverPlayAgainBegin {
                    my_ws_uid: rrc.game_data.my_ws_uid,
                    players_ws_uid: rrc.game_data.players_ws_uid.to_string(),
                },
            );
        }
    } else {
        statustaketurnbeginmod::on_msg_take_turn_begin(rrc, this_click_card_index);

        //region: send WsMessage over WebSocket
        websocketcommunicationmod::ws_send_msg(
            &rrc.game_data.ws,
            &WsMessage::MsgPlayerClick2ndCardTakeTurnBegin {
                my_ws_uid: rrc.game_data.my_ws_uid,
                players_ws_uid: rrc.game_data.players_ws_uid.to_string(),
                card_index_of_second_click: rrc.game_data.card_index_of_second_click,
            },
        );
        //endregion
    }
}

///msg player click
pub fn on_msg_player_click_2nd_card_point(
    rrc: &mut RootRenderingComponent,
    card_index_of_second_click: usize,
) {
    rrc.game_data.card_index_of_second_click = card_index_of_second_click;
    //flip the card up
    unwrap!(
        rrc.game_data
            .card_grid_data
            .get_mut(rrc.game_data.card_index_of_second_click),
        "error this_click_card_index"
    )
    .status = CardStatusCardFace::UpTemporary;

    //give points
    unwrap!(rrc
        .game_data
        .players
        .get_mut(unwrap!(rrc.game_data.player_turn.checked_sub(1))))
    .points += 1;

    // the two cards matches. make them permanent FaceUp
    let x1 = rrc.game_data.card_index_of_first_click;
    let x2 = rrc.game_data.card_index_of_second_click;
    unwrap!(rrc.game_data.card_grid_data.get_mut(x1)).status = CardStatusCardFace::UpPermanently;
    unwrap!(rrc.game_data.card_grid_data.get_mut(x2)).status = CardStatusCardFace::UpPermanently;
    //the same player continues to play
    rrc.game_data.game_status = GameStatus::StatusPlayBefore1stCard;
    rrc.check_invalidate_for_all_components();
}
