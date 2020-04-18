use enigma::*;
use multimap::MultiMap;

fn find_chain(mapping: &[u8; 26]) -> Vec<usize> {
    let mut vis = [false; 26];
    let mut length = vec![];
    for j in 0..26 {
        if !vis[j] {
            let mut cur = j as usize;
            let mut len = 1;
            vis[cur] = true;
            // find loop
            while mapping[cur] as usize != j {
                cur = mapping[cur] as usize;
                vis[cur] = true;
                len += 1;
            }
            length.push(len);
        }
    }
    length.sort();
    length
}

fn main() {
    // enumerate 26 * 26 * 26 * 6 = 105456 keys
    // use identity mapping for wiring board
    let rotors = [
        [1, 2, 3],
        [1, 3, 2],
        [2, 1, 3],
        [2, 3, 1],
        [3, 1, 2],
        [3, 2, 1],
    ];
    // reverse lookup
    let mut reverse_map = MultiMap::new();
    for rotor in 0..6 {
        for offset1 in 0..26 {
            for offset2 in 0..26 {
                for offset3 in 0..26 {
                    let enigma = Enigma::new(rotors[rotor], [offset1, offset2, offset3], *IDENTITY);
                    // encrypt AAAAAA, BBBBBB until ZZZZZZ
                    let mut mapping = [[0u8; 26]; 3];
                    for i in 0..26 {
                        let ch = (i + b'A') as char;
                        let enc = enigma.encrypt(&format!("{}{}{}{}{}{}", ch, ch, ch, ch, ch, ch));
                        // mapping from enc[0] to enc[3], enc[1] to enc[4], enc[2] to enc[5]
                        for j in 0..3 {
                            mapping[j][(enc.as_bytes()[j] - b'A') as usize] =
                                enc.as_bytes()[j + 3] - b'A';
                        }
                    }

                    // find chain
                    let mut chain: Vec<Vec<usize>> = vec![vec![]; 3];
                    for i in 0..3 {
                        let length = find_chain(&mapping[i]);
                        chain[i] = length;
                    }

                    // insert into reverse map of enigma setting -> chain
                    let setting = (rotors[rotor], [offset1, offset2, offset3]);
                    reverse_map.insert(chain, setting);
                }
            }
        }
    }
    println!("keys: {}", reverse_map.keys().collect::<Vec<_>>().len());

    // PoC begin
    // suppose that some one used this enigma:
    // rotors: 1 2 3
    // position: 7 3 23
    // wiring board: exchange A <-> G, B <-> H, C <-> I, D <-> J, E <-> K, F <-> L
    let enigma = Enigma::new(
        [1, 2, 3],
        [7, 3, 23],
        Mapping::new("GHIJKLABCDEFMNOPQRSTUVWXYZ"),
    );
    // and we capture enough encrypt(ABCABC)
    // encrypt AAAAAA, BBBBBB until ZZZZZZ
    let mut known_ciphertext = vec![];
    let mut mapping = [[0u8; 26]; 3];
    for i in 0..26 {
        let ch = (i + b'A') as char;
        let enc = enigma.encrypt(&format!("{}{}{}{}{}{}", ch, ch, ch, ch, ch, ch));
        // mapping from enc[0] to enc[3], enc[1] to enc[4], enc[2] to enc[5]
        for j in 0..3 {
            mapping[j][(enc.as_bytes()[j] - b'A') as usize] = enc.as_bytes()[j + 3] - b'A';
        }
        known_ciphertext.push(enc);
    }

    // find chain
    let mut chain: Vec<Vec<usize>> = vec![vec![]; 3];
    for i in 0..3 {
        let length = find_chain(&mapping[i]);
        chain[i] = length;
    }

    // get possible settings from reverse_map
    let settings = reverse_map.get_vec(&chain);
    if let Some(settings) = settings {
        println!("possible: {:?}", settings);
        for (rotor, pos) in settings.iter() {
            println!("for setting {:?} {:?}:", rotor, pos);
            let enigma = Enigma::new(*rotor, *pos, *IDENTITY);
            let mut score = 0;
            for cipher in &known_ciphertext {
                let dec = enigma.decrypt(cipher);
                for i in 0..3 {
                    if dec.as_bytes()[i] == dec.as_bytes()[i+3] {
                        score += 1;
                    }
                }
                println!("{} -> {}", cipher, dec);
            }
            println!("score: {}", score);
        }
    }
}
