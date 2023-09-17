use rand::{rngs::ThreadRng, Rng};
use serde::{Deserialize, Serialize};

// Stores the genetic info of the creature
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Genome {
    pub genome_mr: Allele,
    pub genome_mf: Allele,
    pub lr: Allele,
    pub lf: Allele,
    pub joint_mr: Allele,
    pub joint_mf: Allele,
    pub bone_mr: Allele,
    pub bone_mf: Allele,
    pub muscle_mr: Allele,
    pub internal_clock: Allele,
}
impl Genome {
    pub fn mutate(&mut self, rng: &mut ThreadRng) {
        let mr = self.genome_mr.val;
        let mf = self.genome_mf.val;

        self.genome_mr.mutate_meta(rng, mr, mf);
        self.genome_mf.mutate_meta(rng, mr, mf);
        self.lr.mutate_meta(rng, mr, mf);
        self.lf.mutate_meta(rng, mr, mf);
        self.joint_mr.mutate_meta(rng, mr, mf);
        self.joint_mf.mutate_meta(rng, mr, mf);
        self.bone_mr.mutate_meta(rng, mr, mf);
        self.bone_mf.mutate_meta(rng, mr, mf);
        self.muscle_mr.mutate_meta(rng, mr, mf);
        self.internal_clock.mutate_meta(rng, mr, mf);

        self.genome_mr.mutate_val(rng);
        self.genome_mf.mutate_val(rng);
        self.lr.mutate_val(rng);
        self.lf.mutate_val(rng);
        self.joint_mr.mutate_val(rng);
        self.joint_mf.mutate_val(rng);
        self.bone_mr.mutate_val(rng);
        self.bone_mf.mutate_val(rng);
        self.muscle_mr.mutate_val(rng);
        self.internal_clock.mutate_val(rng);
    }
}
impl Default for Genome {
    fn default() -> Self {
        Self {
            genome_mr: Allele::new(0.01, 0.01, 0.01),
            genome_mf: Allele::new(0.01, 0.01, 0.01),
            lr: Allele::new(0.1, 0.2, 0.2),
            lf: Allele::new(0.1, 0.2, 0.2),
            joint_mr: Allele::new(0.5, 0.02, 0.02),
            joint_mf: Allele::new(5.0, 0.02, 0.02),
            bone_mr: Allele::new(0.01, 0.2, 0.002),
            bone_mf: Allele::new(10.0, 0.2, 0.002),
            muscle_mr: Allele::new(0.01, 0.2, 1.0),
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

    // mutate allele meta data
    pub fn mutate_meta(&mut self, rng: &mut ThreadRng, mr: f32, mf: f32) {
        if rng.gen::<f32>() <= mr {
            let r = -mf..mf;
            self.mutate_rate += rng.gen_range(r.clone());
            self.mutate_factor += rng.gen_range(r);

            self.mutate_rate = self.mutate_rate.clamp(0.001, 1.0);
            self.mutate_factor = self.mutate_factor.max(0.001);
        }
    }

    // Mutate allele based on mutate rate and factor
    pub fn mutate_val(&mut self, rng: &mut ThreadRng) {
        // Check if allele mutates based on mutate rate
        if rng.gen::<f32>() <= self.mutate_rate {
            let mf = self.mutate_factor;

            // Alter allele based of mutate factor
            self.val += rng.gen_range(-mf..mf);

            // Clamp values
            self.val = self.mutate_factor.max(0.01);
        }
    }
}
