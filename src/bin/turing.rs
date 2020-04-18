use enigma::*;
use petgraph::dot::Dot;
use petgraph::graph::Graph;
use petgraph::Undirected;

fn main() {
    // PoC
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

    // known plaintext
    // encrypt ABCDEFGHIJKLMNOPQRSTUVWXYZAAAAAA
    let plain_text = "ABCDEFGHIJKLMNOPQRSTUVWXYZAAAAAA";
    // to NNOJFLOTVHEPDIXLSWKYFSNICW
    let cipher_text = enigma.encrypt(plain_text);
    assert_eq!(cipher_text, "NNOJFLOTVHEPDIXLSWKYFSNICWCTFEBC");
    println!("{}", cipher_text);
    // construct undirected graph
    let mut graph = Graph::<char, usize, Undirected>::new_undirected();
    let mut nodes = vec![];
    for ch in b'A'..(b'Z' + 1) {
        nodes.push(graph.add_node(ch as char));
    }
    for i in 0..plain_text.len() {
        graph.add_edge(
            nodes[(plain_text.as_bytes()[i] - b'A') as usize],
            nodes[(cipher_text.as_bytes()[i] - b'A') as usize],
            i,
        );
    }
    let dot = Dot::with_config(&graph, &[]);
    std::fs::write("graph.dot", format!("{}", dot)).unwrap();

    // cycles starting from A:
    let cycles = [
        // A -> C -> A
        vec![26, 31],
        // A -> B -> N -> A
        vec![30, 1, 0],
        // A -> E -> F -> A
        vec![29, 4, 28],
        // A -> C -> Y -> T -> A
        vec![26, 24, 19, 27],
        // A -> N -> I -> X -> O -> C -> A
        vec![0, 13, 23, 14, 2, 26],
    ];

    // verify that the cycles are correct
    for cycle in &cycles {
        let mut cur = 'A';
        for offset in cycle {
            cur = enigma.encrypt_char(cur, *offset);
        }
        if cur != (b'A') as char {
            panic!("cycle {:?} is incorrect", cycle);
        }
        println!("cycle {:?} is correct", cycle);
    }

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

    let mut possible_settings = vec![];
    for rotor in 0..6 {
        for offset1 in 0..26 {
            for offset2 in 0..26 {
                for offset3 in 0..26 {
                    let enigma = Enigma::new(rotors[rotor], [offset1, offset2, offset3], *IDENTITY);
                    // test : S(A) = A, B, C, ... Z
                    for ch in 0..26 {
                        // verify each cycle
                        let mut fail = false;
                        for cycle in &cycles {
                            let mut cur = (ch + b'A') as char;
                            for offset in cycle {
                                cur = enigma.encrypt_char(cur, *offset);
                            }
                            if cur != (ch + b'A') as char {
                                fail = true;
                                break;
                            }
                        }
                        if !fail {
                            // this is a possible setting
                            possible_settings.push((rotors[rotor], [offset1, offset2, offset3]));
                        }
                    }
                }
            }
        }
    }

    println!("num of possible: {:?}", possible_settings.len());
    println!("possible: {:?}", possible_settings);
}
