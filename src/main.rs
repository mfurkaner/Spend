mod category;
mod date;
mod tx;
mod traits;
mod money;
mod wallet;
mod database;

use category::Category;
use date::Date;
use traits::Printable;
use tx::Transaction;
use money::{Money, Currency};
use wallet::Wallet;
use database::Database;


use serde_json;



fn main() {
    //let fun = Category::new(String::from("Eğlence"), vec![String::from("steam")]);
    //let ek_gelir = Category::new(String::from("Ek gelir"), vec![String::from("ersag")]);

    let mut db = Database{
        category_database_name : String::from("category.json"),
        wallet_database_name : String::from("wallet.json"),
    };

    db.read_categories();

    Category::print_existing_categories();
 /* 
    let tx1 = Transaction::new(
        Money::new(-150.0, Currency::TL),
        Date { day: 2, month: 10, year: 2023 }, 
        String::from("steam satin alimi"));

    let tx2 = Transaction::new(
        Money::new(-900.0, Currency::TL),
        Date { day: 3, month: 10, year: 2023 }, 
        String::from("getir siparişi"));

    let tx3 = Transaction::new(
        Money::new(2200.0, Currency::TL),
        Date { day: 3, month: 10, year: 2023 },
        String::from("ersag ödemesi"));

        let tx4 = Transaction::new(
            Money::new(-231.0, Currency::TL),
            Date { day: 3, month: 10, year: 2023 }, 
            String::from("getiryemek.com"));

    let mut wallet1 = Wallet::new(Money::new(2810.0, Currency::TL));

    wallet1.add_tx(tx1);
    wallet1.add_tx(tx2);
    wallet1.add_tx(tx3);
    wallet1.add_tx(tx4);

    wallet1.print();*/

    

    //db.generate_category();
    db.category_terminal_change();

    db.save_categories();


    //println!("{}",serde_json::json!(tx1));

}