use crate::date::Date;
use crate::money::Money;
use crate::category::Category;
use crate::traits::Printable;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Transaction{
    pub date : Date,

    pub money : Money,

    pub category_id : u16,
    pub description : String,
}

impl Transaction{
    pub fn new(money : Money, date : Date, desc : String) -> Transaction{
        let id = match Category::get_id_by_desc(desc.as_str()) {
            Some(i) => i,
            None => 0
        };
        Transaction { date: date, money: money, category_id: id, description: desc }
    }

    pub fn get_category(&self) -> Option<Box<Category>>{
        Category::get_by_id(self.category_id)
    }


}

impl Printable for Transaction {
    fn print(&self){
        println!("{}", self.description);
        self.money.print();
        println!("\n{}", self.date.to_str());
        let category_name =  match self.get_category() {
            Some(c) => c.name.clone(),
            None => panic!("Category with id {} not found", self.category_id)
        };
        println!("Category: {}", category_name);
    }
}