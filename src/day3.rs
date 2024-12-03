use aoc_runner_derive::aoc;
use regex::Regex;

#[aoc(day3, part1)]
fn part1(input: &str) -> u32
{
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
	re.captures_iter(input).map(|c| c.extract()).map(|(_, [a, b])| {
        let a: u32 = a.parse().unwrap();
        let b: u32 = b.parse().unwrap();
        a * b
    }).sum()
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u32
{
	todo!()
}

#[cfg(test)]
mod tests
{
	use super::*;

	const EXAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

	#[test]
	fn part1_example()
	{
		assert_eq!(part1(EXAMPLE), 161);
	}

	#[test]
	fn part2_example()
	{
		assert_eq!(part2(EXAMPLE), 0);
	}
}
