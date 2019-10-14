// sw.js
var cacheName = 'mem5_game';
var filesToCache = [
    'main.js',
    'css/mem5.css',
    'images/icons-16.png',
    'images/icons-32.png',
    'images/icons-192.png',
    'images/icons-512.png'
];
self.addEventListener('install', function (event) {
    event.waitUntil(
        caches.open(cacheName)
            .then(function (cache) {
                //serviceWorker cannot access SessionStorage or LocalStorage
                console.log('[sw.js] cached all files');
                return cache.addAll(filesToCache);
            })
    );
});