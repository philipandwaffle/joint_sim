use core::panic;
use std::{fmt::Debug, str::FromStr};

use nalgebra::DMatrix;
use rand::Rng;
use serde::{
    de::Visitor,
    ser::{SerializeSeq, SerializeStruct},
    Deserialize, Deserializer, Serialize,
};

pub type Matrix = DMatrix<f32>;
#[derive(Clone)]
pub struct MxMMatrix(pub DMatrix<f32>);
impl Serialize for MxMMatrix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let m = &self.0;
        let mut m_seq = serializer.serialize_seq(Some(m.iter().len()))?;
        for row in m.row_iter() {
            m_seq.serialize_element(&row.iter().map(|x| *x).collect::<Vec<f32>>())?;
        }
        return m_seq.end();
    }
}
impl<'de> Deserialize<'de> for MxMMatrix {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Printing d to check what's inside
        println!("des {:?}", f32::deserialize(d));
        // d.desera
        // Return dummy data for now
        Ok(MxMMatrix(Matrix::from_vec(2, 2, vec![1.0, 2.0, 3.0, 4.0])))
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Brain {
    pub memory: Vec<f32>,
    pub weights: Vec<MxMMatrix>,
    pub biases: Vec<MxMMatrix>,
}
impl Brain {
    pub fn new(structure: Vec<usize>) -> Self {
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
        };
    }

    pub fn get_num_inputs(&self) -> usize {
        return self.weights[0].0.shape().0;
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
        let len = self.weights[0].0.shape().0;
        if in_len != len {
            panic!("brain can only receive {} inputs, received {}", len, in_len);
        }

        let x = Matrix::from_vec(1, in_len, input);
        let y = self.step_forward(x, 0);
        let output = y.iter().map(|x| *x).collect::<Vec<f32>>();
        self.set_memory(output.clone());

        return output;
    }

    fn step_forward(&self, x: Matrix, i: usize) -> Matrix {
        let mut res = x * self.weights[i].0.clone() + self.biases[i].0.clone();
        for cell in res.iter_mut() {
            *cell = f32::tanh(cell.clone());
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

    fn mutate_matrix(m: &mut MxMMatrix, mut_rate: f32, mut_factor: f32) {
        let mut rng = rand::thread_rng();
        for cell in m.0.iter_mut() {
            if rng.gen::<f32>() <= mut_rate {
                *cell += (rng.gen::<f32>() - 0.5) * mut_factor;
            }
        }
    }
}

fn gen_rand_matrix(rows: usize, cols: usize) -> MxMMatrix {
    let mut rng = rand::thread_rng();
    let mut m = Matrix::zeros(rows, cols);

    for cell in m.iter_mut() {
        *cell = rng.gen_range(-1.0..=1.0);
    }

    return MxMMatrix(m);
}
