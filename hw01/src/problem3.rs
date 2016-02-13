use std::f32;

/// Find all prime numbers less than `n`.
/// For example, `sieve(7)` should return `[2, 3, 5]`
pub fn sieve(n: u32) -> Vec<u32> {
    assert!(n > 1);
    let mut result: Vec<u32> = Vec::new();
    let mut crossed: Vec<u32> = Vec::new();
    
    let n_sqrt: u32 = f32::sqrt(n as f32) as u32;
    for i in 2..n_sqrt+1 {
        let i_square = i * i;
        for k in 0..n {
            let j = i_square + i * k;
            if j <= n {
                crossed.push(j);
            }
            else {
                break;
            }
        }
    }
    for i in 2..n {
        if ! crossed.contains(&i) {
            result.push(i);
        }
    }
    result
}
