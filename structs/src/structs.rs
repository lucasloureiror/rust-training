pub struct Transaction {
    pub id: u64,
    pub amount: f64,
    pub description: String,
}

pub struct Ledger {
    pub transactions: Vec<Transaction>,
}

impl Ledger{
    pub fn print(&self){
        println!("\n\nLedger");

        for transaction in &self.transactions {
            println!("Id: {}", transaction.id);
            println!("Amount: {}", transaction.amount);
            println!("Description: {}", transaction.description);
        }
    }
}