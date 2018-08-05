fn main() {
    dna_to_rna("ATTD");
}

fn dna_to_rna(dna: &str) -> String {
     let bytes: Vec<u8> = dna.as_bytes().iter()
        .map( |ch| { 
            let ch_new;
            if *ch == b'T' {
                ch_new = b'U';
            } else {
                ch_new = *ch;
            }
            ch_new
        }).collect();
     String::from_utf8(bytes).unwrap()
}

