use std::{collections::HashSet, fs};

fn main() {
	let content = fs::read_to_string("./inputs/day6.txt").unwrap();

	let lines = content.lines().map(|l| l.as_bytes()).collect::<Vec<_>>();
	let width = lines[0].len();
	let height = lines.len();

	// find the guard
	let (mut guard_dir, mut guard) = lines
		.iter()
		.enumerate()
		.find_map(|(y, l)| {
			l.iter().enumerate().find_map(|(x, c)| {
				Dir::from_byte(*c).map(|dir| (dir, Pos { x, y }))
			})
		})
		.unwrap();

	let mut positions = HashSet::new();
	positions.insert(guard);

	loop {
		let mut x = guard.x as isize;
		let mut y = guard.y as isize;

		match guard_dir {
			Dir::Up => y -= 1,
			Dir::Right => x += 1,
			Dir::Down => y += 1,
			Dir::Left => x -= 1,
		};

		// check if out of bounds
		if x < 0 || y < 0 || x >= width as isize || y >= height as isize {
			break;
		}

		// probably not that ideomatic
		let x = x as usize;
		let y = y as usize;

		// now check if something is at that spot
		if lines[y][x] == b'#' {
			// an obstacle was found
			guard_dir.turn_right();
			// try again
			continue;
		}

		// update the guard
		guard = Pos { x, y };
		positions.insert(guard);
	}

	println!("Positions: {}", positions.len());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
	x: usize,
	y: usize,
}

#[derive(Debug, Clone, Copy)]
enum Dir {
	Up,
	Right,
	Down,
	Left,
}

impl Dir {
	fn from_byte(b: u8) -> Option<Self> {
		Some(match b {
			b'^' => Dir::Up,
			b'>' => Dir::Right,
			b'v' => Dir::Down,
			b'<' => Dir::Left,
			_ => return None,
		})
	}

	fn turn_right(&mut self) {
		*self = match self {
			Dir::Up => Dir::Right,
			Dir::Right => Dir::Down,
			Dir::Down => Dir::Left,
			Dir::Left => Dir::Up,
		};
	}
}
