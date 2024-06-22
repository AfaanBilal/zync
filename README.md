Zync
====

A simple Blockchain implementation in Rust.

Author: **[Afaan Bilal](https://afaan.dev)**

## Run

```bash
$ cargo run

Hello, Zync!
Block mined! ID: 1 Time: 1.1026918s Hash: 00004b35c9ae9836279115a7ea87329776b546bebec42387f1cb09b401c1c679
Block mined! ID: 2 Time: 705.8919ms Hash: 0000477746bcbb80e6e6f8e068e154dd6899180a438e1dcfaa54afbcefbab001
Blockchain {
    chain: [
        Block {
            id: 0,
            timestamp: 1719068800,
            data: [],
            prev_hash: "",
            hash: "31dfce3618f176ccc6c024ff15b9668b993d098ffb3140a8079eb4676fe1504a",
            nonce: 0,
        },
        Block {
            id: 1,
            timestamp: 1719068800,
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
            prev_hash: "31dfce3618f176ccc6c024ff15b9668b993d098ffb3140a8079eb4676fe1504a",
            hash: "00004b35c9ae9836279115a7ea87329776b546bebec42387f1cb09b401c1c679",
            nonce: 56642,
        },
        Block {
            id: 2,
            timestamp: 1719068801,
            data: [
                Transaction {
                    id: 0,
                    timestamp: 1719068801,
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
            prev_hash: "00004b35c9ae9836279115a7ea87329776b546bebec42387f1cb09b401c1c679",
            hash: "0000477746bcbb80e6e6f8e068e154dd6899180a438e1dcfaa54afbcefbab001",
            nonce: 35740,
        },
    ],
    difficulty: 4,
    pending_transactions: [
        Transaction {
            id: 0,
            timestamp: 1719068802,
            from: "System",
            to: "Bob",
            amount: 100,
        },
    ],
    mining_reward: 100,
}
```

## License

[MIT](LICENSE)
