use lazy_static::lazy_static;
use regex::Regex;

static MAX_GRID: u32 = 32400;
static NUM_OFFSET: u32 = 35;

pub fn grid_to_g15(grid: String) -> Result<u32, &'static str> {
    lazy_static! {
        static ref GRID4_RE: Regex = Regex::new(r"^[A-R][A-R][0-9][0-9]$").unwrap();
    }

    if grid.is_empty() {
        return Ok(MAX_GRID + 1);
    } else if grid == "RRR" {
        return Ok(MAX_GRID + 2);
    } else if grid == "RR73" {
        return Ok(MAX_GRID + 3);
    } else if grid == "73" {
        return Ok(MAX_GRID + 4);
    } else if grid.starts_with('+') {
        if let Ok(num) = grid.chars().skip(1).collect::<String>().parse::<u32>() {
            return Ok(MAX_GRID + NUM_OFFSET + num);
        } else {
            return Err("Invalid number after +");
        }
    } else if grid.starts_with('-') {
        if let Ok(num) = grid.chars().skip(1).collect::<String>().parse::<u32>() {
            return Ok(MAX_GRID + NUM_OFFSET - num);
        } else {
            return Err("Invalid number after -");
        }
    } else if GRID4_RE.is_match(grid.as_str()) {
        let mut grid_chars = grid.chars();
        let j1 = grid_chars.next().unwrap() as u32 - 'A' as u32;
        let j2 = grid_chars.next().unwrap() as u32 - 'A' as u32;
        let j3 = grid_chars.next().unwrap() as u32 - '0' as u32;
        let j4 = grid_chars.next().unwrap() as u32 - '0' as u32;
        return Ok(j1 * 18 * 10 * 10 + j2 * 10 * 10 + j3 * 10 + j4);
    }

    Err("grid does not conform to expected pattern")
}

static DECODE_ORDER: [(char, u32); 4] = [('0', 10), ('0', 10), ('A', 18), ('A', 18)];

pub fn g15_to_grid(mut grid: u32) -> Result<String, &'static str> {
    if grid == MAX_GRID + 1 {
        return Ok("".to_string());
    } else if grid == MAX_GRID + 2 {
        return Ok("RRR".to_string());
    } else if grid == MAX_GRID + 3 {
        return Ok("RR73".to_string());
    } else if grid == MAX_GRID + 4 {
        return Ok("73".to_string());
    } else if grid >= MAX_GRID + NUM_OFFSET {
        // positive int
        let num = grid - MAX_GRID - NUM_OFFSET;
        return Ok("+".to_string() + &num.to_string());
    } else if grid >= MAX_GRID + 4 {
        let num = MAX_GRID + NUM_OFFSET - grid;
        return Ok("-".to_string() + &num.to_string());
    }

    let mut ret = String::new();

    for (offset_char, div) in DECODE_ORDER {
        if let Some(v) = char::from_u32(grid % div + offset_char as u32) {
            ret.push(v);
            grid /= div;
        } else {
            return Err("invalid character in grid");
        }
    }

    Ok(ret.chars().rev().collect::<String>())
}

#[cfg(test)]
mod tests {
    use super::g15_to_grid;
    use super::grid_to_g15;

    #[test]
    fn test_grids() {
        let value_list: Vec<(String, u32)> = vec![
            ("RRR".to_string(), 32402),
            ("RR73".to_string(), 32403),
            ("".to_string(), 32401),
            ("73".to_string(), 32404),
            ("DM79".to_string(), 6679),
            ("+1".to_string(), 32436),
            ("+15".to_string(), 32450),
            ("-1".to_string(), 32434),
            ("-15".to_string(), 32420),
        ];

        for (cs_str, cs_int) in value_list {
            assert_eq!(grid_to_g15(cs_str.clone()), Ok(cs_int));
            assert_eq!(g15_to_grid(cs_int), Ok(cs_str));
        }
    }
}
