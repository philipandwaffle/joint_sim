use core::panic;

use nalgebra::DMatrix;
use rand::{rngs::ThreadRng, Rng};
use serde::{de::Visitor, ser::SerializeSeq, Deserialize, Deserializer, Serialize};

pub type Matrix = DMatrix<f32>;

// Wrapper struct so that the nalgebra crate can be extended
#[derive(Clone)]
pub struct MxNMatrix(pub Matrix);
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
        println!("adding io");
        debug_matrix_shapes(&self.weights, &"weights");
        debug_matrix_shapes(&self.biases, &"biases");
        let num_weights = self.weights.len();
        let num_biases = self.biases.len();

        // add input
        insert_row(&mut self.weights[0]);
        insert_row(&mut self.weights[0]);
        // insert_col(&mut self.biases[0]);

        // Add output
        insert_row(&mut self.weights[num_weights - 1]);
        insert_row(&mut self.weights[num_weights - 1]);
        insert_col(&mut self.biases[num_biases - 1]);
        insert_col(&mut self.biases[num_biases - 1]);

        self.memory.push(0.0);
        self.memory.push(0.0);

        debug_matrix_shapes(&self.weights, &"weights");
        debug_matrix_shapes(&self.biases, &"biases");
    }

    pub fn remove_io(&mut self) {
        println!("removing io");
        debug_matrix_shapes(&self.weights, &"weights");
        debug_matrix_shapes(&self.biases, &"biases");
        let num_weights = self.weights.len();
        let num_biases = self.biases.len();

        // add input
        remove_row(&mut self.weights[0]);
        remove_row(&mut self.weights[0]);
        // remove_col(&mut self.biases[0]);

        // Add output
        remove_row(&mut self.weights[num_weights - 1]);
        remove_row(&mut self.weights[num_weights - 1]);
        remove_col(&mut self.biases[num_biases - 1]);
        remove_col(&mut self.biases[num_biases - 1]);

        self.memory.pop();
        self.memory.pop();

        debug_matrix_shapes(&self.weights, &"weights");
        debug_matrix_shapes(&self.biases, &"biases");
    }

    // Set the memory used for feed forward
    pub fn set_memory(&mut self, remember: Vec<f32>) {
        if self.memory.len() != remember.len() {
            panic!(
                "Creature trying to remember {:?}/{:?} items",
                remember.len(),
                self.memory.len(),
            );
        }
        self.memory = remember;
    }

    pub fn process_stimuli(&self, external_stimuli: &Vec<f32>) -> Vec<f32> {
        // Create input from memory
        let mut input = self.memory.clone();
        // Append external stimuli to memory
        input.extend(external_stimuli);

        let in_len = input.len();
        let len = self.weights[0].0.shape().0;
        if in_len != len {
            panic!("brain can only receive {} inputs, received {}", len, in_len);
        }

        // Feed forward input
        let x = Matrix::from_vec(1, in_len, input);
        let mut y = x;
        for i in 0..self.weights.len() {
            y = y * &self.weights[i].0 + &self.biases[i].0;
            for cell in y.iter_mut() {
                *cell = cell.tanh();
            }
        }
        let output = y.iter().map(|x| *x).collect::<Vec<f32>>();

        return output;
    }

    // Mutate brain based on learning rate and learning factor
    pub fn learn(&mut self, rng: &mut ThreadRng, learning_rate: f32, learning_factor: f32) {
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

fn insert_row(m: &mut MxNMatrix) {
    let temp = m.0.clone();
    let rows = temp.shape().0;
    m.0 = temp.insert_row(rows, 0.0);
}
fn remove_row(m: &mut MxNMatrix) {
    let temp = m.0.clone();
    let rows = temp.shape().0;
    m.0 = temp.remove_row(rows - 1);
}

fn insert_col(m: &mut MxNMatrix) {
    let temp = m.0.clone();
    let cols = temp.shape().1;
    m.0 = temp.insert_column(cols, 0.0);
}
fn remove_col(m: &mut MxNMatrix) {
    let temp = m.0.clone();
    let cols = temp.shape().1;
    m.0 = temp.remove_column(cols - 1);
}

fn debug_matrix_shapes(m: &Vec<MxNMatrix>, msg: &str) {
    println!(
        "{} {:?}",
        msg,
        m.iter()
            .map(|x| x.0.shape())
            .collect::<Vec<(usize, usize)>>()
    );
}
