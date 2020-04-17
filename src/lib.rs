use lazy_static::lazy_static;

#[derive(Clone, Copy)]
pub struct Mapping {
    orig: [u8; 26],
    rev: [u8; 26],
}

impl Mapping {
    fn new(input: &str) -> Mapping {
        assert_eq!(input.len(), 26);
        let mut res = [0u8; 26];
        let mut rev = [0u8; 26];
        for (i, ch) in input.as_bytes().iter().enumerate() {
            assert!(b'A' <= *ch && *ch <= b'Z');
            res[i] = *ch - b'A';
            rev[(*ch - b'A') as usize] = i as u8;
        }
        Mapping { orig: res, rev }
    }

    fn get(&self, ch: u8) -> u8 {
        assert!(ch < 26);
        self.orig[ch as usize]
    }

    fn get_rev(&self, ch: u8) -> u8 {
        assert!(ch < 26);
        self.rev[ch as usize]
    }
}

#[derive(Clone, Copy)]
pub struct Rotor {
    mapping: Mapping,
    ring: u8,
}

impl Rotor {
    fn new(mapping: &str, ring: char) -> Rotor {
        assert!('A' <= ring && ring <= 'Z');
        Rotor {
            mapping: Mapping::new(mapping),
            ring: ring as u8 - b'A',
        }
    }
}

lazy_static! {
    pub static ref ROTOR_1: Rotor = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", 'Q');
    pub static ref ROTOR_2: Rotor = Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", 'E');
    pub static ref ROTOR_3: Rotor = Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", 'V');
    pub static ref ROTOR_4: Rotor = Rotor::new("ESOVPZJAYQUIRHXLNFTGKDCMWB", 'J');
    pub static ref ROTOR_5: Rotor = Rotor::new("VZBRGITYUPSDNHLXAWMJQOFECK", 'Z');
    pub static ref ROTORS: [Rotor; 5] = [*ROTOR_1, *ROTOR_2, *ROTOR_3, *ROTOR_4, *ROTOR_5];
    pub static ref REFLECTOR: Mapping = Mapping::new("YRUHQSLDPXNGOKMIEBFZCWVJAT");
    pub static ref IDENTITY: Mapping = Mapping::new("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
}

pub struct Enigma {
    rotors: [Rotor; 3],
    positions: [u8; 3],
    reflector: Mapping,
    wiring: Mapping,
}

impl Enigma {
    pub fn new(
        rotors: [usize; 3],
        positions: [u8; 3],
        reflector: Mapping,
        wiring: Mapping,
    ) -> Enigma {
        let mut r = [*ROTOR_1; 3];
        for i in 0..3 {
            r[i] = ROTORS[rotors[i] - 1];
            assert!(positions[i] < 26);
        }
        Enigma {
            rotors: r,
            positions,
            reflector,
            wiring,
        }
    }

    pub fn encrypt(&self, input: &str) -> String {
        let mut s = String::new();
        let mut pos = self.positions;
        for ch in input.as_bytes() {
            assert!(b'A' <= *ch && *ch <= b'Z');
            // shift position
            if pos[1] == self.rotors[1].ring {
                pos[0] = (pos[0] + 1) % 26;
            }
            if pos[1] == self.rotors[1].ring || pos[2] == self.rotors[2].ring {
                pos[1] = (pos[1] + 1) % 26;
            }
            pos[2] = (pos[2] + 1) % 26;

            let index = ch - b'A';
            // wiring board
            let index1 = self.wiring.get(index);
            // offset
            let index2 = (index1 + pos[2]) % 26;
            // rotor3
            let index3 = self.rotors[2].mapping.get(index2);
            // rotor3 -> rotor2
            let index4 = (index3 + 26 - pos[2] + pos[1]) % 26;
            // rotor2
            let index5 = self.rotors[1].mapping.get(index4);
            // rotor2 -> rotor1
            let index6 = (index5 + 26 - pos[1] + pos[0]) % 26;
            // rotor1
            let index7 = self.rotors[0].mapping.get(index6);
            // rotor1 -> reflector
            let index8 = (index7 + 26 - pos[0]) % 26;
            // reflector
            let index9 = self.reflector.get(index8);
            // reflector -> rotor1
            let index10 = (index9 + pos[0]) % 26;
            // rotor1
            let index11 = self.rotors[0].mapping.get_rev(index10);
            // rotor1 -> rotor2
            let index12 = (index11 + 26 - pos[0] + pos[1]) % 26;
            // rotor2
            let index13 = self.rotors[1].mapping.get_rev(index12);
            // rotor2 -> rotor3
            let index14 = (index13 + 26 - pos[1] + pos[2]) % 26;
            // rotor3
            let index15 = self.rotors[2].mapping.get_rev(index14);
            // offset
            let index16 = (index15 + 26 - pos[2]) % 26;
            // wiring board
            let res = self.wiring.get_rev(index16);
            s.push((res + b'A') as char);
        }

        s
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn aaaa() {
        let enigma = Enigma::new([1, 2, 3], [7, 3, 23], *REFLECTOR, *IDENTITY);
        assert_eq!(
            enigma.encrypt(
                "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
            ),
            "KTWREEOSTUVDOPLLBCOSVXVOLPBZSQKWENMRQHXJWRZDCYDJBZPZBYNRHFODHTINVGBEYBCSZOBTYZ"
        );
    }

    #[test]
    fn alphabet() {
        let enigma = Enigma::new([1, 2, 3], [7, 3, 23], *REFLECTOR, *IDENTITY);
        assert_eq!(
            enigma.encrypt(
                "ABCDEFGHIJKLMNOPQRSTUVWXYZQWERTYUIOPASDFGHJKLZXCVBNM"
            ),
            "KQGJALNNGTESJCXFSWEYLSNCIWYQQYYJBUCOQBTOHLYXVBUPPHMS"
        );
    }
}
