# mem5

mem5 is a simple memory game made primarily for learning the Rust programming language and Wasm/WebAssembly with Virtual Dom Dodrio and WebSocket communication  

[comment]: # (lmake_readme version)  
Look also at the workspace readme on <https://github.com/LucianoBestia/mem5_game>  

## Idea

Playing the memory game alone is boring.  
Playing it with friends is better.  
But if all friends just stare in their smartphones, it is still boring.  
What makes memory games (and other board games) entertaining is the company of friends.  
There must be many friends around the table watching one another and stealing moves and laughing and screaming at each other.  
Today I assume everybody has a decent smartphone. If all friends open the mem5 game and put their smartphones on the center of the table one near the other so that everybody can see them and touch them, this is the closest it gets to a classic board game.  
All the phones will have a small card grid (ex. 3x3). But the combined card grid from all these phones together is not so small anymore. It is now much more interesting to play for the players.  
It can be played with as many friends as there are: 3,4,5,...  
More friends - more fun.  

## Rust and Wasm/WebAssembly

Rust is a pretty new language created by Mozilla for really low level programming.  
It is a step forward from the C language with functionality and features that are best practice today.  
It is pretty hard to learn. Some concepts are so different from other languages it makes it
hard for beginners. Lifetimes are the strangest and most confusing concept.  
The Rust language has been made from the ground up with an ecosystem that makes it productive.  
The language and most of the libraries are Open Source. That is good and bad, but mostly good.  
Rust is the best language today to compile into Wasm/WebAssembly.  
That compiled code works inside a browser directly with the JavaScript engine.  
So finally no need for JavaScript to make cross-platform applications inside browsers.  
I have a lot of hope here.  

## Virtual DOM

