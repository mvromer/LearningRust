extern crate rand;

use rand::{thread_rng, Rng};
use rand::distributions::{Distribution, Uniform};
use std::collections::HashMap;

fn main() {
    let mut rng = thread_rng();
    let mut samples: Vec<i32> = Uniform::new_inclusive( 1, 9 )
        .sample_iter( &mut rng )
        .take( 10 )
        .collect();

    let mut sorted_samples = samples.clone();
    sorted_samples.sort();
    println!( "Samples: {:?}", samples );
    println!( "Sorted samples: {:?}", sorted_samples );

    let mean = compute_mean( &samples );
    println!( "Mean: {}", mean );

    let (modes, counts) = compute_mode( &samples );
    println!( "Modes: {:?}", modes );
    println!( "Counts: {:?}", counts );

    let median = compute_median( &mut samples );
    println!( "Median: {}", median );
}

fn compute_mean( samples: &[i32] ) -> f64 {
    // The iter method returns references.
    //
    // The closure in the map method uses a reference pattern for its closure parameter. this lets us use the value of
    // the variable pointed to by the reference inside the body of the closure.
    //
    // The sum method needs the turbofish operator to supply a type annotation to the compiler, which tells it the
    // implementation of Sum<f64> to use.
    //
    // Rust makes us be explicit about arithmetic with mixed numeric types. specifically, we need to provide explicit
    // type conversions to let the compiler know which conversions we want.
    //
    samples.iter().map( | &sample | sample as f64 ).sum::<f64>() / (samples.len() as f64)
}

fn compute_mode( samples: &[i32] ) -> (Vec<i32>, HashMap<i32, i32>) {
    let mut counts = HashMap::new();

    for sample in samples {
        let count = counts.entry( *sample ).or_insert( 0 );
        *count += 1;
    }

    let max_count = *counts.values().max().unwrap();
    let modes = counts.iter().filter_map( | (&sample, &count) | {
        if count == max_count {
            Some( sample )
        }
        else {
            None
        }
    } ).collect::<Vec<_>>();

    (modes, counts)
}

fn compute_median( samples: &mut [i32] ) -> f64 {
    // Rather than handling even and odd length cases separately, we can treat both as the average
    // of two values in the list of samples. Pulled from section on Wikipedia page on median,
    // adapted for working with integer arithmetic.
    let number_samples = samples.len();
    let k_mid1 = (number_samples - 1) / 2 + 1;
    let k_mid2 = number_samples / 2 + 1;
    let value1 = find_kth_largest( samples, k_mid1 ) as f64;
    let value2 = find_kth_largest( samples, k_mid2 ) as f64;
    (value1 + value2) / 2.0
}

// Based on pseudocode on Quickselect article in Wikipedia.
fn find_kth_largest( samples: &mut [i32], k: usize ) -> i32 {
    // Translate k into a valid index. Initialize left and right indices to total range of samples.
    let k_index = k - 1;
    let mut left = 0;
    let mut right = samples.len() - 1;
    let mut rng = thread_rng();

    loop {
        if left == right {
            return samples[left];
        }

        let pivot_index = rng.gen_range( left, right + 1 );
        let pivot_index = quickselect_partition( samples, left, right, pivot_index );

        if k_index == pivot_index {
            return samples[k_index];
        }
        else if k_index < pivot_index {
            right = pivot_index - 1;
        }
        else {
            left = pivot_index + 1;
        }
    }
}

fn quickselect_partition( samples: &mut [i32], left: usize, right: usize, pivot_index: usize ) -> usize {
    let pivot_value = samples[pivot_index];
    samples.swap( pivot_index, right );
    let mut store_index = left;

    // For quickselect partitioning, we want to go from left to (right - 1) inclusive.
    for i in left..right {
        if samples[i] < pivot_value {
            samples.swap( store_index, i );
            store_index += 1;
        }
    }

    samples.swap( right, store_index );
    store_index
}
