pub enum TransactionType {
    Deposit {
        amount: f64,
        description: String,
    },
    Withdraw {
        amount: f64,
        description: String,
    },
    Signature {
        message: String,
        description: String,
    },
}

impl TransactionType {
    pub fn print(&self) {
        match self {
            TransactionType::Deposit {
                amount,
                description,
            } => println!("Deposit: {} {}", amount, description),
            TransactionType::Withdraw {
                amount,
                description,
            } => println!("Withdraw: {} {}", amount, description),
            TransactionType::Signature {
                message,
                description,
            } => println!("Signature: {} {}", message, description),
        }
    }
}
