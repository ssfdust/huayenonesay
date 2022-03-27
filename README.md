# Hua Yen Says

It's a http server that gives you one say from Avatamsaka Sūtra.

## Start

### 

```shell
export BINDADDR="0.0.0.0"
export BINDPORT=8080
cargo run
```

## API

* GET /api/huayen/onesay

```json
{
    "code": 0,
    "msg": "get one say successfully.",
    "data": {
        "onesay": "其诸菩萨住逝多林，不离如来道场众会。"
    }
}
```
