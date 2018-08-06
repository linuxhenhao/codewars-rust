use std::collections::{HashMap, BTreeMap, BTreeSet};

fn main() {
    println!("Hello, world!");
}

fn mix(s1: &str, s2: &str) -> String {
    println!("{}", s1);
    let hash1 = get_hash(s1);
    let mut hash2 = get_hash(s2);

    let mut tree_map: BTreeMap<i32, Vec<(char, char)>> = BTreeMap::new();
    let mut s='_';
    let mut count=0;
    println!("{:?} {:?}", hash1, hash2);
    for (ch, count1) in hash1 {

        if hash2.contains_key(&ch) {
            let count2 = hash2[&ch];
            hash2.remove(&ch);
            match count1-count2 {
                0=> {s = '='; count=count1;},
                x if x > 0 => {s = '1'; count=count1;}
                x if x < 0 => {s = '2'; count=count2;}
                _ => ()
            }
        }
        else {
            s = '1';
            count = count1;
        }

        if tree_map.contains_key(&count) {
            tree_map.get_mut(&count).unwrap().push((ch, s));
        }
        else {
            let mut current_vec = Vec::new();
            current_vec.push((ch, s));
            tree_map.insert(count, current_vec);
        }
        println!("{:?}", tree_map);
    }

    for (ch, count) in hash2 {
        if tree_map.contains_key(&count) {
            tree_map.get_mut(&count).unwrap().push((ch, '2'));
        }
        else{
            let mut current_vec = Vec::new();
            current_vec.push((ch, '2'));
            tree_map.insert(count, current_vec);
        }
    }

    let mut result = String::new();
    for (&count, vec) in tree_map.range(..).rev() {
        if vec.len() == 1 {
            let (ch, head) = vec[0];
            let mut tmp_str = String::new();
            tmp_str.push(head);
            tmp_str.push(':');
            tmp_str.push_str(&ch.to_string().repeat(count as usize));
            result.push_str(&tmp_str);
            result.push('/');
        }
        else {
            let mut str_set = BTreeSet::new();
            for (ch, head) in vec {
                let mut tmp_str = String::new();
                tmp_str.push(*head);
                tmp_str.push(':');
                tmp_str.push_str(&ch.to_string().repeat(count as usize));
                str_set.insert(tmp_str);
            }

            for strng in str_set {
                result.push_str(&strng);
                result.push('/');
            }
        }
    }

    if result.is_empty() {
        return result;
    }
    let len = result.len();
    result.split_off(len-1);
    result
}

fn get_hash(s: &str) -> HashMap<char, i32> {
    let mut hash: HashMap<char, i32> = HashMap::new();
    for ch in s.chars() {
        if !ch.is_alphabetic() || ch.is_uppercase() {
            continue;
        }

        if hash.contains_key(&ch) {
            *hash.get_mut(&ch).unwrap() += 1;
        }
        else {
            hash.insert(ch, 1);
        }
    }
    let mut hash_ret: HashMap<char, i32> = HashMap::new();
    for (ch, count) in hash.iter()
        .filter(|&t| if *t.1 == 1 {false} else {true}) {
        hash_ret.insert(*ch, *count);
    }
    hash_ret
}

#[cfg(test)]
mod tests {
    use super::*;

fn testing(s1: &str, s2: &str, exp: &str) -> () {
    assert_eq!(&mix(s1, s2), exp)
}

#[test]
fn basics_mix() {

    testing("Are they here", "yes, they are here", 
        "2:eeeee/2:yy/=:hh/=:rr");
    testing("looping is fun but dangerous", "less dangerous than coding", 
        "1:ooo/1:uuu/2:sss/=:nnn/1:ii/2:aa/2:dd/2:ee/=:gg");
    testing(" In many languages", " there's a pair of functions", 
        "1:aaa/1:nnn/1:gg/2:ee/2:ff/2:ii/2:oo/2:rr/2:ss/2:tt");
    testing("Lords of the Fallen", "gamekult", "1:ee/1:ll/1:oo");
    testing("codewars", "codewars", "");
    testing("A generation must confront the looming ", "codewarrs", 
        "1:nnnnn/1:ooooo/1:tttt/1:eee/1:gg/1:ii/1:mm/=:rr");
    
}

}