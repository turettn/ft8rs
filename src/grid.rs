use lazy_static::lazy_static;
use regex::Regex;

static MAX_GRID: u32 = 32400;
static NUM_OFFSET: u32 = 35;

static G15_ENCODE_ORDER: [(char, u32); 4] = [('A', 18), ('A', 10), ('0', 10), ('0', 1)];

pub fn grid_to_g15(grid: String) -> Result<u32, &'static str> {
    lazy_static! {
        static ref GRID4_RE: Regex = Regex::new(r"^[A-R][A-R][0-9][0-9]$").unwrap();
    }

    if grid.is_empty() {
        Ok(MAX_GRID + 1)
    } else if grid == "RRR" {
        Ok(MAX_GRID + 2)
    } else if grid == "RR73" {
        Ok(MAX_GRID + 3)
    } else if grid == "73" {
        Ok(MAX_GRID + 4)
    } else if grid.starts_with('+') {
        if let Ok(num) = grid.chars().skip(1).collect::<String>().parse::<u32>() {
            Ok(MAX_GRID + NUM_OFFSET + num)
        } else {
            Err("Invalid number after +")
        }
    } else if grid.starts_with('-') {
        if let Ok(num) = grid.chars().skip(1).collect::<String>().parse::<u32>() {
            Ok(MAX_GRID + NUM_OFFSET - num)
        } else {
            Err("Invalid number after -")
        }
    } else if GRID4_RE.is_match(&grid) {
        let mut grid_chars = grid.chars();
        let mut ret = 0u32;

        for (offset_char, factor) in G15_ENCODE_ORDER {
            ret += grid_chars.next().unwrap() as u32 - offset_char as u32;
            ret *= factor;
        }
        Ok(ret)
    } else {
        Err("grid does not conform to expected pattern")
    }
}

static G25_ENCODE_ORDER: [(char, u32); 6] = [
    ('A', 18),
    ('A', 10),
    ('0', 10),
    ('0', 24),
    ('A', 24),
    ('A', 1),
];

pub fn grid_to_g25(grid: String) -> Result<u32, &'static str> {
    lazy_static! {
        static ref GRID6_RE: Regex = Regex::new(r"^[A-R][A-R][0-9][0-9][A-X][A-X]$").unwrap();
    }

    if GRID6_RE.is_match(&grid) {
        let mut grid_chars = grid.chars();
        let mut ret = 0u32;
        for (offset_char, factor) in G25_ENCODE_ORDER {
            ret += grid_chars.next().unwrap() as u32 - offset_char as u32;
            ret *= factor;
        }
        Ok(ret)
    } else {
        Err("invalid grid")
    }
}

static G15_DECODE_ORDER: [(char, u32); 4] = [('0', 10), ('0', 10), ('A', 18), ('A', 18)];

pub fn g15_to_grid(mut grid: u32) -> Result<String, &'static str> {
    if grid == MAX_GRID + 1 {
        Ok("".to_string())
    } else if grid == MAX_GRID + 2 {
        Ok("RRR".to_string())
    } else if grid == MAX_GRID + 3 {
        Ok("RR73".to_string())
    } else if grid == MAX_GRID + 4 {
        Ok("73".to_string())
    } else if grid >= MAX_GRID + NUM_OFFSET {
        // positive int
        let num = grid - MAX_GRID - NUM_OFFSET;
        Ok("+".to_string() + &num.to_string())
    } else if grid >= MAX_GRID + 4 {
        let num = MAX_GRID + NUM_OFFSET - grid;
        Ok("-".to_string() + &num.to_string())
    } else {
        let mut ret = String::new();

        for (offset_char, div) in G15_DECODE_ORDER {
            if let Some(v) = char::from_u32(grid % div + offset_char as u32) {
                ret.push(v);
                grid /= div;
            } else {
                return Err("invalid character in grid");
            }
        }

        if grid != 0 {
            Err("grid value out of bounds")
        } else {
            Ok(ret.chars().rev().collect::<String>())
        }
    }
}

static G25_DECODE_ORDER: [(char, u32); 6] = [
    ('A', 24),
    ('A', 24),
    ('0', 10),
    ('0', 10),
    ('A', 18),
    ('A', 18),
];

pub fn g25_to_grid(mut grid: u32) -> Result<String, &'static str> {
    let mut ret = String::new();
    for (offset_char, div) in G25_DECODE_ORDER {
        if let Some(v) = char::from_u32(grid % div + offset_char as u32) {
            ret.push(v);
            grid /= div;
        } else {
            return Err("invalid character in grid");
        }
    }

    if grid != 0 {
        Err("grid value out of bounds")
    } else {
        Ok(ret.chars().rev().collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use super::g15_to_grid;
    use super::g25_to_grid;
    use super::grid_to_g15;
    use super::grid_to_g25;

    #[test]
    fn test_g15_grids() {
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

    #[test]
    fn test_g25_grids() {
        let value_list: Vec<(String, u32)> = vec![
            ("AB23EF".to_string(), 70949),
            ("AA00AA".to_string(), 0),
            ("RR99XX".to_string(), 18662399),
        ];

        for (cs_str, cs_int) in value_list {
            assert_eq!(grid_to_g25(cs_str.clone()), Ok(cs_int));
            assert_eq!(g25_to_grid(cs_int), Ok(cs_str));
        }
    }
}
