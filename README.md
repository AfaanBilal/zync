Zync
====

A simple Blockchain implementation in Rust.

Author: **[Afaan Bilal](https://afaan.dev)**

## Run

```bash
$ cargo run

Hello, Zync!
Block mined! ID: 1 Hash: 0000c1a341766a840d6a673fd78a7a6f7a4093285e42a85f526e2ed82b84fa03
Block mined! ID: 2 Hash: 000022c3a392acc1dd2ffe580c2fa22fc38b5229818f380007ead9793c3d64f9
Blockchain {
    chain: [
        Block {
            id: 0,
            timestamp: 1719061733,
            data: [],
            prev_hash: "",
            hash: "4594d3aa5b342ab305fbd0ce117e9ca203f085aa7c1264c080dc88a5d4a664b9",
            nonce: 0,
        },
        Block {
            id: 1,
            timestamp: 1719061733,
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
            prev_hash: "4594d3aa5b342ab305fbd0ce117e9ca203f085aa7c1264c080dc88a5d4a664b9",
            hash: "0000c1a341766a840d6a673fd78a7a6f7a4093285e42a85f526e2ed82b84fa03",
            nonce: 16464,
        },
        Block {
            id: 2,
            timestamp: 1719061733,
            data: [
                Transaction {
                    id: 0,
                    timestamp: 1719061733,
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
            prev_hash: "0000c1a341766a840d6a673fd78a7a6f7a4093285e42a85f526e2ed82b84fa03",
            hash: "000022c3a392acc1dd2ffe580c2fa22fc38b5229818f380007ead9793c3d64f9",
            nonce: 76159,
        },
    ],
    difficulty: 4,
    pending_transactions: [
        Transaction {
            id: 0,
            timestamp: 1719061735,
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
