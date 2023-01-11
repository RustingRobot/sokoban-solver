use std::fs;
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
enum Element {
    Block,
    Wall,
    Space,
    Player,
}
#[derive(Debug)]
struct Board {
    size: (usize, usize),
    fields: Vec<Element>,
}

fn main() {
    let contents = fs::read_to_string("txt/intro.txt").expect("failed to read intro.txt");
    println!("{}", contents);
    let mut board: Vec<String> = vec![];
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let content = line.as_ref().unwrap();
        if content == "" {
            //confirm board
            break;
        } else if content.chars().any(|c| c == '.') {
            //file name entered
            let board_file = fs::read_to_string(content).expect("not a valid file!");
            board = board_file.split("\r\n").map(|x| x.to_string()).collect();
            break;
        } else if content.chars().any(|c| !"bwsp".contains(c)) {
            //not a valid board
            println!("--> [line contained a character that wasn't b, w, s or p]");
            continue;
        }
        board.push(line.unwrap().clone());
    }
    let full_board = build_board(board);
    println!("{:?}", full_board);
}

fn build_board(b: Vec<String>) -> Board {
    let mut fields: Vec<Element> = vec![];
    for l in b.iter() {
        for c in l.chars() {
            fields.push(match c {
                'b' => Element::Block,
                'w' => Element::Wall,
                's' => Element::Space,
                _ => Element::Player,
            });
        }
    }

    Board {
        size: (b[0].len(), b.len()),
        fields: fields,
    }
}
