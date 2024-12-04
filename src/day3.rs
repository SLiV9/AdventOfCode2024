use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32
{
	let input = input.as_bytes();
	let mut sum = 0;
	let mut i = 0;
	let n = input.len();
	const MUL: u32 = u32::from_be_bytes([b'm', b'u', b'l', b'(']);
	'clean: while i + 7 < n
	{
		// Safety: we just checked that input[i + 7] was not out of bounds.
		let view: &[u8; 4] = unsafe {
			input.get_unchecked(i..).first_chunk().unwrap_unchecked()
		};
		let mut view = u32::from_be_bytes(*view);
		i += 4;
		while view != MUL
		{
			if i + 4 < n
			{
				// Safety: we just checked that input[i + 4] was not out of bounds.
				let x = unsafe { *input.get_unchecked(i) };
				view = (view << 8) | (x as u32);
				i += 1;
			}
			else
			{
				return sum;
			}
		}

		// We have just seen "mul(" and we have checked that input[i + 3] was not out of bounds.
		let next: &[u8; 4] = unsafe {
			input.get_unchecked(i..).first_chunk().unwrap_unchecked()
		};
		let next_view = u32::from_be_bytes(*next);
		i += 4;
		let _: std::convert::Infallible = match next_view & 0xF0F0F0F0
		{
			0x30203020 =>
			{
				let is_match: bool = (next[0] <= b'9')
					& (next[1] == b',')
					& (next[2] <= b'9')
					& (next[3] == b')');
				let a = (next[0] - b'0') as u32;
				let b = (next[2] - b'0') as u32;
				sum += u32::from(is_match) * a * b;
				continue 'clean;
			}
			0x30203030 =>
			{
				let is_match = (next[0] <= b'9')
					& (next[1] == b',')
					& (next[2] <= b'9')
					& (next[3] <= b'9');
				if is_match
				{
					let a = (next[0] - b'0') as u32;
					let mut b =
						(next[2] - b'0') as u32 * 10 + (next[3] - b'0') as u32;
					let j = i;
					while i < n
					{
						// Safety: we just checked that input[i] is not out of bounds.
						let x = unsafe { *input.get_unchecked(i) };
						i += 1;
						match x
						{
							b'0'..=b'9' =>
							{
								if i >= j + 2
								{
									continue 'clean;
								}
								b = b * 10 + (x - b'0') as u32;
							}
							b')' =>
							{
								sum += a * b;
								continue 'clean;
							}
							_ =>
							{
								i -= 1;
								continue 'clean;
							}
						}
					}
					return sum;
				}
				else
				{
					continue 'clean;
				}
			}
			0x30302030 =>
			{
				let is_match = (next[0] <= b'9')
					& (next[1] <= b'9')
					& (next[2] == b',')
					& (next[3] <= b'9');
				if is_match
				{
					let a =
						(next[0] - b'0') as u32 * 10 + (next[1] - b'0') as u32;
					let mut b = (next[3] - b'0') as u32;
					let j = i;
					while i < n
					{
						// Safety: we just checked that input[i] is not out of bounds.
						let x = unsafe { *input.get_unchecked(i) };
						i += 1;
						match x
						{
							b'0'..=b'9' =>
							{
								if i >= j + 3
								{
									continue 'clean;
								}
								b = b * 10 + (x - b'0') as u32;
							}
							b')' =>
							{
								sum += a * b;
								continue 'clean;
							}
							_ =>
							{
								i -= 1;
								continue 'clean;
							}
						}
					}
					return sum;
				}
				else
				{
					continue 'clean;
				}
			}
			0x30303020 =>
			{
				let is_match = (next[0] <= b'9')
					& (next[1] <= b'9')
					& (next[2] <= b'9')
					& (next[3] == b',');
				if is_match
				{
					let a = (next[0] - b'0') as u32 * 100
						+ (next[1] - b'0') as u32 * 10
						+ (next[2] - b'0') as u32;
					let mut b = 0;
					let j = i;
					while i < n
					{
						// Safety: we just checked that input[i] is not out of bounds.
						let x = unsafe { *input.get_unchecked(i) };
						i += 1;
						match x
						{
							b'0'..=b'9' =>
							{
								if i >= j + 4
								{
									continue 'clean;
								}
								b = b * 10 + (x - b'0') as u32;
							}
							b')' =>
							{
								sum += a * b;
								continue 'clean;
							}
							_ =>
							{
								i -= 1;
								continue 'clean;
							}
						}
					}
					return sum;
				}
				else
				{
					continue 'clean;
				}
			}
			_ =>
			{
				i -= 4;
				continue 'clean;
			}
		};
	}
	sum
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32
{
	let input = input.as_bytes();
	let mut sum = 0;
	let mut i = 0;
	let n = input.len();
	const MUL: u32 = u32::from_be_bytes([b'm', b'u', b'l', b'(']);
	const DON: u32 = u32::from_be_bytes([b'd', b'o', b'n', b'\'']);
	'clean: while i + 7 < n
	{
		// Safety: we just checked that input[i + 7] was not out of bounds.
		let view: &[u8; 4] = unsafe {
			input.get_unchecked(i..).first_chunk().unwrap_unchecked()
		};
		let mut view = u32::from_be_bytes(*view);
		i += 4;
		while view != MUL
		{
			if view == DON
			{
				if i + 2 < n
				{
					// We have just checked that input[i + 2] was not out of bounds.
					let next: &[u8; 3] = unsafe {
						input
							.get_unchecked(i..)
							.first_chunk()
							.unwrap_unchecked()
					};
					if *next == [b't', b'(', b')']
					{
						i += 3;
						while i + 4 + 8 - 1 < n
						{
							// We have just checked that input[i + 3] was not out of bounds.
							let next: &[u8; 4] = unsafe {
								input
									.get_unchecked(i..)
									.first_chunk()
									.unwrap_unchecked()
							};
							if *next == [b'd', b'o', b'(', b')']
							{
								i += 4;
								continue 'clean;
							}
							i += 1;
						}
						return sum;
					}
				}
				else
				{
					return sum;
				}
			}
			if i + 4 < n
			{
				// Safety: we just checked that input[i + 4] was not out of bounds.
				let x = unsafe { *input.get_unchecked(i) };
				view = (view << 8) | (x as u32);
				i += 1;
			}
			else
			{
				return sum;
			}
		}

		// We have just seen "mul(" and we have checked that input[i + 3] was not out of bounds.
		let next: &[u8; 4] = unsafe {
			input.get_unchecked(i..).first_chunk().unwrap_unchecked()
		};
		let next_view = u32::from_be_bytes(*next);
		i += 4;
		let _: std::convert::Infallible = match next_view & 0xF0F0F0F0
		{
			0x30203020 =>
			{
				let is_match: bool = (next[0] <= b'9')
					& (next[1] == b',')
					& (next[2] <= b'9')
					& (next[3] == b')');
				let a = (next[0] - b'0') as u32;
				let b = (next[2] - b'0') as u32;
				sum += u32::from(is_match) * a * b;
				continue 'clean;
			}
			0x30203030 =>
			{
				let is_match = (next[0] <= b'9')
					& (next[1] == b',')
					& (next[2] <= b'9')
					& (next[3] <= b'9');
				if is_match
				{
					let a = (next[0] - b'0') as u32;
					let mut b =
						(next[2] - b'0') as u32 * 10 + (next[3] - b'0') as u32;
					let j = i;
					while i < n
					{
						// Safety: we just checked that input[i] is not out of bounds.
						let x = unsafe { *input.get_unchecked(i) };
						i += 1;
						match x
						{
							b'0'..=b'9' =>
							{
								if i >= j + 2
								{
									continue 'clean;
								}
								b = b * 10 + (x - b'0') as u32;
							}
							b')' =>
							{
								sum += a * b;
								continue 'clean;
							}
							_ =>
							{
								i -= 1;
								continue 'clean;
							}
						}
					}
					return sum;
				}
				else
				{
					continue 'clean;
				}
			}
			0x30302030 =>
			{
				let is_match = (next[0] <= b'9')
					& (next[1] <= b'9')
					& (next[2] == b',')
					& (next[3] <= b'9');
				if is_match
				{
					let a =
						(next[0] - b'0') as u32 * 10 + (next[1] - b'0') as u32;
					let mut b = (next[3] - b'0') as u32;
					let j = i;
					while i < n
					{
						// Safety: we just checked that input[i] is not out of bounds.
						let x = unsafe { *input.get_unchecked(i) };
						i += 1;
						match x
						{
							b'0'..=b'9' =>
							{
								if i >= j + 3
								{
									continue 'clean;
								}
								b = b * 10 + (x - b'0') as u32;
							}
							b')' =>
							{
								sum += a * b;
								continue 'clean;
							}
							_ =>
							{
								i -= 1;
								continue 'clean;
							}
						}
					}
					return sum;
				}
				else
				{
					continue 'clean;
				}
			}
			0x30303020 =>
			{
				let is_match = (next[0] <= b'9')
					& (next[1] <= b'9')
					& (next[2] <= b'9')
					& (next[3] == b',');
				if is_match
				{
					let a = (next[0] - b'0') as u32 * 100
						+ (next[1] - b'0') as u32 * 10
						+ (next[2] - b'0') as u32;
					let mut b = 0;
					let j = i;
					while i < n
					{
						// Safety: we just checked that input[i] is not out of bounds.
						let x = unsafe { *input.get_unchecked(i) };
						i += 1;
						match x
						{
							b'0'..=b'9' =>
							{
								if i >= j + 4
								{
									continue 'clean;
								}
								b = b * 10 + (x - b'0') as u32;
							}
							b')' =>
							{
								sum += a * b;
								continue 'clean;
							}
							_ =>
							{
								i -= 1;
								continue 'clean;
							}
						}
					}
					return sum;
				}
				else
				{
					continue 'clean;
				}
			}
			_ =>
			{
				i -= 4;
				continue 'clean;
			}
		};
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

	#[test]
	fn part1_simple()
	{
		assert_eq!(part1("xmul(2,4)asd"), 8);
	}

	const EXAMPLE2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

	#[test]
	fn part2_example()
	{
		assert_eq!(part2(EXAMPLE2), 48);
	}

	#[test]
	fn part2_example1()
	{
		assert_eq!(part2(EXAMPLE1), 161);
	}
}
