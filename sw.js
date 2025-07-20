// Service Worker for The Incredible Rust
const CACHE_NAME = 'incredible-rust-v1.0.0';
const STATIC_CACHE = 'static-v1.0.0';
const DYNAMIC_CACHE = 'dynamic-v1.0.0';

// Assets to cache immediately
const STATIC_ASSETS = [
  '/',
  '/assets/css/style.css',
  '/assets/css/syntax.css',
  '/assets/favicon.ico',
  '/assets/favicon.svg',
  '/manifest.json',
  // Add critical pages
  '/lessons/',
  '/lessons/installation/',
  '/lessons/hello-world/',
  '/lessons/cargo/',
  '/lessons/variables/',
  '/lessons/data-types/'
];

// Install event - cache static assets
self.addEventListener('install', event => {
  console.log('Service Worker: Installing...');
  event.waitUntil(
    caches.open(STATIC_CACHE)
      .then(cache => {
        console.log('Service Worker: Caching static assets');
        return cache.addAll(STATIC_ASSETS);
      })
      .then(() => {
        console.log('Service Worker: Static assets cached');
        return self.skipWaiting();
      })
      .catch(err => {
        console.error('Service Worker: Error caching static assets', err);
      })
  );
});

// Activate event - clean up old caches
self.addEventListener('activate', event => {
  console.log('Service Worker: Activating...');
  event.waitUntil(
    caches.keys()
      .then(cacheNames => {
        return Promise.all(
          cacheNames.map(cacheName => {
            if (cacheName !== STATIC_CACHE && cacheName !== DYNAMIC_CACHE) {
              console.log('Service Worker: Deleting old cache', cacheName);
              return caches.delete(cacheName);
            }
          })
        );
      })
      .then(() => {
        console.log('Service Worker: Activated');
        return self.clients.claim();
      })
  );
});

// Fetch event - serve from cache, fallback to network
self.addEventListener('fetch', event => {
  const { request } = event;
  const url = new URL(request.url);

  // Skip non-GET requests
  if (request.method !== 'GET') {
    return;
  }

  // Skip external requests
  if (url.origin !== location.origin) {
    return;
  }

  event.respondWith(
    caches.match(request)
      .then(cachedResponse => {
        if (cachedResponse) {
          console.log('Service Worker: Serving from cache', request.url);
          return cachedResponse;
        }

        // Not in cache, fetch from network
        return fetch(request)
          .then(networkResponse => {
            // Don't cache non-successful responses
            if (!networkResponse || networkResponse.status !== 200 || networkResponse.type !== 'basic') {
              return networkResponse;
            }

            // Clone the response
            const responseToCache = networkResponse.clone();

            // Cache dynamic content
            caches.open(DYNAMIC_CACHE)
              .then(cache => {
                console.log('Service Worker: Caching dynamic content', request.url);
                cache.put(request, responseToCache);
              });

            return networkResponse;
          })
          .catch(err => {
            console.error('Service Worker: Fetch failed', err);
            
            // Return offline page for navigation requests
            if (request.destination === 'document') {
              return caches.match('/offline.html');
            }
            
            // Return cached version or throw error
            return caches.match(request);
          });
      })
  );
});

// Background sync for offline actions
self.addEventListener('sync', event => {
  console.log('Service Worker: Background sync', event.tag);
  
  if (event.tag === 'background-sync') {
    event.waitUntil(
      // Handle background sync tasks
      console.log('Service Worker: Performing background sync')
    );
  }
});

// Push notifications
self.addEventListener('push', event => {
  console.log('Service Worker: Push received');
  
  const options = {
    body: event.data ? event.data.text() : 'New content available!',
    icon: '/assets/favicon.svg',
    badge: '/assets/favicon.svg',
    vibrate: [100, 50, 100],
    data: {
      dateOfArrival: Date.now(),
      primaryKey: 1
    },
    actions: [
      {
        action: 'explore',
        title: 'Explore',
        icon: '/assets/favicon.svg'
      },
      {
        action: 'close',
        title: 'Close',
        icon: '/assets/favicon.svg'
      }
    ]
  };

  event.waitUntil(
    self.registration.showNotification('The Incredible Rust', options)
  );
});

// Notification click
self.addEventListener('notificationclick', event => {
  console.log('Service Worker: Notification clicked');
  
  event.notification.close();

  if (event.action === 'explore') {
    event.waitUntil(
      clients.openWindow('/')
    );
  }
});

// Message handling
self.addEventListener('message', event => {
  console.log('Service Worker: Message received', event.data);
  
  if (event.data && event.data.type === 'SKIP_WAITING') {
    self.skipWaiting();
  }
});

// Periodic background sync (if supported)
self.addEventListener('periodicsync', event => {
  console.log('Service Worker: Periodic sync', event.tag);
  
  if (event.tag === 'content-sync') {
    event.waitUntil(
      // Sync content in background
      console.log('Service Worker: Syncing content')
    );
  }
});