use std::fs;
use std::num::ParseIntError;
use std::path::Path;

fn pow10(exp: u32) -> Option<u128> {
    //safe power-of-10 for u128, returns None on overflow
    let mut r: u128 = 1;
    for _ in 0..exp {
        r = r.checked_mul(10)?;
    }
    Some(r)
}

fn parse_u128(s: &str) -> Result<u128, ParseIntError> {
    s.trim().parse::<u128>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = fs::read_to_string(Path::new("input.txt"))?;
    let text = text.trim();

    let mut total: u128 = 0;

    for piece in text.split(',').map(|p| p.trim()).filter(|p| !p.is_empty()) {
        //parse range "lo-hi"
        let mut parts = piece.splitn(2, '-').map(|s| s.trim());
        let lo_s = parts.next().ok_or("bad range")?;
        let hi_s = parts.next().ok_or("bad range")?;
        let lo = parse_u128(lo_s)?;
        let hi = parse_u128(hi_s)?;
        if lo > hi { continue; }

        //loop over half length l (number of digits in the repeated block)
        //number = half * 10^l + half = half * (10^l + 1), and half must have no leading zero:
        //half in [10^(l-1) .. 10^l - 1] (for l=1 that's [1..9])
        for l in 1..=38u32 { // u128 can't hold 10^39, so 38 is safe upper bound
            let ten_l = match pow10(l) { Some(v) => v, None => break };
            let ten_l_minus1 = match pow10(l - 1) { Some(v) => v, None => break };

            //smallest possible doubled number for this l:
            //min_half = 10^(l-1)
            //min_num = min_half * (10^l + 1)
            let min_half = ten_l_minus1;
            let multiplier = ten_l.checked_add(1).ok_or("mult overflow")?;
            let min_num = match min_half.checked_mul(multiplier) {
                Some(v) => v,
                None => break,
            };
            if min_num > hi {
                //for larger l, numbers only get bigger -> stop
                break;
            }

            //compute allowed half range based on [lo,hi]:
            //half_low = ceil(lo / (10^l + 1))
            //half_high = floor(hi / (10^l + 1))
            let half_low_by_range = (lo + multiplier - 1) / multiplier; 
            let half_high_by_range = hi / multiplier;

            //clamp to [10^(l-1), 10^l - 1]
            let ten_l_minus_1 = ten_l_minus1;
            let ten_l_minus_1_upper = match ten_l.checked_sub(1) {
                Some(v) => v,
                None => 0,
            };
            let half_low = std::cmp::max(min_half, half_low_by_range);
            let half_high = std::cmp::min(ten_l_minus_1_upper, half_high_by_range);

            if half_low > half_high {
                continue;
            }
            
            for half in half_low..=half_high {
                
                let num = match half.checked_mul(multiplier) {
                    Some(v) => v,
                    None => continue,
                };
                
                if num >= lo && num <= hi {
                    total = total.checked_add(num).ok_or("sum overflow")?;
                }
            }
        }
    }

    println!("{}", total);
    Ok(())
}
