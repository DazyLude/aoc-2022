// #![allow(dead_code, unused_variables)]

use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

// use std::vec::Vec;

fn main() {
    let mut folders: HashMap<String, usize> = HashMap::new();
    read_input_file_to_hashmap(&mut folders);

    let pt1: usize = folders.values().filter(|size| size < &&100000usize).sum();
    println!("Pt1: {pt1}");

    let disk_size = 70000000;
    let required_space = 30000000;
    let free_disk_space = disk_size - folders[""];
    let min_folder_size = required_space - free_disk_space;

    let pt2: usize = *folders
        .values()
        .filter(|size| size > &&min_folder_size)
        .min()
        .unwrap();

    println!(
        "Occupied space: {} Available space: {free_disk_space}, minimal sized folder to delete: {pt2}",
        folders[""]
    );
}

fn read_input_file_to_hashmap(folder_sizes: &mut HashMap<String, usize>) {
    let input_file = File::open("input").expect("Could not open input file");
    let input_buf_read = BufReader::new(input_file);

    let mut folders: HashMap<String, usize> = HashMap::new();
    let mut current_directory = Directory::new();

    for wrapped_line in input_buf_read.lines() {
        let line = wrapped_line.expect("could not read line from the input");
        let split_line = line.split_ascii_whitespace().collect::<Vec<_>>();

        match split_line[0] {
            "$" => {
                if split_line[1] == "cd" {
                    current_directory.change(split_line[2]);
                }
            }
            "dir" => {}
            _ => {
                let size = split_line[0].parse::<usize>().unwrap();
                folders
                    .entry(current_directory.get())
                    .and_modify(|e| *e += size)
                    .or_insert(size);
            }
        }
    }

    folders.iter().for_each(|(path, size)| {
        let dir = Directory::from_str(path);
        dir.get_parent_directories().iter().for_each(|sup_dir| {
            folder_sizes
                .entry(sup_dir.to_string())
                .and_modify(|e| *e += size)
                .or_insert(*size);
        })
    });
}

struct Directory {
    name: String,
}

impl Directory {
    fn new() -> Directory {
        Directory {
            name: "".to_string(),
        }
    }

    fn from_str(name: &str) -> Directory {
        Directory {
            name: name.to_string(),
        }
    }

    fn change(&mut self, go_where: &str) -> Option<String> {
        match go_where {
            ".." => match self.name.rfind('/') {
                Some(cur) => self.name.truncate(cur),
                None => return None,
            },
            "/" => self.name = "".to_string(),
            _ => {
                self.name.push('/');
                self.name.push_str(go_where);
            }
        }
        Some(self.name.clone())
    }

    fn get_parent_directories(mut self) -> Vec<String> {
        let mut result = Vec::<String>::new();
        result.push(self.name.clone());

        while !self.name.is_empty() {
            self.change("..");
            result.push(self.name.clone());
        }

        result
    }

    fn get(&self) -> String {
        self.name.clone()
    }
}

// what is this virtual file system? is this about these wechuubas again?

// struct VirtualFileSystem {}

// struct VFSNode {
//     parent: Option<Rc<VFSNode>>,
//     children: Option<RefCell<Vec<Weak<VFSNode>>>>,
//     size: usize,
// }

// impl VFSNode {
//     fn root() -> VFSNode {
//         VFSNode {
//             parent: None,
//             children: None,
//             size: 0,
//         }
//     }

//     fn with_parent(parent: Rc<VFSNode>) -> VFSNode {
//         VFSNode {
//             parent: Some(parent),
//             children: None,
//             size: 0,
//         }
//     }

//     fn attach_child(mut self, child: Weak<VFSNode>) {
//         if self.children.is_none() {
//             self.children = Some(RefCell::new(Vec::<Weak<VFSNode>>::new()));
//         }
//         self.children.unwrap().borrow_mut().push(child);
//     }
// }
