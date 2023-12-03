use std::{io::{self, Write}, fs};

use colored::Colorize;

use crate::{database::Database, wallet::Wallet, money::{Money, Currency}, parser::XlsxParser};

pub struct Application{
    db: Database,
    wallet: Box<Wallet>,
}

impl Application{
    pub fn new() -> Application{
        Application{
            db : Database {
                category_database_name : String::from("category.json"),
                wallet_database_name : String::from("wallet.json"),
            },
            wallet : Wallet::new(Money::new(0.0, Currency::TL))
        }
    }


    pub fn handle_terminal(&mut self){
        loop{
            let mut command = String::new();
            println!("\n###########################################");
            println!("İlerlemek istediğiniz menüyü seçin : ");
            println!("- {} : {}\n- {} : {}\n- {} : {}\n- {} : {}",
            "Verileri oku".yellow().italic(), ".o".yellow().italic(),
            "Cüzdan".blue().italic(), ".c".blue().italic(), 
            "Kategori veritabanı".green().italic(), ".k".green().italic(),
            "Çıkış".red(), ".q".red());
            _ = io::stdout().flush();
            _ = io::stdin().read_line(&mut command);
            println!("###########################################\n");
            match command.trim_end(){
                ".o" => self.read_from_file(),
                ".c" => self.wallet.handle_terminal(),
                ".k" => self.db.category_terminal_change(),
                ".q" => break,
                _ => { eprintln!("Yanlış komut girişi yaptınız! Girdiğiniz konutu kontrol edin : {}", command.trim_end()) }
            }

        }
    }


    pub fn read_categories(&mut self){
        self.db.read_categories();
    }

    pub fn save_categories(&mut self){
        self.db.save_categories();
    }

    pub fn read_from_file(&mut self){
        loop{
            let mut command = String::new();
            println!("\n###########################################");
            print!("Dosya adını girin (dönüş : '{}' ) : ", ".q".red().bold());
            _ = io::stdout().flush();
            _ = io::stdin().read_line(&mut command);

            if command.trim_end() == ".q"{
                break;
            }
    
            match fs::metadata(command.trim_end()){
                Ok(_) => {},
                Err(_) => {
                    println!("{} adlı dosya bulunamadı!", command.trim_end());
                    continue;
                }
            }
    
            let mut parser = XlsxParser::open(command.trim_end());
            match parser.read_tx(){
                None => {eprintln!("{} adlı dosyada girdi bulunamadı!", command.trim_end());},
                Some(txs) => {
                    for tx in txs{
                        if tx.money.amount < 0.0 {
                            self.wallet.add_tx(tx);
                        }
                    }
                    break;
                }
            }
        }

    }

}