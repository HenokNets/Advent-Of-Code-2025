use std::fs;
use std::path::Path;

//check if n is a repeated sequence (atleast twice)
fn is_repeated(n: u128) -> bool {
    let s = n.to_string();
    let len = s.len();

    //try every possble block length
    for l in 1..=len / 2 {
        if len % l != 0 { continue; } 
        let block = &s[0..l];
        if block.starts_with('0') { continue; } 
        let times = len / l;
        let mut valid = true;
        for i in 1..times {
            if &s[i*l..(i+1)*l] != block {
                valid = false;
                break;
            }
        }
        if valid && times >= 2 {
            return true;
        }
    }
    false
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = fs::read_to_string(Path::new("input.txt"))?;
    let text = text.trim();

    let mut total: u128 = 0;

    for piece in text.split(',').map(|p| p.trim()).filter(|p| !p.is_empty()) {
        let mut parts = piece.splitn(2, '-').map(|s| s.trim());
        let lo: u128 = parts.next().ok_or("bad range")?.parse()?;
        let hi: u128 = parts.next().ok_or("bad range")?.parse()?;

        for n in lo..=hi {
            if is_repeated(n) {
                total = total.checked_add(n).ok_or("sum overflow")?;
            }
        }
    }

    println!("{}", total);
    Ok(())
}
