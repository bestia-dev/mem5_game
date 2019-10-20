# mem5_game

![loc](https://img.shields.io/badge/lines_of_Rust_code-4591-success)
![loc](https://img.shields.io/badge/lines_of_comments-1231-informational)

**Learning Rust Wasm/WebAssembly, Virtual Dom Dodrio, WebSocket communication and PWA (Progressive Web Apps) - part five**  
*Things are changing fast. This is the situation on 2019-10-18.*

## Documentation

Documentation generated from source code:  
<https://lucianobestia.github.io/mem5_game/mem5/index.html>  
<https://lucianobestia.github.io/mem5_game/mem5_server/index.html>  
The workspace mem5_game is made of:  

1. Wasm/WebAssembly (for browsers) frontend - mem5  
2. web server Warp backend - mem5_server  
3. common structures - mem5_common  

Every project has its own readme.md.  

- [mem5/README.md](
https://github.com/LucianoBestia/mem5_game/blob/master/mem5/README.md)  
- [mem5_common/README.md](https://github.com/LucianoBestia/mem5_game/blob/master/mem5_common/README.md)  
- [mem5_server/README.md](https://github.com/LucianoBestia/mem5_game/blob/master/mem5_server/README.md)  

## Info and working game

You can play the game here (hosted on google cloud platform):  
<https://bestia.dev/mem5>  
Read the `Previous project`:  
<https://github.com/LucianoBestia/mem4_game>  

## Cargo make

I prepared some flows and tasks for Cargo make for the workspace.  
`cargo make` - lists the possible available/public flows/tasks  
`cargo make dev` - builds the development version and runs the server and the browser  
`cargo make release` - builds the release version and runs the server and the browser  
`cargo make audit` - check dependencies  
`cargo make fmt` - format source code  
`cargo make doc` - build the `/target/docs` folder and copy to the `/docs` folder  
`cargo make sshadd` - adds identity to ssh-agent for git and publish operations  
`cargo make gitpush` - push the commits to github  
`cargo make publish` - publish the webfolder to google vm  

## TODO and CHANGELOG

Read files [TODO.md](https://github.com/LucianoBestia/mem5_game/blob/master/TODO.md) and [CHANGELOG.md](https://github.com/LucianoBestia/mem5_game/blob/master/CHANGELOG.md).  
