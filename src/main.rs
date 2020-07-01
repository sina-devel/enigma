use std::fs::File;
use std::io::prelude::*;
use std::process::exit;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz ";

struct Enigma {
    state: u64,
    rotors: (String, String, String),
}

impl Enigma {
    fn new(rotors: (String, String, String)) -> Self {
        Enigma {
            state: 0,
            rotors: rotors,
        }
    }

    fn reflector(&self, c: char) -> char {
        ALPHABET
            .chars()
            .nth(ALPHABET.len() - 1 - ALPHABET.find(c).unwrap())
            .unwrap()
    }

    fn enigma_on_char(&mut self, c: char) -> char {
        self.state += 1;
        let c1 = self
            .rotors
            .0
            .chars()
            .nth(ALPHABET.find(c).unwrap())
            .unwrap();
        let c2 = self
            .rotors
            .1
            .chars()
            .nth(ALPHABET.find(c1).unwrap())
            .unwrap();
        let c3 = self
            .rotors
            .2
            .chars()
            .nth(ALPHABET.find(c2).unwrap())
            .unwrap();
        let reflected = self.reflector(c3);
        let c3 = ALPHABET
            .chars()
            .nth(self.rotors.2.find(reflected).unwrap())
            .unwrap();
        let c2 = ALPHABET
            .chars()
            .nth(self.rotors.2.find(c3).unwrap())
            .unwrap();
        let c1 = ALPHABET
            .chars()
            .nth(self.rotors.2.find(c2).unwrap())
            .unwrap();
        c1
    }

    fn rotate_rotors(&mut self) {
        self.rotors.0 = format!(
            "{}{}",
            self.rotors.0.get(1..).unwrap(),
            self.rotors.0.chars().nth(0).unwrap()
        );

        if self.state % 27 == 0 {
            self.rotors.1 = format!(
                "{}{}",
                self.rotors.1.get(1..).unwrap(),
                self.rotors.1.chars().nth(0).unwrap()
            );
        }

        if self.state % (27 * 27) == 0 {
            self.rotors.2 = format!(
                "{}{}",
                self.rotors.2.get(1..).unwrap(),
                self.rotors.2.chars().nth(0).unwrap()
            );
        }
    }

    fn code_decode(&mut self, plain: &str) -> String {
        let mut cipher = String::new();
        for c in plain.chars() {
            self.state += 1;
            cipher.push(self.enigma_on_char(c));
            self.rotate_rotors();
        }
        cipher
    }
}

fn read_rotors() -> (String, String, String) {
    let file = File::open("today_rotors.enigma");
    let mut file = match file {
        Ok(file) => file,
        Err(err) => {
            eprintln!("error: {}", err);
            exit(1);
        }
    };
    let mut raw = String::new();
    file.read_to_string(&mut raw).unwrap();
    let rotors: Vec<&str> = raw.split("\n").collect();
    let r1 = rotors[0].to_string();
    let r2 = rotors[1].to_string();
    let r3 = rotors[2].to_string();
    (r1, r2, r3)
}

fn main() {
    let mut enigma = Enigma::new(read_rotors());
    println!("hello world");
    println!("{}", enigma.code_decode("hello world"));
}
