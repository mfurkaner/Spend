use crate::tx::Transaction;
use crate::money::Money;
use crate::date::Date;
use crate::traits::Printable;


pub struct Wallet{
    money : Money,
    transactions : Vec<Transaction>,
}


impl Wallet{
    pub fn new(money: Money) -> Wallet{
        Wallet{money : money, transactions : vec!()}
    }

    pub fn add_tx(&mut self, tx: Transaction) {
        self.money.add(&tx.money);
        self.transactions.push(tx);
    }

    pub fn add_tx_new(&mut self, date : Date, money: Money, desc : &str, category_id : u16) {
        self.money.add(&money);
        self.transactions.push(Transaction{
            date : date,
            money : money,
            description : desc.to_string(),
            category_id : category_id
        });
    }
}

impl Printable for Wallet{
    fn print(&self){
        println!("Cüzdan durumu : ");
        print!("Bakiye:");
        self.money.print();
        if self.transactions.len() > 0{
            println!("\nİşlemler : ");
            for i in 0..self.transactions.len(){
                print!("{}) ", i + 1);
                self.transactions[i].print();
                println!();
            }
        }

    }
}