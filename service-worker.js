// Service Worker - Offline desteği için

const CACHE_NAME = 'pomodoro-v1';
const urlsToCache = [
  '/',
  '/index.html',
  '/pkg/pomodoro.js',
  '/pkg/pomodoro_bg.wasm',
  '/manifest.json'
];

// Uygulama yüklendiğinde
self.addEventListener('install', event => {
  event.waitUntil(
    caches.open(CACHE_NAME)
      .then(cache => {
        return cache.addAll(urlsToCache);
      })
  );
  self.skipWaiting();
});

// Eski cache'leri temizle
self.addEventListener('activate', event => {
  event.waitUntil(
    caches.keys().then(cacheNames => {
      return Promise.all(
        cacheNames.map(cacheName => {
          if (cacheName !== CACHE_NAME) {
            return caches.delete(cacheName);
          }
        })
      );
    })
  );
  self.clients.claim();
});

// Offline olduğunda cache'den sun
self.addEventListener('fetch', event => {
  event.respondWith(
    caches.match(event.request)
      .then(response => {
        if (response) {
          return response;
        }
        return fetch(event.request).catch(() => {
          return caches.match('/index.html');
        });
      })
  );
});
