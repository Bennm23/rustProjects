use std::{collections::HashMap, env, fs::File, io::Read, process::exit, time::Instant};

pub mod utils;

const DEFAULT_LENGTH: usize = 15;
const DEFAULT_OPTIONS: usize = 5;

const CHAR_MIN: u32 = 33;  // !
const CHAR_MAX: u32 = 126; // ~

fn print_usage() {
    println!("--- Usage ---");
    println!(" -l|-len=%d   |  Set the length of each password to be generated");
    println!(" -o|-opts=%d  |  Set the number of passwords to be generated");
    exit(0);
}

fn parse_args(args: &Vec<String>) -> (usize, usize) {

    let mut length = DEFAULT_LENGTH;
    let mut options = DEFAULT_OPTIONS;

    for arg in args.iter().skip(1) {
        if arg == "help" || arg == "-h" || arg == "--help" {
            print_usage();
        } else if arg.starts_with("-l=") {
            let try_length: usize = arg.split_at(3).1.parse().expect("Invalid Opts Format");
            length = try_length;
        } else if arg.starts_with("-len=") {
            let try_length: usize = arg.split_at(5).1.parse().expect("Invalid Opts Format");
            length = try_length;
        } else if arg.starts_with("-o=") {
            let try_opts: usize = arg.split_at(3).1.parse().expect("Invalid Opts Format");
            options = try_opts;
        } else if arg.starts_with("-opts=") {
            let try_opts: usize = arg.split_at(6).1.parse().expect("Invalid Opts Format");
            options = try_opts;
        } else {
            println!("\n\x1b[31mUnrecognized Argument = {arg}\x1b[0m");
            print_usage();
        }
    }

    (length, options)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (length, options) = parse_args(&args);
    
    let mut rnd = File::open("/dev/random").expect("Failed to open /dev/random");
    let mut buf = vec![0u8; length];

    println!("Options");
    for i in 0 .. options {
        let res = rnd.read_exact(&mut buf);

        if let Some(e) = res.err() {
            println!("Random Read Err = {e}");
        } else {
            println!("    {}. {}", i+1, gen_password_from_bytes(&buf));
        }
    }
}

#[allow(unused)]
fn benchmark() {
    let start = Instant::now();
    let mut rnd = File::open("/dev/random").expect("Failed to open /dev/random");

    let mut buf: [u8; DEFAULT_LENGTH] = [0; DEFAULT_LENGTH];

    let mut generated: Vec<String> = Vec::new();

    let mut str;
    for _ in 0 .. 1000000 {
        let res = rnd.read_exact(&mut buf);

        if let Some(e) = res.err() {
            println!("Random Read Err = {e}");
        } else {
            // println!("Bytes Read = {:?}", buf);
            str = gen_password_from_bytes(&buf);
            // println!("Str Val = {}", gen_password_from_bytes(&buf));
            generated.push(str);
        }
    }

    println!("Generation Took {} us", start.elapsed().as_micros());
    println!("------------");

    let char_cntr: usize = generated.iter()
        .map(|s| -> usize { s.len()})
        .sum();

    let mut occurances: HashMap<char, usize> = HashMap::new();

    for opt in CHAR_MIN..=CHAR_MAX  {
        occurances.insert(char::from_u32(opt).expect("E"), 0);
    }

    for str in generated {
        for c in str.chars() {
            occurances.entry(c).and_modify(|cntr| {*cntr += 1;}).or_insert(1);
        }
    }

    for (c, count) in occurances {
        if count == 0 {
            println!("{c} = 0%")
        } else {
            println!("{c} = {}%", (count as f32 / char_cntr as f32) * 100.0);
        }
    }
    println!("Total Char Cnt = {char_cntr}");
}

fn gen_password_from_bytes(buf : &[u8]) -> String {
    let mut passwd = String::new();
    
    let mut bounded_val: u32;
    for b in buf {

        bounded_val = ((*b as u32) % (CHAR_MAX - CHAR_MIN + 1)) + CHAR_MIN;
        passwd.push(char::from_u32(bounded_val).expect("Char Gen Failed"));
    }
    passwd
}