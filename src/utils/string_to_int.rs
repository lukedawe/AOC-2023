const RADIX: u32 = 10;

pub fn string_to_int(string: &String) -> u32 {
    let mut index = u32::try_from(string.len()).unwrap();
    let mut sum = 0;
    for char in string.chars().into_iter() {
        sum += char.to_digit(RADIX).unwrap() * 10_u32.pow(index - 1);
        assert!(index > 0);
        index -= 1;
    }
    return sum;
}

pub fn string_to_int_u64(string: &String) -> u64 {
    let mut index = u32::try_from(string.len()).unwrap();
    let mut sum: u64 = 0;
    for char in string.chars().into_iter() {
        sum += u64::from(char.to_digit(RADIX).unwrap() * 10_u32.pow(index - 1));
        assert!(index > 0);
        index -= 1;
    }
    return sum;
}

pub fn is_int(c: &char) -> bool {
    if let Some(_) = c.to_digit(RADIX) {
        return true;
    }
    return false;
}
