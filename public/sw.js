const cacheName = 'cache-v1'

const precacheResources = [
    '/offline.html',
    '/yekanBold.woff',
    '/dozar.svg',
    '/dozar.ico',
    '/dozar-logo192.png',
    '/dozar-logo48.png',
    // '/gooje-splash.jpg',
    // '/gooje-screenshot1.jpeg',
    // '/gooje-screenshot2.jpeg',
]

self.addEventListener('install', event => {
    console.log('Service worker install event!')

    event.waitUntil(
        caches.open(cacheName).then(cache => cache.addAll(precacheResources))
    )

    self.skipWaiting()
})

self.addEventListener('activate', event => {
    console.log('Service worker activate event!')

    event.waitUntil(
        (async () => {
            // Enable navigation preload if it's supported.
            // See https://developers.google.com/web/updates/2017/02/navigation-preload
            if ('navigationPreload' in self.registration) {
                await self.registration.navigationPreload.enable()
            }
        })()
    )

    self.clients.claim()
})

self.addEventListener('fetch', event => {
    event.respondWith(
        fetch(event.request).catch(async () => {
            return (
                (await caches.match(event.request)) ||
                (await caches.match('/offline.html'))
            )
        })
    )
})
