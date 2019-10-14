// sw.js
var cacheName = 'WWW-EXAMPLE-COM-V1';
var filesToCache = [
    '/',                // index.html
    '/main.js',
    '/css/mem5.css',
    '/images/icons-16.png',
    '/images/icons-32.png',
    '/images/icons-192.png',
    '/images/icons-512.png'
];
self.addEventListener('install', function (event) {
    event.waitUntil(
        caches.open(cacheName)
            .then(function (cache) {
                console.info('[sw.js] cached all files');
                return cache.addAll(filesToCache);
            })
    );
});