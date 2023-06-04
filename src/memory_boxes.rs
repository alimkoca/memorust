use std::io::{stdin, stdout, Write};

pub fn read_to_box(boxes: String) -> Vec<Vec<String>> {
    let mut box_list: Vec<Vec<String>> = Vec::new();
    let mut question = String::new();
    let mut answer = String::new();
    let mut box_chars = boxes.chars().peekable();

    loop {
        if box_chars.peek().is_none() {
            break;
        }

        while let Some(c) = box_chars.next() {
            if c == ':' {
                break;
            }
            question.push(c);
        }

        while let Some(c) = box_chars.next() {
            if c == '\n' {
                break;
            }
            answer.push(c);
        }

        box_list.push(vec![question.clone(), answer.clone()]);
        question = String::from("");
        answer = String::from("");
    }

    box_list
}

pub fn test_boxes(boxes: Vec<Vec<String>>) {
    let mut answer = String::new();

    for cards in boxes {
        print!("{}: ", cards[0]);
        let _ = stdout().flush();
        stdin().read_line(&mut answer)
            .unwrap();

        if answer.trim() != cards[1] {
            println!("(memorust: answer is incorrect, skipping...)");
            let _ = stdout().flush();
        } else {
            println!("(memorust: answer is correct! appreciating you!)");
        }
    }
}
