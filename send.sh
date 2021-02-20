
curl http://0.0.0.0:8000/api/users

curl -X DELETE http://0.0.0.0:8000/api/users

resp=$(curl -s -X POST -d '{"username": "A", "first_name": "B", "last_name": "C", "email": "D", "age": 14, "active": true, "picture": "Z"}' -H "Content-Type: application/json" http://0.0.0.0:8000/api/users)
echo $resp
user_id=$(echo $resp | jq .user_id)

# Error
curl http://0.0.0.0:8000/api/users/99999

curl http://0.0.0.0:8000/api/users/$user_id

# Error
curl -X DELETE http://0.0.0.0:8000/api/users/99999

curl -X DELETE http://0.0.0.0:8000/api/users/$user_id

empty=$(curl -s http://0.0.0.0:8000/api/users)
if [ ${#empty[@]} -eq 0 ]; then
    echo -e FATAL ERROR: Non empty - $empty;
fi

echo -e

resp=$(curl -s -X POST -d '{"username": "A", "first_name": "B", "last_name": "C", "email": "D", "age": 14, "active": true, "picture": "Z"}' -H "Content-Type: application/json" http://0.0.0.0:8000/api/users/99999)
echo $resp

resp=$(curl -s -X POST -d '{"username": "A", "first_name": "B", "last_name": "C", "email": "D", "age": 14, "active": true, "picture": "Z"}' -H "Content-Type: application/json" http://0.0.0.0:8000/api/users/99999)
echo $resp

resp=$(curl -s -X PUT -d '{"username": "B", "first_name": "C", "last_name": "D", "email": "E", "age": 11, "active": false, "picture": "Y"}' -H "Content-Type: application/json" http://0.0.0.0:8000/api/users/99999)
echo $resp

resp=$(curl -s -X PUT -d '{"username": "B", "first_name": "C", "last_name": "D", "email": "E", "age": 11, "active": false, "picture": "Y"}' -H "Content-Type: application/json" http://0.0.0.0:8000/api/users/99)
echo $resp

echo -e

resp=$(curl -s -X POST -d '{"username": "A", "first_name": "B", "last_name": "C", "email": "D", "age": 14, "active": true, "picture": "Z"}' -H "Content-Type: application/json" http://0.0.0.0:8000/api/users)
# echo $resp
user_id=$(echo $resp | jq .user_id)
# Custom user ids should not affect autoincrement
if (( $user_id > 1000 )); then
    echo -e FATAL ERROR: value is too large - $user_id;
fi


curl "http://0.0.0.0:8000/api/users/?page=0"
curl "http://0.0.0.0:8000/api/users/?page=2"
