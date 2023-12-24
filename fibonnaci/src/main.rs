const OCCURENCES: (i32, i32) = (0, 1);

fn main() {
    println!("{}", fibonnaci(6));
}

fn fibonnaci(n: i32) -> i32 {
    let (mut x, mut y) = OCCURENCES;

    for _ in 0..n {
        let temp = x;
        x = y;
        y = x + temp
    }

    return x;
}
