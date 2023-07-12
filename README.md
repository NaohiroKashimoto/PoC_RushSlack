# PoC_RustSlack

以下のクリートの動作確認および検証
https://github.com/abdolence/slack-morphism-rust

# SlackのEvent

https://api.slack.com/apis/connections/events-api

https://api.slack.com/events

SlackのイベントがBotに通知するJSONのサンプル

```json
{
    "type": "event_callback",
    "token": "XXYYZZ",
    "team_id": "T123ABC456",
    "api_app_id": "A123ABC456",
    "event": {
            "type": "name_of_event",
            "event_ts": "1234567890.123456",
            "user": "U123ABC456",
            ...
    },
    "event_context": "EC123ABC456",
    "event_id": "Ev123ABC456",
    "event_time": 1234567890,
    "authorizations": [
        {
            "enterprise_id": "E123ABC456",
            "team_id": "T123ABC456",
            "user_id": "U123ABC456",
            "is_bot": false,
            "is_enterprise_install": false,
        }
    ],
    "is_ext_shared_channel": false,
    "context_team_id": "T123ABC456",
    "context_enterprise_id": null
}
```

メッセージ

```json
{
	"type": "message",
	"channel": "C123ABC456",
	"user": "U123ABC456",
	"text": "Hello world",
	"ts": "1355517523.000005"
}
```

```json
{
	"type": "message",
	"channel": "C123ABC456",
	"user": "U123ABC456",
	"text": "Hello, world!",
	"ts": "1355517523.000005",
	"edited": {
		"user": "U123ABC456",
		"ts": "1355517536.000001"
	}
}

```



チャレンジ

```json
{
    "token": "Jhj5dZrVaK7ZwHHjRyZWjbDl",
    "challenge": "3eZbrw1aBm2rZgRNFdxV2595E9CY3gmdALWMmHkvFXO7tYXAYM8P",
    "type": "url_verification"
}

```

チャレンジへの応答

```
HTTP 200 OK
Content-type: text/plain
3eZbrw1aBm2rZgRNFdxV2595E9CY3gmdALWMmHkvFXO7tYXAYM8P
```


```
HTTP 200 OK
Content-type: text/plain
3eZbrw1aBm2rZgRNFdxV2595E9CY3gmdALWMmHkvFXO7tYXAYM8P
```

```
HTTP 200 OK
Content-type: application/json
{"challenge":"3eZbrw1aBm2rZgRNFdxV2595E9CY3gmdALWMmHkvFXO7tYXAYM8P"}
```


message
https://api.slack.com/events/message


https://api.slack.com/events/message/file_share

app_mention

```json
{
    "type": "app_mention",
    "user": "U061F7AUR",
    "text": "<@U0LAN0Z89> is it everything a river should be?",
    "ts": "1515449522.000016",
    "channel": "C123ABC456",
    "event_ts": "1515449522000016"
}
```

```json
{
    "token": "ZZZZZZWSxiZZZ2yIvs3peJ",
    "team_id": "T123ABC456",
    "api_app_id": "A123ABC456",
    "event": {
        "type": "app_mention",
        "user": "U123ABC456",
        "text": "What is the hour of the pearl, <@U0LAN0Z89>?",
        "ts": "1515449522.000016",
        "channel": "C123ABC456",
        "event_ts": "1515449522000016"
    },
    "type": "event_callback",
    "event_id": "Ev123ABC456",
    "event_time": 1515449522000016,
    "authed_users": [
        "U0LAN0Z89"
    ]
}

```

https://api.slack.com/events/message/file_mention
```json
{
	"type": "message",
	"subtype": "file_mention",
	"ts": "1358877455.000010",
	"text": "<@cal> mentioned a file: <https:...7.png|7.png>",
	"file": {...},
	"user": "U2147483697"
}

```
https://api.slack.com/events/message/me_message

```json
{
	"type": "message",
	"subtype": "me_message",
	"channel": "C123ABC456",
	"user": "U123ABC456",
	"text": "is doing that thing",
	"ts": "1355517523.000005"
}
```


file_created

```json
{
	"type": "file_created",
	"file_id": "F2147483862",
	"file": {
		"id": "F2147483862"
	}
}
```

3623007291.5562131510389
ClientSecret:6a91450940267bdd0ce5474bb5270007
SigningSecret:7bf4b6d54027b49588c17e23c90c84e6
VerificationToken:OYyhwAhrRjrL4IODKsH1DxEN