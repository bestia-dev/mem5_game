# mem5_game

**Learning Rust Wasm/WebAssembly, Virtual Dom Dodrio, WebSocket communication and PWA (Progressive Web Apps) - part five**  
***version: 5.0  date: 2019-12-31 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/mem5_game)***  

![status](https://img.shields.io/badge/obsolete-yellow) 
![status](https://img.shields.io/badge/tutorial-yellow) 
![loc](https://img.shields.io/badge/lines_of_Rust_code-4591-success)
![loc](https://img.shields.io/badge/lines_of_docs/comments-1231-informational)
![Hits](https://bestia.dev/webpage_hit_counter/get_svg_image/748188469.svg)

Hashtags: #rustlang #game #tutorial  
My projects on Github are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

## Documentation

Documentation generated from source code:  
<https://bestia-dev.github.io/mem5_game/mem5/index.html>  
The workspace mem5_game is made of projects:  

1. Wasm/WebAssembly (for browsers) frontend - mem5  
2. web server Warp backend - mem5_server  
3. common structures - mem5_common  

Every project has its own readme.md.  

- [mem5/README.md](
https://github.com/bestia-dev/mem5_game/blob/master/mem5/README.md)  
- [mem5_common/README.md](https://github.com/bestia-dev/mem5_game/blob/master/mem5_common/README.md)  
- [mem5_server/README.md](https://github.com/bestia-dev/mem5_game/blob/master/mem5_server/README.md)  
  
Read also my `Previous projects` on Github:  
<https://github.com/bestia-dev/mem4_game>  

## Working game server

You can play the game (mobile only) hosted on google cloud platform:  
<https://bestia.dev/mem5>  

## Cargo make

I prepared some flows and tasks for Cargo make for the workspace.  
`cargo make` - lists the possible available/public flows/tasks  
`cargo make dev` - builds the development version and runs the server and the browser  
`cargo make release` - builds the release version and runs the server and the browser  
`cargo make audit` - cargo audit warnings about dependencies  
`cargo make fmt` - format source code  
`cargo make doc` - build the `/target/docs` folder and copy to the `/docs` folder  
`cargo make sshadd` - adds identity to ssh-agent for git and publish operations  
`cargo make gitpush` - push the commits to github, uses ssh agent  
`cargo make publish` - publish the webfolder to google vm  
`cargo make udeps` - lists unused dependencies  
`cargo make loc` - Lines Of Rust Code with tokei  
`cargo make depver` - list of not latest dependencies  

## cargo crev reviews and advisory

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
On the web use this url to read crate reviews. Example:  
<https://web.crev.dev/rust-reviews/crate/num-traits/>  

## TODO and CHANGELOG

Read files [TODO.md](https://github.com/bestia-dev/mem5_game/blob/master/TODO.md) and [CHANGELOG.md](https://github.com/bestia-dev/mem5_game/blob/master/CHANGELOG.md).  

## Open-source and free as a beer

My open-source projects are free as a beer (MIT license).  
I just love programming.  
But I need also to drink. If you find my projects and tutorials helpful, please buy me a beer by donating to my [PayPal](https://paypal.me/LucianoBestia).  
You know the price of a beer in your local bar ;-)  
So I can drink a free beer for your health :-)  
[Na zdravje!](https://translate.google.com/?hl=en&sl=sl&tl=en&text=Na%20zdravje&op=translate) [Alla salute!](https://dictionary.cambridge.org/dictionary/italian-english/alla-salute) [Prost!](https://dictionary.cambridge.org/dictionary/german-english/prost) [Nazdravlje!](https://matadornetwork.com/nights/how-to-say-cheers-in-50-languages/) 🍻

[//bestia.dev](https://bestia.dev)  
[//github.com/bestia-dev](https://github.com/bestia-dev)  
[//bestiadev.substack.com](https://bestiadev.substack.com)  
[//youtube.com/@bestia-dev-tutorials](https://youtube.com/@bestia-dev-tutorials)  
