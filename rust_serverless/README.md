## LocalでLambda単体テスト
```
$ yarn sls invoke local -f example-function --path src/example_request.json
```

### デプロイ
```
$ yarn sls deploy --aws-profile nabezokodaikon
```

## AWS上でLambda単体テスト
```
$ yarn sls invoke -f example-function --path src/example_request.json --aws-profile nabezokodaikon
```

## API Gateway経由の結合テスト
```
$ curl -X GET https://sypvc0y3ud.execute-api.ap-northeast-1.amazonaws.com/dev/
```
