//! rootrenderingcomponent.rs - renders the web page

//region: use, const
use crate::divcardmoniker;
use crate::divfordebugging;
use crate::divfullscreen;
use crate::divgridcontainer;
use crate::divplayeractions;
use crate::divplayersandscores;
use crate::divrulesanddescription;
use crate::gamedata::GameData;
//use crate::logmod;

use dodrio::builder::text;
use dodrio::bumpalo::{self, Bump};
use dodrio::{Cached, Node, Render};
use mem5_common::GameStatus;
use typed_html::dodrio;
use web_sys::WebSocket;
use conv::{ConvAsUtil};
//endregion

///Root Render Component: the card grid struct has all the needed data for play logic and rendering
pub struct RootRenderingComponent {
    ///game data will be inside of Root
    pub game_data: GameData,
    ///subComponent: players and scores. The data is a cached copy of GameData.
    pub cached_players_and_scores: Cached<divplayersandscores::PlayersAndScores>,
    ///subComponent: the static parts can be cached.
    pub cached_rules_and_description: Cached<divrulesanddescription::RulesAndDescription>,
}

//region:RootRenderingComponent struct is the only persistent data we have in Rust Virtual Dom.dodrio
//in the constructor we initialize that data.
//Later on click we change this data.
//at every animation frame we use only this data to render the virtual Dom.
//It knows nothing about HTML and Virtual dom.
impl RootRenderingComponent {
    /// Construct a new `RootRenderingComponent` component. Only once at the beginning.
    pub fn new(ws: WebSocket, my_ws_uid: usize) -> Self {
        let game_data = GameData::new(ws, my_ws_uid);

        let game_rule_01 = divrulesanddescription::RulesAndDescription {};
        let cached_rules_and_description = Cached::new(game_rule_01);
        let cached_players_and_scores =
            Cached::new(divplayersandscores::PlayersAndScores::new(my_ws_uid));

        RootRenderingComponent {
            game_data,
            cached_players_and_scores,
            cached_rules_and_description,
        }
    }
    ///check invalidate render cache for all sub components
    pub fn check_invalidate_for_all_components(&mut self) {
        if self
            .cached_players_and_scores
            .update_intern_cache(&self.game_data)
        {
            Cached::invalidate(&mut self.cached_players_and_scores);
        }
    }

    ///prepares the game data
    pub fn game_data_init(&mut self) {
        self.game_data.content_folder_name = self.game_data.asked_folder_name.clone();
        self.game_data.prepare_random_data();
        self.game_data.game_status = GameStatus::PlayBefore1stCard;
        self.game_data.player_turn = 1;
    }
    ///reset the data to replay the game
    pub fn reset(&mut self) {
        self.game_data.card_grid_data = GameData::prepare_for_empty();
        self.game_data.card_index_of_first_click = 0;
        self.game_data.card_index_of_second_click = 0;
        self.game_data.players.clear();
        self.game_data.game_status = GameStatus::InviteAskBegin;
        self.game_data.content_folder_name = "alphabet".to_string();
        self.game_data.asked_folder_name = "".to_string();
        self.game_data.my_player_number = 1;
        self.game_data.player_turn = 0;
        self.game_data.game_config = None;

        self.check_invalidate_for_all_components();
    }
    //region: all functions for receive message (like events)
    // I separate the code into functions to avoid looking at all that boilerplate in the big match around futures and components.
    // All the data changing must be encapsulated inside these functions.
    ///msg response with uid, just to check. because the WebSocket server
    ///gets the uid from the client in the url_param. The client generates a random number.
    pub fn on_response_ws_uid(&mut self, your_ws_uid: usize) {
        if self.game_data.my_ws_uid != your_ws_uid {
            self.game_data.error_text = "my_ws_uid is incorrect!".to_string();
        }
    }

    ///on game data init
    pub fn on_msg_game_data_init(
        &mut self,
        card_grid_data: &str,
        game_config: &str,
        players: &str,
    ) {
        self.game_data.content_folder_name = self.game_data.asked_folder_name.clone();
        self.game_data.game_status = GameStatus::PlayBefore1stCard;
        self.game_data.player_turn = 1;
        self.game_data.card_grid_data = unwrap!(
            serde_json::from_str(card_grid_data),
            "error serde_json::from_str(card_grid_data)"
        );

        self.game_data.game_config = unwrap!(
            serde_json::from_str(game_config),
            "error serde_json::from_str(game_config)"
        );

        self.game_data.players = unwrap!(
            serde_json::from_str(players),
            "error serde_json::from_str(players)"
        );

        //find my player number
        for index in 0..self.game_data.players.len() {
            if unwrap!(
                self.game_data.players.get_mut(index),
                "self.game_data.players.get_mut(index)"
            )
            .ws_uid
                == self.game_data.my_ws_uid
            {
                self.game_data.my_player_number = unwrap!(index.checked_add(1));
            }
        }
        self.check_invalidate_for_all_components();
    }
    //endregion
}
//endregion

