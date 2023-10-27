# distributed-system-challenges

## Commands
Echo
~/maelstrom/maelstrom test -w echo --bin ./target/debug/echo --node-count 5 --time-limit 10

Unique ID
~/maelstrom/maelstrom test -w unique-ids --bin ./target/debug/unique_id --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition

Broadcast 3a
~/maelstrom/maelstrom test -w broadcast --bin ./target/debug/broadcast --node-count 1 --time-limit 20 --rate 10