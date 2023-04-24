// Character sets
static A1: &str = " 0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
static A2: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
static A3: &str = "0123456789";
static A4: &str = " ABCDEFGHIJKLMNOPQRSTUVWXYZ";

// Sets in order for easy application
static ENCODE_ORDER: [(&str, bool); 6] = [
    (A1, false),
    (A2, true),
    (A3, true),
    (A4, true),
    (A4, true),
    (A4, true),
];
static DECODE_ORDER: [&str; 6] = [A4, A4, A4, A3, A2, A1];

// Weird constants
static NTOKENS: usize = 2063592;
static MAX22: usize = 4194304;

pub fn callsign_to_int(callsign: String) -> Result<u32, &'static str> {
    // std_call_to_c28 does a right-pad to length 6
    // this does not match what ft8code does
    // this behavior was inferred from experimenting with ft8code as a black-box
    // TODO: Actually validate this
    let padded_callsign = match callsign.len() {
        3 => format!(" {callsign}  "),
        4 => format!(" {callsign} "),
        5 => format!(" {callsign}"),
        6 => callsign,
        _ => return Err("Invalid callsign length"),
    };

    let mut cs = padded_callsign.chars();
    let mut ret: usize = 0;

    for (encoder_chars, multiply) in ENCODE_ORDER {
        if multiply {
            ret *= encoder_chars.len()
        }

        if let Some(v) = cs.next() {
            if let Some(p) = encoder_chars.chars().position(|x| x == v) {
                ret += p;
            } else {
                return Err("invalid character");
            }
        }
    }

    ret += NTOKENS + MAX22;

    TryInto::<u32>::try_into(ret).map_err(|_| "value out of bounds")
}

pub fn int_to_callsign(callsign: u32) -> Result<String, &'static str> {
    let mut cs: usize = callsign as usize;
    let mut reversed = String::new();

    if cs < NTOKENS + MAX22 {
        return Err("callsign value is too small");
    }

    cs -= NTOKENS + MAX22;

    for elem in DECODE_ORDER {
        reversed.push(elem.chars().nth(cs % elem.len()).unwrap());
        cs /= elem.len();
    }

    if cs != 0 {
        // leftovers are present => value was not fully consumed
        return Err("callsign value is too large");
    }

    Ok(reversed.trim().chars().rev().collect::<String>())
}

#[cfg(test)]
mod tests {
    use super::callsign_to_int;
    use super::int_to_callsign;

    #[test]
    fn test_callsigns() {
        let value_list: Vec<(String, u32)> = vec![
            ("N3ALT".to_string(), 10845108),
            ("KC3ABC".to_string(), 157483171),
            ("W0TZ".to_string(), 12571738), // std_call_to_c28 emits 6041220, ft8code emits 12571738
            ("S2S".to_string(), 11822353),  // std_call_to_c28 emits 6055226, ft8code emits 11822353
        ];

        for (cs_str, cs_int) in value_list {
            assert_eq!(int_to_callsign(cs_int), Ok(cs_str.clone()));
            assert_eq!(callsign_to_int(cs_str), Ok(cs_int));
        }
    }
}
