use std::fmt::{Display, Formatter};
use std::io::{self, Read, Write};
use std::fs::{self, File};
use std::process::exit;
use std::error::Error;
use std::env;

type GenericError = Box<dyn Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

#[derive(Debug, Clone)]
struct EmptyFileError;

impl Error for EmptyFileError {}

impl Display for EmptyFileError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "the target file is empty")
    }
}

#[derive(Debug, Clone)]
struct MarkdownNotFoundError;

impl Error for MarkdownNotFoundError {}

impl Display for MarkdownNotFoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "the target file does not have a markdown extension")
    }
}

trait MdNumberer {
    fn add_prefix_number(&mut self, limit: usize, starting_num: usize);
    fn delete_prefix_number(&mut self);
}

impl MdNumberer for Vec<String> {
    fn add_prefix_number(&mut self, limit: usize, starting_num: usize) {
        let starting_num = starting_num as isize - 1;

        let mut numbers = [starting_num; 7];

        for i in self.iter_mut() {
            if i.is_empty() {
                continue;
            }

            let mut split = (*i)
                .splitn(2, ' ')
                .map(String::from)
                .collect::<Vec<_>>();

            let num_of_hashtags = split[0].chars().count();

            if num_of_hashtags < limit || !split[0].chars().all(|x| x == '#') {
                continue;
            }

            numbers[num_of_hashtags] += 1;

            for j in num_of_hashtags + 1 .. numbers.len() {
                numbers[j] = starting_num;
            }

            split.insert(
                1,
                numbers[limit ..= num_of_hashtags]
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(".")
                    + ".",
            );

            *i = split.join(" ");
        }
    }

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
}

fn apply_number_to_dir(target_dir: String, limit: usize, starting_num: usize) -> GenericResult<()> {
    let paths = fs::read_dir(target_dir)?;

    for path in paths {
        let Ok(path) = path else {
            continue;
        };

        let path = path
            .path()
            .to_str()
            .unwrap()
            .to_owned();

        match apply_number(path.clone(), limit, starting_num) {
            Ok(()) => continue,
            Err(err) => {
                if let Some(_) = err.downcast_ref::<MarkdownNotFoundError>() {
                    continue;
                }
                else {
                    let msg_string =
                        if let Some(msg) = err.downcast_ref::<EmptyFileError>() {
                            msg.to_string()
                        }
                        else if let Some(msg) = err.downcast_ref::<io::Error>() {
                            msg.to_string()
                        }
                        else {
                            err.to_string()
                        };

                    println!("[FAIL] {} : {}", path, msg_string);
                }
            },
        }
    }

    Ok(())
}

fn apply_number(target_path: String, limit: usize, starting_num: usize) -> GenericResult<()> {
    let mark_comment = String::from("<!--worked by md_numberer. DO NOT EDIT this line-->");

    if &target_path[target_path.len() - 3 ..] != ".md" {
        return Err(GenericError::from(MarkdownNotFoundError));
    }

    let mut file     = File::open(target_path.clone())?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let mut contents = contents
        .trim()
        .split('\n')
        .map(String::from)
        .collect::<Vec<_>>();

    if contents.len() == 1 && contents[0] == "" {
        return Err(GenericError::from(EmptyFileError));
    }

    if contents.last().unwrap() == &mark_comment {
        contents.delete_prefix_number();
    }
    else {
        contents.push(mark_comment);
    }

    contents.add_prefix_number(limit, starting_num);

    let mut file = File::create(target_path)?;

    file.write_all(contents
        .join("\n")
        .as_bytes()
    )?;

    Ok(())
}

fn print_help(file_name: String) {
    println!("sample : {} --file sample.md -l 2 -s 1", file_name);
    println!("syntax :");
    println!("    {} --help | -h", file_name);
    println!("    {} --file <file_name> [-l <header_number_limit>] [-s <starting_number>]", file_name);
    println!("    {} --directory <directory_name> [-l <header_number_limit>] [-s <starting_number>]", file_name);
    println!("range :");
    println!("    header_number_limit(integer; default: 1) : [1, 6]");
    println!("    starting_number(integer; default: 1)     : [0, 1]");
}

fn main() {
    let mut args = env::args();
    let mut limit = 1;
    let mut starting_num = 1;

    let file_name = args.next().unwrap();
    let is_dir = match args.next() {
        Some(x) if &x == "--directory" => true,
        Some(x) if &x == "--file" => false,
        Some(x) if &x == "-h" || &x == "--help" => {
            print_help(file_name);
            exit(0);
        },
        _ => panic!("required option does not exist"),
    };
    let target_path = args
        .next()
        .expect("option is given without a parameter");

    loop {
        match args.next() {
            Some(x) => {
                *match &*x {
                    "-l" => &mut limit,
                    "-s" => &mut starting_num,
                    _ => panic!("illegal option is given"),
                } = args
                    .next()
                    .expect("option is given without a parameter")
                    .parse()
                    .expect("parameter format is illegal")
            },
            None => {
                if !(0..=1).contains(&starting_num) || !(1..=6).contains(&limit) {
                    panic!("parameter range is illegal");
                }

                break;
            },
        }
    }

    match (if is_dir { apply_number_to_dir } else { apply_number })(target_path.clone(), limit, starting_num) {
        Ok(()) => (),
        Err(err) => {
            let msg_string =
                if let Some(msg) = err.downcast_ref::<MarkdownNotFoundError>() {
                    msg.to_string()
                }
                else if let Some(msg) = err.downcast_ref::<EmptyFileError>() {
                    msg.to_string()
                }
                else if let Some(msg) = err.downcast_ref::<io::Error>() {
                    msg.to_string()
                }
                else {
                    err.to_string()
                };

            println!("[FAIL] {} : {}", target_path, msg_string);
        },
    }
}
