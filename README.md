# DWS <!-- omit in toc -->

The first fully opensource Skyblock mod backend lol.

- [Running](#running)
- [Features](#features)
- [Rest](#rest)
  - [GET `/cosmetics`](#get-cosmetics)
  - [POST `/broadcast`](#post-broadcast)
  - [GET `/metrics`](#get-metrics)
- [Websockets](#websockets)
  - [Connecting](#connecting)
  - [Requesting user status](#requesting-user-status)
  - [Requesting user status bulk](#requesting-user-status-bulk)
  - [Pings](#pings)
  - [Update cosmetic](#update-cosmetic)
  - [Cosmetic Ack event](#cosmetic-ack-event)
  - [Irc](#irc)
  - [Broadcasts](#broadcasts)
  - [Errors](#errors)
- [Cosmetics](#cosmetics)
- [Contributing](#contributing)
- [License](#license)

## Running

```bash
export BROADCAST_SECRET="supersecretkey"
export PORT="3000"
export HOST="0.0.0.0"
export DISCORD_PUBLIC_KEY=
export DISCORD_TOKEN=
export DISCORD_CLIENT_ID=
cargo run --release
```

## Features

- [x] Discord bot
- [x] Cosmetics
- [x] Checking if players are online
- [x] Irc
- [x] Prometheus metrics
- [x] Discord linking
- [x] Admin dashboard

## Rest

### GET `/cosmetics`

```json
{
  "cosmetics": [
    {
      "data": "&a",
      "description": "Prefix: invis_test1",
      "id": 0,
      "name": "invis_test1",
      "required_flags": 2,
      "type": 1
    },
    {
      "data": "§e",
      "description": "Prefix: supporter2",
      "id": 1,
      "name": "supporter2",
      "required_flags": 32,
      "type": 1
    },
    {
      "data": "§b[l'élite]",
      "description": "Prefix: invis_plexus3",
      "id": 2,
      "name": "invis_plexus3",
      "required_flags": 2,
      "type": 2
    }
  ],
  "users": { "41a9b6aa-168a-4be8-8df8-cac17daf7384": 1 }
}
```

### POST `/broadcast`

to requires a list of uuids or nothing to send to all users.

```
authorization: $BROADCAST_SECRET
```

```json
{
  "message": "Hello world",
  "to": []
}
```

---

```
Ok
```

### GET `/metrics`

Returns a bunch of prometheus metrics

## Websockets

See the insomnia example for more detailed info,
Nonces are a optional field

### Connecting

<!-- TEST_MODE -->

for development none of this validated. server_id is expected to be a hashed server id that is needed for <https://wiki.vg/Protocol_Encryption#Authentication>

```json
{
  "t": "/connect",
  "c": {
    "server_id": "Hello world from irc ws lol",
    "username": "trickedmc"
  }
}
```

---

```json
{
  "t": "/connected",
  "c": true
}
```

### Requesting user status

<!-- TEST_MODE -->

```json
{
  "t": "/is_online",
  "c": { "uuid": "41a9b6aa-168a-4be8-8df8-cac17daf6324", "nonce": "HI!" }
}
```

---

```json
{
  "t": "/is_online",
  "c": {
    "is_online": true,
    "uuid": "41a9b6aa-168a-4be8-8df8-cac17daf6324",
    "nonce": "HI!"
  }
}
```

### Requesting user status bulk

<!-- TEST_MODE -->

```json
{
  "t": "/is_online/bulk",
  "c": { "uuids": ["41a9b6aa-168a-4be8-8df8-cac17daf6324"], "nonce": "HI!" }
}
```

---

```json
{
  "t": "/is_online/bulk",
  "c": {
    "users": {
      "41a9b6aa-168a-4be8-8df8-cac17daf6324": true
    },
    "nonce": "HI!"
  }
}
```

### Pings

<!-- TEST_MODE -->

```json
{
  "t": "/ping"
}
```

---

```json
{
  "t": "/pong"
}
```

### Update cosmetic

<!-- TEST_MODE -->

```json
{
  "t": "/cosmetics/update",
  "c": {
    "cosmetic_id": 1,
    "nonce": "hi1"
  }
}
```

---

```json
{
  "t": "/cosmetics/updated",
  "c": {
    "cosmetic_id": 1,
    "nonce": "hi1"
  }
}
```

### Cosmetic Ack event

It is suggested to update cosmetics between 1-5 minutes after this event is received to account for any other updates and to not trigger ddos protection.

---

```json
{
  "t": "/cosmetics/ack"
}
```

### Irc

<!-- TEST_MODE -->

The irc chat can be linked to discord and you can blacklist uuids from the irc using `/irc blacklist Add <uuid>`, if a user is blacklisted irc messages send by this user will be silently ignored.

```json
{
  "t": "/irc/create",
  "c": {
    "message": "HI!"
  }
}
```

---

```json
{
  "t": "/irc/created",
  "c": {
    "message": "HI!",
    "sender": "41a9b6aa-168a-4be8-8df8-cac17daf7384",
    "date": 1668109163235
  }
}
```

### Broadcasts

Broadcasts are only received not send you can view the broadcast post request to find out how those work

---

```json
{
  "t": "/broadcast",
  "c": "Hello world"
}
```

### Errors

Errors are only recieved and look like this, errors can include a nonce for when necessary

---

```json
{
  "t": "/error",
  "c": {
    "error": "Already connected"
  }
}
```

## Cosmetics

A cosmetics file looks something like this, The ran instance uses type 1 to identify colors and type 2 prefixes

```json
{
  "cosmetics": [
    {
      "data": "&a",
      "description": "Prefix: invis_test1",
      "id": 0,
      "name": "invis_test1",
      "required_flags": 2,
      "type": 1
    },
    {
      "data": "§e",
      "description": "Prefix: supporter2",
      "id": 1,
      "name": "supporter2",
      "required_flags": 32,
      "type": 1
    },
    {
      "data": "§b[l'élite]",
      "description": "Prefix: invis_plexus3",
      "id": 2,
      "name": "invis_plexus3",
      "required_flags": 2,
      "type": 2
    }
  ],
  "users": {
    "a1937b73-ecff-4d6c-aa7b-6702b957dbd6": {
      "flags": 8,
      "enabled_prefix": 8
    },
    "4e29caf5-9317-454b-8863-eca22877e0ec": {
      "flags": 8,
      "enabled_prefix": 12
    }
  }
}
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change. To test out the websocket server you can use either insomnia or the provided `test.ts` tool with [Deno](https://deno.land/), The tool automatically picks up the requests from the README file.

## License

We use the Mozilla Public License 2.0. MPL tries to find a balance between permissive (MIT, Apache, Zlib) and copyleft licenses (GPL, LGPL). <sup><sup>[MORE INFO](https://www.mozilla.org/en-US/MPL/2.0/FAQ/)</sup></sup>
