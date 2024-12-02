use smallvec::SmallVec;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32
{
	let mut xs: SmallVec<[i32; 1024]> = SmallVec::new();
	let mut ys: SmallVec<[i32; 1024]> = SmallVec::new();
	for line in input.lines()
	{
		if line.is_empty()
		{
			continue;
		}
		let mut parts = line.split_ascii_whitespace();
		let x = parts.next().unwrap();
		let y = parts.next().unwrap();
		let x: i32 = x.parse().unwrap();
		let y: i32 = y.parse().unwrap();
		xs.push(x);
		ys.push(y);
	}
	xs.sort_unstable();
	ys.sort_unstable();
	let xs = xs.into_iter();
	let ys = ys.into_iter();
	xs.zip(ys).map(|(x, y)| (x - y).abs()).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32
{
	let mut xs: SmallVec<[u32; 1024]> = SmallVec::new();
	let mut ys: SmallVec<[u32; 1024]> = SmallVec::new();
	for line in input.lines()
	{
		if line.is_empty()
		{
			continue;
		}
		let mut parts = line.split_ascii_whitespace();
		let x = parts.next().unwrap();
		let y = parts.next().unwrap();
		let x: u32 = x.parse().unwrap();
		let y: u32 = y.parse().unwrap();
		xs.push(x);
		ys.push(y);
	}
	xs.sort_unstable();
	ys.sort_unstable();
	let xs: &[u32] = &xs;
	let ys: &[u32] = &ys;
	let mut i = 0;
	let mut j = 0;
	let mut score: u32 = 0;
	loop
	{
		let x = xs[i];
		while x > ys[j]
		{
			j += 1;
			if j == ys.len()
			{
				return score;
			}
		}
		let mut b = 0;
		while x == ys[j]
		{
			b += 1;
			j += 1;
			if j == ys.len()
			{
				break;
			}
		}
		if b == 0
		{
			i += 1;
			if i == xs.len()
			{
				return score;
			}
			continue;
		}
		let mut a = 1;
		i += 1;
		if i < xs.len()
		{
			while xs[i] == x
			{
				a += 1;
				i += 1;
				if i == xs.len()
				{
					break;
				}
			}
		}
		score += x * a * b;
		if i == xs.len() || j == ys.len()
		{
			return score;
		}
	}
}

#[cfg(test)]
mod tests
{
	use pretty_assertions::assert_eq;

	use super::*;

	#[test]
	fn test_day1_part1_given()
	{
		let given = "3   4
            4   3
            2   5
            1   3
            3   9
            3   3";
		assert_eq!(part1(given), 11);
	}

	#[test]
	fn test_day1_part2_given()
	{
		let given = "3   4
            4   3
            2   5
            1   3
            3   9
            3   3";
		assert_eq!(part2(given), 31);
	}
}
