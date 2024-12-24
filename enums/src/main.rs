mod library;
use library::TransactionType;
use std::io;

fn main() {
    let mut transactions: Vec<TransactionType> = Vec::new();
    loop {
        println!("Please enter the type of the transaction");
        println!("Options:");
        println!("0 - Exit");
        println!("1 - Deposit");
        println!("2 - Withdraw");
        println!("3 - Sign");
        println!("4 - Print transactions");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "0" {
            break;
        }
        if input.trim() == "1" {
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
            transactions.push(TransactionType::Deposit {
                amount,
                description,
            });
            continue;
        } else if input.trim() == "2" {
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
            transactions.push(TransactionType::Withdraw {
                amount,
                description,
            });
            continue;
        } else if input.trim() == "3" {
            let mut message = String::new();
            println!("Message: ");
            io::stdin()
                .read_line(&mut message)
                .expect("Failed to read line");
            println!("Description: ");
            let mut description = String::new();
            io::stdin()
                .read_line(&mut description)
                .expect("Failed to read line");
            transactions.push(TransactionType::Signature {
                message,
                description,
            });
            continue;
        }

        if input.trim() == "4" {
            for transaction in &transactions {
                transaction.print();
            }
            continue;
        }
    }
}
