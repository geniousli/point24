use std::cell::RefCell;
use std::collections::HashMap;
use std::iter::Iterator;
use std::mem::drop;
use std::rc::Rc;
use std::rc::Weak;
use std::boxed::Box;
use std::str::FromStr;
use std::vec::Vec;
use std::collections::VecDeque;
use std::{thread, time};
use std::cmp::PartialEq;

pub struct Solution {}

#[derive(Debug)]
pub struct Node {
    x: usize,
    y: usize,
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
        if y >= 1{
            ary.push((x, y - 1));
        }
        ary.push((x, y + 1));
        let new_ary = ary.iter().filter(|&item| {item.0 >= 0 && item.1 >= 0 && item.0 < max_x && item.1 < max_y}).map(|&x| x).collect();
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
        Node {
            x,
            y,
        }
    }

    fn options(&self, max_x: usize, max_y: usize) -> NodeOption {
        NodeOption::new(self.x, self.y, max_x, max_y)
    }
}


impl Iterator for NodeOption {
    type Item = Node;
    fn next(&mut self) -> Option<Node> {
        let result = if let Some(data) = self.options.get(self.index) {
            Some(Node::new(
                data.0,
                data.1,
                self.max_x,
                self.max_y
            ))
        }else {
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

impl  Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let words: Vec<char> = word.chars().collect();
        let max_x = board.len();
        let max_y = board.get(0).unwrap().len();

        let mut i_x = 0 ;
        let mut i_y = 0;

        while i_x < max_x {
            i_y = 0;
            while i_y < max_y {
                let value = board[i_x][i_y];
                let mut visited_node = VecDeque::new();
                if value == words[0] {
                    if Solution::dfs_find_char(&board, &words[1..], &mut visited_node, Node::new(i_x, i_y, max_x, max_y), max_x, max_y) {
                        return true;
                    }

                }
                i_y += 1;
            }

            i_x += 1;
        }
        return false;
    }

    fn dfs_find_char(board: &Vec<Vec<char>>, words: &[char], mut nodes: &mut VecDeque< Node>, lst_node: Node, max_x: usize, max_y: usize) -> bool {
        if words.len() == 0 {
            return true;
        }
        let options = lst_node.options(max_x, max_y);
        nodes.push_back(lst_node);
        let char_find = words[0];
        for node in options {
            let result = nodes.iter().find(|&item| *item == node);
            if result.is_none() {
                if board[node.x][node.y] == char_find {
                    if Solution::dfs_find_char(board, &words[1..], &mut nodes, node, max_x, max_y) {
                        return true;
                    }else {
                        nodes.pop_back();
                    }
                }
            }
        }
        return false;
    }
}
