use rand::Rng;

#[derive(Clone)]
pub struct Genome {
    pub learning_rate: Allele,
    pub learning_factor: Allele,
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
            learning_rate: Allele::default(),
            learning_factor: Allele::default(),
            internal_clock: Allele::default(),
        }
    }
}

#[derive(Clone)]
pub struct Allele {
    pub val: f32,
    pub mutate_rate: f32,
    pub mutate_factor: f32,
}
impl Default for Allele {
    fn default() -> Self {
        Self {
            val: 0.5,
            mutate_rate: 1.0,
            mutate_factor: 0.5,
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
