use aoc_runner_derive::aoc;
use smallvec::SmallVec;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize
{
	input
		.lines()
		.filter(|x| !x.is_empty())
		.map(is_safe)
		.filter(|&x| x)
		.count()
}

fn is_safe(line: &str) -> bool
{
	let line = line.trim();
	let numbers = line.split(' ').map(|x| x.parse().unwrap());
	is_safe_impl(numbers)
}

fn is_safe_impl(mut numbers: impl Iterator<Item = u32>) -> bool
{
	let a: u32 = numbers.next().unwrap();
	let b = numbers.next().unwrap();
	if a < b
	{
		if a + 3 < b
		{
			return false;
		}
		let mut x = b;
		while let Some(y) = numbers.next()
		{
			if x >= y || x + 3 < y
			{
				return false;
			}
			x = y;
		}
		true
	}
	else if a > b
	{
		if a > b + 3
		{
			return false;
		}
		let mut x = b;
		while let Some(y) = numbers.next()
		{
			if x <= y || x > y + 3
			{
				return false;
			}
			x = y;
		}
		true
	}
	else
	{
		false
	}
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize
{
	input
		.lines()
		.filter(|x| !x.is_empty())
		.map(is_safe_with_dampener)
		.filter(|&x| x)
		.count()
}

fn is_safe_with_dampener(line: &str) -> bool
{
	let line = line.trim();
	let numbers: SmallVec<[(usize, u32); 10]> = line
		.split(' ')
		.map(|x| x.parse().unwrap())
		.enumerate()
		.collect();
	for i in 0..numbers.len()
	{
		let numbers = numbers.iter().filter(|(j, _)| *j != i).map(|(_, x)| *x);
		if is_safe_impl(numbers)
		{
			return true;
		}
	}
	false
}

#[cfg(test)]
mod tests
{
	use super::*;

	const EXAMPLE: &str = "7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9";

	#[test]
	fn part1_example()
	{
		assert_eq!(part1(EXAMPLE), 2);
	}

	#[test]
	fn part1_simple()
	{
		assert_eq!(part1("1 4"), 1);
		assert_eq!(part1("1 5"), 0);
	}

	#[test]
	fn part2_example()
	{
		assert_eq!(part2(EXAMPLE), 4);
	}
}
