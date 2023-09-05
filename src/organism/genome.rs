use rand::Rng;
use serde::{Deserialize, Serialize};

// Stores the genetic info of the creature
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Genome {
    pub learning_rate: Allele,
    pub learning_factor: Allele,
    pub joint_mutate_rate: Allele,
    pub joint_mutate_factor: Allele,
    pub internal_clock: Allele,
}
impl Genome {
    pub fn mutate(&mut self) {
        self.learning_rate.mutate();
        self.learning_factor.mutate();
        self.joint_mutate_rate.mutate();
        self.joint_mutate_factor.mutate();
        self.internal_clock.mutate();
    }
}
impl Default for Genome {
    fn default() -> Self {
        Self {
            learning_rate: Allele {
                val: 0.1,
                mutate_rate: 0.2,
                mutate_factor: 0.2,
            },
            learning_factor: Allele {
                val: 0.1,
                mutate_rate: 0.2,
                mutate_factor: 0.2,
            },
            joint_mutate_rate: Allele {
                val: 0.3,
                mutate_rate: 0.2,
                mutate_factor: 0.2,
            },
            joint_mutate_factor: Allele {
                val: 10.0,
                mutate_rate: 1.0,
                mutate_factor: 1.0,
            },
            internal_clock: Allele {
                val: 3.0,
                mutate_rate: 0.2,
                mutate_factor: 0.2,
            },
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
    // Mutate allele based on mutate rate and factor
    pub fn mutate(&mut self) {
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
