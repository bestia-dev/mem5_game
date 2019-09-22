//! divrulesanddescription.rs - renders the div that shows rules and descriptions
//! All is a static content. Great for implementing dodrio cache.

//region: use
use dodrio::builder::{br, text};
use dodrio::bumpalo::{self, Bump};
use dodrio::{Node, Render};
use typed_html::dodrio;
//endregion

///Text of game rules.
///Multiline string literal just works.
///End of line in the code is simply and intuitively end of line in the string.
///The special character \ at the end of the line in code means that it is NOT the end of the line for the string.
///The escape sequence \n means end of line also. For doublequote simply \" .
const GAME_RULES:& str = "This game is for many players. More players - more fun.  
It is fun to play only on smartphones. It works in all modern browsers.  
All the players must open this web app to allow communication.  
Put all the smartphones on the table near each other, so all players can see them and touch \
them. It should look like a board game at this point.  
The first player clicks on 'MsgInvite for play?'.  
He can choose different types of game visuals: alphabet, animal, playing cards,...  
Other players then see on the screen 'Click here to Accept play!'.  
Player1 sees how many players have accepted. Then he starts the game.  
On the screen under the grid are clear signals which player plays and which waits.  
Player1 flips over two cards with two clicks. This cards can be on any smartphone. \
The cards are accompanied by sounds and text on the screen.  
If the cards do not match, the other player clicks on 'Click here to Take your turn' and both cards \
are flipped back face down. Then it is his turn and he clicks to flip over his two cards.  
If the cards match, they are left face up permanently and the player receives a point. He continues \
to play, he opens the next two cards.  
The game is over when all the cards are permanently face up.  
Click on \"Play again?\" to re-start the game.  ";

///game description
const GAME_DESCRIPTION:& str = "Learning to use Rust Wasm/WebAssembly with Dodrio Virtual Dom and WebSockets communication - fourth iteration.";

///Render Component: The static parts can be cached easily.
pub struct RulesAndDescription {}

impl Render for RulesAndDescription {
    ///This rendering will be rendered and then cached . It will not be rerendered untill invalidation.
    ///In this case I don't need to invalidate because it is a static content.
    fn render<'a, 'bump>(&'a self, bump: &'bump Bump) -> Node<'bump>
    where
        'a: 'bump,
    {
        dodrio!(bump,
        <div>
            <h4>
                {text_with_br_newline(GAME_DESCRIPTION,bump)}
            </h4>
            <h2>
            {vec![text(
                bumpalo::format!(in bump, "Memory game rules: {}", "").into_bump_str(),
            )]}
            </h2>
            <h4>
                {text_with_br_newline(GAME_RULES, bump)}
            </h4>
            <h6>
                {vec![text(bumpalo::format!(in bump, "Learning Rust programming: {}", "").into_bump_str(),)]}
                <a href= "https://github.com/LucianoBestia/mem5_game" target="_blank">
                    {vec![text(bumpalo::format!(in bump, "https://github.com/LucianoBestia/mem5_game{}", "").into_bump_str(),)]}
                </a>
            </h6>
        </div>
        )
    }
}

///change the newline lines ending into <br> node
fn text_with_br_newline<'a>(txt: &'a str, bump: &'a Bump) -> Vec<Node<'a>> {
    let mut vec_text_node = Vec::new();
    let spl = txt.lines();
    for part in spl {
        vec_text_node.push(text(part));
        vec_text_node.push(br(bump).finish());
    }
    vec_text_node
}

