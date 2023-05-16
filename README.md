# Change Detector - but rusty

A simple web scraping service that notifies you via your desired method whenever a webpage changes. 
Ill have a central yaml config file with all the needed info. Then I will create a client that contains 
a cached response 

```
// main.rs
config = CreateConfig()
client = CreateClient()
poller = CreatePoller(config, client)

poller.Poll()
```

```
// client.rs
struct Client {
  url String
  logger Logger
}

fn (c Client) Query() -> String {
    response = http.Get(url)
    return response
}
```

```
// poller.rs
struct Poller {
  client Client
  cachedResponse ResponseCache
  logger Logger
  notifier Notifier
}

fn (p Poller) Poll() {
  for {
    newResponse = p.client.Query()

    if newResponse != p.cachedResponse() {
      p.notifier.Notify("New was different than cache")
      p.cachedResponse.UpdateValue()
    }
  }
}

```

```
// responseCache.rs
struct ResponseCache {
  File fs.File
  lastUpdate time.Instant
}

fn (r ResponseCache) UpdateValue(newVal File) {
  c.File = newVal
  c.lastUpdate = time.Now()
}
```

structure:
```
change-detector/src
|-- main.rs
|-- internal
|   |-- poller.rs
|   |-- configuration.rs
|-- pkg/v1
|   |-- notifier
|   |   |-- notifier.rs
|   |-- client
|   |   |-- client.rs
```