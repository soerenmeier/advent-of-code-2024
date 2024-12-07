use std::fs;

fn main() {
	let content = fs::read_to_string("./inputs/day2.txt").unwrap();

	let safe_reports = content
		.lines()
		.map(|line| {
			let numbers =
				line.split(" ").map(|num| num.parse::<usize>().unwrap());

			safe_report(numbers) as usize
		})
		.sum::<usize>();

	println!("Safe reports: {}", safe_reports);

	// do the safety report with the dampener
	let safe_reports_dampened = content
		.lines()
		.map(|line| {
			let numbers =
				line.split(" ").map(|num| num.parse::<usize>().unwrap());

			safe_report_dampener(numbers) as usize
		})
		.sum::<usize>();

	println!("Safe reports with dampener: {}", safe_reports_dampened);
}

// needs at least one report
fn safe_report(mut reports: impl Iterator<Item = usize>) -> bool {
	let mut previous = reports.next().unwrap();
	let mut increasing = None;

	for report in reports {
		let dist = report.abs_diff(previous);
		if dist < 1 || dist > 3 {
			return false;
		}

		let incr = previous < report;

		let increasing = increasing.get_or_insert(incr);
		if *increasing != incr {
			return false;
		}

		previous = report;
	}

	true
}

// needs at least one report
fn safe_report_dampener(reports: impl Iterator<Item = usize>) -> bool {
	// well i though i could do this linearly in one go but i don't know
	// how
	let reports = reports.collect::<Vec<_>>();

	// let's first check with the whole report
	if safe_report(reports.iter().copied()) {
		return true;
	}

	// well now we need to find which one is the bad one
	for i in 0..reports.len() {
		if safe_report(
			reports
				.iter()
				.enumerate()
				.filter(|(j, _)| *j != i)
				.map(|(_, v)| *v),
		) {
			return true;
		}
	}

	false
}
