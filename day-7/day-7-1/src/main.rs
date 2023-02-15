use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let mut files = Storage::new();
    let mut pwd: &mut Node = files.files.get_mut("/").unwrap();

    for (command, output) in input
        .split("$ ")
        .map(|cmdout| cmdout.split_once("\n").unwrap_or((cmdout, "")))
    {
        match &command[..2] {
            "cd" => {
                let dir = &command[3..];
                if dir == "/" {
                    pwd = files.files.get_mut("/").unwrap();
                    continue;
                } else if !pwd.unwrap_dir().contains_key(dir) {
                    pwd.unwrap_dir()
                        .insert(String::from(dir), Node::Dir(HashMap::new()));
                }
                pwd = pwd.unwrap_dir().get_mut(dir).unwrap();
            }
            "ls" => {
                for (node_type, name) in output.lines().map(|line| line.split_once(" ").unwrap()) {
                    if node_type == "dir" {
                        pwd.unwrap_dir()
                            .insert(String::from(name), Node::Dir(HashMap::new()));
                    } else {
                        let size = node_type.parse().unwrap();
                        pwd.unwrap_dir()
                            .insert(String::from(name), Node::File(size));
                    }
                }
            }
            "" => continue,
            _ => {
                panic!("Oopsie poopsie!")
            }
        }
    }
}

struct Storage {
    pub files: HashMap<String, Node>,
}

impl Storage {
    fn new() -> Storage {
        let mut files = HashMap::new();
        files.insert(String::from("/"), Node::Dir(HashMap::new()));
        Storage { files }
    }
}

enum Node {
    Dir(HashMap<String, Node>),
    File(usize),
}

impl Node {
    fn unwrap_dir<'a>(&'a mut self) -> &'a mut HashMap<String, Node> {
        match self {
            Node::Dir(x) => x,
            _ => panic!("Expected a directory"),
        }
    }
}
