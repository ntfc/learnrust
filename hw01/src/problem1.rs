/// Computes the sum of all elements in the input i32 slice named `slice`
pub fn sum(slice: &[i32]) -> i32 {
    let mut total = 0;
    for &i in slice {
        total += i;
    }
    total
}

/// Deduplicates items in the input vector `vs`. Produces a vector containing
/// the first instance of each distinct element of `vs`, preserving the
/// original order.
pub fn dedup(vs: &Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for orig in vs {
        // Vec.contains() expects to receive a reference, not the actual value
        if ! result.contains(orig) {
            result.push(*orig);
        }
    }
    result
}

/// Filters a vector `vs` using a predicate `pred` (a function from `i32` to
/// `bool`). Returns a new vector containing only elements that satisfy `pred`.
pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32> {
    let mut result = Vec::new();
    for orig in vs {
        if pred(*orig) {
            result.push(*orig);
        }
    }
    result
}
