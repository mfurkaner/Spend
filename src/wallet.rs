use std::rc::Rc;

use crossterm::style::Stylize;

use crate::category::Category;
use crate::tx::Transaction;
use crate::money::Money;
use crate::date::Date;
use crate::traits::Printable;


#[derive(Clone)]
pub struct Wallet{
    id : u16,
    money : Money,
    transactions : Vec<Transaction>,
}
static mut EXISTING_WALLETS : Vec<Box<Wallet>> = Vec::new();

impl Wallet{
    fn generate_id() -> u16{
        let mut rv : u16;
        loop{
            rv = rand::random::<u16>();
            if rv == 0 {
                continue;
            }
            let mut exists = false;
            unsafe{
                for i in 0..EXISTING_WALLETS.len(){
                    if rv == EXISTING_WALLETS[i].id{
                        exists = true;
                        break;
                    }
                }
            }
            if !exists {
                break;
            }
        }
        rv
    }

    pub fn new(money: Money) -> Box<Wallet>{
        
        let nc = Box::new(Wallet{id : Wallet::generate_id(), money : money, transactions : vec!()});
        unsafe{
            EXISTING_WALLETS.push(nc.clone());
        }
        nc
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

    pub fn print_category_dist(&self){
        let mut categories_in : Vec<String> = Vec::new();
        let mut categories_money : Vec<Money> = Vec::new();
        let mut tot_money = Money::new(0.0, crate::money::Currency::TL);

        for i in 0..self.transactions.len(){
            let c = Category::get_by_id(self.transactions[i].category_id).unwrap();
            if !categories_in.contains(&c.name){
                categories_in.push(c.name.to_string());
                categories_money.push(self.transactions[i].money);
            }
            else{
                for j in 0..categories_in.len(){
                    if(categories_in[j] == c.name){
                        categories_money[j].add(&self.transactions[i].money);
                    }
                }
            }
            tot_money.add(&self.transactions[i].money);
        }

        println!("Distribution of category spendings : ");
        for i in 0..categories_in.len(){
            let perc = categories_money[i].amount / tot_money.amount;
            print!("{} : ", categories_in[i]);
            categories_money[i].print();
            println!("   {}%",format!("{:.2}", perc*100.0).on_cyan());
        }


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