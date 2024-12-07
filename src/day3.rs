use std::fs;

use byte_parser::{ParseIterator, StrParser};
use memchr::{memchr2_iter, memmem};

fn main() {
	let content = fs::read_to_string("./inputs/day3.txt").unwrap();

	let mut total = 0;

	for idx in memmem::find_iter(content.as_bytes(), "mul(") {
		let mut parser = StrParser::new(&content[idx + 4..]);

		let Some(a) = parse_number(&mut parser) else {
			continue;
		};

		// now should come a comma
		if parser.next_if(|c| *c == b',').is_none() {
			continue;
		}

		let Some(b) = parse_number(&mut parser) else {
			continue;
		};

		if parser.next_if(|c| *c == b')').is_none() {
			continue;
		}

		total += a * b;
	}

	println!("Total: {total}");

	let mut total = 0;
	let mut ignore_mul = false;

	for idx in memchr2_iter(b'd', b'm', content.as_bytes()) {
		let slice = &content[idx..];

		if slice.starts_with("do()") {
			ignore_mul = false;
			continue;
		} else if slice.starts_with("don't()") {
			ignore_mul = true;
			continue;
		} else if !slice.starts_with("mul(") || ignore_mul {
			continue;
		}

		let mut parser = StrParser::new(&content[idx + 4..]);

		let Some(a) = parse_number(&mut parser) else {
			continue;
		};

		// now should come a comma
		if parser.next_if(|c| *c == b',').is_none() {
			continue;
		}

		let Some(b) = parse_number(&mut parser) else {
			continue;
		};

		if parser.next_if(|c| *c == b')').is_none() {
			continue;
		}

		total += a * b;
	}

	println!("Total: {total}");
}

fn parse_number<'s>(iter: &mut impl ParseIterator<'s>) -> Option<usize> {
	let mut iter = iter.record();

	let count = iter
		.while_byte_fn(u8::is_ascii_digit)
		.consume_at_least_and_count(1);

	if !matches!(count, Ok(c) if c <= 3) {
		return None;
	}

	iter.to_str().parse().ok()
}
