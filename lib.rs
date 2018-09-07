// Bryan Zhao
// ECS 40, Fall Quarter '17

use std::{ops, fmt};

// You will define a `Matrix` type, overload the `+`, `-`, and `*` operators, and 
// implement the `Display` trait on it by providing the following API:

#[derive(PartialEq, Debug)]
pub struct Matrix<T> {
    /// Stores elements in [row-major order](https://en.wikipedia.org/wiki/Row-major_order)
    data: Vec<T>,
    /// Number of rows
    row: usize,
    /// Number of columns
    col: usize,
}

impl<T: Copy + std::fmt::Debug> Matrix<T> { // implement methods for type Matrix (bounded by copy trait)

    /// Creates a new matrix of `row` rows and `col` columns, and initializes
    /// the matrix with the elements in `values` in row-major order.
    pub fn new(row: usize, col: usize, values: &[T]) -> Matrix<T> {
        let mut out_mat: Matrix<T> = Matrix { data: Vec::new(), row: 0, col: 0 }; // initialize
        out_mat.row = row; 
        out_mat.col = col;
        out_mat.data.extend(values.iter().cloned()); // copy "values" into data vector
        out_mat // output
    }

    /// Creates a new, empty matrix of `row` rows and `col` columns.
    /// `data` contains no element.
    pub fn new_empty(row: usize, col: usize) -> Matrix<T> {
        let mut empty_mat: Matrix<T> = Matrix { data: Vec::new(), row: 0, col: 0}; // initialize
        empty_mat.row = row;
        empty_mat.col = col;
        empty_mat // output
    }

    /// Returns a shared reference to `data`
    pub fn data(&self) -> &Vec<T> {
        &self.data // output 
    }

    /// Returns a mutable reference to `data`
    pub fn mut_data(&mut self) -> &mut Vec<T> {
        &mut self.data // output
    }

    /// Returns the number of rows and columns in the first and second
    /// elements of the tuple, respectively.
    pub fn size(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}

impl<'a, T: ops::Add<Output = T> + Copy> ops::Add for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = Vec::new(); // initialize result

        // error handling for dimension mismatch
        if self.row != rhs. row || self.col != rhs.col {
            panic!();
        } else {
            // nothing
        }

      for (i, ele) in self.data.iter().enumerate() {
        sum.push(self.data[i] + rhs.data[i]);
      }
      Matrix { data: sum, row: self.row, col: self.col } // output
    }
}

impl<'a, T: ops::Add<Output = T> + Copy> ops::Add<Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row || self.col != rhs.col`, panic.

    fn add(self, rhs: Matrix<T>) -> Self::Output {
        // error handling for dimension mismatch
        if self.row != rhs. row || self.col != rhs.col {
            panic!();
        } else {
            // nothing
        }

        let mut sum = Vec::new(); // initialize
      for (i, ele) in self.data.iter().enumerate() {
        sum.push(self.data[i] + rhs.data[i]);
      }
      Matrix { data: sum, row: self.row, col: self.col } // output
    }
}

impl<T: ops::Add<Output = T> + Copy> ops::Add for Matrix<T> {
    type Output = Self;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn add(self, rhs: Self) -> Self::Output {
        // error handling for dimension mismatch
        if self.row != rhs. row || self.col != rhs.col {
            panic!();
        } else {
            // nothing
        }

        let mut sum = Vec::new(); // initialize
      for (i, ele) in self.data.iter().enumerate() {
        sum.push(self.data[i] + rhs.data[i]);
      }
      Matrix { data: sum, row: self.row, col: self.col } // output
    }
}

impl<'a, T: ops::Add<Output = T> + Copy> ops::Add<&'a Self> for Matrix<T> {
    type Output = Self;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn add(self, rhs: &Self) -> Self::Output {
        // error handling for dimension mismatch
        if self.row != rhs. row || self.col != rhs.col {
            panic!();
        } else {
            // nothing
        }

        let mut sum = Vec::new(); // initialize result
      for (i, ele) in self.data.iter().enumerate() {
        sum.push(self.data[i] + rhs.data[i]);
      }
      Matrix { data: sum, row: self.row, col: self.col } // output
    }
}

