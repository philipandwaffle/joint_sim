use nalgebra::{DMatrix, Dyn, OMatrix, SMatrix};
use rand::Rng;

type NxNMatrix = DMatrix<f32>;

pub struct Brain {
    weights: Vec<NxNMatrix>,
    biases: Vec<NxNMatrix>,
    activation_fn: fn(f32) -> f32,
}
impl Brain {
    pub fn new(n: Vec<usize>, activation_fn: fn(f32) -> f32) -> Self {
        let mut weights = vec![];
        let mut biases = vec![];

        for i in 1..n.len() {
            weights.push(gen_rand_matrix(n[i - 1], n[i - 1]));
            biases.push(gen_rand_matrix(1, n[i]));
        }

        return Self {
            weights,
            biases,
            activation_fn,
        };
    }

    pub fn feed_forward(&self, input: Vec<f32>) -> Vec<f32> {
        let x = NxNMatrix::from_vec(1, input.len(), input);
        let res = self.step_forward(x, 0);

        return res.iter().map(|x| *x).collect();
    }
    fn step_forward(&self, x: NxNMatrix, i: usize) -> NxNMatrix {
        let mut res = self.weights[i].clone() * x + self.biases[i].clone();
        for cell in res.iter_mut() {
            *cell = (self.activation_fn)(cell.clone());
        }
        if i == self.weights.len() {
            return res;
        } else {
            return self.step_forward(res, i);
        }
    }
}

fn gen_rand_matrix(rows: usize, cols: usize) -> NxNMatrix {
    let mut rng = rand::thread_rng();
    let mut m = NxNMatrix::zeros(rows, cols);

    for cell in m.iter_mut() {
        *cell = rng.gen::<f32>();
    }

    return m;
}
