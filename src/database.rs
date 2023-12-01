use std::fs;
use std::io;
use std::io::Write;
use crate::category::Category;
use crate::traits::Printable;
use serde_json;
use colored::*;

pub struct Database{
    pub category_database_name: String,
    pub wallet_database_name: String
}

impl Database {
    //// CATEGORY DATABASE IMPL START
    pub fn read_categories(&self){
        let res = fs::read_to_string(self.category_database_name.as_str()).unwrap();
        let read : Vec<Category> = serde_json::from_str(&res).unwrap();

        for cat in read {
            Category::insert_into_existing_categories(cat);
        }
    }

    pub fn save_categories(&self){
        let cat_json = Category::serialize_existing_categories();

        match fs::write(self.category_database_name.as_str(), cat_json) {
            Ok(_) => {},
            Err(e) => eprint!("Error writing category database : {}",e)
        }
    }

    pub fn generate_category(&self){
        let mut command = String::new();
        print!("Enter a {} for the category you want to create\n : ", "name".bold());
        _ = io::stdout().flush();
        _ = io::stdin().read_line(&mut command);
        let name = command.trim_end().to_string();
        let mut kws : Vec<String> = Vec::new();

        println!("Enter the {} of this category (maximum 10 keywords, '{}' to exit)", "keywords".bold(), ".q".red().bold());
        
        for i in 0..10{
            let mut command = String::new();
            print!("keyword {} : ", i + 1);
            _ = io::stdout().flush();
            _ = io::stdin().read_line(&mut command);
            if command.trim_end() == ".q"{
                break;
            }
            kws.push(command.trim_end().to_string());
        }
        Category::new(name, kws);
    }

    pub fn edit_category(&self){
        let mut command = String::new();
        print!("Enter the {} of the category you want to edit\n : ", "id".bold());
        _ = io::stdout().flush();
        _ = io::stdin().read_line(&mut command);
        let id : u16 = command.trim_end().to_string().parse().unwrap();

        let c = Category::get_by_id(id).unwrap();
        let mut name = c.name.to_string();
        let mut kws : Vec<String> = Vec::new();
        for s in c.keywords.iter(){
            kws.push(s.to_string());
        }

        println!("Editing category : ");
        c.print();

        loop{
            println!("   edit - {} : {}, {} : {}, {} : {}, {} : {}", 
            "name".blue().italic(), ".n".blue().italic(), 
            "add keyword".green().italic(), ".ak".green().italic(),
            "remove keyword".yellow().italic(), ".rk".yellow().italic(),
            "stop edit".red(), ".q".red());
            print!("      : ");
            _ = io::stdout().flush();
            let mut command = String::new();
            _ = io::stdin().read_line(&mut command);

            if command.trim_end() == ".n" {
                println!("   New {} : ", "name".bold());
                _ = io::stdout().flush();
                _ = io::stdin().read_line(&mut command);
                name = command.trim_end().to_string();
            }
            else if command.trim_end() == ".ak" {
                print!("   New {} : ", "keyword".bold());
                _ = io::stdout().flush();
                let mut nk = String::new();
                _ = io::stdin().read_line(&mut nk);
                kws.push(nk.trim_end().to_string());
            }
            else if command.trim_end() == ".rk" {
                print!("   Remove {} : ", "keyword".bold());
                _ = io::stdout().flush();
                let mut rk = String::new();
                _ = io::stdin().read_line(&mut rk);
                for i in 0..kws.len() {
                    if kws[i] == rk.trim_end(){
                        kws.remove(i);
                    }
                }
            }
            else if command.trim_end() == ".q"{
                break;
            }
            else{
                panic!("Unknown command");
            }
        }

        Category::replace(id, name, kws);
    }

    pub fn remove_category(&mut self){
        let mut command = String::new();
        print!("Enter the {} of the category you want to remove\n : ", "id".bold());
        _ = io::stdout().flush();
        _ = io::stdin().read_line(&mut command);
        let id : u16 = command.trim_end().to_string().parse().unwrap();

        let c = Category::get_by_id(id).unwrap();

        println!("{} category : ", "Removing".red().bold());
        c.print();
        print!("Are you sure? ('{}', '{}') : ", "y".bold(), "n".bold());
        _ = io::stdout().flush();
        let mut command = String::new();
        _ = io::stdin().read_line(&mut command);
        if command.trim_end() == "y"{
            Category::remove_by_id(id);
            println!("Category removed.");
        }
    }
    //// CATEGORY DATABASE IMPL END
    

}