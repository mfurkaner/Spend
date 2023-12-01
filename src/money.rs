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

#[derive(Serialize, Deserialize)]
pub struct Money{
    amount : f32,
    currency : Currency
}

impl Money {
    pub fn new(amount : f32, currency : Currency) -> Money{
        Money{amount : amount, currency : currency}
    }

    pub fn as_tl(&self) -> f32 {
        match self.currency {
            Currency::TL => self.amount,
            _ => 0.0
        }
    }
    
    pub fn add(& mut self, other : &Money){
        if(self.currency == other.currency){
            self.amount = self.amount + other.amount;
        }
        else{
            panic!("Addition of different currencies!");
        }
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
        println!("{}",  camount);
    }
}