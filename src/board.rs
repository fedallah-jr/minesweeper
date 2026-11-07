use rand::seq::index;

pub const M: usize = 9;
pub const N: usize = 10;
pub const MINES: usize = 10;

#[derive(Clone,Copy)]
pub enum CellState {
    Hidden(bool),
    Flagged(bool),
    Revealed(u8),
    Exploded(),
}

pub enum Outcome {
    Error(&'static str),
    Continue { revealed: usize },
    Lost(),
}

pub struct Board {
    pub cells: [[CellState; N]; M],
    }

impl Board {
    pub fn new() -> Self {
	Self{
	    cells: [[CellState::Hidden(false);N];M],
	}
    }
    pub fn initialize(&mut self) {
	let mut rng = rand::rng();
	let numbers = index::sample(&mut rng, M*N,MINES);
	let mut m: Vec<usize> = Vec::with_capacity(M*N);
	let mut n: Vec<usize> = Vec::with_capacity(M*N);
	for number in numbers {
	    m.push( number / N);
	    n.push(number % N);
	}
	for i in 0..m.len() {
	    self.cells[m[i]][n[i]] = CellState::Hidden(true);
	}
    }
    // Returns an enum on the possible outcomes of the move. 
    pub fn advance(&mut self, p_move: String) -> Outcome {
	let parts = p_move.split_whitespace();
	let mut p_move_vec: Vec<char> = Vec::with_capacity(3);
	if parts.clone().count() != 3 {
	    return Outcome::Error("Invalid input: write a move like \"f 2 3\" or \"r 2 3\"");
	}
	for part in parts {
	    // We cannot parse something bigger than 10 with this code. So be it!
	    if part.len() > 1  {
		return Outcome::Error("Invalid input: each part must be a single character");
	    }
	    let Some(pp) = part.chars().next() else
	    {
		return Outcome::Error("Invalid input: empty part");
	    };
	    
	    p_move_vec.push(pp);
	    
	}

	let (Some(row), Some(col)) = (p_move_vec[1].to_digit(10), p_move_vec[2].to_digit(10)) else {
	    return Outcome::Error("Invalid input: row and column must be digits");
	};
	let row = row as usize;
	let col = col as usize;

	if row > 8 || col > 9 {
	    return Outcome::Error("Out of bounds: row must be 0-8 and column 0-9");
	}
	else {
	    let se = &mut self.cells[row][col];
	    if p_move_vec[0] == 'f' || p_move_vec[0] == 'F' {
		match *se {
		    CellState::Hidden(cev) => {
			*se = CellState::Flagged(cev);
		    }
		    CellState::Flagged(cev) => {
			*se = CellState::Hidden(cev);
		    }
		    CellState::Revealed(_) => {
			return Outcome::Error("Do not flag a revealed position");
		    }
		    CellState::Exploded() => unreachable!("the game ends when a mine explodes"),
		}
		return Outcome::Continue { revealed: 0 };
	    }
	    else if p_move_vec[0] == 'r' || p_move_vec[0] == 'R' {
		match *se {
		    CellState::Hidden(true) => {*se = CellState::Exploded(); return Outcome::Lost();},
		    CellState::Hidden(false) => {
			return Outcome::Continue { revealed: self.reveal(row, col) };
		    }
		    CellState::Flagged(_) => return Outcome::Error("Cannot reveal a flagged cell, unflag it first"),
		    CellState::Revealed(_) => return Outcome::Error("Cell is already revealed"),
		    CellState::Exploded() => unreachable!("the game ends when a mine explodes"),
		}
	    }
	    else {
		return Outcome::Error("Unknown move: use 'f' to flag or 'r' to reveal");
	    }
	}

	
	
    }

    // Counts the mines (hidden or flagged) around (row, col).
    fn count_adjacent_mines(&self, row: usize, col: usize) -> u8 {
	let mut count: u8 = 0;
	for (nr, nc) in neighbors(row, col) {
	    match self.cells[nr][nc] {
		CellState::Hidden(true) | CellState::Flagged(true) => count += 1,
		_ => {}
	    }
	}
	count
    }
    // Mark it revealed BEFORE recursing, so neighbors do not reveal it again.
    fn reveal(&mut self, row: usize, col: usize) -> usize {

	let CellState::Hidden(false) = self.cells[row][col] else {
	    return 0;
	};

	let count = self.count_adjacent_mines(row, col);
	self.cells[row][col] = CellState::Revealed(count);

	let mut revealed = 1;
	if count == 0 {
	    for (nr, nc) in neighbors(row, col) {
		revealed += self.reveal(nr, nc);
	    }
	}
	revealed
    }
}

fn neighbors(m: usize, n: usize) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::with_capacity(8);
    for dm in -1isize..=1 {
	for dn in -1isize..=1 {
	    if dm == 0 && dn == 0 {
		continue;
	    }
	    let row = m as isize + dm;
	    let col = n as isize + dn;
	    if row >= 0 && row < M as isize && col >= 0 && col < N as isize {
		result.push((row as usize, col as usize));
	    }
	}
    }
    result
}

