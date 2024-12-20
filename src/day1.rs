use std::{collections::HashMap, fs, time::Instant};

fn main() {
	let content = fs::read_to_string("./inputs/day1.txt").unwrap();

	let (mut arr1, mut arr2): (Vec<usize>, Vec<usize>) = content
		.lines()
		.map(|line| {
			let mut numbers =
				line.split("   ").map(|num| num.parse::<usize>().unwrap());

			(numbers.next().unwrap(), numbers.next().unwrap())
		})
		.unzip();

	arr1.sort();
	arr2.sort();

	let dist = arr1
		.iter()
		.zip(arr2.iter())
		.map(|(a, &b)| a.abs_diff(b))
		.sum::<usize>();

	println!("Distance: {}", dist);

	let start = Instant::now();

	let mut occ2 = HashMap::with_capacity(arr2.len());

	for b in arr2 {
		*occ2.entry(b).or_insert(0) += 1;
	}

	let similarity_score = arr1
		.iter()
		.map(|a| a * occ2.get(a).unwrap_or(&0))
		.sum::<usize>();

	let duration = start.elapsed();

	println!(
		"Similarity score: {} took: {}ns",
		similarity_score,
		duration.as_nanos()
	);
}
