pub fn print_2d_vec(vec_2d: &Vec<Vec<char>>) {
    for vec in vec_2d {
        for char in vec {
            print!("{}", char);
        }
        print!("\n");
    }
}

pub fn print_vec(vec: &Vec<char>) {
    for char in vec {
        print!("{}", char);
    }
}