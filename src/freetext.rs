static CHARSET: &str = " 0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ+-./?";

pub fn freetext_to_int(data: String) -> Result<u128, &'static str> {
    let mut ret = 0;
    for elem in data.chars() {
        if let Some(v) = CHARSET.chars().position(|x| x == elem) {
            ret = ret * 42 + v as u128;
        } else {
            return Err("Invalid character in input");
        }
    }
    Ok(ret)
}

pub fn int_to_freetext(mut data: u128) -> Result<String, &'static str> {
    let mut ret = String::new();
    for _ in 0..13 {
        ret.push(CHARSET.chars().nth((data % 42) as usize).unwrap());
        data /= 42;
    }
    if data != 0 {
        Err("left over data in f71")
    } else {
        Ok(ret.trim().chars().rev().collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use super::freetext_to_int;
    use super::int_to_freetext;

    #[test]
    fn test_freetext() {
        let value_list: Vec<(String, u128)> = vec![
            ("A".to_string(), 0b1011),
            ("AA".to_string(), 0b111011001),
            (
                "ABCDEFGHIJKLM".to_string(),
                0b00100100111001000010000000001011111011100010010110011110110001100110111,
            ),
            (
                "STUVWXYZ+-./?".to_string(),
                0b01100001000111111111010100111000010101000110010101001111010100101110101,
            ),
        ];

        for (cs_str, cs_int) in value_list {
            assert_eq!(freetext_to_int(cs_str.clone()), Ok(cs_int));
            assert_eq!(int_to_freetext(cs_int), Ok(cs_str));
        }
    }
}
