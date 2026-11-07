mod board;
use std::io;
use board::Board;
mod render;
fn main() {
    let mut game_board = Board::new();
    game_board.initialize();
    let mut count: usize = 0;
    let mut message = "Game Starts!";

    loop {
	print!("\x1B[2J\x1B[H"); //this wierd thing clears the screen somehow.
        render::render(&game_board);
	println!("{message}");
        println!("Write your move:");
        let mut input = String::new();
        let bytes = io::stdin().read_line(&mut input).expect("Failed to read the line");
	if bytes == 0 {
	    break();
	    // AI audit told me that this is a bug I need to adress but I don't know why this is a bug
	}
        let result = Board::advance(&mut game_board, input);
	match result {
	    board::Outcome::Error(cev) => { message = cev; },
	    board::Outcome::Lost() => {print!("\x1B[2J\x1B[H"); render::render_all(&game_board); println!("You lose"); break();},
	    board::Outcome::Continue{revealed} => { count = count + revealed; message = "";},
	}
	if count >= board::M * board::N - board::MINES {
	    print!("\x1B[2J\x1B[H");
	    render::render_all(&game_board);
	    println!("You won!");
	   
	    break();
	}
       
    }
}
