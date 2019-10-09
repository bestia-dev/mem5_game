//! divcardmonikermod.rs - renders the card moniker (card name/title)

//region: use, const
use crate::rootrenderingcomponentmod::RootRenderingComponent;

use unwrap::unwrap;
use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::Node;
use typed_html::dodrio;

///game title
const GAME_TITLE: &str = "mem5";
//endregion

///the header can show only the game title or two card monikers. Not everything together.
pub fn div_grid_card_moniker<'a>(rrc: &'a RootRenderingComponent, bump: &'a Bump) -> Node<'a> {
    //this game_data mutable reference is dropped on the end of the function
    let game_data = &rrc.game_data;

    //if the card_monikers are visible, than don't show GameTitle, because there is not
    //enought space on smartphones
    if game_data.card_index_of_first_click != 0 || game_data.card_index_of_second_click != 0 {
        let left_text = unwrap!(unwrap!(rrc.game_data.game_config.as_ref())
            .card_moniker
            .get(
                unwrap!(game_data
                    .card_grid_data
                    .get(game_data.card_index_of_first_click))
                .card_number_and_img_src
            ))
        .to_string();
        let left_text_len = left_text.len();
        let left_fontsize = calc_font_size(left_text_len);
        let left_style_string =
            bumpalo::format!(in bump, "font-size:{}px;", left_fontsize).into_bump_str();

        let right_text = unwrap!(unwrap!(rrc.game_data.game_config.as_ref())
            .card_moniker
            .get(
                unwrap!(game_data
                    .card_grid_data
                    .get(game_data.card_index_of_second_click))
                .card_number_and_img_src
            ))
        .to_string();
        let right_text_len = right_text.len();
        let right_fontsize = calc_font_size(right_text_len);
        let right_style_string =
            bumpalo::format!(in bump, "font-size:{}px;", right_fontsize).into_bump_str();
        //return
        dodrio!(bump,
        <div class= "grid_container_header" style="grid-template-columns: 50% 50%;min-height: 60px;">
            <div id="card_moniker_left" class= "grid_item" style={left_style_string} >
                {vec![text(bumpalo::format!(in bump, "{}",left_text).into_bump_str())]}
            </div>
            <div id="card_moniker_right" class= "grid_item" style={right_style_string} >
                {vec![text(bumpalo::format!(in bump, "{}", right_text).into_bump_str())]}
            </div>
        </div>
        )
    } else {
        {
            let version = env!("CARGO_PKG_VERSION");
            let style_string = bumpalo::format!(in bump, "font-size:{}px;", 30).into_bump_str();
            dodrio!(bump,
            <div class= "grid_container_header" style= "grid-template-columns: auto;min-height: 60px;">
                <div id="card_moniker_center" class= "grid_item" style={style_string} >
                    {vec![text(GAME_TITLE),
                        text(" - "),
                        text(version)]}
                </div>
            </div>
            )
        }
    }
}

///when the lenght is bigger, the fontsize get smaller
///if the len is 10 the fontsize is 40, if the len is 20 the fontsize is 20
///this means that the 400 is constant:  10*40=400 20*20=400
#[allow(clippy::integer_arithmetic,
    clippy::integer_division
)]
fn calc_font_size(text_len: usize) -> usize {
    if text_len < 10 {
        40
    } else {
        350 / text_len
    }
}
