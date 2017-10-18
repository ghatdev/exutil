use std::fs;
use std::env;
use std::path::Path;

fn upper_filename(file_name: &String, file_stem: &String) {
    let uppered_stem = file_stem.to_uppercase();

    let change_result = fs::rename(file_name, file_name.replace(file_stem,&uppered_stem));
    
    match change_result {
        Ok(_) => return,
        Err(e) => panic!(e),
    }
}

fn main() {
    let arguments = env::args();

    if arguments.len() < 2 {
        println!("Usage: upper filename-to-upper");
        return;
    }

    let todo = env::args().skip(1);
    
    for argument in todo {
        let path = Path::new(&argument);

        if !path.exists() {
            println!("{} is not exists", path.display());
            return;
        }

        if !path.is_file() {
            println!("{} is not a file. Only file allowd", path.display());
            return;
        }

        let stem = path.file_stem().unwrap().to_str().unwrap().to_string();

        upper_filename(&argument, &stem);
    }

}
