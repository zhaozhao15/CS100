use std::{ops, fmt};

#[derive(PartialEq, Debug)]
pub struct Matrix<T> {
    /// Stores elements in [row-major order](https://en.wikipedia.org/wiki/Row-major_order)
    data: Vec<T>,
    /// Number of rows
    row: usize,
    /// Number of columns
    col: usize,
}

impl<T: Copy> Matrix<T> {
    /// Creates a new matrix of `row` rows and `col` columns, and initializes
    /// the matrix with the elements in `values` in row-major order.
    pub fn new(row: usize, col: usize, values: &[T]) -> Matrix<T> {
        let val = values.to_owned();
        Matrix{data: val, row: row, col :col}
    }

    /// Creates a new, empty matrix of `row` rows and `col` columns.
    /// `data` contains no element.
    pub fn new_empty(row: usize, col: usize) -> Matrix<T> {
        let val:Vec<T> = vec![];
        Matrix{data: val, row: row, col :col}
    }

    /// Returns a shared reference to `data`
    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    /// Returns a mutable reference to `data`
    pub fn mut_data(&mut self) -> &mut Vec<T> {
        &mut self.data
    }

    /// Returns the number of rows and columns in the first and second
    /// elements of the tuple, respectively.
    pub fn size(&self) -> (usize, usize) {
        (self.row,self.col)
    }
}

impl<'a, T: ops::Add<Output = T> + Copy> ops::Add for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.row,rhs.row);
        assert_eq!(self.col,rhs.col);
        let mut val:Vec<T> = vec![];
        for i in 0..self.data.len() {
            let tmp = self.data[i] + rhs.data[i];
            val.push(tmp);
        }
        let ans = Matrix{data: val, row: self.row, col :self.col};
        return ans;
    }
}

impl<'a, T: ops::Add<Output = T> + Copy> ops::Add<Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row`, panic.
    fn add(self, rhs: Matrix<T>) -> Self::Output {
        assert_eq!(self.row,rhs.row);
        assert_eq!(self.col,rhs.col);
        let mut val:Vec<T> = vec![];
        for i in 0..self.data.len() {
            let tmp = self.data[i] + rhs.data[i];
            val.push(tmp);
        }
        let ans = Matrix{data: val, row: self.row, col :self.col};
        return ans;
    }
}

impl<T: ops::Add<Output = T> + Copy> ops::Add for Matrix<T> {
    type Output = Self;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row`, panic.
    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.row,rhs.row);
        assert_eq!(self.col,rhs.col);
        let mut val:Vec<T> = vec![];
        for i in 0..self.data.len() {
            let tmp = self.data[i] + rhs.data[i];
            val.push(tmp);
        }
        let ans = Matrix{data: val, row: self.row, col :self.col};
        return ans;
    }
}

impl<'a, T: ops::Add<Output = T> + Copy> ops::Add<&'a Self> for Matrix<T> {
    type Output = Self;

    /// Returns the sum of `self` and `rhs`. If `self.row != rhs.row`, panic.
    fn add(self, rhs: &Self) -> Self::Output {
        assert_eq!(self.row,rhs.row);
        assert_eq!(self.col,rhs.col);
        let mut val:Vec<T> = vec![];
        for i in 0..self.data.len() {
            let tmp = self.data[i] + rhs.data[i];
            val.push(tmp);
        }
        let ans = Matrix{data: val, row: self.row, col :self.col};
        return ans;
    }
}

impl<'a, T: ops::Sub<Output = T> + Copy> ops::Sub for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row || self.col != rhs.col`, panic.
    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.row,rhs.row);
        assert_eq!(self.col,rhs.col);
        let mut val:Vec<T> = vec![];
        for i in 0..self.data.len() {
            let tmp = self.data[i] - rhs.data[i];
            val.push(tmp);
        }
        let ans = Matrix{data: val, row: self.row, col :self.col};
        return ans;
    }
}

