curl http://localhost:9933 \
-X POST \
-H "Content-Type:application/json;charset=utf-8" \
-d '{ "jsonrpc":"2.0", "id":1, "method":"get_value"}'


curl http://localhost:9933 \
-X POST \
-H "Content-Type:application/json;charset=utf-8" \
-d '{ "jsonrpc":"2.0", "id":1, "method":"sumStorage_get"}'

curl http://localhost:9933 \
-X POST \
-H "Content-Type:application/json;charset=utf-8" \
-d '{ "jsonrpc":"2.0", "id":1, "method":"kitty_get", "params": [null, "0xbf1cfee7d9d80626ac6e8d4e2c2efb6d5f49acdfa17be6a996e8e313e36367b7"] }'
