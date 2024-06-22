/**
 * Zync
 *
 * A simple blockchain implementation in Rust.
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 */
mod block;
mod blockchain;
mod transaction;
mod utils;

fn main() {
    println!("Hello, Zync!");

    let mut blockchain = blockchain::Blockchain::new(4);

    blockchain.add_transaction(transaction::Transaction {
        id: 0,
        timestamp: 0,
        from: String::from("Alice"),
        to: String::from("Bob"),
        amount: 100,
    });

    blockchain.add_transaction(transaction::Transaction {
        id: 1,
        timestamp: 0,
        from: String::from("Bob"),
        to: String::from("Charlie"),
        amount: 200,
    });

    blockchain.mine(String::from("Alice"));

    blockchain.add_transaction(transaction::Transaction {
        id: 2,
        timestamp: 0,
        from: String::from("Charlie"),
        to: String::from("Bob"),
        amount: 300,
    });

    blockchain.mine(String::from("Bob"));

    println!("{:#?}", blockchain);
}
