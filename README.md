# tictactoe-console-rust

fn straight(_fsquare : u32, _lsquare : u32) -> Vec<(u32, u32)> {
	let squares = vec![0, 1, 2];
	squares.map(|x| squares.map (|y| vec![x, y]));
}
