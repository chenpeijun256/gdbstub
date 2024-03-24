
pub fn parse_hex_u32_err_to_0(n_str: &str) -> u32 {
    match u32::from_str_radix(n_str, 16) {
        Ok(steps) => steps,
        Err(_) => 0,
    }
}

pub fn parse_i32_err_to_min(n_str: &str) -> i32 {
    match i32::from_str_radix(n_str, 10) {
        Ok(steps) => steps,
        Err(_) => i32::MIN,
    }
}

pub fn split_string(line: String) -> Vec<String> {
    let mut res = Vec::new();

    line.trim().split_ascii_whitespace().for_each(|elem| {
        res.push(elem.to_owned());
    });
    return res;
}

pub fn hex_to_u8(hex: &str) -> u8 {
    match u8::from_str_radix(hex, 16) {
        Ok(ret) => ret,
        Err(_) => 0,
    }
}

pub fn hex_to_usize(hex: &str) -> usize {
    match usize::from_str_radix(hex, 16) {
        Ok(ret) => ret,
        Err(_) => 0,
    }
}

pub fn hex_to_u32(hex: &str) -> u32 {
    match u32::from_str_radix(hex, 16) {
        Ok(ret) => ret,
        Err(_) => 0,
    }
}

pub fn u8_to_hex(u: u8) -> String {
    format!("{:02x}", u)
}

pub fn u32_to_hex(u: u32) -> String {
    let u0 = u as u8;
    let u1 = u>>8 as u8;
    let u2 = u>>16 as u8;
    let u3 = u>>24 as u8;
    format!("{:02x}{:02x}{:02x}{:02x}", u0, u1, u2, u3)
}

pub fn str_add_sum(s: &str) -> u8 {
    let mut sum: u32 = 0;
    for c in s.bytes() {
        sum += c as u32;
    }
    // 限制校验码为8位（0-255）
    sum as u8
}
