# Run a network of nodes
Instructions on how to run and interact with a network of (currently 2) instances of the zk-node.

## Setup the file structure for 2 nodes

```bash
export RUN_SETUP=1
export PATH_TO_DB="/Users/chef/Desktop/paradox-6/database"
nix develop
```
or `RUN_SETUP=0` - `RUN_SETUP=1` will override the `database` and recreate `noir.db` and `circom.db`.

## Run a network of 2 nodes

```bash
nix develop

cd zk_node

cargo run PATH_TO_DB/node-1 8000 8001

// in a seperate terminal - or tmux view

nix develop

cd zk_node

cargo run PATH_TO_DB/node-2 8001 8000

...
```

*With the default configuration, RUN_SETUP will create directories for `node-1` to `node-5`*.

To add additional peers, append the `cargo run` command: cargo run ... 8002 8003 ... PORT_N.

This will be resolved as *http://127.0.0.1:SOME_PORT*.

## Submit proofs using Mock CLI

```bash
nix develop

cd mock_client

cargo test noir

cargo test circom
```

## Ping a node
`GET http://NODE_IP:PORT/ping`

## Query a node
```bash
GET http://NODE_IP:PORT/qnoir?id=SOME_PROOF_ID

GET http://NODE_IP:PORT/qcircom?id=SOME_PROOF_ID
```
