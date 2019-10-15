if ('serviceWorker' in navigator) {
    navigator.serviceWorker.register('service-worker.js').then(function (registration) {
        // registration worked
        console.log('Registration succeeded.');
        /*
        button.onclick = function () {
            registration.update();
        }
        */
    }).catch(function (error) {
        // registration failed
        console.log('Registration failed with ' + error);
    });
};
