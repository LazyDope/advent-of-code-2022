use core::fmt;
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{Display, Formatter},
    fs,
    rc::Rc,
};

fn main() {
    let input = fs::read_to_string("input").unwrap();

    let mut storage = Storage::new();
    let mut cwd: Rc<RefCell<Directory>> = storage.root.clone();
    let mut cwd_name = "/";

    for (command, output) in input
        .split("$ ")
        .map(|cmdout| cmdout.split_once("\n").unwrap_or((cmdout, "")))
    {
        match command.get(..2) {
            Some("cd") => {
                let name = command.get(3..).expect("No dir name found!");
                if name == ".." {
                    let tmp =
                        cwd.borrow().parent.clone().expect(&format!(
                        "Can't switch to parent dir without a parent! {} does not have a parent!",
                        cwd_name));
                    cwd = tmp;
                    cwd_name = name;
                    continue;
                } else if name == "/" {
                    cwd = storage.root.clone();
                    continue;
                }
                cwd = match cwd.clone().borrow().files.get(name) {
                    Some(x) => match x {
                        Node::Dir(dir) => dir.clone(),
                        _ => panic!("Can't change cwd to a file!"),
                    },
                    _ => {
                        let new_dir = cwd.borrow_mut().new_dir(name, cwd.clone());
                        storage.directories.push(new_dir.clone());
                        new_dir
                    }
                };
                cwd_name = name;
            }
            Some("ls") => {
                for (node_type, name) in output.lines().map(|line| line.split_once(" ").unwrap()) {
                    if node_type == "dir" {
                        let new_dir = cwd.borrow_mut().new_dir(name, cwd.clone());
                        storage.directories.push(new_dir.clone());
                    } else {
                        let size = node_type.parse().unwrap();
                        cwd.borrow_mut().new_file(name, size);
                    }
                }
            }
            None => continue,
            _ => {
                panic!("Invalid command! {} could not be parsed!", command);
            }
        }
    }

    let space_needed = 30_000_000 + storage.root.borrow().file_size() - 70_000_000;
    let result: usize = storage
        .directories
        .iter()
        .filter_map(|dir| {
            let size = dir.borrow().file_size();
            if size >= space_needed {
                Some(size)
            } else {
                None
            }
        })
        .min()
        .expect("No smallest directory found that would free the necessary space.");
    println!("{}", result);
}

struct Storage {
    pub root: Rc<RefCell<Directory>>,
    pub directories: Vec<Rc<RefCell<Directory>>>,
}

struct Directory {
    pub files: HashMap<String, Node>,
    pub parent: Option<Rc<RefCell<Directory>>>,
}

enum Node {
    Dir(Rc<RefCell<Directory>>),
    File(usize),
}

impl Storage {
    fn new() -> Storage {
        let root = Rc::new(RefCell::new(Directory::new()));
        Storage {
            root: root.clone(),
            directories: vec![root],
        }
    }
}

impl Directory {
    fn new() -> Directory {
        Directory {
            files: HashMap::new(),
            parent: None,
        }
    }

    fn new_dir(&mut self, name: &str, parent: Rc<RefCell<Directory>>) -> Rc<RefCell<Directory>> {
        let new_dir = Rc::new(RefCell::new(Directory::from(parent)));
        self.files
            .insert(name.to_string(), Node::Dir(new_dir.clone()));
        new_dir
    }

    fn new_file(&mut self, name: &str, file_size: usize) {
        self.files.insert(name.to_string(), Node::File(file_size));
    }
}

impl From<Rc<RefCell<Directory>>> for Directory {
    fn from(value: Rc<RefCell<Directory>>) -> Directory {
        Directory {
            files: HashMap::new(),
            parent: Some(value),
        }
    }
}

trait Sizable {
    fn file_size(&self) -> usize;
}

impl Sizable for Node {
    fn file_size(&self) -> usize {
        match self {
            Node::Dir(x) => x.borrow().file_size(),
            Node::File(x) => *x,
        }
    }
}

impl Sizable for Directory {
    fn file_size(&self) -> usize {
        self.files.values().map(|v| v.file_size()).sum()
    }
}

impl Display for Storage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "storage: {{ files ({}): [ ",
            self.root.borrow().file_size()
        )?;
        for (k, v) in self.root.borrow().files.iter() {
            write!(f, "{{ {k}:{v} }}, ", k = k, v = v)?;
        }
        write!(f, "] }}")
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Node::Dir(x) => {
                write!(f, " dir {{ files ({}): [ ", x.borrow().file_size())?;
                for (k, v) in x.borrow().files.iter() {
                    write!(f, "{{ {k}:{v} }}, ", k = k, v = v)?;
                }
                write!(f, "] }}")
            }
            Node::File(x) => write!(f, " file: {{ {} }}", x),
        }
    }
}
