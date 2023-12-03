use std::io::{self, Write};
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


    pub fn handle_terminal(&mut self){

        loop{
            let mut command = String::new();
            println!("\n###########################################");
            println!("Cüzdanında yapmak istediğin işlemi seç: ");
            println!("- {} : {}\n- {} : {}\n- {} : {}\n- {} : {}\n- {} : {}", 
            "Cüzdanı detaylı yazdır".blue().italic(), ".p".blue().italic(), 
            "Bir kategorideki harcamalarını yazdır".green().italic(), ".pt".green().italic(),
            "Kategorilendirilmemiş harcamalarını yazdır".yellow().italic(), ".pud".yellow().italic(),
            "Kategori harcama dağılımı".magenta().italic(), ".pd".magenta().italic(),
            "geri".red(), ".q".red());
            _ = io::stdout().flush();
            _ = io::stdin().read_line(&mut command);
            println!("###########################################\n");
            match command.trim_end(){
                ".p" => self.print(),
                ".pt" => self.print_by_category_name_from_user(),
                ".pud" => self.print_undefined_txs(),
                ".pd" => self.print_category_dist(),
                ".q" => break,
                _ => { eprintln!("Yanlış komut girişi yaptınız! Girdiğiniz konutu kontrol edin : {}", command.trim_end()) }
            }

        }
    }

}

impl Printable for Wallet{
    fn print(&self){
        println!("###########################################");
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
        println!("###########################################");
    }
}

impl Wallet{
    fn print_by_category_name_from_user(&self){
        println!("###########################################");
        loop{
            println!("Kategorilerim : ");
            Category::print_existing_category_names();
            print!("Yazdırmak istediğin kategorinin {} gir ({} : return): ", "ismini".bold(), ".q".red().bold());
            let mut command = String::new();
            _ = io::stdout().flush();
            _ = io::stdin().read_line(&mut command);
            if command.trim_end() == ".q"{
                break;
            }
            match self.print_by_category_name(command.trim_end()){
                true => break,
                false => {
                    eprintln!("{} bilinen kategoriler arasında yok.", command.trim_end());
                }
            }
        }
        println!("###########################################");
    }
}

impl Wallet{

    pub fn print_undefined_txs(&self){
        if self.transactions.is_empty(){
            return;
        }
        println!("###########################################");
        let mut there_are_undefined_txs = false;
        for i in 0..self.transactions.len(){
            let c = Category::get_by_id(self.transactions[i].category_id).unwrap();
            if c.id == 0{
                there_are_undefined_txs = true;
                break;
            }
        }
        if there_are_undefined_txs == false {
            println!("Kategorilendirilmemiş işlem bulunmamakta.");
            println!("###########################################");
            return;
        }

        println!("\nKategorilendirilmemiş işlemler : ");
        let mut count : usize = 0;
        for i in 0..self.transactions.len(){
            let c = Category::get_by_id(self.transactions[i].category_id).unwrap();
            if c.id == 0 {
                print!("{}) ", count + 1);
                self.transactions[i].print();
                println!();
                count+=1;
            }
        }
        println!("###########################################");
    }

    pub fn print_by_category_name(&self, category_name : &str) -> bool{
        let x = match Category::get_by_name(category_name){
            Some(c) => c,
            None => {
                return false;
            }
        };
        self.print_by_category_id(x.id);
        true
    }

    pub fn print_by_category_id(&self, category_id : u16){
        if self.transactions.is_empty(){
            return;
        }
        if category_id == 0{
            return self.print_undefined_txs();
        }
        let x = Category::get_by_id(category_id).expect("Bu idli bir kategori yok.");
        println!("\n{} kategorisindeki işlemler : ", x.name);
        let mut count : usize = 0;
        for i in 0..self.transactions.len(){
            let c = Category::get_by_id(self.transactions[i].category_id).unwrap();
            if c.id == category_id {
                
                print!("{}) ", count + 1);
                self.transactions[i].print();
                println!();
                count+=1;
            }
        }
    }

    pub fn print_category_dist(&self){
        let mut categories_in : Vec<String> = Vec::new();
        let mut categories_money : Vec<Money> = Vec::new();
        let mut tot_money = Money::new(0.0, crate::money::Currency::TL);

        for i in 0..self.transactions.len(){
            let c = Category::get_by_id(self.transactions[i].category_id).unwrap();
            if !categories_in.contains(&c.name.to_string()){
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
        println!("###########################################");
        print!("Cüzdanda toplam harcama : ");
        tot_money.print();
        println!();
        println!("Harcama kategorileri : ");

        for i in 0..categories_in.len(){
            let perc = categories_money[i].amount / tot_money.amount;
            if categories_in[i].is_empty(){
                categories_in[i] = crate::category::UNKNOWN_CATEGORY_NAME.to_string();
            }
            print!("{} : ", categories_in[i]);
            categories_money[i].print();
            println!("   {}",format!("{:.2}%", perc*100.0).yellow());
        }

        println!("###########################################");
    }
}