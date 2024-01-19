<div align="center">

  <h1><code>Krown POS Blockchain Node</code></h1>

  <strong>Blockchain Network Node for the Krown Network. This is a Rust Blockchain project using <a href="https://github.com/paritytech/substrate">Substrate</a>.</strong>

  <h3>
    <a href="https://docs.substrate.io/">docs.Substrate.io</a>
    <span> | </span>
    <a href="https://paritytech.github.io/polkadot-sdk/master/sc_service/index.html">Rust Docs</a>
  </h3>

</div>

## Features

* Consensus related pallets: babe & grandpa
* Staking related pallets: staking, session, authorship, im-online, offences, utility
* Wallet related pallets: multisig, nicks,

## Develop with Krown-Blockchain

Follow the steps below to get started.

### Rust Setup

First, complete the [Dev Docs Installation](https://docs.substrate.io/install/).

### Build and Run

Use the following command to build the node and run it after build successfully:

```sh
cargo build --release
./target/release/krown-node --dev
```

## Run local testnet
* Clear Database (fresh start):
```shell 
  ./target/release/krown-node purge-chain --base-path /tmp/alice --chain local
```
* Generate Node Key:
```shell
  ./target/release/krown-node key generate-node-key
```
* Start First Node on Local Machine:
```shell
  ./target/release/krown-node \
    --base-path /tmp/alice \
    --chain local \
    --name alice \
    --port 30333 \
    --rpc-port 9944 \
    --node-key <your-node-key>
```
* Start Second Terminal on Local Machine
* Clear Database for Validator Bob (fresh start)
```shell
  ./target/release/krown-node purge-chain --base-path /tmp/bob --chain local -y
```
* Start the Validator Node
  (Replace IP and Bootnode-ID)
```shell
  ./target/release/krown-node \
    --base-path /tmp/bob \
    --chain local \
    --bob \
    --port 30334 \
    --rpc-port 9946 \
    --validator \    
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/<your-bootnode-peerid> \
```

### In Case you want to add more Validators:
* Start Third Terminal on Local Machine
* Clear Database for Validator example Dev Account "Dave" (fresh start)
```shell
  ./target/release/krown-node purge-chain --base-path /tmp/bob --chain local -y
```
* Start the Dave Validator Node
  (Replace IP and Bootnode-ID)
```shell
  ./target/release/krown-node \
    --base-path /tmp/dave \
    --chain local \
    --dave \
    --port 30334 \
    --rpc-port 9947 \
    --validator \
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/<your-bootnode-peerid>
```
* Add Validator via polkadot-js Apps UI https://polkadot.js.org/
* Open New Terminal and Call author_rotateKeys
```shell
  curl -H "Content-Type: application/json" --data '{"id":1, "jsonrpc":"2.0", "method": "author_rotateKeys", "params":[]}' http://localhost:9947
```
* Add rotateKey to Validator registration in polkadot-js Apps UI


## Run public testnet

* Purge Chain Data: `./target/release/krown-node purge-chain --base-path /tmp/bootnode1 --chain local`
* Build spec, `./target/release/krown-node build-spec --chain staging > krown-staging.json`
* Change original spec to encoded raw spec, `./target/release/krown-node build-spec --chain=krown-staging.json --raw > krown-staging-raw.json`
* Start your bootnodes, node key can be generate with command `./target/release/krown-node key generate-node-key`.
  ```shell
  ./target/release/krown-node \
       --node-key <your-node-key> \
       --base-path /tmp/bootnode1 \
       --chain krown-staging.json \
       --name bootnode1
  ```
* Start your initial validators,
  ```shell
    ./target/release/krown-node \
      --base-path  /tmp/validator1 \
      --chain   krown-staging-json-raw.json \
      --bootnodes  /ip4/<your-bootnode-ip>/tcp/30333/p2p/<your-bootnode-peerid> \
	    --port 30336 \
	    --ws-port 9947 \
	    --rpc-port 9936 \
      --name  validator1 \
      --validator
  ```
* [Insert session keys for Babe & Grandpa](https://docs.substrate.io/tutorials/build-a-blockchain/add-trusted-nodes/#generate-your-account-and-keys)
* Attract enough validators in waiting position
* Call force_new_era in staking pallet with sudo, rotate to PoS validators
* Check if transfer is enabled and other functions
