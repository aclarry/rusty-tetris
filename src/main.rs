extern crate termion;

use termion::{color, cursor, clear, style};
use termion::raw::IntoRawMode;
use std::io::{Write, stdout};

const X_SIZE: usize = 20;
const Y_SIZE: usize = 20;

#[derive(Clone, Copy)]
enum Block {
    Blank,
    T,
    I,
    L,
    LRev,
    Z,
    S,
    Square
}
    
impl Block {

    fn draw(self, mut out: &mut termion::raw::RawTerminal<std::io::Stdout>) {
        match self {
            Block::Blank  => write!(&mut out, " "),
            Block::T      => write!(&mut out,
                                    "{}{}T",
                                    style::Bold, color::Fg(color::LightRed).to_string()),
            Block::I      => write!(&mut out,
                                    "{}{}I",
                                    style::Bold, color::Fg(color::Yellow).to_string()),
            Block::L      => write!(&mut out,
                                    "{}{}L",
                                    style::Bold, color::Fg(color::LightBlue).to_string()),
            Block::LRev   => write!(&mut out,
                                    "{}{}J",
                                    style::Bold, color::Fg(color::LightRed).to_string()),
            Block::Z      => write!(&mut out,
                                    "{}{}Z",
                                    style::Bold, color::Fg(color::LightRed).to_string()),
            Block::S      => write!(&mut out,
                                    "{}{}S",
                                    style::Bold, color::Fg(color::LightRed).to_string()),
            Block::Square => write!(&mut out,
                                    "{}{}D",
                                   style::Bold, color::Fg(color::LightRed).to_string()),
        }.unwrap();
    }
}


fn main() {
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut board = init_board();
    draw_board(&board, &mut stdout);

    write!(&mut stdout, "\n\r").unwrap();
    //write!(&mut stdout, "{}\r{}", clear::All, cursor::Goto(1, 1)).unwrap();
}

fn gameloop(mut board: [[Block; X_SIZE]; Y_SIZE],
            mut out: &mut termion::raw::RawTerminal<std::io::Stdout>) {

    while (true) {
        draw_board(&mut board, &mut out);
    }

}


fn init_board() -> [[Block; X_SIZE] ;Y_SIZE] {
    let mut board = [[Block::Blank; X_SIZE]; Y_SIZE];
    board[1][3] = Block::T;
    board[1][4] = Block::T;
    board[1][5] = Block::T;
    board[2][4] = Block::T;
    board
}


fn draw_board(board: &[[Block; X_SIZE]; Y_SIZE],
              mut out: &mut termion::raw::RawTerminal<std::io::Stdout>) {

    write!(&mut out, "{}", clear::All).unwrap();
    write!(&mut out, "{}", cursor::Goto(1, 1)).unwrap();
    for _ in 0..X_SIZE+2 {
        write!(&mut out, "=").unwrap();
    }

    write!(&mut out, "\n\r").unwrap();

    for row in board.iter() {
        write!(&mut out, "{}|", color::Fg(color::Reset)).unwrap();
        for square in row.iter() {
            square.draw(&mut out);
        }
        write!(&mut out, "{}|\n\r", color::Fg(color::Reset)).unwrap();
    }

    for _ in 0..X_SIZE+2 {
        write!(&mut out, "=").unwrap();
    }
}


