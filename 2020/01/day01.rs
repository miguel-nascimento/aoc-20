// Only for Rust study, i use tsoding help D:
fn main () {
    let input = std::fs::read_to_string("input.txt")
        .unwrap();
    
    let mut xs = 
        input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let n = xs.len();
    better_try(&mut xs, n);
}

// O(n²)
fn first_try(xs: Vec<i32>, n: usize) {
    //    First part
    for i in 0..n - 1 {
        for j in i + 1..n {
            if xs[i] + xs[j] == 2020 {
                println!("{}", xs[i] * xs[j]);
                return;
            }
        }
    }

    // Second Part -> O(n³)
    // for i in 0..n - 1 {
    //     for j in i + 1..n {
    //         for k in j + 1..n {
    //             if xs[i] + xs[j] + xs[k] == 2020 {
    //                 println!("{}", xs[i] * xs[j] * xs[k]);
    //                 return;
    //             }
    //         }
    //     }
    // }
}
// O(N² log n)
fn better_try(xs: &mut Vec<i32>, n: usize) {
    xs.sort();
    // part 1
    for i in 0..n {
        if let Ok(j) = xs.binary_search(&(2020 - xs[i])) {
            if i != j {
                println!("{}", xs[i] * xs[j]);
                return;
            }
        }
    }

    // // part 2
    // for i in 0..n {
    //     for j in i + 1..n - 1 {
    //         if let Ok(k) = xs.binary_search(&(2020 - xs[i] - xs[j])) {
    //             if j != k {
    //                 println!("{}", xs[i] * xs[j] * xs[k]);
    //                 return;
    //             }
    //         }
    //     }
    // }
}