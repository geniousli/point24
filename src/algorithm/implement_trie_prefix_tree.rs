use std::collections::HashMap;
use std::boxed::Box;
use std::string::String;
use std::cell::RefCell;
use std::ops::Deref;

// Solution 1 ------------------
// #[derive(Debug)]
// pub struct Trie {
//     config: RefCell::<Box::<HashMap<String, RefCell<Box<Trie>>>>>,
//     world: bool,
// }

// impl Trie {

//     pub fn new() -> Self {
//         Trie {
//             world: false,
//             config: RefCell::new(Box::new(HashMap::new())),
//         }
//     }

//     pub fn insert(&mut self, word: String) {
//         let i_first_word = word.chars().nth(0);
//         if let Some(first_word) = i_first_word {
//             let mut config_walker = self.config.borrow_mut();
//             let data = config_walker.entry(first_word.to_string()).or_insert(RefCell::new(Box::new(Trie::new())));
//             data.borrow_mut().insert(word[1..].to_string());
//         }else {
//             self.world = true;
//         }


//     }
//     pub fn search(&self, word: String) -> bool {
//         let i_first_word = word.chars().nth(0);
//         if let Some(first_word) = i_first_word {
//             let mut config_walker = self.config.borrow();
//             if let Some(data) = config_walker.get(&first_word.to_string()) {
//                 data.borrow().search(word[1..].to_string())
//             }else{
//                false
//             }
//         }else
//         {
//             self.world
//         }
//     }

//     pub fn starts_with(&self, word: String) -> bool {
//         let i_first_word = word.chars().nth(0);
//         if let Some(first_word) = i_first_word {
//             let mut config_walker = self.config.borrow();
//             if let Some(data) = config_walker.get(&first_word.to_string()) {
//                 data.borrow().starts_with(word[1..].to_string())
//             }else{
//                false
//             }
//         }else
//         {
//             self.world || !self.config.borrow().is_empty()
//         }
//     }
// }


// solution 2 -------
#[derive(Debug)]
pub struct Trie {
    config: HashMap<String, RefCell<Box<Trie>>>,
    world: bool,
}


impl Trie {

    pub fn new() -> Self {
        Trie {
            world: false,
            config: HashMap::new(),
        }
    }

    pub fn insert(&mut self, word: String) {
        let i_first_word = word.chars().nth(0);
        if let Some(first_word) = i_first_word {
            let data = self.config.entry(first_word.to_string()).or_insert(RefCell::new(Box::new(Trie::new())));
            data.borrow_mut().insert(word[1..].to_string());
        }else {
            self.world = true;
        }
    }

    pub fn search(&self, word: String) -> bool {
        let i_first_word = word.chars().nth(0);
        if let Some(first_word) = i_first_word {
            if let Some(data) = self.config.get(&first_word.to_string()) {
                data.borrow().search(word[1..].to_string())
            }else{
               false
            }
        }else
        {
            self.world
        }
    }

    pub fn starts_with(&self, word: String) -> bool {
        let i_first_word = word.chars().nth(0);
        if let Some(first_word) = i_first_word {
            if let Some(data) = self.config.get(&first_word.to_string()) {
                data.borrow().starts_with(word[1..].to_string())
            }else{
               false
            }
        }else
        {
            self.world || !self.config.is_empty()
        }
    }

}
