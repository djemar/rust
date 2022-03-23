/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut luhn = 0;
    let mut doubling;
    let cleaned_code = code.replace(" ", "").chars().rev().collect::<String>();
    let mut j = 1;
    if cleaned_code.chars().count() < 2 { return false; }

    for ch in cleaned_code.chars() {
        match ch {
            '0'..='9' => {
                if j%2 == 0 {
                    doubling = ch.to_digit(10).unwrap() * 2;
                    if doubling > 9 { doubling -= 9; }
                    luhn += doubling;
                }
                else { luhn += ch.to_digit(10).unwrap(); }
            }
            _ => return false,
        }
        j += 1;
    }
    luhn % 10 == 0
}
