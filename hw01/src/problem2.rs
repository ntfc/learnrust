/// Represents a matrix in row-major order
pub type Matrix = Vec<Vec<f32>>;

pub fn vec_mult(a: &Vec<i32>, n: i32) -> i32 {
    let mut res = 0;
    for elem in a {
        res += n * (*elem);
    }
    res
}

/// Computes the product of the inputs `mat1` and `mat2`.
pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    // compare the number of cols in mat1 agains the numer of rows in mat2
    assert_eq!(mat1[0].len(), mat2.len());
    let n = mat1.len();
    let m = mat2.len();
    let p = mat2[0].len();

    // create the result matrix n rows and p columns
    let mut mult: Matrix = vec![vec![0.0; p]; n];
    for i in 0..n {
        for j in 0..p {
            for k in 0..m {
                mult[i][j] += mat1[i][k] * mat2[k][j];
            }
        }
    }
    
    mult
}
