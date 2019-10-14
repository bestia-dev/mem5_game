// main.js

if ('serviceWorker' in navigator) {
    window.addEventListener('load', function () {
        navigator.serviceWorker.register('sw.js').then(function (registration) {
            sessionStorage.setItem("debug_text", "Service worker successfully registered on scope " + registration.scope + "\n" + sessionStorage.getItem("debug_text"));
        }).catch(function (error) {
            sessionStorage.setItem("debug_text", "Service worker failed to register " + error + "\n" + sessionStorage.getItem("debug_text"));
        });
    });
}
