use std::{collections::HashMap, fs};

#[derive(Debug)]
struct DirTraverse {
    cwd: Vec<String>,
}

impl DirTraverse {
    pub fn new() -> DirTraverse {
        DirTraverse { cwd: vec![] }
    }

    pub fn cd(&mut self, dir: &str) {
        if dir == ".." {
            self.cwd.pop();
        } else {
            self.cwd.push(String::from(dir));
        }
    }

    pub fn iter(&self) -> DirIterator {
        let mut cwd = vec![""];
        cwd.append(&mut self.cwd.iter().map(|f| f.as_str()).collect());
        DirIterator { cwd: cwd }
    }
}

struct DirIterator<'a> {
    cwd: Vec<&'a str>,
}

impl<'a> Iterator for DirIterator<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let result = dir_path(&self.cwd);
        self.cwd.pop().and(Some(result))
    }
}

fn dir_path(cwd: &Vec<&str>) -> String {
    cwd.iter().map(|f| format!("/{f}")).collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");

    let mut iter = input.lines();
    let mut cwd = DirTraverse::new();
    let mut dir_sizes: HashMap<String, i32> = HashMap::new();

    assert_eq!("$ cd /", iter.next().expect("Input is empty"));

    iter.for_each(|l| match l {
        "$ ls" => {}

        _ if l.starts_with("$ cd ") => {
            let dir = l.replace("$ cd ", "");
            cwd.cd(&dir);
        }

        _ if l.starts_with("dir") => {}

        _ => {
            let size = l.split(" ").next().unwrap();
            let size = size.parse::<i32>().unwrap();
            cwd.iter().for_each(|d| {
                *dir_sizes.entry(d).or_insert(0) += size;
            })
        }
    });

    let space_to_free = 30000000 - (70000000 - dir_sizes["/"]);

    println!(
        "Part 1: {}\nPart 2: {:?}",
        dir_sizes
            .iter()
            .filter(|f| *f.1 <= 100000)
            .map(|f| f.1)
            .sum::<i32>(),
        dir_sizes
            .iter()
            .filter(|f| *f.1 >= space_to_free)
            .map(|f| f.1)
            .min()
            .unwrap()
    );
}
