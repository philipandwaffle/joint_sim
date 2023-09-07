use core::panic;

use nalgebra::DMatrix;
use rand::{rngs::ThreadRng, Rng};
use serde::{de::Visitor, ser::SerializeSeq, Deserialize, Deserializer, Serialize};

pub type Matrix = DMatrix<f32>;

// Wrapper struct so that the nalgebra crate can be extended
#[derive(Clone)]
pub struct MxNMatrix(pub DMatrix<f32>);
impl Serialize for MxNMatrix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let m = &self.0;
        // Allocate space for sequence
        let mut m_seq = serializer.serialize_seq(Some(m.iter().len()))?;

        // Add matrix shape data to sequence
        m_seq.serialize_element(&m.shape().0)?;
        m_seq.serialize_element(&m.shape().1)?;

        // Add each cell value to the sequence
        for cell in m.iter() {
            m_seq.serialize_element(&cell)?;
        }
        return m_seq.end();
    }
}
impl<'de> Deserialize<'de> for MxNMatrix {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let m = d.deserialize_seq(MxMMatrixVisitor)?;
        return Ok(m);
    }
}

// Visitor for deserializing MxMMatrix
pub struct MxMMatrixVisitor;
impl<'de> Visitor<'de> for MxMMatrixVisitor {
    type Value = MxNMatrix;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        return formatter.write_str("Matrix");
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        // Get the shape of the matrix
        let shape = [
            seq.next_element::<f32>()?.unwrap() as usize,
            seq.next_element::<f32>()?.unwrap() as usize,
        ];

        // Loop through element to get matrix data
        let mut data = vec![];
        loop {
            let cell = seq.next_element::<f32>()?;
            match cell {
                Some(val) => data.push(val),
                None => break,
            }
        }
        println!("{:?}", data);

        return Ok(MxNMatrix(Matrix::from_vec(shape[0], shape[1], data)));
    }
}

// Basic neural network
#[derive(Clone, Serialize, Deserialize)]
pub struct Brain {
    pub memory: Vec<f32>,
    pub weights: Vec<MxNMatrix>,
    pub biases: Vec<MxNMatrix>,
}
impl Brain {
    // Create a new brain based on the structure provided
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

    pub fn add_io(&mut self) {
        let last_index = self.weights.len() - 1;
        let insert_index = self.get_num_inputs() - 1;
        let mem_len = self.memory.len();

        // add input
        let temp = self.weights[0].clone();
        self.weights[0] = MxNMatrix(temp.0.insert_row(insert_index, 0.0));
        let temp = self.biases[0].clone();
        self.biases[0] = MxNMatrix(temp.0.insert_column(insert_index, 0.0));

        // Add output
        let temp = self.weights[last_index].clone();
        self.weights[last_index] = MxNMatrix(temp.0.insert_row(insert_index - mem_len - 1, 0.0));
        let temp = self.biases[last_index].clone();
        self.biases[last_index] = MxNMatrix(temp.0.insert_column(insert_index - mem_len - 1, 0.0));
    }

    pub fn remove_io(&mut self) {
        let last_index = self.weights.len() - 1;
        let remove_index = self.get_num_inputs() - 1;
        let mem_len = self.memory.len();

        println!(
            "remove index: {:?}, last index: {:?}",
            remove_index, last_index
        );
        // Remove input
        let temp = self.weights[0].clone();
        self.weights[0] = MxNMatrix(temp.0.remove_row(remove_index));
        let temp = self.biases[0].clone();
        println!("m: {:?}", temp.0.shape());
        self.biases[0] = MxNMatrix(temp.0.remove_column(remove_index));

        // Remove output
        let temp = self.weights[last_index].clone();
        self.weights[last_index] = MxNMatrix(temp.0.remove_row(remove_index - mem_len - 1));
        let temp = self.biases[last_index].clone();
        println!("m: {:?}", temp.0.shape());
        self.biases[last_index] = MxNMatrix(temp.0.remove_column(remove_index - mem_len - 1));
    }

    // Set the memory used for feed forward
    fn set_memory(&mut self, memory: Vec<f32>) {
        if self.memory.capacity() != memory.capacity() {
            panic!("Creature trying to remember more that allocated");
        }
        self.memory = memory;
    }

    pub fn feed_forward(&mut self, mut external_stimuli: Vec<f32>) -> Vec<f32> {
        // Create input from memory
        let mut input = self.memory.clone();
        // Append external stimuli to memory
        input.append(&mut external_stimuli);

        let in_len = input.len();
        let len = self.weights[0].0.shape().0;
        if in_len != len {
            panic!("brain can only receive {} inputs, received {}", len, in_len);
        }

        // Feed forward input
        let x = Matrix::from_vec(1, in_len, input);
        let y = self.step_forward(x, 0);
        let output = y.iter().map(|x| *x).collect::<Vec<f32>>();

        // Set memory to previous output
        self.set_memory(output.clone());

        return output;
    }

    // Process a layer
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

    // Mutate brain based on learning rate and learning factor
    pub fn mutate(&mut self, rng: &mut ThreadRng, learning_rate: f32, learning_factor: f32) {
        for weight in self.weights.iter_mut() {
            Self::mutate_matrix(rng, weight, learning_rate, learning_factor);
        }

        for bias in self.biases.iter_mut() {
            Self::mutate_matrix(rng, bias, learning_rate, learning_factor);
        }
    }

    fn mutate_matrix(rng: &mut ThreadRng, m: &mut MxNMatrix, mut_rate: f32, mut_factor: f32) {
        for cell in m.0.iter_mut() {
            if rng.gen::<f32>() <= mut_rate {
                *cell += (rng.gen::<f32>() - 0.5) * mut_factor;
            }
        }
    }
}

fn gen_rand_matrix(rows: usize, cols: usize) -> MxNMatrix {
    let mut rng = rand::thread_rng();
    let mut m = Matrix::zeros(rows, cols);

    for cell in m.iter_mut() {
        *cell = rng.gen_range(-1.0..=1.0);
    }

    return MxNMatrix(m);
}
