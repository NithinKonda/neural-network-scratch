

use super::{activation::Activation, matrix::Matrix};

pub struct Network<'a> {
    layers : Vec<usize>,
    weights : Vec<Matrix>,
    biases : Vec<Matrix>,
    data : Vec<Matrix>,
    activation : Activation<'a>,
}

impl Network <'_>{

    pub fn new(layers: Vec<usize>) -> Network {
        let mut weights = vec![];
        let mut biases = vec![];
        for i in 0..layers.len()-1 {
            weights.push(Matrix::random(layers[i+1], layers[i]));
            biases.push(Matrix::random(layers[i+1], 1));
        }


        Network{
            layers,
            weights,
            biases,
            data: vec![],
        }
    }


    pub fn feed_forward(&mut self, inputs: Vec<f64>) -> Vec<f64> {
        if inputs.len() != self.layers[0] {
            panic!("Invalid number of inputs");
        }
        let mut current = Matrix::from(vec![inputs]).transpose();
        self.data = vec![current.clone()];
        for i in 0..self.layers.len()-1 {
            current = self.weights[i].dot_multiply(&current).add(&self.biases[i]);
            current = current.sigmoid();
            self.data.push(current.clone());
        }

        current.data[0].clone()
    }
}