impl<'a, T: ops::Sub<Output = T> + Copy> ops::Sub<Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row`, panic.
    fn sub(self, rhs: Matrix<T>) -> Self::Output {
        assert_eq!(self.row,rhs.row);
        assert_eq!(self.col,rhs.col);
        let mut val:Vec<T> = vec![];
        for i in 0..self.data.len() {
            let tmp = self.data[i] - rhs.data[i];
            val.push(tmp);
        }
        let ans = Matrix{data: val, row: self.row, col :self.col};
        return ans;
    }
}

impl<T: ops::Sub<Output = T> + Copy> ops::Sub for Matrix<T> {
    type Output = Self;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row`, panic.
    fn sub(self, rhs: Self) -> Self::Output {
        assert_eq!(self.row,rhs.row);
        assert_eq!(self.col,rhs.col);
        let mut val:Vec<T> = vec![];
        for i in 0..self.data.len() {
            let tmp = self.data[i] - rhs.data[i];
            val.push(tmp);
        }
        let ans = Matrix{data: val, row: self.row, col :self.col};
        return ans;
    }
}

impl<'a, T: ops::Sub<Output = T> + Copy> ops::Sub<&'a Self> for Matrix<T> {
    type Output = Self;

    /// Returns the subtraction of `rhs` from `self`. If `self.row != rhs.row`, panic.
    fn sub(self, rhs: &Self) -> Self::Output {
        assert_eq!(self.row,rhs.row);
        assert_eq!(self.col,rhs.col);
        let mut val:Vec<T> = vec![];
        for i in 0..self.data.len() {
            let tmp = self.data[i] - rhs.data[i];
            val.push(tmp);
        }
        let ans = Matrix{data: val, row: self.row, col :self.col};
        return ans;
    }
}

impl<'a, T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> ops::Mul for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.col,rhs.row);
        let row = self.row;
        let col = rhs.col;
        let mut stack:Vec<T> = vec![];
        let mut ans:Vec<T> = vec![];
        
        let mut m1:Vec<Vec<T>> = vec![Vec::<T>::new(); self.row];
        for i in 0..(self.data.len()) {
            let tmp = i / self.col;
            m1[tmp].push(self.data[i]);
        }

        let mut m2:Vec<Vec<T>> = vec![Vec::<T>::new(); rhs.row];
        for i in 0..(rhs.data.len()) {

            let tmp = i / rhs.col;
            m2[tmp].push(rhs.data[i]);
        }

        for i in 0..self.row {
            for j in 0..rhs.col {
                for k in 0..self.col {
                    let tmp = m1[i][k]*m2[k][j];
                    stack.push(tmp);
                }
            }
        }

        let n = self.col;
        let mut tmp = stack[0];
        for i in 0..stack.len(){
            if i % n == 0 {
                tmp = stack[i];
            }
            else {
                tmp = tmp + stack[i];
                if i % n == (n-1) {
                    ans.push(tmp);
                }
            }
        }
        let answer = Matrix{data: ans, row: row, col : col};
        return answer;
    }
}

impl<'a, T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> ops::Mul<Matrix<T>> for &'a Matrix<T> {
    type Output = Matrix<T>;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        assert_eq!(self.col,rhs.row);
        let row = self.row;
        let col = rhs.col;
        let mut stack:Vec<T> = vec![];
        let mut ans:Vec<T> = vec![];
        
        let mut m1:Vec<Vec<T>> = vec![Vec::<T>::new(); self.row];
        for i in 0..(self.data.len()) {
            let tmp = i / self.col;
            m1[tmp].push(self.data[i]);
        }

        let mut m2:Vec<Vec<T>> = vec![Vec::<T>::new(); rhs.row];
        for i in 0..(rhs.data.len()) {

            let tmp = i / rhs.col;
            m2[tmp].push(rhs.data[i]);
        }

        for i in 0..self.row {
            for j in 0..rhs.col {
                for k in 0..self.col {
                    let tmp = m1[i][k]*m2[k][j];
                    stack.push(tmp);
                }
            }
        }

        let n = self.col;
        let mut tmp = stack[0];
        for i in 0..stack.len(){
            if i % n == 0 {
                tmp = stack[i];
            }
            else {
                tmp = tmp + stack[i];
                if i % n == (n-1) {
                    ans.push(tmp);
                }
            }
        }
        let answer = Matrix{data: ans, row: row, col : col};
        return answer;
    }
}

impl<T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> ops::Mul for Matrix<T> {
    type Output = Self;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.col,rhs.row);
        let row = self.row;
        let col = rhs.col;
        let mut stack:Vec<T> = vec![];
        let mut ans:Vec<T> = vec![];
        
        let mut m1:Vec<Vec<T>> = vec![Vec::<T>::new(); self.row];
        for i in 0..(self.data.len()) {
            let tmp = i / self.col;
            m1[tmp].push(self.data[i]);
        }

        let mut m2:Vec<Vec<T>> = vec![Vec::<T>::new(); rhs.row];
        for i in 0..(rhs.data.len()) {

            let tmp = i / rhs.col;
            m2[tmp].push(rhs.data[i]);
        }

        for i in 0..self.row {
            for j in 0..rhs.col {
                for k in 0..self.col {
                    let tmp = m1[i][k]*m2[k][j];
                    stack.push(tmp);
                }
            }
        }

        let n = self.col;
        let mut tmp = stack[0];
        for i in 0..stack.len(){
            if i % n == 0 {
                tmp = stack[i];
            }
            else {
                tmp = tmp + stack[i];
                if i % n == (n-1) {
                    ans.push(tmp);
                }
            }
        }
        let answer = Matrix{data: ans, row: row, col : col};
        return answer;
    }
}

impl<'a, T: ops::Add<Output = T> + ops::Mul<Output = T> + Copy> ops::Mul<&'a Self> for Matrix<T> {
    type Output = Self;

    /// Returns the multiplication of `self` by `rhs`. If `self.col != rhs.row`, panic.
    fn mul(self, rhs: &Self) -> Self::Output {
        assert_eq!(self.col,rhs.row);
        let row = self.row;
        let col = rhs.col;
        let mut stack:Vec<T> = vec![];
        let mut ans:Vec<T> = vec![];
        
        let mut m1:Vec<Vec<T>> = vec![Vec::<T>::new(); self.row];
        for i in 0..(self.data.len()) {
            let tmp = i / self.col;
            m1[tmp].push(self.data[i]);
        }

        let mut m2:Vec<Vec<T>> = vec![Vec::<T>::new(); rhs.row];
        for i in 0..(rhs.data.len()) {

            let tmp = i / rhs.col;
            m2[tmp].push(rhs.data[i]);
        }

        for i in 0..self.row {
            for j in 0..rhs.col {
                for k in 0..self.col {
                    let tmp = m1[i][k]*m2[k][j];
                    stack.push(tmp);
                }
            }
        }

        let n = self.col;
        let mut tmp = stack[0];
        for i in 0..stack.len(){
            if i % n == 0 {
                tmp = stack[i];
            }
            else {
                tmp = tmp + stack[i];
                if i % n == (n-1) {
                    ans.push(tmp);
                }
            }
        }
        let answer = Matrix{data: ans, row: row, col : col};
        return answer;
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    /// Formats the matrix as follows:
    /// * Writes each row on a separate line. No empty lines before or after any row.
    /// * On each row, writes each element followed by a single space, except no space following the last element of the row.
    /// Outputs using `write!(f, ...)`.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ans = String::new();
        for i in 0..self.data.len() {
            if i % self.col == (self.col-1) {
                let a = self.data[i].to_string();
                ans.push_str(&a);
                if i != (self.data.len()-1) {
                    ans.push_str("\n");
                }
            }
            else {
                let a = self.data[i].to_string();
                ans.push_str(&a);
                ans.push_str(" ");
            }
        }
        ans.push_str("\n");
        write!(f,"{}",ans)
    }
}

fn main() {
    let x = Matrix::new(2, 3, &[-2, -1, 0, 1, 2, 3]);
    let y = Matrix::new(2, 3, &[1, 2, 3, 4, 5, 6]);
    assert_eq!(&x + &y - &y, x);
    assert_eq!(format!("{}", x), "-2 -1 0\n1 2 3\n");
}