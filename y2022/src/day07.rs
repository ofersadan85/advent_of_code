use advent_of_code_common::file::split_lines_trim;
use anyhow::{Context, Result};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

type FolderMap = HashMap<String, Folder>;

const PATH: &str = "inputs/day07.txt";
const EXAMPLE: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

#[derive(Debug, Clone)]
struct Folder {
    name: String,
    folders: HashMap<String, FolderRef>,
    files: HashMap<String, u32>,
    parent: Option<FolderRef>,
}

type FolderRef = Rc<RefCell<Folder>>;

impl Folder {
    fn new(name: &str, parent: Option<FolderRef>) -> FolderRef {
        Rc::new(RefCell::new(Self {
            name: name.to_string(),
            folders: HashMap::new(),
            files: HashMap::new(),
            parent,
        }))
    }

    fn root() -> FolderRef {
        Self::new("/", None)
    }

    fn size(&self) -> u32 {
        let file_sizes: u32 = self.files.values().sum();
        let folder_sizes: u32 = self.folders.values().map(|f| f.borrow().size()).sum();
        file_sizes + folder_sizes
    }
}

fn input(example: bool) -> Result<FolderRef> {
    let lines = if example {
        split_lines_trim(EXAMPLE)
    } else {
        split_lines_trim(&std::fs::read_to_string(PATH).context("Failed to read input file")?)
    };
    let root = Folder::root();
    let mut current_folder = root.clone();
    let mut read_output = false;
    for row in lines {
        current_folder = if row == "$ cd /" {
            root.clone()
        } else if row == "$ cd .." {
            current_folder
                .borrow()
                .parent
                .clone()
                .unwrap_or_else(|| root.clone())
        } else if row.starts_with("$ cd") {
            let next_folder = row
                .split_ascii_whitespace()
                .last()
                .context("Invalid row")?
                .to_string();
            current_folder
                .borrow_mut()
                .folders
                .entry(next_folder.clone())
                .or_insert_with(|| Folder::new(&next_folder, Some(current_folder.clone())))
                .clone()
        } else {
            current_folder
        };

        if row == "$ ls" {
            read_output = true;
        } else if row.starts_with('$') {
            read_output = false;
        } else if read_output && row.starts_with("dir") {
            let next_folder = row
                .split_ascii_whitespace()
                .last()
                .context("Invalid row")?
                .to_string();
            current_folder
                .borrow_mut()
                .folders
                .entry(next_folder.clone())
                .or_insert_with(|| Folder::new(&next_folder, Some(current_folder.clone())));
        } else {
            let (file_size, file_name) = row.split_once(' ').context("Invalid row")?;
            current_folder
                .borrow_mut()
                .files
                .insert(file_name.to_string(), file_size.parse().unwrap_or_default());
        }
    }
    Ok(root)
}

fn part_1(root: &FolderRef) -> u32 {
    let mut sum: u32 = root
        .borrow()
        .folders
        .values()
        .filter(|f| f.borrow().size() <= 100_000)
        .map(|f| f.borrow().size())
        .sum();

    for f in root.borrow().folders.values() {
        sum += part_1(&f.clone());
    }
    sum
}

fn get_big_folders(root: &FolderRef, min_size: u32) -> Vec<u32> {
    let mut big_folders: Vec<u32> = root
        .borrow()
        .folders
        .values()
        .filter(|f| f.borrow().size() >= min_size)
        .map(|f| f.borrow().size())
        .collect();
    for f in root.borrow().folders.values() {
        big_folders.extend(get_big_folders(&f.clone(), min_size));
    }
    big_folders
}

fn part_2(root: &FolderRef) -> Option<u32> {
    let min_size = root.borrow().size() - 40_000_000;
    get_big_folders(root, min_size).iter().min().copied()
}

#[test]
fn example_1() {
    assert_eq!(part_1(&input(true).unwrap()), 95437);
}

#[test]
fn task_1() {
    assert_eq!(part_1(&input(false).unwrap()), 1_182_909);
}

#[test]
fn example_2() {
    assert_eq!(part_2(&input(true).unwrap()), Some(24_933_642));
}

#[test]
fn task_2() {
    assert_eq!(part_2(&input(false).unwrap()), Some(2_832_508));
}
