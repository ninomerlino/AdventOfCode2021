use std::{
    fs::File,
    io::{BufRead, BufReader}, fmt::{Display, Debug}, collections::HashMap,
};

use crate::common::Result;
use crate::{common::SillyStringError, tree::Tree};

#[derive(Debug, Clone)]
enum FSElement {
    File(String, usize),
    Dir(String),
}

impl Display for FSElement{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FSElement::File(s, _) => f.write_str(s),
            FSElement::Dir(s) => f.write_str(s),
        }
    }
}

fn parse_filesystem(file: &File) -> Result<Tree<FSElement>> {
    let stream = BufReader::new(file);
    let mut tree = Tree::new(FSElement::Dir(String::from("/")));
    let mut iter = stream.lines();
    iter.next();
    for line in iter {
        let line = line?;
        let words: Vec<_> = line.split(' ').collect();
        match words[0] {
            "$" => {
                match words[1] {
                    "ls" => continue,
                    "cd" => {
                        match words[2] {
                            ".." => {
                                tree.go_to_parent()
                                    .ok_or(SillyStringError::new(&"Where are you going"))?;
                            }
                            dir_name => {
                                let child_index = tree
                                    .get_children()
                                    .iter()
                                    .position(
                                        |el| matches!(el, FSElement::Dir(name) if name == dir_name),
                                    )
                                    .ok_or(SillyStringError::new(&"Where are you going"))?;
                                tree.go_to_child(child_index)?;
                            }
                        };
                    }
                    _ => return Err(SillyStringError::new(&"WOW WTF?").into()),
                };
            }
            "dir" => tree.add_child(FSElement::Dir(words[1].to_owned())),
            size => {
                let name = words[1].to_owned();
                tree.add_child(FSElement::File(name, size.parse()?));
            }
        };
    }
    tree.return_to_root();
    Ok(tree)
}

fn find_size(tree: &mut Tree<FSElement>, known_sizes: &mut HashMap<String,usize>) -> usize {
    let size = match tree.get_current().clone() {
        FSElement::File(_, size) => size,
        FSElement::Dir(_) => {
            let mut total_size = 0;
            for index in 0..tree.children_count() {
                tree.go_to_child(index).unwrap();
                total_size += find_size(tree, known_sizes);
            }
            assert_eq!(known_sizes.insert(tree.pwd(),total_size), None);
            total_size
        },
    };
    tree.go_to_parent();
    size
}

pub fn challenge() {
    let input_file = File::open("inputs/day7.txt").unwrap();
    let mut filesystem = parse_filesystem(&input_file).unwrap();
    let mut known_sizes = HashMap::new();
    let _ = find_size(&mut filesystem, &mut known_sizes);
    println!("{:#?}",known_sizes);
    let count : usize = known_sizes.values().filter_map(|element|if *element<=100000 {Some(*element)} else {None}).sum();
    println!("{count}");
}

pub fn challenge_2() {
    let input_file = File::open("inputs/day7.txt").unwrap();
    let mut filesystem = parse_filesystem(&input_file).unwrap();
    let mut known_sizes = HashMap::new();
    let needed = 30000000 - (70000000 - find_size(&mut filesystem, &mut known_sizes));
    println!("{needed}");
    let dir_size = known_sizes.values().filter(|size| **size>=needed).min().unwrap();
    println!("{dir_size}");
}
