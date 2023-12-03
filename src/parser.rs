use std::{io::BufReader, fs::File};

use calamine::{Reader, Xlsx, open_workbook, DataType};

use crate::{tx::Transaction, money::{Money,Currency}, date::Date};

pub struct XlsxParser{
    file : Xlsx<BufReader<File>>,
}

impl XlsxParser {
    pub fn open(filename :&str)-> XlsxParser{
        XlsxParser { file : open_workbook(filename).expect(format!("open {} failed.", filename).as_str())} 
    }


    pub fn read_tx(&mut self) -> Option<Vec<Transaction>>{
        let mut txs : Vec<Transaction> = Vec::new();


        if let Some(Ok(r)) = self.file.worksheet_range("Sheet1") {
            for row in r.rows() {
                if row[0].is_string() == false {
                    continue;
                }
                else if Date::can_parse(row[0].as_string().unwrap().as_str()) == false{
                    continue;
                }

                let mut money = Money::new(0.0, Currency::TL);
                let date =  match Date::from_str(row[0].as_string().unwrap().as_str()){
                    Some(x) => x,
                    None => continue,
                };
                let mut desc = String::new();

                let mut desc_set : bool = false;
                let mut money_set : bool = false;

                for i in 1..row.len(){
                    match &row[i]{
                        DataType::String(x) => {
                            if desc_set == false{
                                desc = x.to_string();
                                desc_set = true;
                            }
                            else if money_set == false{
                                match Money::from_str(row[i].as_string().unwrap().as_str()){
                                    Some(m) => {
                                        money = m;
                                        money.amount = money.amount * -1.0;
                                        money_set = true;
                                        break;
                                    },
                                    None => {}
                                }
                            }
                        },
                        _ => {continue;}
                    }
                }

                if desc_set && money_set {
                    txs.push(Transaction::new(money, date, desc));
                }
            }
        }

        if txs.len() > 0{
            return Some(txs);
        }
        return None;
    }
}