# mem5_game

**Learning Rust Wasm/WebAssembly with Virtual Dom Dodrio and WebSocket communication - part five.**  
*Things are changing fast. This is the situation on 2019-09-13.*

## Documentation

Documentation generated from source code:  
<https://lucianobestia.github.io/mem5_game/mem5/index.html>  
The workspace mem5_game is made of:  

1. Wasm/WebAssembly  (for browser) frontend - mem5  
2. web server Warp backend - mem5_server  
3. common structures - mem5_common  

## Info and working game

You can play the game here (hosted on google cloud platform):  
<http://bestia.shorturl.com/mem5>  
Read the `Previous project`:  
<https://github.com/LucianoBestia/mem4_game>  

## Cargo make

I prepared some flows and tasks for Cargo make for the workspace.  
`cargo make` - lists the possible available/public flows/tasks  
`cargo make dev` - builds the development version and runs the server and the browser  
`cargo make release` - builds the release version and runs the server and the browser  
`cargo make doc` - build the `/target/docs` folder and copy to the `/docs` folder.  

## TODO and CHANGELOG

Read files TODO.md and CHANGELOG.md.  
