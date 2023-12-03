mod category;
mod date;
mod tx;
mod traits;
mod money;
mod wallet;
mod database;
mod parser;
mod application;

use std::alloc::System;

use application::Application;
use category::Category;
use date::Date;
use parser::XlsxParser;
use traits::Printable;
use tx::Transaction;
use money::{Money, Currency};
use wallet::Wallet;
use database::Database;



use calamine::{Reader, Xlsx, open_workbook};

fn main() {
    /* 
    let mut db = Database{
        category_database_name : String::from("category.json"),
        wallet_database_name : String::from("wallet.json"),
    };

    db.read_categories();

    Category::print_existing_categories();
 
    let tx1 = Transaction::new(
        Money::new(-150.0, Currency::TL),
        Date { day: 2, month: 10, year: 2023 }, 
        String::from("steam satin alimi"));

    let tx2 = Transaction::new(
        Money::new(-900.0, Currency::TL),
        Date { day: 3, month: 10, year: 2023 }, 
        String::from("getir siparişi"));

    let tx3 = Transaction::new(
        Money::new(-2200.0, Currency::TL),
        Date { day: 3, month: 10, year: 2023 },
        String::from("hastane ödemesi"));

    let tx4 = Transaction::new(
            Money::new(-231.0, Currency::TL),
            Date { day: 3, month: 10, year: 2023 }, 
            String::from("getiryemek.com"));

    let mut wallet1 = Wallet::new(Money::new(2810.0, Currency::TL));

    wallet1.add_tx(tx1);
    wallet1.add_tx(tx2);
    wallet1.add_tx(tx3);
    wallet1.add_tx(tx4);

    wallet1.print();


    wallet1.print_category_dist();
    

    //db.generate_category();
    db.category_terminal_change();

    db.save_categories();


    //println!("{}",serde_json::json!(tx1));

    */

    /* 
    let mut excel: Xlsx<_> = open_workbook("kk.xlsx").unwrap();
    if let Some(Ok(r)) = excel.worksheet_range("Sheet1") {
        for row in r.rows() {
            println!("row={:?}, row[0]={:?}", row, row[0]);
        }
    }*/
/* 
    let mut db = Database{
        category_database_name : String::from("category.json"),
        wallet_database_name : String::from("wallet.json"),
    };

    db.read_categories();

    let mut parser = XlsxParser::open("kk.xlsx");

    match parser.read_tx(){
        None => {},
        Some(txs) => {
            let mut w = Wallet::new(Money::new(0.0, Currency::TL));
            for tx in txs{
                if(tx.money.amount < 0.0){
                    w.add_tx(tx);
                }
            }
            w.handle_terminal();
        }
    }

    //db.category_terminal_change();
    db.save_categories();*/

    print!("\x1B[2J\x1B[1;1H");
    let mut app = Application::new();

    app.read_categories();
    app.handle_terminal();
    app.save_categories();
    
}