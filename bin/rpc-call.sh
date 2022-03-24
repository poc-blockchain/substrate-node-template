curl http://localhost:9933 \
-X POST \
-H "Content-Type:application/json;charset=utf-8" \
-d '{ "jsonrpc":"2.0", "id":1, "method":"get_value"}'
