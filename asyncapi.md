# DWS 1.0.0 documentation

* License: [MPL 2.0](https://www.mozilla.org/en-US/MPL/2.0/)


## Table of Contents

* [Servers](#servers)
  * [ws](#ws-server)
  * [rest](#rest-server)
* [Operations](#operations)
  * [SUB /broadcast](#sub-broadcast-operation)
  * [SUB /cosmetics](#sub-cosmetics-operation)
  * [PUB /ws](#pub-ws-operation)
  * [SUB /ws](#sub-ws-operation)

## Servers

### `ws` Server

* URL: `wss://virginity.kokoniara.software/ws`
* Protocol: `wss`

WSS



### `rest` Server

* URL: `https://virginity.kokoniara.software`
* Protocol: `https`

Rest api



## Operations

### SUB `/broadcast` Operation

Broadcast channel


#### `rest` Channel specific information

| Name | Type | Description | Value | Constraints | Notes |
|---|---|---|---|---|---|
| (root) | - | - | `"NULL"` | - | - |

#### Message `broadcastMessage`

*Broadcast message*

Broadcast message

##### Payload

| Name | Type | Description | Value | Constraints | Notes |
|---|---|---|---|---|---|
| (root) | object | - | - | - | **additional properties are allowed** |
| message | string | - | - | - | - |
| to | array<string> | - | - | - | - |
| to (single item) | string | A minecraft user uuid | - | - | - |

> Examples of payload _(generated)_

```json
{
  "message": "string",
  "to": [
    "41a9b6aa-168a-4be8-8df8-cac17daf7384"
  ]
}
```



### SUB `/cosmetics` Operation

Cosmetics channel


#### `rest` Channel specific information

| Name | Type | Description | Value | Constraints | Notes |
|---|---|---|---|---|---|
| (root) | - | - | `"NULL"` | - | - |

#### Message `cosmeticsData`

*Cosmetics data message*

Cosmetics data message

##### Payload

| Name | Type | Description | Value | Constraints | Notes |
|---|---|---|---|---|---|
| (root) | object | - | - | - | **additional properties are allowed** |
| cosmetics | array<any> | - | - | - | - |
| cosmetics (single item) | any | - | - | - | **additional properties are allowed** |
| users | object | - | - | - | - |
| users (additional properties) | integer | The id of the cosmetic | - | - | - |

> Examples of payload _(generated)_

```json
{
  "cosmetics": [
    null
  ],
  "users": {
    "property1": 0,
    "property2": 0
  }
}
```



### PUB `/ws` Operation

* Operation ID: `processReceivedMessage`

Send messages to the API

#### `ws` Operation specific information

| Name | Type | Description | Value | Constraints | Notes |
|---|---|---|---|---|---|
| (root) | - | - | `"NULL"` | - | - |

Accepts **one of** the following messages:

#### Message `connect`

*Connect to the websocket server*

Connect to the websocket server

##### Payload

| Name | Type | Description | Value | Constraints | Notes |
|---|---|---|---|---|---|
| (root) | object | - | - | - | **additional properties are allowed** |
| t | string | - | const (`"/connect"`) | - | - |
| c | string | A minecraft user uuid | - | - | - |

> Examples of payload _(generated)_

```json
{
  "t": "/connect",
  "c": "41a9b6aa-168a-4be8-8df8-cac17daf7384"
}
```


#### Message extensions

| Name | Type | Description | Value | Constraints | Notes |
|---|---|---|---|---|---|
| x-response | - | - | - | - | - |
| x-response.summary | - | - | `"Response to connect message"` | - | - |
| x-response.description | - | - | `"Response to connect message"` | - | - |
| x-response.payload | object | - | - | - | **additional properties are allowed** |
| x-response.payload.t | string | - | const (`"/connected"`) | - | - |
| x-response.payload.c | boolean | - | - | - | - |
| x-response.schemaFormat | - | - | `"application/vnd.aai.asyncapi;version=2.5.0"` | - | - |

#### Message `ping`

*Ping server to determine whether connection is alive*

Client can ping server to determine whether connection is alive, server responds with pong. This is an application level ping as opposed to default ping in websockets standard which is server initiated

##### Payload

| Name | Type | Description | Value | Constraints | Notes |
|---|---|---|---|---|---|
| (root) | object | - | - | - | **additional properties are allowed** |
| t | string | - | const (`"/ping"`) | - | **required** |
| c | string | A nonce can be given to identiy a request | - | - | - |

> Examples of payload _(generated)_

```json
{
  "t": "/ping",
  "c": "ranmdomstringlol"
}
```


#### Message extensions

| Name | Type | Description | Value | Constraints | Notes |
|---|---|---|---|---|---|
| x-response | - | - | - | - | - |
| x-response.summary | - | - | `"Pong is a response to ping message"` | - | - |
| x-response.description | - | - | `"Server pong response to a ping to determine whether connection is alive. This is an application level pong as opposed to default pong in websockets standard which is sent by client in response to a ping"` | - | - |
| x-response.payload | object | - | - | - | **additional properties are allowed** |
| x-response.payload.t | string | - | const (`"/pong"`) | - | - |
| x-response.payload.c | string | A nonce can be given to identiy a request | - | - | - |
| x-response.schemaFormat | - | - | `"application/vnd.aai.asyncapi;version=2.5.0"` | - | - |

#### Message `isOnline`

*You can use this message to check whether a user is online*

Checks the api itself to see if the user is connected to the websocket server

##### Payload

| Name | Type | Description | Value | Constraints | Notes |
|---|---|---|---|---|---|
| (root) | object | - | - | - | **additional properties are allowed** |
| t | string | - | const (`"/is_online"`) | - | - |
| c | object | - | - | - | **additional properties are allowed** |
| c.uuid | string | A minecraft user uuid | - | - | - |
| c.nonce | string | A nonce can be given to identiy a request | - | - | - |

> Examples of payload _(generated)_

```json
{
  "t": "/is_online",
  "c": {
    "uuid": "41a9b6aa-168a-4be8-8df8-cac17daf7384",
    "nonce": "ranmdomstringlol"
  }
}
```


#### Message extensions

| Name | Type | Description | Value | Constraints | Notes |
|---|---|---|---|---|---|
| x-response | - | - | - | - | - |
| x-response.summary | - | - | `"Response to isOnline message"` | - | - |
| x-response.description | - | - | `"Response to isOnline message"` | - | - |
| x-response.payload | object | - | - | - | **additional properties are allowed** |
| x-response.payload.t | string | - | const (`"/is_online"`) | - | - |
| x-response.payload.c | object | - | - | - | **additional properties are allowed** |
| x-response.payload.c.uuid | string | A minecraft user uuid | - | - | - |
| x-response.payload.c.online | boolean | - | - | - | - |
| x-response.payload.c.nonce | string | A nonce can be given to identiy a request | - | - | - |
| x-response.schemaFormat | - | - | `"application/vnd.aai.asyncapi;version=2.5.0"` | - | - |

#### Message `isOnlineBulk`

*You can use this message to check whether a list of users are online*

Checks the api itself to see if the users are connected to the websocket server

##### Payload

| Name | Type | Description | Value | Constraints | Notes |
|---|---|---|---|---|---|
| (root) | object | - | - | - | **additional properties are allowed** |
| t | string | - | const (`"/is_online_bulk"`) | - | - |
| c | object | - | - | - | **additional properties are allowed** |
| c.uuids | array<string> | - | - | - | - |
| c.uuids (single item) | string | A minecraft user uuid | - | - | - |
| c.nonce | string | A nonce can be given to identiy a request | - | - | - |

> Examples of payload _(generated)_

```json
{
  "t": "/is_online_bulk",
  "c": {
    "uuids": [
      "41a9b6aa-168a-4be8-8df8-cac17daf7384"
    ],
    "nonce": "ranmdomstringlol"
  }
}
```


#### Message extensions

| Name | Type | Description | Value | Constraints | Notes |
|---|---|---|---|---|---|
| x-response | - | - | - | - | - |
| x-response.summary | - | - | `"Response to isOnlineBulk"` | - | - |
| x-response.description | - | - | `"Response to isOnlineBulk"` | - | - |
| x-response.payload | object | - | - | - | **additional properties are allowed** |
| x-response.payload.t | string | - | const (`"/is_online/bulk"`) | - | - |
| x-response.payload.c | object | - | - | - | **additional properties are allowed** |
| x-response.payload.c.uuids | object | - | - | - | - |
| x-response.payload.c.uuids (additional properties) | boolean | - | - | - | - |
| x-response.payload.c.nonce | string | A nonce can be given to identiy a request | - | - | - |
| x-response.schemaFormat | - | - | `"application/vnd.aai.asyncapi;version=2.5.0"` | - | - |

#### Message `cosmeticsUpdate`

*Cosmetics update message*

Cosmetics update message

##### Payload

| Name | Type | Description | Value | Constraints | Notes |
|---|---|---|---|---|---|
| (root) | object | - | - | - | **additional properties are allowed** |
| t | string | - | const (`"/cosmetics/update"`) | - | - |
| c | object | - | - | - | **additional properties are allowed** |
| c.cosmetic_id | integer | - | - | - | - |
| c.nonce | string | A nonce can be given to identiy a request | - | - | - |

> Examples of payload _(generated)_

```json
{
  "t": "/cosmetics/update",
  "c": {
    "cosmetic_id": 0,
    "nonce": "ranmdomstringlol"
  }
}
```


#### Message extensions

| Name | Type | Description | Value | Constraints | Notes |
|---|---|---|---|---|---|
| x-response | - | - | - | - | - |
| x-response.summary | - | - | `"Cosmetics updated message"` | - | - |
| x-response.description | - | - | `"Cosmetics updated message"` | - | - |
| x-response.payload | object | - | - | - | **additional properties are allowed** |
| x-response.payload.t | string | - | const (`"/cosmetics/updated"`) | - | - |
| x-response.payload.c | object | - | - | - | **additional properties are allowed** |
| x-response.payload.c.cosmetic_id | integer | - | - | - | - |
| x-response.payload.c.nonce | string | A nonce can be given to identiy a request | - | - | - |
| x-response.schemaFormat | - | - | `"application/vnd.aai.asyncapi;version=2.5.0"` | - | - |


### SUB `/ws` Operation

* Operation ID: `sendMessage`

Messages that you receive from the API

#### `ws` Operation specific information

| Name | Type | Description | Value | Constraints | Notes |
|---|---|---|---|---|---|
| (root) | - | - | `"NULL"` | - | - |

Accepts **one of** the following messages:

#### Message `pong`

*Pong is a response to ping message*

Server pong response to a ping to determine whether connection is alive. This is an application level pong as opposed to default pong in websockets standard which is sent by client in response to a ping

##### Payload

| Name | Type | Description | Value | Constraints | Notes |
|---|---|---|---|---|---|
| (root) | object | - | - | - | **additional properties are allowed** |
| t | string | - | const (`"/pong"`) | - | - |
| c | string | A nonce can be given to identiy a request | - | - | - |

> Examples of payload _(generated)_

```json
{
  "t": "/pong",
  "c": "ranmdomstringlol"
}
```


#### Message `isOnlineBulkResponse`

*Response to isOnlineBulk*

Response to isOnlineBulk

##### Payload

| Name | Type | Description | Value | Constraints | Notes |
|---|---|---|---|---|---|
| (root) | object | - | - | - | **additional properties are allowed** |
| t | string | - | const (`"/is_online/bulk"`) | - | - |
| c | object | - | - | - | **additional properties are allowed** |
| c.uuids | object | - | - | - | - |
| c.uuids (additional properties) | boolean | - | - | - | - |
| c.nonce | string | A nonce can be given to identiy a request | - | - | - |

> Examples of payload _(generated)_

```json
{
  "t": "/is_online/bulk",
  "c": {
    "uuids": {
      "property1": true,
      "property2": true
    },
    "nonce": "ranmdomstringlol"
  }
}
```


#### Message `isOnlineResponse`

*Response to isOnline message*

Response to isOnline message

##### Payload

| Name | Type | Description | Value | Constraints | Notes |
|---|---|---|---|---|---|
| (root) | object | - | - | - | **additional properties are allowed** |
| t | string | - | const (`"/is_online"`) | - | - |
| c | object | - | - | - | **additional properties are allowed** |
| c.uuid | string | A minecraft user uuid | - | - | - |
| c.online | boolean | - | - | - | - |
| c.nonce | string | A nonce can be given to identiy a request | - | - | - |

> Examples of payload _(generated)_

```json
{
  "t": "/is_online",
  "c": {
    "uuid": "41a9b6aa-168a-4be8-8df8-cac17daf7384",
    "online": true,
    "nonce": "ranmdomstringlol"
  }
}
```


#### Message `connectResponse`

*Response to connect message*

Response to connect message

##### Payload

| Name | Type | Description | Value | Constraints | Notes |
|---|---|---|---|---|---|
| (root) | object | - | - | - | **additional properties are allowed** |
| t | string | - | const (`"/connected"`) | - | - |
| c | boolean | - | - | - | - |

> Examples of payload _(generated)_

```json
{
  "t": "/connected",
  "c": true
}
```


#### Message `error`

*Error message*

Error message

##### Payload

| Name | Type | Description | Value | Constraints | Notes |
|---|---|---|---|---|---|
| (root) | object | - | - | - | **additional properties are allowed** |
| t | string | - | const (`"/error"`) | - | - |
| c | object | - | - | - | **additional properties are allowed** |
| c.error | string | - | - | - | - |
| c.nonce | string | A nonce can be given to identiy a request | - | - | - |

> Examples of payload _(generated)_

```json
{
  "t": "/error",
  "c": {
    "error": "string",
    "nonce": "ranmdomstringlol"
  }
}
```


#### Message `broadcast`

*Broadcast message*

Broadcast message

##### Payload

| Name | Type | Description | Value | Constraints | Notes |
|---|---|---|---|---|---|
| (root) | object | - | - | - | **additional properties are allowed** |
| t | string | - | const (`"/broadcast"`) | - | - |
| c | string | - | - | - | - |

> Examples of payload _(generated)_

```json
{
  "t": "/broadcast",
  "c": "string"
}
```


#### Message `cosmeticAck`

*Cosmetic ack message*

Cosmetic ack message

##### Payload

| Name | Type | Description | Value | Constraints | Notes |
|---|---|---|---|---|---|
| (root) | object | - | - | - | **additional properties are allowed** |
| t | string | - | const (`"/cosmetics/ack"`) | - | - |

> Examples of payload _(generated)_

```json
{
  "t": "/cosmetics/ack"
}
```



