# Validator #1
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d @aura1
curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d @grandpa1
# Validator #2
curl http://localhost:9934 -H "Content-Type:application/json;charset=utf-8" -d @aura2
curl http://localhost:9934 -H "Content-Type:application/json;charset=utf-8" -d @grandpa2

# # Validator #3
curl http://localhost:9935 -H "Content-Type:application/json;charset=utf-8" -d @aura3
curl http://localhost:9935 -H "Content-Type:application/json;charset=utf-8" -d @grandpa3
