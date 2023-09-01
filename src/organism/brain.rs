use core::panic;
use std::fmt::Debug;

use nalgebra::DMatrix;
use rand::Rng;

type NxNMatrix = DMatrix<f32>;

#[derive(Clone)]
pub struct Brain {
    pub memory: Vec<f32>,
    pub weights: Vec<NxNMatrix>,
    pub biases: Vec<NxNMatrix>,
    pub activation_fn: fn(f32) -> f32,
}
impl Brain {
    pub fn new(structure: Vec<usize>, activation_fn: fn(f32) -> f32) -> Self {
        let mut weights = vec![];
        let mut biases = vec![];
        let num_layers = structure.len();

        for i in 1..num_layers {
            weights.push(gen_rand_matrix(structure[i - 1], structure[i]));
            biases.push(gen_rand_matrix(1, structure[i]));
        }

        return Self {
            memory: vec![0.0; structure[num_layers - 1]],
            weights,
            biases,
            activation_fn,
        };
    }

    pub fn set_memory(&mut self, memory: Vec<f32>) {
        if self.memory.capacity() != memory.capacity() {
            panic!("Creature trying to remember more that allocated");
        }
        self.memory = memory;
    }

    pub fn feed_forward(&mut self, mut external_stimuli: Vec<f32>) -> Vec<f32> {
        let mut input = self.memory.clone();
        input.append(&mut external_stimuli);

        let in_len = input.len();
        let len = self.weights[0].shape().0;
        if in_len != len {
            panic!("brain can only receive {} inputs, received {}", len, in_len);
        }

        let x = NxNMatrix::from_vec(1, in_len, input);
        let y = self.step_forward(x, 0);
        let output = y.iter().map(|x| *x).collect::<Vec<f32>>();

        return output;
    }

    fn step_forward(&self, x: NxNMatrix, i: usize) -> NxNMatrix {
        let mut res = x * self.weights[i].clone() + self.biases[i].clone();
        for cell in res.iter_mut() {
            *cell = (self.activation_fn)(cell.clone());
        }
        if i == self.weights.len() - 1 {
            return res;
        } else {
            return self.step_forward(res, i + 1);
        }
    }

    pub fn mutate(&mut self, learning_rate: f32, learning_factor: f32) {
        for weight in self.weights.iter_mut() {
            Self::mutate_matrix(weight, learning_rate, learning_factor);
        }

        for bias in self.biases.iter_mut() {
            Self::mutate_matrix(bias, learning_rate, learning_factor);
        }
    }

    fn mutate_matrix(m: &mut NxNMatrix, mut_rate: f32, mut_factor: f32) {
        let mut rng = rand::thread_rng();
        for cell in m.iter_mut() {
            if rng.gen::<f32>() <= mut_rate {
                *cell += (rng.gen::<f32>() - 0.5) * mut_factor;
            }
        }
    }
}
impl Debug for Brain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Brain")
            .field("weights", &self.weights)
            .field("biases", &self.biases)
            .finish()
    }
}

fn gen_rand_matrix(rows: usize, cols: usize) -> NxNMatrix {
    let mut rng = rand::thread_rng();
    let mut m = NxNMatrix::zeros(rows, cols);

    for cell in m.iter_mut() {
        *cell = rng.gen_range(-1.0..=1.0);
    }

    return m;
}
