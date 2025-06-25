Goal

Build an ANS-104 indexer CLI for the Arweave network in Rust.

The CLI should allow you to pass in an ANS-104 bundle transaction ID, index it and dump a parsed array to a file. Bundles use the ANS-104 specification.

To Run:

1.) Go to Meta Bundler for Arweave: https://viewblock.io/arweave/address/yCxjLRyXjzHJ4gMZK8HFYiW146dykI9QgP6CSsVXFwk

2.) Pick any tx hash with format binary and version 2.0.0 and copy it

3.) cargo run -- <tx_id> --out <output.json>

example: 

cargo run -- IVwoQf3SWY7aVQyNJH4WjV28vEP7hROHnOo3JalJalo --out bundle.json 
