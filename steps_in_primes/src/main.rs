use std::collections::VecDeque;
use std::cell::RefCell;

fn main() {
    basics_step();
}

fn testing(g: i32, m: u64, n: u64, exp: Option<(u64, u64)>) -> () {
    assert_eq!(step(g, m, n), exp)
}

fn basics_step() {
    testing(2,100,110, Some((101, 103)));
    testing(4,100,110, Some((103, 107)));
    testing(8,30000,100000, Some((30089, 30097)));
    testing(11,30000,100000, None);
    testing(2,10000000,11000000, Some((10000139, 10000141)));
}

thread_local! {
    static PRIMES: RefCell<Vec<u64>> = RefCell::new(vec![3, 5, 7, 11, 13]);
}


struct Candidates {
    data: VecDeque<u64>,
    step: u64
}

impl Candidates {
    pub fn enqueue(&mut self, number: u64) -> Option<(u64, u64)> {
        if self.data.len() == 0 {
            self.data.push_back(number);
            return None
        } 

        let oldest = self.data[0];

        match number-oldest {
            x if x == self.step => return Some((oldest, number)),
            x if x < self.step => self.data.push_back(number),
            _ => {
                 self.data.pop_front();
                 self.enqueue(number);
            }
        }
        None
    }

    pub fn get_back(&self) -> Option<u64> {
        if let Some(number) = self.data.get(self.data.len()-1) {
            return Some(*number);
        }
        None
    }
}


fn step(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
    PRIMES.with( |cell| {
        let mut vector = cell.borrow_mut();
        step_with_primes(g, m, n, &mut *vector)
    })
}
fn step_with_primes(g: i32, m: u64, n: u64, mut primes: &mut Vec<u64>) -> Option<(u64, u64)> {
    for i in [3, 5, 7, 11, 13].iter() {
            primes.push(*i);
    }

    let mut candidates = Candidates {
        data: VecDeque::new(),
        step: g as u64
    };

    match n-m {
        // the distance between m, n is less than g
        x if x < g as u64=> { return None },
        x if x == g as u64=> { match is_prime(m, &mut primes) && is_prime(n, &mut primes) {
                true => return Some((n, n)),
                false => return None
            }

        },
        // n - m > g
        _ => {
            match find_prime(m, n+1, &mut primes) {
                Some(i) => candidates.enqueue(i),
                None => return None
            };
            loop {
                match find_prime(candidates.get_back().unwrap()+2, n+1, &mut primes) {
                    Some(i) => {
                        if let Some(tup) = candidates.enqueue(i) {
                            return Some(tup);
                        }
                        i
                    },
                    None => return None
                };
            }
        }
    }
}

/// find a prime from start to stop, include start, exclude stop
fn find_prime(start: u64, stop: u64, mut primes: &mut Vec<u64>) -> Option<u64> {
    let mut start = start;
    if start >= stop {
        return None;
    }

    if start % 2 == 0 {
        start += 1;
    }

    if is_prime(start, &mut primes) {
        return Some(start);
    } else {
        start += 2;
        while start < stop {
            if is_prime(start, &mut primes){
                return Some(start);
            }
            start += 2;
        }
        None
    }
}

/// is_prime, number is odd
fn is_prime(number: u64, mut primes: &mut Vec<u64>) -> bool {
    match number {
        number if number <= 2 => true,
        number if number %2 == 0 => false,
        _ => _is_prime(number, 0, &mut primes)
    }
}

fn _is_prime(number: u64, index: usize, mut primes: &mut Vec<u64>) -> bool {
    while index >= primes.len() {
        find_next_prime(&mut primes);
    }

    let current = primes[index];
    match number/current < current {
        // true means all primes less than number was tested, and number cannot
        // be divided by them
        true => { 
            return true;
            },
        false => {
            if number % current == 0 {
                return false;
            } else {
                return _is_prime(number, index+1, &mut primes);
            }
        }
    }
}


fn find_next_prime(mut primes: &mut Vec<u64>) {
    let len = primes.len();   
    let last = primes[len-1];
    let mut possible_next = last + 2;
    while !is_prime(possible_next, &mut primes) {
            possible_next += 2;
    }
    primes.push(possible_next);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_find_next_prime() {
        let mut primes = vec![3, 5, 7, 11];
        super::find_next_prime(&mut primes);
        assert_eq!(primes, vec![3, 5, 7, 11, 13]);
    }

    #[test]
    fn test_is_prime() {
        let mut primes = vec![3, 5, 7, 11, 13];
        for number in [14, 15, 16].iter() {
            assert_eq!(super::is_prime(*number, &mut primes), false);
        }
        assert_eq!(super::is_prime(17, &mut primes), true);
    }
    

}