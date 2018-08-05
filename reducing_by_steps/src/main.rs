use std::env::args;

fn main() {
    let args_vec: Vec<String> = args().collect();
    let a = args_vec[1].parse::<i32>().unwrap();
    let b = args_vec[2].parse::<i32>().unwrap();
    gcdi(a as i64, b as i64);
}
fn gcdi(m: i64, n: i64) -> i64 {
   if m == n {
       return m;
   } else {
       let (mut big, mut small)  = if m > n {(m, n) } else { (n, m) };
       loop {
           println!("{},{}", big, small);
           if big % small == 0 {
               return small;
           }
           else {
               let tmp = big - small;
               if tmp > small { big = tmp; } else { big = small; small = tmp;};
           }
       }
   }
}

fn testing_gcdi(a: &[i64], exp:  &Vec<i64>) -> () {
    assert_eq!(&oper_array(gcdi, a, a[0]), exp);
}

fn oper_array(f: fn(i64, i64)->i64, a: &[i64], init: i64) -> Vec<i64> {
    let mut result: Vec<i64> = vec![];
    result.push(f(init, a[0]));
    
    for (index, &value) in a.iter().skip(1).enumerate() {
        let temp = result[index];
        result.push(f(temp, value));
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics_gcdi() {
        testing_gcdi(&[ 18, 69, -90, -78, 65, 40 ], &vec![ 18, 3, 3, 3, 1, 1 ]);
    }
}
