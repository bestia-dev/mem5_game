Things are changing fast. This is the situation on 2019-09-13. LucianoBestia
# mem5_game
Learning Rust Wasm/WebAssembly with Virtual Dom Dodrio and WebSocket communication - part five.
## Documentation
Documentation generated from source code:  
https://lucianobestia.github.io/mem5_game/mem5/index.html  
The workspace mem5_game is made of:  
1. Wasm/WebAssembly  (for browser) frontend - mem5  
2. web server Warp backend - mem5_server  
3. common structures - mem5_common  
## Info and working game
Read the `Last project`:  
https://github.com/LucianoBestia/mem4_game  
You can play the game here (hosted on google cloud platform):  
http://bestia.shorturl.com/mem5   

## Cargo make
I prepared some flows and tasks for Cargo make for the workspace.  
`cargo make` - lists the possible available/public flows/tasks  
`cargo make dev` - builds the development version and runs the server and the browser  
`cargo make release` - builds the release version and runs the server and the browser  
`cargo make doc` - build the `/target/docs` folder and copy to the `/docs` folder.  

## TODO:
Make debug div closeable for user.  
limit the size of debug_text  
- show the times of msg travel in the UI. Websocket looks awfully slow. 
- send in msg only what changed and not the whole grid_data
- minimize the data over websocket. json is so verbose. csv is less.
- sync data from player1 to others after reconnect.
- different content for English learning: numbers (cardinal, ordinal), food, orientation, alphabet simple spelling, drinks, days/months, questions, colors, transportation, ... 
- fullscreen from http://robnyman.github.io/fullscreen/  
- onfullscreen vdom schedule render  
- iPhone/android webapp manifest file  
- why/how to reset/reload the webapp in "add to homescreen" on iPhone?  
- add server database to write points - it is not informative, but it shows how to use a database  

## Changelog
2019-09-13 mem4 is finished, start the mem5  
2019-09-15 having make scripts for windows and linux is too much work. I need only linux. Also VSCode will work in Linux with the extension for Remote development.  
2019-09-16 nickname saved in the browser localstorage. The smartphones today are really personal.  
2019-09-19 working on writing debugging text in the UI because phones don't have F12. I try to use session_storage. 
 




