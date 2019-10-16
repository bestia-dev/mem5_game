
window.onload = maxWindow;

function maxWindow() {
    sessionStorage["debug_text"] = "onload screen.availWidth " + screen.availWidth + "\n" + sessionStorage["debug_text"];
    sessionStorage["debug_text"] = "window.devicePixelRatio " + window.devicePixelRatio + "\n" + sessionStorage["debug_text"];
    sessionStorage["debug_text"] = "screen.width " + screen.width + "\n" + sessionStorage["debug_text"];
    sessionStorage["debug_text"] = "document.body.clientWidth " + document.body.clientWidth + "\n" + sessionStorage["debug_text"];

    if (screen.availWidth > 600) {
        top.window.outerWidth = 600
    } else if (screen.availWidth < 300) {
        top.window.outerWidth = 300
    } else {
        top.window.outerWidth = screen.availWidth;
    }
}