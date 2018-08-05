fn main() {
    println!("Hello, world!");
}

fn solequa(n: u64)->Vec<(u64, u64)> {
    let mut result: Vec<(u64, u64)> = vec![];
    let upper = (n as f64).sqrt().floor() as u64;
    let mut n1: u64;
    let mut n2: u64;
    for i in 1..upper+1 {
        if n%i == 0 {
            n1 = i;
            n2 = n/n1;
            if (n2 - n1)%4 == 0 {
                result.push(((n1+n2)/2, (n2-n1)/4));
            }
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
fn testing(n: u64, exp: Vec<(u64, u64)>) -> () {
    assert_eq!(solequa(n), exp)
}

#[test]
fn basics_solequa() {

    testing(5, vec![(3, 1)]);
    testing(20, vec![(6, 2)]); 
    testing(9001, vec![(4501, 2250)]);
    testing(9004, vec![(2252, 1125)]);
  
}
}
