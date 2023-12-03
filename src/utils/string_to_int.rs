pub fn string_to_int(string: &String) -> u32 {
    const RADIX: u32 = 10;
    let mut index = u32::try_from(string.len()).unwrap();
    let mut sum = 0;
    for char in string.chars().into_iter() {
        sum += char.to_digit(RADIX).unwrap() * 10_u32.pow(index-1);
        assert!(index>0);
        index -= 1; 
    }
    return sum;
}
