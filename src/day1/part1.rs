use crate::day1::common::parse_final;

fn check2Sum(start: u64, values: Vec<u64>, sum: u64) -> Option<u64>{
    for val in values.clone() {
        if start + val == sum {
            return Some(start * val);
        }
    }
    return None;
}

pub fn exo() -> u64 {
    let values = parse_final();
    let mut vals = values.clone();

    loop {
        if vals.is_empty() {
            break;
        }
        let start = vals.pop().unwrap_or(u64::MAX);
        if start == u64::MAX {
            break;
        }
        match check2Sum(start, vals.clone(), 2020) {
            Some(res) => {
                return res;
            },
            _ => {}
        }
    }

    return u64::MAX;
}