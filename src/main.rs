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

    let system = String::from("[System]");
    let alice = String::from("Alice");
    let bob = String::from("Bob");
    let charlie = String::from("Charlie");

    let mut blockchain = blockchain::Blockchain::new(4);

    blockchain.add_transaction(transaction::Transaction::new(
        system.clone(),
        alice.clone(),
        100,
    ));

    blockchain.add_transaction(transaction::Transaction::new(
        system.clone(),
        bob.clone(),
        100,
    ));

    blockchain.add_transaction(transaction::Transaction::new(
        system.clone(),
        charlie.clone(),
        100,
    ));

    blockchain.mine(system.clone());

    blockchain.add_transaction(transaction::Transaction::new(
        alice.clone(),
        bob.clone(),
        50,
    ));

    blockchain.add_transaction(transaction::Transaction::new(
        bob.clone(),
        charlie.clone(),
        75,
    ));

    blockchain.add_transaction(transaction::Transaction::new(
        charlie.clone(),
        alice.clone(),
        80,
    ));

    blockchain.mine(alice.clone());

    println!(
        "[Balance] Alice: {} Bob: {} Charlie: {}",
        blockchain.get_balance(&alice),
        blockchain.get_balance(&bob),
        blockchain.get_balance(&charlie)
    );

    println!("{:#?}", blockchain);
}
