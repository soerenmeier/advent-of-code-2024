use std::fs;

#[rustfmt::skip]
const SEARCHES_PART_1: &[&[&[Option<u8>]]] = &[
	&[
		&[Some(b'X'), Some(b'M'), Some(b'A'), Some(b'S')],
	],
	&[
		&[Some(b'S'), Some(b'A'), Some(b'M'), Some(b'X')],
	],
	&[
		&[Some(b'X')],
		&[Some(b'M')],
		&[Some(b'A')],
		&[Some(b'S')],
	],
	&[
		&[Some(b'S')],
		&[Some(b'A')],
		&[Some(b'M')],
		&[Some(b'X')],
	],
	&[
		&[Some(b'X'), None, None, None],
		&[None, Some(b'M'), None, None],
		&[None, None, Some(b'A'), None],
		&[None, None, None, Some(b'S')],
	],
	&[
		&[Some(b'S'), None, None, None],
		&[None, Some(b'A'), None, None],
		&[None, None, Some(b'M'), None],
		&[None, None, None, Some(b'X')],
	],
	&[
		&[None, None, None, Some(b'X')],
		&[None, None, Some(b'M'), None],
		&[None, Some(b'A'), None, None],
		&[Some(b'S'), None, None, None],
	],
	&[
		&[None, None, None, Some(b'S')],
		&[None, None, Some(b'A'), None],
		&[None, Some(b'M'), None, None],
		&[Some(b'X'), None, None, None],
	]
];

#[rustfmt::skip]
const SEARCHES_PART_2: &[&[&[Option<u8>]]] = &[
	&[
		&[Some(b'M'), None, Some(b'S')],
		&[None, Some(b'A'), None],
		&[Some(b'M'), None, Some(b'S')],
	],
	&[
		&[Some(b'S'), None, Some(b'S')],
		&[None, Some(b'A'), None],
		&[Some(b'M'), None, Some(b'M')]
	],
	&[
		&[Some(b'M'), None, Some(b'M')],
		&[None, Some(b'A'), None],
		&[Some(b'S'), None, Some(b'S')]
	],
	&[
		&[Some(b'S'), None, Some(b'M')],
		&[None, Some(b'A'), None],
		&[Some(b'S'), None, Some(b'M')]
	],
];

fn main() {
	let content = fs::read_to_string("./inputs/day4.txt").unwrap();

	// the word can appear forwards or backwards
	// vertically or horizontally
	// diagonally or not diagonally

	// This will maybe not be that optimized but should be fine

	let lines = content.lines().collect::<Vec<_>>();

	let found = search(SEARCHES_PART_1, &lines);

	println!("Found XMAS {found} times");

	let found = search(SEARCHES_PART_2, &lines);

	println!("Found X-MAS {found} times");
}

fn search(searches: &[&[&[Option<u8>]]], lines: &[&str]) -> usize {
	let width = lines[0].len();
	let height = lines.len();
	let mut found = 0;

	for y in 0..height {
		for x in 0..width {
			'search: for search in searches {
				let swidth = search[0].len();
				let sheight = search.len();

				// we cannot do the search
				if swidth > width - x || sheight > height - y {
					continue;
				}

				for (sy, sline) in search.iter().enumerate() {
					let line = &lines[y + sy][x..];

					for (a, b) in sline.iter().zip(line.as_bytes()) {
						// if the search does not match let's check the next one
						if matches!(a, Some(a) if a != b) {
							continue 'search;
						}
					}
				}

				// the search was successful
				found += 1;
			}
		}
	}

	found
}
