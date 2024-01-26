use std::io::{Read, Write};
use std::process::exit;
use std::fs::File;
use std::env;

trait MdNumberer {
    fn delete_prefix_number(&mut self);
    fn add_prefix_number(&mut self);
}

impl MdNumberer for Vec<String> {
    fn delete_prefix_number(&mut self) {
        for i in self.iter_mut() {
            if i.is_empty() {
                continue;
            }

            let mut split = (*i)
                .splitn(3, ' ')
                .map(String::from)
                .collect::<Vec<_>>();

            if !split[0].chars().all(|x| x == '#') {
                continue;
            }

            split.remove(1);

            *i = split.join(" ");
        }
    }

    fn add_prefix_number(&mut self) {
        let mut numbers = [0usize; 7];

        for i in self.iter_mut() {
            if i.is_empty() {
                continue;
            }

            let mut split = (*i)
                .splitn(2, ' ')
                .map(String::from)
                .collect::<Vec<_>>();

            let fst_word        = split[0]
                .chars()
                .collect::<Vec<_>>();
            let num_of_hashtags = fst_word.len();

            if num_of_hashtags < 2 || !fst_word.iter().all(|&x| x == '#') {
                continue;
            }

            numbers[num_of_hashtags] += 1;

            for j in num_of_hashtags + 1 .. numbers.len() {
                numbers[j] = 0;
            }

            split.insert(
                1, 
                numbers[2 ..= num_of_hashtags]
                    .iter()
                    .map(usize::to_string)
                    .collect::<Vec<_>>()
                    .join(".")
                    + ".",
            );

            *i = split.join(" ");
        }
    }
}

fn main() {
    let mut args = env::args().skip(1);

    let target_path = args.next();

    if target_path.is_none() || args.count() != 0 {
        panic!("invalid format\nsyntax : md_numberer <FILE_PATH>\nsample : md_numberer test.md");
    }

    let mark_comment = String::from("<!--worked by md_numberer. DO NOT EDIT this line-->");
    let target_path  = target_path.unwrap();

    if &target_path[target_path.len() - 3 ..] != ".md" {
        panic!("invalid file path");
    }

    let mut file     = File::open(target_path.clone()).expect("failed to open the file");
    let mut contents = String::new();

    file
        .read_to_string(&mut contents)
        .expect("failed to read the file");

    let mut contents = contents
        .split('\n')
        .map(String::from)
        .collect::<Vec<String>>();

    if contents.is_empty() {
        exit(0);
    }

    if contents.last().unwrap() == &mark_comment {
        contents.delete_prefix_number();
    }
    else {
        contents.push(mark_comment);
    }

    contents.add_prefix_number();

    let mut file = File::create(target_path).expect("failed to open the file");

    file.write_all(contents
        .join("\n")
        .as_bytes()
    )
        .expect("failed to write the file");
}
