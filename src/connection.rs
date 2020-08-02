use crate::settings::Settings;

use std::clone::Clone;
use std::cmp::PartialEq;
use std::fmt;

use rand::prelude::*;
use rand::{self, thread_rng};
use rand_distr::{Distribution, Normal};

pub struct Connection {
    pub innov: u32,
    pub weight: f64,
    pub from: u32,
    pub to: u32,
    pub enabled: bool,
    pub from_layer: f64,
    pub to_layer: f64,
}

impl fmt::Debug for Connection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Conn({}, {}, {})", self.innov, self.from, self.to)
    }
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.innov == other.innov
    }
}

impl Clone for Connection {
    fn clone(&self) -> Self {
        Self {
            innov: self.innov,
            from: self.from,
            to: self.to,
            weight: self.weight,
            enabled: self.enabled,
            from_layer: self.from_layer,
            to_layer: self.to_layer,
        }
    }
}

impl Connection {
    pub fn new(innov: u32, from: u32, to: u32, weight: f64, enabled: bool) -> Self {
        let conn = Self {
            innov,
            from,
            to,
            weight,
            enabled,
            from_layer: 0.,
            to_layer: 1.,
        };

        conn
    }

    pub fn set_from_layer(&mut self, layer: f64) {
        self.from_layer = layer;
    }

    pub fn set_to_layer(&mut self, layer: f64) {
        self.to_layer = layer;
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn mutate_weight(&mut self, sets: &Settings) {
        let mut rng = thread_rng();

        if rng.gen::<f64>() < sets.wt_shift_rate {
            self.weight += Normal::new(0., 0.04).unwrap().sample(&mut rng);

            if self.weight < -1. {
                self.weight = -1.;
            } else if self.weight > 1. {
                self.weight = 1.
            }
        } else {
            self.weight = rng.gen::<f64>() * 2. - 1.;
        }
    }
}
