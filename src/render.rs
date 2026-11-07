use crate::board::Board;
use crate::board::CellState::*;
use crate::board::N;

// You might ask why there are 2 function with nearly identical bodies.
// Truth is copy pasting was more convenient.
pub fn render(cevat: &Board) {
    print!("  ");
    for col in 0..N {
	print!(" {} ", col);
    }
    println!();

    for (row, cellstate_hor) in cevat.cells.iter().enumerate() {
	// Row number at the start of each line
	print!("{} ", row);
	for cellstate in cellstate_hor {
	    match cellstate {
		Hidden(_) => { print!(" . ");},
		Flagged(_) => { print!(" F ");},
		Revealed(cemsit) => { print!(" {} ", cemsit);},
		Exploded() => { print!(" * ");},
	    }

	}
	println!();


    }
}
pub fn render_all(cevat: &Board) {
    print!("  ");
    for col in 0..N {
	print!(" {} ", col);
    }
    println!();

    for (row, cellstate_hor) in cevat.cells.iter().enumerate() {
	// Row number at the start of each line
	print!("{} ", row);
	for cellstate in cellstate_hor {
	    match cellstate {
		Hidden(false) => { print!(" . ");},
		Hidden(true) => { print!(" M ");},
		Flagged(false) => { print!(" F ");},
		Flagged(true) => { print!(" M ");}
		Revealed(cemsit) => { print!(" {} ", cemsit);},
		Exploded() => { print!(" * ");},
	    }

	}
	println!();


    }
}
