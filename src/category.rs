use std::{rc::Rc, str::FromStr};
use serde::{Serialize, Deserialize};
use serde_json;
use rand::prelude::*;
use crate::traits::Printable;
 

#[derive(Serialize, Deserialize, Clone)]
pub struct Category{
    pub name : String,
    pub id : u16,
    pub keywords : Vec<String>,
}

static mut EXISTING_CATEGORIES : Vec<Box<Category>> = Vec::new();

pub const UNKNOWN_CATEGORY_NAME : &'static str = "Bilinmeyen";
pub const UNKNOWN_CATEGORY : Category = Category{
    name : String::new(),
    id : 0,
    keywords : vec![],
};

impl Category {
    fn generate_id() -> u16{
        let mut rv : u16;
        loop{
            rv = rand::random::<u16>();
            if rv == 0 {
                continue;
            }
            let mut exists = false;
            unsafe{
                for i in 0..EXISTING_CATEGORIES.len(){
                    if rv == EXISTING_CATEGORIES[i].id{
                        exists = true;
                        break;
                    }
                }
            }
            if !exists {
                break;
            }
        }
        rv
    }

    pub fn new(name : String, kw : Vec<String>) -> Box<Category>{
        match Category::get_by_name(&name){
            Some(_) => panic!("Category with name '{}' already exists!", name),
            None => {}
        }
        let nc = Box::new(Category{ name : name , id : Category::generate_id() , keywords : kw});
        unsafe{
            EXISTING_CATEGORIES.push(nc.clone());
        }
        nc
    }

    pub fn replace(id : u16, name : String, kw : Vec<String>){
        Category::remove_by_id(id);
        let nc = Box::new(Category{ name : name , id : id , keywords : kw});
        unsafe{
            EXISTING_CATEGORIES.push(nc.clone());
        }
    }

    pub fn insert_into_existing_categories(cat : Category){
        unsafe{
            EXISTING_CATEGORIES.push(Box::new(cat));
        }
    }

    pub fn serialize_existing_categories() -> String{
        let mut v : Vec<Category> = Vec::new();

        unsafe{
            for i in 0..EXISTING_CATEGORIES.len(){

                v.push(
                    Category{ 
                        name : EXISTING_CATEGORIES[i].name.clone(), 
                        id : EXISTING_CATEGORIES[i].id, 
                        keywords : EXISTING_CATEGORIES[i].keywords.clone()
                    });
            }
        }

        serde_json::to_string_pretty(&v).unwrap()
    }

    pub fn print_existing_categories(){
        unsafe{
            for i in 0..EXISTING_CATEGORIES.len(){
                print!("{}) ", i + 1);
                EXISTING_CATEGORIES[i].print();
                println!();
            }
        }
    }

    pub fn print_existing_category_names(){
        unsafe{
            for i in 0..EXISTING_CATEGORIES.len(){
                println!("{}) {}", i + 1, EXISTING_CATEGORIES[i].name);
            }
            println!();
        }
    }

    pub fn get_id_by_desc(desc : &str) -> Option<u16>{
        unsafe{
            for i in 0..EXISTING_CATEGORIES.len(){
                if EXISTING_CATEGORIES[i].is_transaction_in(desc){
                    return Some(EXISTING_CATEGORIES[i].id);
                }
            }
        }
        None
    }

    pub fn get_by_id(id : u16) -> Option<Box<Category>>{
        if id == 0 {
            return Some(Box::new(UNKNOWN_CATEGORY));
        }
        unsafe{
            for i in 0..EXISTING_CATEGORIES.len() {
                if EXISTING_CATEGORIES[i].id == id {
                    return Some(EXISTING_CATEGORIES[i].clone());
                }
            }
        }
        None
    }

    pub fn get_by_name(name : &str) -> Option<Box<Category>>{
        if name == "" {
            return Some(Box::new(UNKNOWN_CATEGORY));
        }
        unsafe{
            for i in 0..EXISTING_CATEGORIES.len() {
                if EXISTING_CATEGORIES[i].name.to_lowercase() == name.to_lowercase() {
                    return Some(EXISTING_CATEGORIES[i].clone());
                }
            }
        }
        None
    }

    pub fn remove_by_id(id : u16) {
        unsafe{
            for i in 0..EXISTING_CATEGORIES.len() {
                if EXISTING_CATEGORIES[i].id == id {
                    EXISTING_CATEGORIES.remove(i);
                    break;
                }
            }
        }
    }

    fn is_transaction_in(&self, trans_desc : &str) -> bool {
        for kw in self.keywords.iter(){
            match trans_desc.to_uppercase().replace("İ", "I").find(kw.to_uppercase().as_str()){
                Some(_) => return true,
                None => continue
            }
        }
        false
    }
}

impl Printable for Category{
    fn print(&self) {
        println!("{} :", self.name);
        println!("   id: {}", self.id);
        println!("   anahtar kelimeler:");
        for i in 0..self.keywords.len(){
            println!("      - \"{}\"", self.keywords[i]);
        }
    }
}