//region: `Render` trait implementation on RootRenderingComponent struct
///It is called for every Dodrio animation frame to render the vdom.
///Probably only when something changes. Here it is a click on the cards.
///Not sure about that, but I don't see a reason to make execute it otherwise.
///It is the only place where I create HTML elements in Virtual Dom.
impl Render for RootRenderingComponent {
    #[inline]
    fn render<'a, 'bump>(&'a self, bump: &'bump Bump) -> Node<'bump>
    where
        'a: 'bump,
    {
        //the card grid is a html css grid object (like a table) with <img> inside
        //other html elements are pretty simple.

        //region: private helper fn for Render()
        //here I use private functions for readability only, to avoid deep code nesting.
        //I don't understand closures enough to use them properly.
        //These private functions are not in the "impl Render forRootRenderingComponent" because of the error
        //method `from_card_number_to_img_src` is not a member of trait `Render`
        //there is not possible to write private and public methods in one impl block there are only pub methods.
        //`pub` not permitted there because it's implied
        //so I have to write functions outside of the impl block but inside my "module"

        //region: create the whole virtual dom. The verbose stuff is in private functions

        if self.game_data.error_text == "" {
            let xmax_grid_size = divgridcontainer::max_grid_size(self);
            let xmax_grid_size_add_two = unwrap!(xmax_grid_size.hor.checked_add(2));
            let xstyle2 = format!("width:{}px;", xmax_grid_size_add_two);
            //logmod::debug_write(&format!("width m_container {}", xmax_grid_size_add_two));

            //the main HTML render
            dodrio!(bump,
            <div class= "m_container" style={xstyle2}>
                {vec![divcardmoniker::div_grid_card_moniker(self, bump)]}
                {
                    if self.game_data.is_status_for_grid_container(){
                        vec![divgridcontainer::div_grid_container(self,bump,&xmax_grid_size)]
                    }else {
                        vec![dodrio!(bump,
                            <div>
                            </div>
                        )]
                    }
                }
                {vec![divplayeractions::div_player_actions_from_game_status(self, bump)]}
                {
                    if self.game_data.is_status_for_grid_container(){
                        vec![self.cached_players_and_scores.render(bump)]
                    }else {
                        vec![dodrio!(bump,
                            <div>
                            </div>
                        )]
                    }
                }
                {vec![divfordebugging::div_for_debugging(self, bump)]}
                {vec![divfullscreen::div_for_fullscreen(self, bump)]}
                {vec![self.cached_rules_and_description.render(bump)]}
            </div>
            )
        } else {
            //render only the error text to the screen.
            //because I want to debug the WebSocket lost connection
            dodrio!(bump,
                <div>
                    <h1 style= "color:red;" >
                        {vec![text(
                            bumpalo::format!(in bump, "error_text {} !", self.game_data.error_text)
                                .into_bump_str(),
                            )]}
                    </h1>
                </div>
            )
        }
        //endregion
    }
}
//endregion

/// return window inner height
/// the size of  the visible part of the window
pub fn usize_window_inner_height() -> usize {
    let window = unwrap!(web_sys::window(), "error: web_sys::window");
    let jsvalue_inner_height = unwrap!(window.inner_height(), "window.inner_height");

    let f64_inner_height = unwrap!(
        jsvalue_inner_height.as_f64(),
        "jsValue_inner_height.as_f64()"
    );
    let usize_inner_height: usize = unwrap!(f64_inner_height.approx());
    //return
    usize_inner_height
}

/// return window inner width
/// the size of  the visible part of the window
pub fn usize_window_inner_width() -> usize {
    let window = unwrap!(web_sys::window(), "error: web_sys::window");

    let jsvalue_inner_width = unwrap!(window.inner_width(), "window.inner_width");

    let f64_inner_width = unwrap!(
        jsvalue_inner_width.as_f64(),
        "jsValue_inner_width.as_string()"
    );
    let usize_inner_width: usize = unwrap!(f64_inner_width.approx());
    //return
    usize_inner_width
}

/// return window inner width, but maximum 600px
/// the size of  the visible part of the window
pub fn usize_window_inner_width_but_max_600() -> usize {
    let x = usize_window_inner_width();
    if x > 600 {
        //return
        600
    } else {
        //return
        x
    }
}
