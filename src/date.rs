use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Date{
    pub day : u8,
    pub month : u8,
    pub year : u16
}

impl Date{
    pub fn to_str(&self) -> String{
        format!("{:0>2}-{:0>2}-{:0>4}", self.day, self.month, self.year)
    }
}