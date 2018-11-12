extern crate rand;

use rand::thread_rng;
use rand::distributions::{Distribution, Uniform};

fn main() {
    let mut rng = thread_rng();
    let samples: Vec<i32> = Uniform::new_inclusive( 1, 9 )
        .sample_iter( &mut rng )
        .take( 20 )
        .collect();

    for sample in samples {
        print!( "{} ", sample );
    }

    println!();

    for sample in &samples {
        print!( "{} ", sample );
    }
}