impl<'a, T: ops::Sub<Output = T> + Copy> ops::Sub for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn sub(self, rhs: Self) -> Self::Output {
        // error handling for dimension mismatch
        if self.row != rhs. row || self.col != rhs.col {
            panic!();
        } else {
            // nothing
        }

        let mut sum = Vec::new(); // initialize result
        for (i, ele) in self.data.iter().enumerate() {
            sum.push(self.data[i] - rhs.data[i]);
        }
        Matrix { data: sum, row: self.row, col: self.col } // output
    }    
}

impl<'a, T: ops::Sub<Output = T> + Copy> ops::Sub<Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn sub(self, rhs: Matrix<T>) -> Self::Output {
        // error handling for dimension mismatch
        if self.row != rhs. row || self.col != rhs.col {
            panic!();
        } else {
            // nothing
        }

        let mut sum = Vec::new(); // initialize result
        for (i, ele) in self.data.iter().enumerate() {
            sum.push(self.data[i] - rhs.data[i]);
        }
        Matrix { data: sum, row: self.row, col: self.col } // output
    }
}

impl<T: ops::Sub<Output = T> + Copy> ops::Sub for Matrix<T> {
    type Output = Self;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn sub(self, rhs: Self) -> Self::Output {
        // error handling for dimension mismatch
        if self.row != rhs. row || self.col != rhs.col {
            panic!();
        } else {
            // nothing
        }

        let mut sum = Vec::new(); // initialize result
        for (i, ele) in self.data.iter().enumerate() {
            sum.push(self.data[i] - rhs.data[i]);
        }
        Matrix { data: sum, row: self.row, col: self.col } // output
    }
}

impl<'a, T: ops::Sub<Output = T> + Copy> ops::Sub<&'a Self> for Matrix<T> {
    type Output = Self;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn sub(self, rhs: &Self) -> Self::Output {
        // error handling for dimension mismatch
        if self.row != rhs. row || self.col != rhs.col {
            panic!();
        } else {
            // nothing
        }

        let mut sum = Vec::new(); // initialize result
        for (i, ele) in self.data.iter().enumerate() {
            sum.push(self.data[i] - rhs.data[i]);
        }
        Matrix { data: sum, row: self.row, col: self.col } // output    
    }
}

impl<'a, T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> ops::Mul for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: Self) -> Self::Output {
        // error handling for dimension mismatch
        if self.col != rhs.row {
            panic!();
        } else {
            // nothing
        }

        let mut sum = Vec::new();
        let mut mul_out: Matrix<T> = Matrix { data: sum, row: 0, col: 0}; // initialize result
        mul_out.row = self.row; // output size takes the inner dimensions of self and rhs
        mul_out.col = rhs.col;
        let out_length = mul_out.row * mul_out.col; // len of vector with row-major format

        let mut result = self.data[0]*rhs.data[0]; // initialize to first value!

        for i in 0..self.row { // 
            for j in 0..rhs.col {
                result = self.data[(i * self.col)]*rhs.data[j];
                for k in 0..self.col { // self.col = rhs.row (N in MxN and NxP)
                	if k == 0 {
                		result = self.data[k + (i * self.col)] * rhs.data[j + (k * rhs.col)];
                	} else {
                    	result = result + self.data[k + (i * self.col)] * rhs.data[j + (k * rhs.col)];
                		}
                	}
                mul_out.data.push(result);
            }
        }
        mul_out // output
    }
}

impl<'a, T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> ops::Mul<Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        // error handling for dimension mismatch
        if self.col != rhs.row {
            panic!();
        } else {
            // nothing
        }

        let mut sum = Vec::new();
        let mut mul_out: Matrix<T> = Matrix { data: sum, row: 0, col: 0}; // initialize result
        mul_out.row = self.row; // output size takes the inner dimensions of self and rhs
        mul_out.col = rhs.col;
        let out_length = mul_out.row * mul_out.col; // len of vector with row-major format

        let mut result = self.data[0]*rhs.data[0]; // initialize to first value!

        for i in 0..self.row { // 
            for j in 0..rhs.col {
                result = self.data[(i * self.col)]*rhs.data[j];
                for k in 0..self.col { // self.col = rhs.row (N in MxN and NxP)
                	if k == 0 {
                		result = self.data[k + (i * self.col)] * rhs.data[j + (k * rhs.col)];
                	} else {
                    	result = result + self.data[k + (i * self.col)] * rhs.data[j + (k * rhs.col)];
                		}
                	}
                mul_out.data.push(result);
            }
        }
        mul_out // output
    }
}

