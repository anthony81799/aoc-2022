use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Default)]
struct Directory {
    _name: String,
    size: RefCell<usize>,
    parent: Option<Rc<Directory>>,
    sub_directories: RefCell<HashMap<String, Rc<Directory>>>,
}

impl Directory {
    fn get_size(&self) -> usize {
        *self.size.borrow()
            + self
                .sub_directories
                .borrow()
                .values()
                .fold(0, |a, b| a + b.get_size())
    }
}

fn main() {
    let mut root = Rc::new(Directory::default());
    let file_contents = aoc_2022::read_string_input("src/inputs/day_7.txt");
    parse_input(&file_contents, &mut root);
    part_1(&mut root);
    part_2(&mut root);
}

fn parse_input(input: &[String], root: &mut Rc<Directory>) {
    let mut cwd = Rc::clone(root);
    for line in input.iter() {
        let words = line.split(' ').collect::<Vec<&str>>();
        match (words[0], words[1]) {
            ("$", "ls") => {}
            ("$", "cd") => match words[2] {
                "/" => cwd = Rc::clone(root),
                ".." => cwd = Rc::clone(cwd.parent.as_ref().unwrap()),
                dirname => {
                    let new_dir = cwd.sub_directories.borrow().get(dirname).unwrap().clone();
                    cwd = new_dir;
                }
            },
            ("dir", dirname) => {
                cwd.sub_directories.borrow_mut().insert(
                    dirname.to_string(),
                    Rc::new(Directory {
                        _name: dirname.to_string(),
                        size: RefCell::new(0),
                        parent: Some(Rc::clone(&cwd)),
                        sub_directories: RefCell::new(HashMap::new()),
                    }),
                );
            }
            (size, _name) => {
                *cwd.size.borrow_mut() += size.parse::<usize>().unwrap();
            }
        }
    }
}

fn part_1(root: &mut Rc<Directory>) {
    let mut to_visit = vec![Rc::clone(root)];
    let mut total = 0;

    while let Some(dir) = to_visit.pop() {
        to_visit.extend(dir.sub_directories.borrow().values().map(Rc::clone));

        let size = dir.get_size();
        if size <= 100000 {
            total += size;
        }
    }

    println!("{total}")
}

fn part_2(root: &mut Rc<Directory>) {
    let total_size = root.get_size();
    let free_space = 70000000 - total_size;
    let space_needed = 30000000 - free_space;

    let mut to_visit = vec![Rc::clone(root)];
    let mut best = usize::MAX;

    while let Some(dir) = to_visit.pop() {
        to_visit.extend(dir.sub_directories.borrow().values().map(Rc::clone));

        let size = dir.get_size();
        if size >= space_needed {
            best = best.min(size)
        }
    }

    println!("{best}")
}
