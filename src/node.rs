pub struct Node {
    pub innov: u32,
    pub x: f64,
    pub y: f64,
}

impl std::cmp::PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.innov == other.innov
    }
}

impl Node {
    pub fn new(innov: u32, x: f64, y: f64) -> Self {
        Self { innov, x, y }
    }

    pub fn activate(val: f64, layer: f64) -> f64 {
        if layer == 0. {
            return val;
        }

        1. / (1. + (-4.9 * val).exp())
    }
}

impl std::clone::Clone for Node {
    fn clone(&self) -> Self {
        Self {
            innov: self.innov,
            x: self.x,
            y: self.y,
        }
    }
}