Constructing a HTML page with Virtual DOM (vdom) is easier  
because it is rendered completely on every tick (animation frame).  
Sometimes is hard for the developer to think what should change in the UI when some data changes.  
The data can change from many different events and very chaotically (asynchronously).  
It is easier to think how to render the complete DOM for the given data.  
The Rust Dodrio library has ticks, time intervals when it do something.  
If a rendering is scheduled, it will be done on the next tick.  
If a rendering is not scheduled I believe nothing happens.  
This enables asynchronous changing of data and rendering. They cannot happen theoretically in the
same exact moment. So, no data race here.  
When GameData change and we know it will affect the DOM, then rendering must be scheduled.  
The main component of the Dodrio Virtual Dom is the root rendering component.  
It is the component that renders the complete user interface (HTML).  
The root rendering component is easily splitted  into sub-components.  
![subcomponents](https://github.com/LucianoBestia/mem5_game/raw/master/docs/img/subcomponents.png)  
Some subcomponents don't need any extra data and can be coded as simple functions.  
The subcomponent "players and scores" has its own data. This data is cached from the GameData.  
When this data does not match, invalidation is called to cache them.
That also schedules the rendering of the subcomponent.  
If no data has changed, the cached subcomponent Node is used. This is more efficient and performant.  

## GameData

All the game data are in this simple struct.  

## WebSocket communication

HTML5 has finally bring a true stateful bidirectional communication.  
Most of the programming problems are more easily and effectively solved this way.  
The old unidirectional stateless communication is very good for static html pages,  
but is terrible for any dynamic page. The WebSocket is very rudimental and often the  
communication breaks for many different reasons. The programmer must deal with it inside the application. On any network problem it is possible that the message is sent but not received. This is not working well for this game. So I must implement that the receiver sends a confirmation message. The sender must wait for this message to continue the workflow.  
The protocol has nothing that can be used to deal with reconnections.  
I send simple structs text messages in json format between the players.  
They are all in the WsMsg enum and therefore interchangeable.  
The WebSocket server is coded especially for this game and recognizes 3 types of msg:

- msg to broadcast to every other player
- msg to send only to the actual game players

## WS reconnect

TODO: It looks that plain web sockets have often connection problems and they disconnect here and there. Creating a good reconnect is pretty challenging.  

## The game flow

In a few words: Status1 - User action - Status2, Status1 - WsMessage - Status2
In one moment the game is in a certain Game Status. The user then makes an action.
This action changes the GameData and the GameStatus.  
Then a message is sent to other players so they can also change their local GameData and GameStatus.  
The rendering is scheduled and it will happen shortly (async).  

| Game Status1       | Render                     | User action                                 | Condition                            | GameStatus2 t.p.   | Sends Msg          | On rcv Msg o.p.              | GameStatus2 o.p.                   |
| ------------------ | -------------------------- | ------------------------------------------- | ------------------------------------ | ----------------   | ----------------   | --------------------------   | --------------------------------   |
| StatusInviteAskBegin     | div_invite_ask_begin       | div_invite_ask_begin_on_click               | -                                    | StatusInviteAsking       | MsgInvite             | on_msg_invite                | StatusInviteAsked                        |
| StatusInviteAsked        | div_invite_asked, div_play_accepted | div_invite_asked_on_click          | -                                    | StatusPlayAccepted       | MsgPlayAccept         | on_msg_play_accept           | -                                  |
| StatusInviteAsking       | div_invite_asking          | game_data_init                              | -                                    | StatusPlayBefore1stCard  | MsgGameDataInit       | on_msg_game_data_init        | StatusPlayBefore1stCard                  |
| StatusPlayBefore1stCard  | div_grid_container         | div_grid_item_on_click, on_click_1st_card();| -                                    | StatusPlayBefore2ndCard  | MsgPlayerClick1stCard | on_msg_player_click_1st_card | StatusPlayBefore2ndCard                  |
| StatusPlayBefore2ndCard  | div_grid_container         | div_grid_item_on_click, on_click_2nd_card();| If card match and points<all point   | StatusPlayBefore1stCard  | MsgPlayerClick2ndCardPoint | on_msg_player_click_2nd_card | StatusPlayBefore1stCard                  |
| -II-               | -II-                       | -II-                                        | If card match and points=>all points | StatusGameOverPlayAgainBegin | StatusGameOverPlayAgainBegin  | on_msg_play_again   | StatusGameOverPlayAgainBegin             |
| -II-               | -II-                       | -II-                                        | else                                 | MsgPlayerClick2ndCardTakeTurnBegin      | MsgPlayerClick2ndCardTakeTurnBegin      | on_msg_take_turn             | MsgPlayerClick2ndCardTakeTurnBegin                      |
| MsgPlayerClick2ndCardTakeTurnBegin      | div_take_turn_begin        | div_take_turn_begin_on_click                | -                                    | StatusPlayBefore1stCard  | MsgTakeTurnEnd        | on_msg_take_turn_end         | StatusPlayBefore1stCard, the next player |
| StatusGameOverPlayAgainBegin | div_play_again         | window.location().reload()                  | -                                    | -                  | -                  | -                            | -                                  |
|  |  |  |  |  |  |  |  |

t.p. = this player,   o.p. = other players,  rrc = rrc, rcv = receive

1. Some actions can have different results. For example the condition card match or card don’t match.  
2. one action must be only for one status1. This action changes Status for this player and sends Msg to other players.  
3. on receive msg can produce only one status2.  
4. in this table I ignore msgs for the server like GetConfig  

## Futures and Promises, Rust and JavaScript

JavaScript is all asynchronous. Wasm is nothing else then a shortcut to the JavaScript engine.  
So everything is asynchronous too. This is pretty hard to grasp. Everything is Promises and Futures.  
There is a constant jumping from thinking in Rust to thinking is JavaScript and back. That is pretty confusing.  
JavaScript does not have a good idea of Rust datatypes. All there is is a generic JSValue type.  
The library `wasm-bindgen` has made a fantastic job of giving Rust the ability to call
anything JavaScript can call, but the way of doing it is sometimes very hard to understand.  

## Typed html

Writing html inside Rust code is much easier with the macro `html!` from the `crate typed-html`  
<https://github.com/bodil/typed-html>  
It has also a macro `dodrio!` created exclusively for the dodrio vdom.  
Everything is done in compile time, so the runtime is nothing slower.

## Browser console

At least in modern browsers (Firefox and Chrome) we have the developer tools F12 and there is a
console we can output to. So we can debug what is going on with our Wasm program.
But not on smartphones that are the only target for this app.  

## Safari on iOS and FullScreen

Apple is very restrictive and does not allow fullscreen Safari on iPhones.  
The workaround is to make a shortcut for the webapp on the homescreen.  

## mem5 as webapp on HomeScreen

On both android and iPhone is possible to "Add to homescreen" the webapp.  
Then it will open in fullscreen and be beautiful.  
In safari the share icon (a square with arrow up) has "Add to home screen".
<https://developer.apple.com/library/archive/documentation/AppleApplications/Reference/SafariWebContent/ConfiguringWebApplications/ConfiguringWebApplications.html>  

## Modules

Rust code is splitted into modules. They are not exactly like classes, but can be similar.  
Rust has much more freedom to group code in different ways. So that is best suits the problem.  
I splitted the rendering into sub-components.  
And then I splitted the User Actions by the Status1 to easy follow the flow of the game.  

## Clippy

Clippy is very useful to teach us how to program in a better way.  
These are not syntax errors, but hints how to do it in a more Rusty way (idiomatic).  
Some lints are problematic and they are explicitly allowed here.

## Cargo make

I prepared some flows and tasks for Cargo make.  
`cargo make` - lists the possible available/public flows/tasks  
`cargo make dev` - builds the development version and runs the server and the browser  
`cargo make release` - builds the release version and runs the server and the browser  
`cargo make doc` - build the `/target/doc` folder and copy to the `../docs` folder.  
