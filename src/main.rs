#![feature(test)]

use std::hint::black_box;
use std::time::SystemTime;

// RESULTS:
// if vec is reused 60+ times then FIRST METHOD is the most efficent
// if only used ~5 times then SECOND and THIRD are suprisingly similar
// Anywhere inbetween SECOND is the most efficent

fn main() {
    let mut inp = String::new();
    std::io::stdin().read_line(&mut inp).unwrap();
    let n = inp.trim().parse::<usize>().unwrap();

    // FIRST METHOD
    let t = SystemTime::now();
    let mut vec: Vec<usize> = vec![0; 10];
    for i1 in 0..n {
        for i in 0..10 {
            vec[i] = i1 + n;
            black_box(&vec);
        }
    }
    let took = t.elapsed().unwrap();
    println!("first took: {:?}", took);

    // SECOND METHOD
    let t = SystemTime::now();
    let mut vec: Vec<usize> = Vec::with_capacity(10);
    for i in 0..n {
        vec.clear();
        for _ in 0..10 {
            vec.push(i + n);
            black_box(&vec);
        }
    }
    let took = t.elapsed().unwrap();
    println!("second took: {:?}", took);

    // THIRD METHOD
    let t = SystemTime::now();
    for i in 0..n {
        let mut vec: Vec<usize> = Vec::with_capacity(10);
        for _ in 0..10 {
            vec.push(i + n);
            black_box(&vec);
        }
    }
    let took = t.elapsed().unwrap();
    println!("third took: {:?}", took);
}
