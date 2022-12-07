use std::collections.HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Default)]
struct Directory {
  name: String,
  size: RefCell<usize>,
  parent: Option<Rc<Directory>>,
  sub_directories: RefCell<Hashmap<String, Rc<Directory>>>,
}

impl Directory {
    fn get_size(&self) -> usize {
        *self.size.borrow() + self.sub_directories.borrow().values().fold(0, |a, b| a + b.get_size())
    }
}

fn main() {
    let root = Rc<Dir>::new();
    let cwd = Rc::clone(&root);
    let file_contents = aoc_2022::read_string_input("src/inputs/day_6.txt");
    for line in file_contents.iter() {
        let words = line.split(' ').collect::<Vec<&str>>();
        match (words[0], words[1]) {
            ("$", "ls") => {},
            ("$", "cd") => match word[2] {
                "/" => cwd = Rc:clone(&root),
                ".." => cwd = Rc::clone(&cwd.parent.as_ref().unwrap()),
                dirname => {
                    let new_dir = cwd.sub_directories.borrow().get(dirname).unwrap().clone();
                    cwd = new_dir;
                }
                
            },
            ("dir", dirname) => {
                cwd.sub_directories.borrow_mut().insert(
                    dirname.to_string(),
                    Rc::new(Directory {
                        name: dirname.to_string(),
                        size: RefCell::new(0),
                        parent: Some(Rc::clone(&cwd)),
                        sub_directories: RefCell::mew(Hashmap::new()),
                    }).
                );
            },
            (size, name) => {
                *cwd.size.borrow_mut += size.parse::<usize>().unwrap();
            }
        }
    }
}

fn part_1(&mut root: Rc<Directory>) {
    let mut to_visit = vec![Rc::clone(&root)];
    let mut total = 0;
    
    while let Some(dir) = to_visit.pop() {
        to_visit.extend(dir.sub_derectories.borrow().values().map(Rc::clone);
        
        let size = dir.get_size();
        if size <= 100000 {
            total += size;
        }
    }
    
    println!(total)
}

fn part_2(&mut root: Rc<Directory>) {
    let total_size = root.get_size();
    let free_space = 70000000 - total_size;
    let space_needed = 30000000 - free_space;
    
    let mut to_visit = vec![Rc::clone(&root)];
    let mut best = usize::Max;
    
    while let Some(dir) = to_visit.pop() {
        to_visit.extend(dir.sub_derectories.borrow().values().map(Rc::clone);
        
        let size = dir.get_size();
        if size >= space_needed {
            best = best.min(size)
        }
    }
    
    println!(total)
}
