use std::fs;
use std::time::Instant;

fn main() {
    let (key1, key2) = read_keys("input");

    let timer = Instant::now();
    // Smaller keys lead to smaller loop-sizes. The order of
    // transformations does not matter.
    let result = if key1 > key2 {
        transform(7, key2, key1)
    } else {
        transform(7, key1, key2)
    };
    let time = timer.elapsed();

    println!("Result: {}", result);
    println!("Time: {:?}", time);
}

// Performs the transformation as described by the rules. Note: To get
// the loop-size of the first key we have to transform a value
// (initial = 1) by the given rules until the the value equal the key.
// Then we have to transform key2 loop-size-times. However since we 
// loop an equal number of times for both transformations we might as
// well perform both transformations in one loop.
fn transform(subj: u64, key1: u64, key2: u64) -> u64 {
    // We must perform one transformation outside the loop,
    // otherwise we end up transforming the second key one too
    // many times. The starting value 1 is here implied ->
    // (1 * subj) % 20201227 = subj % 20201227
    let mut val = subj % 20201227;
    let mut en_key = key2;

    while val != key1 {
        val = (val * subj) % 20201227;
        en_key = (key2 * en_key) % 20201227;
    }
    en_key
}

fn read_keys(filename: &str) -> (u64, u64) { 
    let buffer = fs::read_to_string(filename).unwrap();
    let mut parts = buffer.trim().split('\n');
    (
        parts.next().unwrap().parse().unwrap(),
        parts.next().unwrap().parse().unwrap()
    )
}
