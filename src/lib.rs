use std::collections::HashSet;
use rand::Rng;
use std::sync::Mutex;
#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref USED_NAMES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

pub struct Robot{
    name: String
}

impl Robot {
    pub fn new() -> Self {
        Robot{
            name: get_name()
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        USED_NAMES.lock().unwrap().remove(&self.name);
        self.name = get_name()
    }

}

pub fn generate_random_name() -> String {
    let mut rng = rand::thread_rng();
    let letter1: char = rng.gen_range(b'A'.. b'Z') as char;
    let letter2: char = rng.gen_range(b'A'.. b'Z') as char;
    let number1: u32 = rng.gen_range(0..10 );
    let number2: u32 = rng.gen_range(0..10 );
    let number3: u32 = rng.gen_range(0..10 );
    format!("{}{}{}{}{}", letter1, letter2, number1, number2, number3)
}

pub fn get_name() -> String {
    let mut name = generate_random_name();
    while USED_NAMES.lock().unwrap().contains(&name){
        name = generate_random_name();
    }
    USED_NAMES.lock().unwrap().insert(name.clone());
    name
}


