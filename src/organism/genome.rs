use rand::{rngs::ThreadRng, Rng};
use serde::{Deserialize, Serialize};

// Stores the genetic info of the creature
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Genome {
    pub genome_mutate_rate: Allele,
    pub genome_mutate_factor: Allele,
    pub learning_rate: Allele,
    pub learning_factor: Allele,
    pub joint_mutate_rate: Allele,
    pub joint_mutate_factor: Allele,
    pub internal_clock: Allele,
}
impl Genome {
    pub fn mutate(&mut self) {
        self.genome_mutate_rate.mutate_self();
        self.genome_mutate_factor.mutate_self();
        self.learning_rate.mutate_self();
        self.learning_factor.mutate_self();
        self.joint_mutate_rate.mutate_self();
        self.joint_mutate_factor.mutate_self();
        self.internal_clock.mutate_self();
    }
}
impl Default for Genome {
    fn default() -> Self {
        Self {
            genome_mutate_rate: Allele::new(0.1, 0.2, 0.2),
            genome_mutate_factor: Allele::new(0.1, 0.2, 0.2),
            learning_rate: Allele::new(0.1, 0.2, 0.2),
            learning_factor: Allele::new(0.1, 0.2, 0.2),
            joint_mutate_rate: Allele::new(0.3, 0.2, 0.2),
            joint_mutate_factor: Allele::new(10.0, 1.0, 1.0),
            internal_clock: Allele::new(3.0, 0.2, 0.2),
        }
    }
}

// Encodes data the trait of an organism
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Allele {
    pub val: f32,
    pub mutate_rate: f32,
    pub mutate_factor: f32,
}
impl Allele {
    pub fn new(val: f32, mr: f32, mf: f32) -> Self {
        return Self {
            val: val,
            mutate_rate: mr,
            mutate_factor: mf,
        };
    }

    pub fn mutate(&mut self, mut rng: &mut ThreadRng, mr: f32, mf: f32) {
        // if rng.gen() <=mr
    }

    // Mutate allele based on mutate rate and factor
    pub fn mutate_self(&mut self) {
        let mut rng = rand::thread_rng();

        // Check if allele mutates based on mutate rate
        if rng.gen::<f32>() <= self.mutate_rate {
            let mf = self.mutate_factor;

            // Alter allele based of mutate factor
            self.val += rng.gen_range(-mf..mf);
            self.mutate_rate += rng.gen_range(-mf..mf);
            self.mutate_factor += rng.gen_range(-mf..mf);

            // Clamp values
            self.val = self.mutate_factor.max(0.01);
            self.mutate_rate = self.mutate_rate.clamp(0.01, 1.0);
            self.mutate_factor = self.mutate_factor.max(0.01);
        }
    }
}
