use rand;

pub struct Matrix{
    pub rows : usize,
    pub cols : usize,
    pub data : Vec<f64>,
}

impl Matrix {


    pub fn random(rows:usize, cols:usize) -> Matrix {
        let mut buffer : Vec<f64> = Vec::<f64>::with_capacity(rows * cols);

        for _ in 0..rows*cols {
            let num : f64 = rand::thread_rng().gen_range(0,0..1.0);
            buffer.push(num);
        }

        Matrix{rows,cols,data:buffer}
    }
}