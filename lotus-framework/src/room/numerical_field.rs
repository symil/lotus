pub struct NumericalField {
    pub base_value: f64,
    pub add_value: f64,
    pub mult_value: f64,
    pub final_value: f64
}

impl NumericalField {
    pub fn new() -> Self {
        Self {
            base_value: 0.,
            add_value: 0.,
            mult_value: 1.,
            final_value: 0.,
        }
    }

    pub fn reset(&mut self) {
        self.add_value = 0.;
        self.mult_value = 1.;
    }

    pub fn compute(&mut self) {
        self.final_value = (self.base_value + self.add_value) * self.mult_value;
    }

    pub fn add(&mut self, value: f64) {
        self.add_value += value;
    }

    pub fn sub(&mut self, value: f64) {
        self.add_value -= value;
    }

    pub fn mult(&mut self, value: f64) {
        self.mult_value *= value;
    }

    pub fn div(&mut self, value: f64) {
        self.mult_value /= value;
    }
}