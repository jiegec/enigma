use lazy_static::lazy_static;

struct Rotor {
    mapping: [u8; 26],
    ring: u8,
}

type Reflector = [u8; 26];

impl Rotor {
    fn new(mapping: &str, ring: char) -> Rotor {
        assert_eq!(mapping.len(), 26);
        let mut res = [0u8; 26];
        for (i, ch) in mapping.as_bytes().iter().enumerate() {
            assert!(b'A' <= *ch && *ch <= b'Z');
            res[i] = *ch - b'A';
        }
        assert!('A' <= ring && ring <= 'Z');
        Rotor {
            mapping: res,
            ring: ring as u8 - b'A'
        }
    }
}

lazy_static! {
    static ref ROTOR_1: Rotor = Rotor::new("EKMFLGDQVZNTOWYHXUSPAIBRCJ", 'Q');
    static ref ROTOR_2: Rotor = Rotor::new("AJDKSIRUXBLHWTMCQGZNPYFVOE", 'E');
    static ref ROTOR_3: Rotor = Rotor::new("BDFHJLCPRTXVZNYEIWGAKMUSQO", 'V');
    static ref ROTOR_4: Rotor = Rotor::new("ESOVPZJAYQUIRHXLNFTGKDCMWB", 'J');
    static ref ROTOR_5: Rotor = Rotor::new("VZBRGITYUPSDNHLXAWMJQOFECK", 'Z');
}

struct Enigma {

}