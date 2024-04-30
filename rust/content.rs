use std::fs;

pub struct Config {
    pub file_path: String,
}

pub fn read_content(config: Config) -> Vec<usize> {
    let content = fs::read(config.file_path).expect("Should have been able to read the file");
    let mut result: Vec<usize> = Vec::with_capacity(content.len());

    for &element in content.iter() {
        result.push(element as usize);
    }

    result
}

fn parse_config(args: &[String]) -> Config {
    if args.len() < 2 {
        panic!("Not enough arguments");
    }

    Config {
        file_path: args[1].clone(),
    }
}

pub fn test_content(args: &[String]) {
    let config = parse_config(args);
    let content = read_content(config);

    println!("Voici les nombres qui ont été récupérées : \n");
    for el in content.iter() {
        dbg!(el);
    }
}

pub fn build_content(args: &[String]) -> Vec<usize> {
    let config = parse_config(args);

    read_content(config)
}
