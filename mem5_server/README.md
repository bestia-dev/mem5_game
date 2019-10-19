# mem5_server 

[comment]: # (lmake_readme version)  

**Html and WebSocket server for the mem5 game**
Primarily made for learning to code Rust for a http + WebSocket server on the same port  
Using Warp for a simple memory game for kids - mem5.  
On the IP address on port 8085 listens to http and WebSocket.  
Route for http `/` serves static files from folder `/mem5/`  
Route `/mem5ws/` broadcast all WebSocket msg to all connected clients except sender  

## Google vm

One working server is installed on my google vm.  
There is a nginx server reverse proxy that accepts https http2 on 443 and relay to internal 8085.
Nginx also redirects all http 80 to https 443.  
You can play the game here (hosted on google cloud platform):  
https://bestia.dev/mem5  
