use day_07::*;
use std::collections::HashMap;



pub fn build_file_tree(lines: &[&str]) -> HashMap<String, Vec<FileLike>> {

    let mut file_tree: HashMap<String, Vec<FileLike>> = HashMap::new();

    let mut line_counter: usize = 0;
    let mut parent_heirarchy: Vec<String> = Vec::new();

    while line_counter < lines.len() {
        
        let mut current_line = lines[line_counter];
        
        let Ok(operation) = Operation::try_from(current_line) else {
            panic!("We should only have to parse operation from the outer loop but we encountered: {:?}", current_line);
        };

        match operation {
            Operation::Cd(dirname) => {
                match dirname.as_str() {
                    ".." => {
                        parent_heirarchy.pop();
                    },
                    dirname => {
                        parent_heirarchy.push(
                            dirname.to_owned()
                        );
                    }
                }
                line_counter += 1;
            },
            Operation::Ls => {

                line_counter += 1;

                let mut dir_entries: Vec<FileLike> = vec![];
                current_line = lines[line_counter];

                while let Ok(file_like) = FileLike::try_from(current_line) {
                    dir_entries.push(file_like);
                    line_counter += 1;
                    if line_counter >= lines.len() {
                        break;
                    }
                    current_line = lines[line_counter];
                }

                file_tree
                .entry(parent_heirarchy.join("/"))
                .and_modify(|val| val.extend_from_slice(&dir_entries))
                .or_insert(dir_entries);
            }
        }
    }
    file_tree
}


fn solve_part1(file_tree: &HashMap<String, Vec<FileLike>>) -> usize {
    file_tree
    .keys()
    .map(|k| get_size_of_dir(file_tree, &k))
    .filter(|&s| s <= 100_000)
    .sum()
}


fn solve_part2(file_tree: &HashMap<String, Vec<FileLike>>) -> usize {

    let root_used_space = get_size_of_dir(file_tree, "/");
    let total_disk_space : usize = 70_000_000;
    let total_available_space : usize = total_disk_space - root_used_space;
    let unused_needed: usize = 30_000_000;
    let unused_needed_more: usize = unused_needed - total_available_space;

    file_tree
    .keys()
    .map(|k| get_size_of_dir(file_tree, &k))
    .filter(|size| *size >= unused_needed_more)
    .min_by_key(|&size| size)
    .unwrap()
}


fn get_size_of_dir(file_tree: &HashMap<String, Vec<FileLike>>, dirname: &str) -> usize {

    let Some(file_likes) = file_tree.get(dirname) else {
        return 0
    };

    let mut total_size: usize = 0;

    for item in file_likes.iter() {
        match item {
            FileLike::Dir(sub_dir) => {
                total_size += get_size_of_dir(file_tree, &format!("{}/{}", dirname, sub_dir));
            },
            FileLike::File(file) => {
                total_size += file.size;
            }
        }
    }

    total_size
}

fn main() {

    let input = include_str!("input.txt");

    let input = input.split("\n").collect::<Vec<&str>>();
    let file_tree = build_file_tree(&input);

    let part1 = solve_part1(&file_tree);
    let part2 = solve_part2(&file_tree);
    println!("Part 1: {:?}", part1);
    println!("Part 2: {:?}", part2);
}


#[cfg(test)]
mod tests {
    use crate::{build_file_tree, solve_part1, get_size_of_dir, solve_part2};

    #[test]
    fn test_smol() {
        let test_case: String = "$ cd /
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
7214296 k".into();

        let input = test_case.split("\n").collect::<Vec<&str>>();
        let file_tree = build_file_tree(&input);
        let root_sum = get_size_of_dir(&file_tree, "/");
        assert_eq!(root_sum, 48381165);
        
        let result = solve_part1(&file_tree);
        assert_eq!(result, 95437);

        let result2 = solve_part2(&file_tree);
        assert_eq!(result2, 24933642);

    }
}