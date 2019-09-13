Things are changing fast. This is the situation on 2019-09-13. LucianoBestia
# mem5_game
Learning Rust Wasm/WebAssembly with Virtual Dom Dodrio and WebSocket communication - part four.
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
## Shorturl.com
Google cloud platform does not give any subdomain name for free. Google is also a domain registrar and it looks like they are trying to push me to buy a domain.  
I didn't like to have the raw IP in the url. People don't like numbers like that.  
I created a subdomain on shorturl.com. It is not the perfect solution, but it is free or very cheap.  
## Cargo make
I prepared some flows and tasks for Cargo make for the workspace.  
`cargo make` - lists the possible available/public flows/tasks  
`cargo make dev` - builds the development version and runs the server and the browser  
`cargo make release` - builds the release version and runs the server and the browser  
`cargo make doc` - build the `/target/docs` folder and copy to the `/docs` folder.  

## TODO:
- sync data from player1 to others after reconnect.
- different content for English learning: numbers (cardinal, ordinal), food, orientation, alphabet simple spelling, drinks, days/months, questions, colors, transportation, ... 
- fullscreen from http://robnyman.github.io/fullscreen/  
- onfullscreen vdom schedule render  
- iPhone/android webapp manifest file  
- why/how to reset/reload the webapp in "add to homescreen" on iPhone?  

## Changelog
2019-09-13 mem4 is finished, start the mem5  


