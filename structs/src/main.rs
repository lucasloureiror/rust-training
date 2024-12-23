use std::io;
mod structs;
use structs::Transaction;
use structs::Ledger;

fn main() {
    let mut ledger = Ledger {
        transactions: Vec::new(),
    };

    let mut counter: u64 = 0;

    loop {
        println!("Transaction {counter}");
        println!("Options: 0 to exit, 1 to continue and 2 to show ledger");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "0" {
            break;
        }
        if input.trim() == "2" {
            ledger.print();
            continue;
        }

        let mut amount = String::new();
        println!("Amount: ");
        io::stdin()
            .read_line(&mut amount)
            .expect("Failed to read line");
        let amount: f64 = amount.trim().parse().expect("Please type a number!");

        let mut description = String::new();
        println!("Description: ");
        io::stdin()
            .read_line(&mut description)
            .expect("Failed to read line");

        let transaction = Transaction {
            id: counter,
            amount: amount,
            description: description,
        };

        ledger.transactions.push(transaction);
        counter += 1;
    }
}
