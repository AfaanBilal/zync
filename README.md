Zync
====

A simple Blockchain implementation in Rust.

Author: **[Afaan Bilal](https://afaan.dev)**

## Run

```bash
$ cargo run

Hello, Zync!
Block mined! ID: 1 Hash: 0000a7c7654733801e7c78a5143ddb1f78e9101cf27125a0662b5b62fc1f065a Time: 167.2302ms
Block mined! ID: 2 Hash: 0000f6a8a21c32fd1a1cd6ead6701df224973a26132dbd76041e9e6cdb4752d8 Time: 514.7356ms
Blockchain {
    chain: [
        Block {
            id: 0,
            timestamp: 1719068416,
            data: [],
            prev_hash: "",
            hash: "8d32e82c77a6f1eab1ffaf3c9ea0c0cdad15e9194b3c72f5b0c2593759f1dee3",
            nonce: 0,
        },
        Block {
            id: 1,
            timestamp: 1719068416,
            data: [
                Transaction {
                    id: 0,
                    timestamp: 0,
                    from: "Alice",
                    to: "Bob",
                    amount: 100,
                },
                Transaction {
                    id: 1,
                    timestamp: 0,
                    from: "Bob",
                    to: "Charlie",
                    amount: 200,
                },
            ],
            prev_hash: "8d32e82c77a6f1eab1ffaf3c9ea0c0cdad15e9194b3c72f5b0c2593759f1dee3",
            hash: "0000a7c7654733801e7c78a5143ddb1f78e9101cf27125a0662b5b62fc1f065a",
            nonce: 8621,
        },
        Block {
            id: 2,
            timestamp: 1719068416,
            data: [
                Transaction {
                    id: 0,
                    timestamp: 1719068416,
                    from: "System",
                    to: "Alice",
                    amount: 100,
                },
                Transaction {
                    id: 2,
                    timestamp: 0,
                    from: "Charlie",
                    to: "Bob",
                    amount: 300,
                },
            ],
            prev_hash: "0000a7c7654733801e7c78a5143ddb1f78e9101cf27125a0662b5b62fc1f065a",
            hash: "0000f6a8a21c32fd1a1cd6ead6701df224973a26132dbd76041e9e6cdb4752d8",
            nonce: 25926,
        },
    ],
    difficulty: 4,
    pending_transactions: [
        Transaction {
            id: 0,
            timestamp: 1719068417,
            from: "System",
            to: "Bob",
            amount: 100,
        },
    ],
    mining_reward: 100,
}
```

## License

[MIT](https://choosealicense.com/licenses/mit/)