impl<T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> ops::Mul for Matrix<T> {
    type Output = Self;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: Self) -> Self::Output {
        // error handling for dimension mismatch
        if self.col != rhs.row {
            panic!();
        } else {
            // nothing
        }

        let mut sum = Vec::new();
        let mut mul_out: Matrix<T> = Matrix { data: sum, row: 0, col: 0}; // initialize result
        mul_out.row = self.row; // output size takes the inner dimensions of self and rhs
        mul_out.col = rhs.col;
        let out_length = mul_out.row * mul_out.col; // len of vector with row-major format

        let mut result = self.data[0]*rhs.data[0]; // initialize to first value!

        for i in 0..self.row { // 
            for j in 0..rhs.col {
                result = self.data[(i * self.col)]*rhs.data[j];
                for k in 0..self.col { // self.col = rhs.row (N in MxN and NxP)
                	if k == 0 {
                		result = self.data[k + (i * self.col)] * rhs.data[j + (k * rhs.col)];
                	} else {
                    	result = result + self.data[k + (i * self.col)] * rhs.data[j + (k * rhs.col)];
                		}
                }
                mul_out.data.push(result);
            }
        }
        mul_out // output
    }
}

impl<'a, T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> ops::Mul<&'a Self> for Matrix<T> {
    type Output = Self;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: &Self) -> Self::Output {
        // error handling for dimension mismatch
        if self.col != rhs.row {
            panic!();
        } else {
            // nothing
        }

        let mut sum = Vec::new();
        let mut mul_out: Matrix<T> = Matrix { data: sum, row: 0, col: 0}; // initialize result
        mul_out.row = self.row; // output size takes the inner dimensions of self and rhs
        mul_out.col = rhs.col;
        let out_length = mul_out.row * mul_out.col; // len of vector with row-major format

        let mut result = self.data[0]*rhs.data[0]; // initialize to first value!

        for i in 0..self.row { // 
            for j in 0..rhs.col {
                result = self.data[(i * self.col)]*rhs.data[j];
                for k in 0..self.col { // self.col = rhs.row (N in MxN and NxP)
                	if k == 0 {
                		result = self.data[k + (i * self.col)] * rhs.data[j + (k * rhs.col)];
                	} else {
                    	result = result + self.data[k + (i * self.col)] * rhs.data[j + (k * rhs.col)];
                		}
                }
                mul_out.data.push(result);
            }
        }
        mul_out // output
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    /// Formats the matrix as follows:
    /// * Writes each row on a separate line. No empty lines before or after any row.
    /// * On each row, writes each element followed by a single space, except no space following the last element of the row.
    /// Outputs using `write!(f, ...)`.

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut i: usize; // row iterator
        let mut j: usize; // col iterator

        for i in 0..self.row {
            for j in 0..self.col {
                if j != (self.col-1) {
                    write!(f, "{} ", self.data[(i*self.col) + j]);
                } else {
                    write!(f, "{}\n", self.data[(i*self.col) + j]);
                }
            }
        } // has to return type Result
        Ok(())
    }
}

// test cases
#[test]
fn test() {
    let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
    let y = Matrix::new(2, 3, &[1, 2, 3, 4, 5, 6]);
    let left = Matrix::new(2, 3, &[-2, 1, 0, 1, 2, 3]);
    let right = Matrix::new(3, 2, &[1, 2, 0, 1, 3, 2]);

    assert_eq!(Matrix::new(2, 2, &[-2, -3, 10, 10]), left * right);
    println!("{:?}", &x + &y); 
    println!("{:?}", &x - &y);
    assert_eq!(format!("{}", x), "-2 -1 0\n1 2 3\n");
}


