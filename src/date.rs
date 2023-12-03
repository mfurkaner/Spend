use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Date{
    pub day : u8,
    pub month : u8,
    pub year : u16
}

impl Date{
    pub fn new(day: u8, month: u8, year: u16) -> Date{
        Date{day, month, year}
    }
    pub fn to_str(&self) -> String{
        format!("{:0>2}-{:0>2}-{:0>4}", self.day, self.month, self.year)
    }

    pub fn can_parse(source : &str) -> bool{
        let v : Vec<&str>;
        if source.contains('/'){
            v = source.split('/').collect();
        }
        else if source.contains('-'){
            v = source.split('-').collect();
        }
        else{
            return false;
        }

        if v.len() == 3 {
            for elem in v{
                let _ : u16 = match elem.parse(){
                    Ok(x) => x,
                    Err(_) => {
                        return false;
                    }
                };
            }
            return true
        }

        false
    }

    pub fn from_str(source : &str) -> Option<Date>{
        if Date::can_parse(source){
            let v : Vec<&str>;
            if source.contains('/'){
                v = source.split('/').collect();
            }
            else if source.contains('-'){
                v = source.split('-').collect();
            }
            else{
                return None;
            }

            let d : u8 = v[0].parse().expect(format!("somehow {} could not be parsed to u8", v[0]).as_str());
            let m : u8 = v[1].parse().expect(format!("somehow {} could not be parsed to u8", v[1]).as_str());
            let y : u16 = v[2].parse().expect(format!("somehow {} could not be parsed to u16", v[2]).as_str());

            return Some(Date::new(d, m, y));
        }
        None
    }
}