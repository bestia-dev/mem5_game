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
pub fn div_grid_card_moniker<'a>(
    rrc: &'a RootRenderingComponent,
    bump: &'a Bump,
) -> Node<'a> {
    //this game_data mutable reference is dropped on the end of the function
    let game_data = &rrc.game_data;
    //if the card_monikers are visible, than don't show GameTitle, because there is not
    //enought space on smartphones
    if game_data.card_index_of_first_click != 0 || game_data.card_index_of_second_click != 0 {
        //return
        dodrio!(bump,
        <div class= "grid_container_header" style={bumpalo::format!(in bump, "grid-template-columns: auto auto;{}","").into_bump_str()}>
            <div class= "grid_item" style= "text-align: left;padding-left: 5%">
                {vec![text(
                    bumpalo::format!(in bump, "{}",
                    unwrap!(unwrap!(rrc.game_data.game_config.clone(),"rrc.game_data.game_config.clone()")
                    .card_moniker.get(unwrap!(game_data.card_grid_data.get(game_data.card_index_of_first_click)).card_number_and_img_src)))
                    .into_bump_str(),
                )]}
                </div>
                <div class= "grid_item" style= "text-align: right;padding-right: 5%">
                    {vec![text(
                    bumpalo::format!(in bump, "{}",
                    unwrap!(unwrap!(rrc.game_data.game_config.clone(),"rrc.game_data.game_config.clone()")
                    .card_moniker.get(unwrap!(game_data.card_grid_data.get(game_data.card_index_of_second_click)).card_number_and_img_src)))
                .into_bump_str(),
                )]}
                </div>
            </div>
            )
    } else {
        {
            let version = env!("CARGO_PKG_VERSION");

            dodrio!(bump,
            <div class= "grid_container_header" style= "grid-template-columns: auto;">
                <div class= "grid_item" style= "text-align: center;">
                    {vec![text(GAME_TITLE),
                        text(" - "),
                        text(version)]}
                </div>
            </div>
            )
        }
    }
}
