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
        print!("Oluşturmak istediğin kategorinin {} : ", "ismi".bold());
        _ = io::stdout().flush();
        _ = io::stdin().read_line(&mut command);
        let name = command.trim_end().to_string();
        let mut kws : Vec<String> = Vec::new();

        println!("Kategorinin {} (max 10, dönmek için '{}')", "anahtar kelimeleri".bold(), ".q".red().bold());
        
        for i in 0..10{
            let mut command = String::new();
            print!("anahtar kelime {} : ", i + 1);
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
        let mut c : Box<Category>;
        loop {
            let mut command = String::new();
            Category::print_existing_category_names();
            print!("Düzenlemek istediğin kategorinin {} (dönmek için '{}')\n : ", "ismi".bold(), ".q".red());
            _ = io::stdout().flush();
            _ = io::stdin().read_line(&mut command);
            if command.trim_end() == ".q"{
                return;
            }
    
            match Category::get_by_name(command.trim_end()) {
                Some(a) => {
                    c = a;
                    break;
                },
                None => {
                    eprintln!("{} adlı kategori bulunamadı.", command.trim_end())
                }
            };
        }

        let mut name = c.name.to_string();
        let mut kws : Vec<String> = Vec::new();
        for s in c.keywords.iter(){
            kws.push(s.to_string());
        }

        println!("Düzenlenen kategori : ");
        c.print();

        loop{
            println!(" {} : {}\n {} : {}\n {} : {}\n {} : {}", 
                    "isim".blue().italic(), ".n".blue().italic(), 
                    "anahtar kelime ekle".green().italic(), ".ak".green().italic(),
                    "anahtar kelime çıkart".yellow().italic(), ".rk".yellow().italic(),
                    "düzenlemeyi bitir".red(), ".q".red());
            print!("      : ");
            _ = io::stdout().flush();
            let mut command = String::new();
            _ = io::stdin().read_line(&mut command);

            match command.trim_end() {
                ".n" => {
                    println!("   Yeni {} : ", "isim".bold());
                    _ = io::stdout().flush();
                    _ = io::stdin().read_line(&mut command);
                    name = command.trim_end().to_string();
                }
                ".ak" => {
                    print!("   Yeni {} : ", "anahtar kelime".bold());
                    _ = io::stdout().flush();
                    let mut nk = String::new();
                    _ = io::stdin().read_line(&mut nk);
                    kws.push(nk.trim_end().to_string());
                }
                ".rk" => {
                    print!("   Çıkartılacak {} : ", "anahtar kelime".bold());
                    _ = io::stdout().flush();
                    let mut rk = String::new();
                    _ = io::stdin().read_line(&mut rk);
                    for i in 0..kws.len() {
                        if kws[i] == rk.trim_end(){
                            kws.remove(i);
                            break;
                        }
                    }
                }
                ".q" => {
                    break;
                }
                other => {
                    eprintln!("Bilinmeyen komut! Girdiğiniz komutu kontrol ediniz : {}", command.trim_end());
                }
            }
        }
        
        Category::replace(c.id, name, kws);
    }

    pub fn remove_category(&mut self){
        let mut c : Box<Category>;
        loop {
            let mut command = String::new();
            Category::print_existing_category_names();
            print!("Çıkartmak istediğin kategorinin {} (dönmek için '{}')\n : ", "ismi".bold(), ".q".red());
            _ = io::stdout().flush();
            _ = io::stdin().read_line(&mut command);

            c = match Category::get_by_name(command.trim_end()){
                Some(a) => a,
                None => {
                    eprintln!("{} adlı kategori bulunamadı.", command.trim_end());
                    continue;
                }
            };

            let id = c.id;

            println!("Kategori {} {} : ", c.name.yellow(),"çıkartılıyor".red().bold());
            c.print();
            print!("Emin misin? ('{}', '{}') : ", "e".bold(), "h".bold());
            _ = io::stdout().flush();
            let mut command = String::new();
            _ = io::stdin().read_line(&mut command);
            if command.trim_end() == "e"{
                Category::remove_by_id(id);
                println!("Kategori çıkartıldı.");
                break;
            }
        }
    }
    
    pub fn category_terminal_change(&mut self){
        loop{
            let mut command = String::new();
            println!("Kategori veritabanında yapmak istediğin işlemi seç : ");
            println!("- {} : {}\n- {} : {}\n- {} : {}\n- {} : {}\n- {} : {}", 
            "kategori ekle".blue().italic(), ".a".blue().italic(), 
            "kategori düzenle".green().italic(), ".e".green().italic(),
            "kategori çıkart".yellow().italic(), ".r".yellow().italic(),
            "kategorileri yazdır".magenta().italic(), ".p".magenta().italic(),
            "geri".red(), ".q".red());
            _ = io::stdout().flush();
            _ = io::stdin().read_line(&mut command);
            match command.trim_end(){
                ".a" => self.generate_category(),
                ".e" => self.edit_category(),
                ".r" => self.remove_category(),
                ".p" => Category::print_existing_categories(),
                ".q" => break,
                _ => {}
            }
        }
    }
    //// CATEGORY DATABASE IMPL END
    

}