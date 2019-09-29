#![feature(test)]
#![feature(const_vec_new)]

use std::hint::black_box;
use std::time::SystemTime;

static mut VEC: Vec<usize> = Vec::new();
static mut VEC2: Vec<usize> = Vec::new();

// RESULTS:
// if vec is reused 60+ times then FIRST METHOD is the most efficent
// if only used ~5 times then SECOND and THIRD are suprisingly similar
// Anywhere inbetween SECOND is the most efficent

fn main() {
    let mut inp = String::new();
    std::io::stdin().read_line(&mut inp).unwrap();
    let n = inp.trim().parse::<usize>().unwrap();

    unsafe {
        // FIRST METHOD
        let t = SystemTime::now();
        VEC2.reserve(10);
        for _ in 0..3 {
            VEC2.push(0);
        }
        for i1 in 0..n {
            for i in 0..3 {
                VEC2[i] = i1 + n;
                black_box(&VEC2);
            }
        }
        let took = t.elapsed().unwrap();
        println!("first(global) took: {:?}", took);
    }

    // FIRST METHOD
    let t = SystemTime::now();
    let mut vec: Vec<usize> = vec![0; 10];
    for i1 in 0..n {
        for i in 0..3 {
            vec[i] = i1 + n;
            black_box(&vec);
        }
    }
    let took = t.elapsed().unwrap();
    println!("first took: {:?}", took);

    unsafe {
        // SECOND METHOD
        let t = SystemTime::now();
        VEC.reserve(10);
        for i in 0..n {
            VEC.clear();
            for _ in 0..3 {
                VEC.push(i + n);
                black_box(&VEC);
            }
        }
        let took = t.elapsed().unwrap();
        println!("second(global) took: {:?}", took);
    }

    // SECOND METHOD
    let t = SystemTime::now();
    let mut vec = Vec::with_capacity(10);
    for i in 0..n {
        vec.clear();
        for _ in 0..3 {
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
        for _ in 0..3 {
            vec.push(i + n);
            black_box(&vec);
        }
    }
    let took = t.elapsed().unwrap();
    println!("third took: {:?}", took);

    // FOURTH METHOD
    let t = SystemTime::now();
    let mut vec: [usize; 10] = [0; 10];
    for i1 in 0..n {
        for i in 0..3 {
            vec[i] = i1 + n;
            black_box(&vec);
        }
    }
    let took = t.elapsed().unwrap();
    println!("fourth took: {:?}", took);
}
