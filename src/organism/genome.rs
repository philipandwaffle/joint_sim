use rand::Rng;

#[derive(Clone, Debug)]
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
                val: 0.1,
                mutate_rate: 0.2,
                mutate_factor: 0.2,
            },
            joint_mutate_factor: Allele {
                val: 10.0,
                mutate_rate: 0.2,
                mutate_factor: 0.2,
            },
            internal_clock: Allele {
                val: 3.0,
                mutate_rate: 0.2,
                mutate_factor: 0.2,
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct Allele {
    pub val: f32,
    pub mutate_rate: f32,
    pub mutate_factor: f32,
}
impl Default for Allele {
    fn default() -> Self {
        Self {
            val: 0.5,
            mutate_rate: 0.1,
            mutate_factor: 0.1,
        }
    }
}
impl Allele {
    pub fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() <= self.mutate_rate {
            let mf = self.mutate_factor;
            self.val += rng.gen_range(-mf..mf);
            self.mutate_rate += rng.gen_range(-mf..mf);
            self.mutate_factor += rng.gen_range(-mf..mf);

            self.mutate_rate = self.mutate_rate.clamp(0.0, 1.0);
        }
    }
}
