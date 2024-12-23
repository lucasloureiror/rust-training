use std::io;

struct Transaction {
    id: u64,
    amount: f64,
    description: String,
}
fn main() {
    let mut ledger: Vec<Transaction> = Vec::new();

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
            print_ledger(&ledger);
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

        ledger.push(transaction);
        counter += 1;
    }
}

fn print_ledger(vec: &Vec<Transaction>) {

        println!("\n\nLedger");

        for transaction in vec {
            println!("Id: {}", transaction.id);
            println!("Amount: {}", transaction.amount);
            println!("Description: {}", transaction.description);
        }
    
}
