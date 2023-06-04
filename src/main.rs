mod memory_boxes;

use std::env::args;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

fn main() {
    let args: Vec<String> = args().collect();
    let home_path = home::home_dir().unwrap();
    let file_path = format!("{}/.memorust.memrs", home_path.display());
    let file_path = Path::new(&file_path);
    let file = std::fs::read_to_string(file_path);

    match args[1].as_ref() {
        "help" | "-h" => {
            println!("memorust [-t|test] | [-a|add] *thing* *description*");
        }

        "-t" | "test" => {
            let boxes = memory_boxes::read_to_box(file.unwrap());
            memory_boxes::test_boxes(boxes);
        }

        "-a" | "add" => {
            let home_path = home::home_dir().unwrap();
            let file_path = format!("{}/.memorust.memrs", home_path.display());
            let file_path = Path::new(&file_path);
            let mut file = OpenOptions::new()
                            .create_new(true)
                            .write(true)
                            .append(true)
                            .open(file_path)
                            .unwrap();

            // [1] is beginning level
            let flash_card = format!("{}:{}\n", args[2], args[3]);
            file.write_all(flash_card.as_bytes()).unwrap();
        }
        _ => {
            panic!("memorust: no argument");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::memory_boxes::*;
    #[test]
    fn check_input() {
        let boxes = read_to_box("Who is father of Turkey?:Atatürk".to_string());
        assert_eq!(boxes, vec![vec!["Who is father of Turkey?", "Atatürk"]]);
    }
}