use std::io::{Read, Write};
use std::fs::{self, File};
use std::error::Error;
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

fn apply_toc_to_dir(target_dir: String) -> Result<(), Box<dyn Error>> {
    let paths = fs::read_dir(target_dir)?;

    for path in paths {
        let _ = apply_toc(path?.path().to_str().unwrap().to_owned());
    }

    Ok(())
}

fn apply_toc(target_path: String) -> Result<(), Box<dyn Error>> {
    let mark_comment = String::from("<!--worked by md_numberer. DO NOT EDIT this line-->");

    if &target_path[target_path.len() - 3 ..] != ".md" {
        return Err(Box::from("is not a markdown file"));
    }

    let mut file     = File::open(target_path.clone())?;
    let mut contents = String::new();

    file
        .read_to_string(&mut contents)?;

    let mut contents = contents
        .split('\n')
        .map(String::from)
        .collect::<Vec<String>>();

    if contents.is_empty() {
        return Err(Box::from("file has no content"));
    }

    if contents.last().unwrap() == &mark_comment {
        contents.delete_prefix_number();
    }
    else {
        contents.push(mark_comment);
    }

    contents.add_prefix_number();

    let mut file = File::create(target_path)?;

    file.write_all(contents
        .join("\n")
        .as_bytes()
    )?;

    Ok(())
}

fn main() {
    let mut args = env::args().skip(1);

    let is_dir = match args.next() {
        Some(x) if &x == "--directory" => true,
        Some(x) if &x == "--file" => false,
        _ => panic!("invalid format"),
    };
    let target_path = args.next();

    if target_path.is_none() || args.count() != 0 {
        panic!("invalid format");
    }

    let target_path  = target_path.unwrap();

    (if is_dir { apply_toc_to_dir } else { apply_toc })(target_path).unwrap();
}
