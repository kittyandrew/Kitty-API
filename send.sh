
function err () {
    echo -e $@
    exit 1
}

function code () {
    compare=\"$2\"
    if [ -z "$1" ]; then
        err Failed: Empty item but expected "$compare"!
    fi
    msg=$(echo "$1" | jq .msg_code)
    if [[ $msg != $compare ]]; then
        err Failed: "$msg" doesn\'t match "$compare"!
    fi
    nm=\"no_message\"
    if [[ $msg == $nm ]]; then
        count=$(echo "$1" | jq '.data | length')
        #if [[ $3 != $count ]]; then
        #    err Failed: length doesn\'t match, expected $3, got $1 items!
        #fi
        echo -e "Good:\t$msg\t\tAmount of items: $count"
        return
    fi
    text=$(echo "$1" | jq .message)
    if [[ "$text" == "null" ]]; then
        text=""
    fi
    echo -e "Good:\t$msg\t$text"
}

DIV="---------------------------------------------------------------------"

resp=$(curl -so /dev/null -w "%{http_code}\n" http://0.0.0.0:8000)
if [[ $resp != 200 ]]; then
    err Couldn\'t connect, you might have forgotten to start the service!
fi

echo -e
echo -e $DIV
echo -e "\tUsers:"
echo -e $DIV

resp=$(curl -s http://0.0.0.0:8000/api/users)
code "$resp" "no_message" 13

resp=$(curl -s -X DELETE http://0.0.0.0:8000/api/users)
code "$resp" "info_items_removed"  # "info_users_removed"

resp=$(curl -s -X POST -d '{"username": "A", "first_name": "B", "last_name": "C", "email": "D", "age": 14, "active": true, "picture": "Z"}' -H "Content-Type: application/json" http://0.0.0.0:8000/api/users)
code "$resp" "info_new_item_ok"
item_id=$(echo "$resp" | jq .item_id)

# Error
resp=$(curl -s http://0.0.0.0:8000/api/users/99999)
code "$resp" "err_item_not_exist"

resp=$(curl -s http://0.0.0.0:8000/api/users/$item_id)
code "$resp" "no_message"

# Error
resp=$(curl -s -X DELETE http://0.0.0.0:8000/api/users/99999)
code "$resp" "err_item_not_exist"

resp=$(curl -s -X DELETE http://0.0.0.0:8000/api/users/$item_id)
code "$resp" "info_remove_item_ok"

empty=$(curl -s http://0.0.0.0:8000/api/users)
if [ ${#empty[@]} -eq 0 ]; then
    echo -e FATAL ERROR: Non empty - "$empty";
fi

resp=$(curl -s -X POST -d '{"username": "A", "first_name": "B", "last_name": "C", "email": "D", "age": 14, "active": true, "picture": "Z"}' -H "Content-Type: application/json" http://0.0.0.0:8000/api/users/99999)
code "$resp" "info_new_item_ok"

resp=$(curl -s -X POST -d '{"username": "A", "first_name": "B", "last_name": "C", "email": "D", "age": 14, "active": true, "picture": "Z"}' -H "Content-Type: application/json" http://0.0.0.0:8000/api/users/99999)
code "$resp" "err_item_exists"

resp=$(curl -s -X PUT -d '{"username": "B", "first_name": "C", "last_name": "D", "email": "E", "age": 11, "active": false, "picture": "Y"}' -H "Content-Type: application/json" http://0.0.0.0:8000/api/users/99999)
code "$resp" "info_item_put_ok"

resp=$(curl -s -X PUT -d '{"username": "B", "first_name": "C", "last_name": "D", "email": "E", "age": 11, "active": false, "picture": "Y"}' -H "Content-Type: application/json" http://0.0.0.0:8000/api/users/99)
code "$resp" "info_item_put_ok"

resp=$(curl -sX PATCH -d '{"first_name": "TEST 1", "active": false}' -H "Content-Type: application/json" http://0.0.0.0:8000/api/users/99)
first_name=$(echo "$resp" | jq ".data.first_name")
active_1=$(echo "$resp" | jq ".data.active")
if [[ $first_name != "\"TEST 1\"" ]]; then
    echo -e "FAILED: $first_name was supposed to be \"TEST 1\""
    exit 1
fi
if [[ $active_1 != "false" ]]; then
    echo -e "FAILED: $active_1 was supposed to be \"false\""
    exit 1
fi
code "$resp" "info_patch_item_ok"

resp=$(curl -sX PATCH -d '{"active": true}' -H "Content-Type: application/json" http://0.0.0.0:8000/api/users/99)
active_2=$(echo "$resp" | jq ".data.active")
if [[ $active_2 != "true" ]]; then
    echo -e "FAILED: $active_2 was supposed to be \"true\""
    exit 1
fi
code "$resp" "info_patch_item_ok"

resp=$(curl -sX PATCH -d '{"first_name": "TEST 1", "active": false}' -H "Content-Type: application/json" http://0.0.0.0:8000/api/users/124214)
code "$resp" "err_item_not_exist"

resp=$(curl -s -X POST -d '{"username": "A", "first_name": "B", "last_name": "C", "email": "D", "age": 14, "active": true, "picture": "Z"}' -H "Content-Type: application/json" http://0.0.0.0:8000/api/users)
code "$resp" "info_new_item_ok"

# echo $resp
item_id=$(echo "$resp" | jq .item_id)
# Custom user ids should not affect autoincrement
if (( $item_id > 1000 )); then
    echo -e FATAL ERROR: value is too large - $item_id;
fi

resp=$(curl -s "http://0.0.0.0:8000/api/users/?page=0")
code "$resp" "no_message"

resp=$(curl -s "http://0.0.0.0:8000/api/users/?page=2")
code "$resp" "no_message"

resp=$(curl -s -H "X-PAGE-SIZE: 2" "http://0.0.0.0:8000/api/users/?page=0")
code "$resp" "no_message"

page_size=$(echo "$resp" | jq .page_size)
echo -e
if (( $page_size != 2)); then
    echo -e X-PAGE-SIZE header didn\'t work! Expected 2, got $page_size.
else
    echo -e X-PAGE-SIZE header assertion succeed!
fi

echo -e 
echo -e $DIV
echo -e "\tCats:"
echo -e $DIV
resp=$(curl -s http://0.0.0.0:8000/api/cats)
code "$resp" "no_message" 3

echo -e
resp=$(curl -sI http://0.0.0.0:8000/api/cats)
if [ -z "$resp" ]; then
    echo HEAD request failed! Response: $resp
else
    echo HEAD request succeed!
fi

echo -e
resp=$(curl -sIX OPTIONS http://0.0.0.0:8000/api/cats)
if [ -z "$resp" ]; then
    echo -e OPTIONS request failed! Response: "$resp"
else
    echo -e OPTIONS request succeed! /\\t$(echo "$resp" | grep "allow")
fi
resp=$(curl -sIX OPTIONS http://0.0.0.0:8000/api/cats/1)
if [ -z "$resp" ]; then
    echo -e OPTIONS request failed! Response: "$resp"
else
    echo -e OPTIONS request succeed! /\<id\>\\t$(echo "$resp" | grep "allow")
fi

echo -e 
echo -e $DIV
echo -e "\tTextCats:"
echo -e $DIV
resp=$(curl -s -H "X-PAGE-SIZE: 9" "http://0.0.0.0:8000/api/textcats/?page=5")
code "$resp" "no_message"

page_size=$(echo "$resp" | jq .page_size)
echo -e
if (( $page_size != 9)); then
    echo -e X-PAGE-SIZE header didn\'t work! Expected 9, got $page_size.
else
    echo -e X-PAGE-SIZE header assertion succeed!
fi

echo -e
