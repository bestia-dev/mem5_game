// sw.js
var cacheName = 'mem5_game';
var filesToCache = [
    'main.js',
    'css/mem5.css'
];

self.addEventListener('install', function (event) {
    //console.log('install');
    event.waitUntil(
        caches.open(cacheName)
            .then(function (cache) {
                //serviceWorker cannot access SessionStorage or LocalStorage
                console.log('[sw.js] cached all files in install event');
                return cache.addAll(filesToCache);
            })
    );
});

self.addEventListener('fetch', function (event) {
    event.respondWith(
        caches.match(event.request)
            .then(function (response) {
                // Cache hit - return response
                if (response) {
                    return response;
                }

                return fetch(event.request).then(
                    function (response) {
                        // Check if we received a valid response
                        if (!response || response.status !== 200 || response.type !== 'basic') {
                            return response;
                        }

                        // IMPORTANT: Clone the response. A response is a stream
                        // and because we want the browser to consume the response
                        // as well as the cache consuming the response, we need
                        // to clone it so we have two streams.
                        var responseToCache = response.clone();

                        caches.open(CACHE_NAME)
                            .then(function (cache) {
                                cache.put(event.request, responseToCache);
                            });

                        return response;
                    }
                );
            })
    );
});