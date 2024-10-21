use rand::{thread_rng,Rng};

pub struct Matrix{
    pub rows : usize,
    pub cols : usize,
    pub data : Vec<Vec<f64>>,
}

impl Matrix {

    pub fn zeros(rows:usize, cols: usize) -> Matrix {
        Matrix {
            rows,
            cols,
            data: vec![vec![0.0; cols];rows]
        }
    }

    


    pub fn random(rows:usize, cols:usize) -> Matrix {
        let mut rng = thread_rng();

        let mut res = Matrix::zeros(rows, cols);

        for i in 0..rows {
            for j in 0..cols {
                res.data[i][j] = rng.gen::<f64>() * 2.0 - 1.0;
            }
        }
        res
    }

    pub fn multiply(&mut self, other: &Matrix) -> Matrix {
        if self.cols != other.rows {
            panic!("UNSUPPORTED DIMENSION MULTIPLICATION");
        }

        let mut res = Matrix::zeros(self.rows, other.cols);

        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    res.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }

        res
    }


    pub fn add(&mut self, other : &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("UNSUPPORTED DIMENSION ADDITION");
        }

        let mut res = Matrix::zeros(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }

        res
    }

    pub fn subtract(&mut self, other : &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("UNSUPPORTED DIMENSION SUBTRACTION");
        }

        let mut res = Matrix::zeros(self.rows, self.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }

        res
    }


    pub fn dot_multiply(&mut self,other : &Matrix) -> Matrix {
        if self.cols != other.rows || self.cols != other.rows{
            panic!("UNSUPPORTED DIMENSION MULTIPLICATION");
        }

        let mut res = Matrix::zeros(self.rows, other.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                    res.data[i][j] += self.data[i][j] * other.data[i][j];
            }
        }

        res
    }


    

}