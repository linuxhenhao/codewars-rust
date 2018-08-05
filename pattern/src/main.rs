fn main() {
    println!("Hello, world!");
    for i in [1, 3, 5, 7, 11].into_iter() {
        println!("{}", zoom(*i));
    }
}

fn zoom(n: i32) -> String {
    let mut result = String::new();
    for y in 0..n {
        for x in 0..n {
            result.push_str(&get_block(n, x, y));
        }
        result.push('\n');
    }
    let len = result.len();
    result.split_off(len-1);
    result
}

fn get_block(n: i32, x: i32, y: i32) -> String {
    // n is the nth cycle of blocks,
    // this function returns true if x,y is 
    // on the nth cycle
    let center = (n-1)/2;
    let xdis = (x-center).abs();
    let ydis = (y-center).abs(); 
    let nth;
    if ydis > xdis {
        nth = ydis;
    }
    else {
        nth = xdis;
    }
    if nth%2 == 0 {
        return "■".to_string();
    } 
    else {
        return "□".to_string();
    }
}