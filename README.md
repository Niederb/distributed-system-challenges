# distributed-system-challenges

## Commands
Echo
~/maelstrom/maelstrom test -w echo --bin ./target/release/echo --node-count 5 --time-limit 10

Unique ID
~/maelstrom/maelstrom test -w unique-ids --bin ./target/release/unique_id --time-limit 30 --rate 1000 --node-count 3 --availability total --nemesis partition
