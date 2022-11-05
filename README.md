## Running

```bash
export BROADCAST_SECRET="supersecretkey"
export PORT="3000"
export HOST="0.0.0.0"
cargo run --release
```

## Rest

See the insomnia docs

## Websockets

See the insomnia example for more detailed info

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
  "c": "41a9b6aa-168a-4be8-8df8-cac17daf7384"
}
```

---

```json
{
  "t": "/is_online",
  "c": {
    "is_online": true,
    "uuid": "41a9b6aa-168a-4be8-8df8-cac17daf7384"
  }
}
```

### Requesting user status bulk

```json
{
  "t": "/is_online/bulk",
  "c": ["41a9b6aa-168a-4be8-8df8-cac17daf7384"]
}
```

---

```json
{
  "t": "/is_online/bulk",
  "c": {
    "41a9b6aa-168a-4be8-8df8-cac17daf7384": false
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
