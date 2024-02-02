# Zk-node specification and documentation
Last updated: `v0.0.1`

Find instructions on how to operate a network [here](https://github.com/cspr-rad/paradox-6/blob/master/SETUP.md)

This document serves as a temporary documentation sheet for the zk-node and api.

## Proofs
Overview of proof datastructures for `noir` and `circom`.

### Proof enum

|      Proof   |            |
|--------------|------------|
| CircomProof  | NoirProof  |


### Proof types

| CircomProof | NoirProof |
|-------------|-----------|
| hash        | hash      |
| vk          | verifier  |
| inputs      |           |
| proof       | proof     |
| circuit     | circuit   |


`hash`: The hash identifier of a proof (used by eventstream/logger)

`vk`: verifying key

`inputs` / `verifier`: public inputs to the circuit

`proof`: the snark proof to be verifier

`circuit`: the identifier of the associated circuit

## Circuits
Supported circuits as of `v0.0.1`

### Circom
- `multiplier`

### Noir
- `multiplier`
- `signature`
- `test`


## Storage
Every proof is stored using the `Storage` module in the associated database (e.g. `circom.db`, `noir.db`, ...):

```rust
impl UniversalStorage for Storage {
    fn create(&self) -> Result<()> {
        let conn = Connection::open(&self.path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS data (
                    id INTEGER PRIMARY KEY,
                    hash TEXT NOT NULL,
                    proof TEXT NOT NULL
                )",
            [],
        )?;
        Ok(())
    }
}
```

## Receiver
The asynchronous receiver handles proofs for any supported backend in a tokio runtime:
```rust
#[async_trait]
impl NoirThread for Thread{        
        tokio::spawn(async move {
            ... // backend specific verifier construction
                thread.store_blocking(storer.clone(), proof.clone()).await;
                thread.gossip_blocking().await;
            ...
        });
}
        


#[async_trait]
impl CircomThread for Thread{
    async fn spawn(&self, storer: Storage){
        tokio::spawn(async move {
        ... // backend specific verifier construction
                thread.store_blocking(storer.clone(), proof.clone()).await;
                thread.gossip_blocking().await;
        ...
    });
    }
}
```

## API 
Proofs are submitted and retrieved via the rocket `v0.5.0` API.

### Submit a proof
See `mock_client` for examples.

Using the ProverCli:

```rust
let response: reqwest::blocking::Response = ProverCli::submit(&cli, proof_message_serialized).unwrap();
assert!(response.status().is_success());
```

all proofs are serialized using `serde_json` and matched by the receiver to identify the backend they were created for and verify them accordingly.

### Query a proof
List of routes:
```bash
/qcircom?<id>
/qnoir?<id>

# Supported Backends
- noir (binary)
tested with nargo `v0.15.0`
- circom (arkworks / Rust)
