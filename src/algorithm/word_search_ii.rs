use std::boxed::Box;
use std::cell::RefCell;
use std::char;
use std::cmp::PartialEq;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::iter::Iterator;
use std::mem::drop;
use std::rc::Rc;
use std::rc::Weak;
use std::str::FromStr;
use std::vec::Vec;
use std::{thread, time};
pub struct Solution {}

#[derive(Debug)]
pub struct Trie {
    config: HashMap<char, Rc<RefCell<Box<Trie>>>>,
    world: bool,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            world: false,
            config: HashMap::new(),
        }
    }

    pub fn is_world(&self) -> bool {
        self.world
    }

    pub fn insert(&mut self, word: String) {
        let i_first_word = word.chars().nth(0);
        if let Some(first_word) = i_first_word {
            let data = self
                .config
                .entry(first_word)
                .or_insert(Rc::new(RefCell::new(Box::new(Trie::new()))));
            data.borrow_mut().insert(word[1..].to_string());
        } else {
            self.world = true;
        }
    }

    pub fn search(&self, word: String) -> bool {
        let i_first_word = word.chars().nth(0);
        if let Some(first_word) = i_first_word {
            if let Some(data) = self.config.get(&first_word) {
                data.borrow().search(word[1..].to_string())
            } else {
                false
            }
        } else {
            self.world
        }
    }

    pub fn starts_with(&self, word: String) -> bool {
        let i_first_word = word.chars().nth(0);
        if let Some(first_word) = i_first_word {
            if let Some(data) = self.config.get(&first_word) {
                data.borrow().starts_with(word[1..].to_string())
            } else {
                false
            }
        } else {
            self.world || !self.config.is_empty()
        }
    }
    pub fn keys(&self) -> Vec<&char> {
        self.config.keys().collect()
    }

    pub fn tree(&self, key: &char) -> Rc<RefCell<Box<Trie>>> {
        let data = self.config.get(key).unwrap();
        Rc::clone(data)
    }

    pub fn has_word(&self) -> bool {
        if  self.config.len() > 0 {
            true
        }else {
            false
        }
    }
}

#[derive(Debug)]
pub struct Node {
    x: usize,
    y: usize,
    max_x: usize,
    max_y: usize,
}
#[derive(Debug)]
struct NodeOption {
    x: usize,
    y: usize,
    max_x: usize,
    max_y: usize,
    options: Box<Vec<(usize, usize)>>,
    index: usize,
}

impl NodeOption {
    fn new(x: usize, y: usize, max_x: usize, max_y: usize) -> NodeOption {
        let mut ary = Vec::new();
        if x >= 1 {
            ary.push((x - 1, y));
        }
        ary.push((x + 1, y));
        if y >= 1 {
            ary.push((x, y - 1));
        }
        ary.push((x, y + 1));
        let new_ary = ary
            .iter()
            .filter(|&item| item.0 >= 0 && item.1 >= 0 && item.0 < max_x && item.1 < max_y)
            .map(|&x| x)
            .collect();
        let options = Box::new(new_ary);
        NodeOption {
            x,
            y,
            max_x,
            max_y,
            options,
            index: 0,
        }
    }
}

impl Node {
    fn new(x: usize, y: usize, max_x: usize, max_y: usize) -> Node {
        Node { x, y, max_x, max_y }
    }

    fn options(&self) -> NodeOption {
        NodeOption::new(self.x, self.y, self.max_x, self.max_y)
    }
}

impl Iterator for NodeOption {
    type Item = Node;
    fn next(&mut self) -> Option<Node> {
        let result = if let Some(data) = self.options.get(self.index) {
            Some(Node::new(data.0, data.1, self.max_x, self.max_y))
        } else {
            None
        };
        self.index += 1;
        result
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, mut words: Vec<String>) -> Vec<String> {
        let mut trie = Trie::new();
        words.iter().for_each(|x| {
            trie.insert(x.clone());
        });

        println!("tries: #{:?}", trie);
        let max_x = board.len();
        let max_y = board.get(0).unwrap().len();

        let mut i_x = 0;
        let mut i_y = 0;
        let mut result: Vec<String> = Vec::new();
        let trie_keys = trie.keys();
        while i_x < max_x {
            i_y = 0;
            while i_y < max_y {
                let value = board[i_x][i_y];
                let mut visited_node = VecDeque::new();
                let keys: Vec<char> = trie_keys
                    .iter()
                    .filter(|&&&x| x == value)
                    .map(|&&x| x.clone())
                    .collect();
                println!("keys: {:?}, value: {}", keys, value);
                if keys.len() > 0 {
                    keys.iter().for_each(|x| {
                        let value = trie.tree(x);
                        Solution::dfs_find_char(
                            &mut result,
                            &board,
                            Rc::clone(&value),
                            &mut visited_node,
                            Node::new(i_x, i_y, max_x, max_y),
                        );
                    })
                }
                i_y += 1;
            }

            i_x += 1;
        }

        result.sort();
        result
    }

    fn dfs_find_char(
        mut result: &mut Vec<String>,
        board: &Vec<Vec<char>>,
        words: Rc<RefCell<Box<Trie>>>,
        mut nodes: &mut VecDeque<Node>,
        lst_node: Node,
    ) {
        let options = lst_node.options();
        nodes.push_back(lst_node);
        if words.borrow().is_world() {
            let string: String = nodes.iter().map(|x| board[x.x][x.y]).collect();
            let r = result.iter().find(|&x| *x == string);
            if r.is_none() {
                result.push(string);
            }
            if !words.borrow().has_word() {
                return ;
            }
        }


        let keys: Vec<char> = words.borrow().keys().iter().map(|&&x| x.clone()).collect();
        println!("----------- try to find keys: {:?}, nodes: {:?}", keys, nodes);
        for node in options {
            let find_result = nodes.iter().find(|&item| *item == node);
            if find_result.is_none() {
                if let Some(data) = keys.iter().find(|&x| *x == board[node.x][node.y]) {
                    let value = words.borrow().tree(data);
                    println!("----------- value: {:?}, nodes: {:?}, last_node: {:?}", value, nodes, node);
                    Solution::dfs_find_char(
                        &mut result,
                        &board,
                        Rc::clone(&value),
                        &mut nodes,
                        node,
                    );
                    nodes.pop_back();
                }
            }
        }
        return;
    }
}
