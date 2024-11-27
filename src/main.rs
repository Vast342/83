use prng::{next_value, Generator};

mod prng;

const TARGET_NUM: u16 = 83;

fn main() {
    println!("here we go lets go find {}", TARGET_NUM);
    let seed = find_seed();
    println!("final seed: {}", seed);
    let mut rng = Generator::new(seed);
    let output = next_value(&mut rng);
    println!("first output from generator with seed: {}", output);
    if output == TARGET_NUM {
        println!("success!!");
    } else {
        println!("FAILURE");
    }
    println!("{}", output);
}

fn find_seed() -> u16 {
    let mut output = 0;
    let mut seed = 0;
    while output != TARGET_NUM {
        seed += 1;
        let mut rng = Generator::new(seed);
        output = next_value(&mut rng);
        println!("seed {} value {}", seed, output);
    }
    seed
}