use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::ops::Add;

#[derive(PartialEq, Clone, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
struct Pos {
    x: i128,
    y: i128,
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct Board {
    size: Pos,
    blocks: Vec<Pos>,
    targets: Vec<Pos>,
    walls: Vec<Pos>,
    player: Pos,
}

impl Board {
    fn new() -> Self {
        Board {
            size: Pos { x: 0, y: 0 },
            blocks: vec![],
            targets: vec![],
            walls: vec![],
            player: Pos { x: 0, y: 0 },
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
struct Step {
    current_board: Board,
    move_nr: i128,
    move_prev: Direction,
    parent: Option<Box<Step>>,
}

fn main() {
    let contents = fs::read_to_string("txt/intro.txt").expect("failed to read intro.txt");
    println!("{}", contents);
    let mut board: Vec<String> = vec![];
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let content = line.as_ref().unwrap();
        let content = "examples/1.txt";
        if content == "" {
            //confirm board
            break;
        } else if content.chars().any(|c| c == 'x') {
            //file name entered
            let board_file = fs::read_to_string(content).expect("not a valid file!");
            board = board_file.split("\r\n").map(|x| x.to_string()).collect();
            break;
        } else if content.chars().any(|c| !"$#@.* ".contains(c)) {
            //not a valid board
            println!("--> [line contained a character that wasn't valid]");
            continue;
        }
        board.push(line.unwrap().clone());
    }

    solve(build_board(board));
}

fn build_board(st: Vec<String>) -> (Board, Vec<Pos>) {
    let (mut b, mut s): (Board, Vec<Pos>) = (Board::new(), vec![]);
    let mut pos: Pos = Pos { x: 0, y: 0 };
    for l in st.iter() {
        for c in l.chars() {
            match c {
                '$' => b.blocks.push(pos),
                '#' => b.walls.push(pos),
                '*' => {
                    b.blocks.push(pos);
                    b.targets.push(pos);
                    s.push(pos);
                }
                '.' => {
                    b.targets.push(pos);
                    s.push(pos);
                }
                '+' => {
                    b.player = pos;
                    b.targets.push(pos);
                    s.push(pos);
                }
                '@' => b.player = pos,
                _ => {}
            }
            pos.x += 1;
        }
        pos.y += 1;
        pos.x = 0;
    }

    b.size = Pos {
        x: st[0].chars().count() as i128,
        y: st.len() as i128,
    };

    if b.targets.len() != b.blocks.len() {
        panic!("The number of targets is not the same as the number of blocks!");
    }
    (b, s)
}

fn solve((b, s): (Board, Vec<Pos>)) {
    let mut visited_boards = HashSet::new();
    let mut queue: VecDeque<Step> = VecDeque::new();
    queue.push_back(Step {
        current_board: b,
        move_nr: 0,
        move_prev: Direction::Left,
        parent: None,
    });

    while !queue.is_empty() {
        let nxt_step = queue.pop_front().unwrap();
        if nxt_step.current_board.blocks == s {
            println!(
                "length: {0} moves: [{1} ]",
                nxt_step.move_nr,
                collect_moves(&nxt_step)
            );
            return;
        }

        for i in 0..4 {
            if let Some(i) = add_move(
                nxt_step.clone(),
                nxt_step.current_board.clone(),
                Pos {
                    x: i % 2 * (1 - 2 * (i > 1) as i128),
                    y: (1 - i % 2) * (1 - 2 * (i > 1) as i128),
                },
            ) {
                if !visited_boards.contains(&i.current_board) {
                    visited_boards.insert(i.current_board.clone());
                    queue.push_back(i);
                } else {
                    println!("not new");
                }
            }
        }
    }
}

fn collect_moves(mut s: &Step) -> String {
    let mut move_str = String::new();
    loop {
        match s.move_prev {
            Direction::Left => move_str.push_str("← "),
            Direction::Right => move_str.push_str("→ "),
            Direction::Up => move_str.push_str("↑ "),
            Direction::Down => move_str.push_str("↓ "),
        }

        if let Some(i) = &s.parent {
            s = &i;
        } else {
            break;
        }
        if s.move_nr == 0 {
            break;
        }
    }
    move_str.chars().rev().collect::<String>()
}

fn add_move(s: Step, mut curr_board: Board, mov: Pos) -> Option<Step> {
    let player_pos = curr_board.player + mov;
    //player is out of bounds
    if !(0..curr_board.size.x + 1).contains(&player_pos.x)
        || !(0..curr_board.size.y + 1).contains(&player_pos.y)
        || curr_board.walls.contains(&player_pos)
    {
        return None;
    }
    //player is pushing a block
    if let Some(i) = curr_board.blocks.iter().position(|x| x == &player_pos) {
        println!("pushing block");
        let block_pos = curr_board.blocks[i] + mov;
        if !(0..curr_board.size.x + 1).contains(&block_pos.x)
            || !(0..curr_board.size.y + 1).contains(&block_pos.y)
            || curr_board.walls.contains(&block_pos)
        {
            println!("blockpos : {:?}", curr_board.blocks[i]);
            println!("failed (wall in da way) {:?}", block_pos);
            println!("all blocks: {:?}", curr_board.blocks);
            return None;
        }

        if curr_board.blocks.contains(&block_pos) {
            println!("failed (block in da way) {:?}", block_pos);
            return None;
        }
        curr_board.blocks[i] = block_pos;
    }
    curr_board.player = curr_board.player + mov;

    if s.move_nr > 33 {
        panic!("end");
    }

    let new_step = Step {
        current_board: curr_board,
        move_nr: s.move_nr + 1,
        move_prev: {
            match mov {
                Pos { x: -1, y: 0 } => Direction::Left,
                Pos { x: 1, y: 0 } => Direction::Right,
                Pos { x: 0, y: -1 } => Direction::Up,
                _ => Direction::Down,
            }
        },
        parent: Some(Box::new(s)),
    };
    println!("{}", collect_moves(&new_step));
    Some(new_step)
}
