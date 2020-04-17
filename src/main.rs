use enigma::*;

fn main() {
    // H D X
    let enigma = Enigma::new([1, 2, 3], [7, 3, 23], *REFLECTOR, *IDENTITY);
    println!("{}", enigma.encrypt("AAA"));
}
