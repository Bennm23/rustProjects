
pub fn hex_to_u32(str: &str) -> u32 {

    let mut total = 0;
    let mut ival;
    for (exp, c) in str.chars().rev().enumerate() {
        ival = c.to_digit(16).expect("Invalid Char");
        total += ival * (16u32).pow(exp as u32);
    }
    total
}