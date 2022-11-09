## Api docs

[./asyncapi.md](./asyncapi.md)

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

## Rest

### GET `/cosmetics`

```json
{
  "cosmetics": [
    {
      "description": "White",
      "display": "WW",
      "id": 1,
      "name": "W",
      "required_flags": 1
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

```json
{
  "t": "/connect",
  "c": "41a9b6aa-168a-4be8-8df8-cac17daf7384"
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

A cosmetics file looks something like this

```json
{
  "cosmetics": [
    {
      "id": 1,
      "name": "W",
      "display": "WW",
      "description": "White",
      "required_flags": 1
    }
  ],
  "users": {
    "41a9b6aa-168a-4be8-8df8-cac17daf7384": {
      "flags": 1,
      "enabled_prefix": null
    }
  }
}
```
