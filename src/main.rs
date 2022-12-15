// use std::fs;

// mod rps_strategy;
// mod elf_snacks;
// mod item_prio;

// use std::str::FromStr;
// mod section_clearing;

mod import_content;
// mod telecom;

fn main() {
    // elf_snacks::print_elf_snacks();
    // rps_strategy::strategy();
    // item_prio::item_priorities();
    // section_clearing::clear_sections();
    // telecom::communication();
    file_tree_print();
}

pub fn file_tree_print() {
    let contents = import_content::import("inputs/files.txt");
    let split = contents.split('\n');
    let f = File::new("dav", 20);
    let mut d = Directory::new("c");
    println!("{}", f.size);
    d.directories.push(Directory::new("y"));
    d.files.push(f);
    for q in d.files{
        println!("{}", q.name);
    }
    for sd in d.directories{
        println!("{}", sd.name);
    }
    for s in split{
        if s != "" {
            println!("{}", s);
            break
        }
    }
}

struct File {
    size: usize,
    name: String
}

impl File {
    pub fn new(name: &str, size: usize) -> File {
        File {
            size: size,
            name: String::from(name)
        }
    }
}

struct Directory {
    name: String,
    files: Vec<File>,
    directories: Vec<Directory>,
    parent: Directory,
    root: Root
}

impl Directory {
    pub fn new(name: &str, root: Root, parent: Directory) -> Directory {
        Directory {
            name: String::from(name), 
            files: vec!(), 
            directories: vec!(),
            parent: parent,
            root: root
        }
    }
}

struct Root {
    files: Vec<File>,
    directories: Vec<Directory>,
}

