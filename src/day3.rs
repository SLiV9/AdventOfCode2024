use aoc_runner_derive::aoc;
use regex::Regex;

#[aoc(day3, part1)]
fn part1(input: &str) -> u32
{
	let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
	re.captures_iter(input)
		.map(|c| c.extract())
		.map(|(_, [a, b])| {
			let a: u32 = a.parse().unwrap();
			let b: u32 = b.parse().unwrap();
			a * b
		})
		.sum()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32
{
	let re =
		Regex::new(r"(do\(\))|(don't\(\))|mul\(([0-9]{1,3}),([0-9]{1,3})\)")
			.unwrap();
	let mut sum = 0;
	let mut is_active = true;
	for c in re.captures_iter(input)
	{
		if c.get(1).is_some()
		{
			is_active = true;
		}
		else if c.get(2).is_some()
		{
			is_active = false;
		}
		else if is_active
		{
			let a: u32 = c.get(3).unwrap().as_str().parse().unwrap();
			let b: u32 = c.get(4).unwrap().as_str().parse().unwrap();
			sum += a * b;
		}
	}
	sum
}

#[cfg(test)]
mod tests
{
	use super::*;

	const EXAMPLE1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

	#[test]
	fn part1_example()
	{
		assert_eq!(part1(EXAMPLE1), 161);
	}

	const EXAMPLE2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

	#[test]
	fn part2_example()
	{
		assert_eq!(part2(EXAMPLE2), 48);
	}

	#[test]
	fn part2_example1() {
		assert_eq!(part2(EXAMPLE1), 161);
	}
}
