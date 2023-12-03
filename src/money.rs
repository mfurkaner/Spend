use serde::{Serialize, Deserialize};
use colored::{Colorize, ColoredString};
use crate::traits::Printable;

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum Currency{
    TL,
    USD,
    EUR
}

impl Currency{
    pub fn to_str(&self) -> String{
        use Currency::*;

        return match self{
            TL => String::from("₺"),
            USD => String::from("$"),
            EUR => String::from("€")
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Money{
    pub amount : f32,
    pub currency : Currency
}

impl Money {
    pub fn new(amount : f32, currency : Currency) -> Money{
        Money{amount : amount, currency : currency}
    }

    pub fn _as_tl(&self) -> f32 {
        match self.currency {
            Currency::TL => self.amount,
            _ => 0.0
        }
    }
    
    pub fn add(& mut self, other : &Money){
        if self.currency == other.currency{
            self.amount = self.amount + other.amount;
        }
        else{
            panic!("Addition of different currencies!");
        }
    }

    pub fn from_str(source : &str) -> Option<Money>{
        struct Tp{
            curr : Currency,
            kws : Vec<&'static str>
        }
        let knowns = [
            Tp{curr: Currency::TL, 
                kws: ["TL", "₺", "tl"].to_vec()
            },
            Tp{curr: Currency::USD, 
                kws: ["USD", "$", "usd"].to_vec()
            },
            Tp{curr: Currency::EUR, 
                kws: ["EUR", "€", "eur"].to_vec()
            },
        ];
        let mut curr = Currency::TL;
        let mut found_curr = false;
        
        let mut v : Vec<&str> = source.split(' ').collect();
        let n : String;

        if v.len() == 3{
            for i in 0..v.len(){
                if v[i] == "-" && i + 1 < v.len(){
                    n  = v[i].to_owned() + v[i+1];
                    v[i] = &n;
                    v.remove(i + 1);
                    break;
                }
            }
        }
        if v.len() == 2{
            for i in knowns{
                for j in 0..i.kws.len(){
                    if v.contains(&i.kws[j]){
                        curr = i.curr;
                        found_curr = true;
                    }
                }
            }
            if found_curr == false {
                return None;
            }

            for i in v{
                let amount : f32 = match i.replace(".", "").replace(",", ".").parse(){
                    Ok(x) => x,
                    Err(_) => continue,
                };
                return Some(Money::new(amount, curr));
            }
        }

        None
    }
}


impl Printable for Money{
    fn print(&self){
        let amount : String = format!("{} {}", self.amount.to_string(), self.currency.to_str());
        let camount : ColoredString;
        if self.amount < 0.0 {
            camount = amount.bright_red().bold();
        }else{
            camount = amount.bright_green().bold();
        }
        print!("{}",  camount);
    }
}