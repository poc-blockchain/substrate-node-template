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
-d '{ "jsonrpc":"2.0", "id":1, "method":"kitty_get", "params": [null, "0x96e9c9084042ad78c20a4b2893ed9c32c5373c7a58ebc1a81c270d598979d98f"] }'


curl http://localhost:9933 \
-X POST \
-H "Content-Type:application/json;charset=utf-8" \
-d '{ "jsonrpc":"2.0", "id":1, "method":"kitty_count" }'
