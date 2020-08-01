use crate::connection::Connection;
use crate::node::Node;
use crate::settings::Settings;

use rand::prelude::*;
use rand::thread_rng;

use std::collections::HashMap;
use std::vec::Vec;

pub struct Genome {
    inputs: u32,
    outputs: u32,
    nodes: Vec<Node>,
    conns: Vec<Connection>,
    pub fitness: f64,
    species_id: u32,
}

impl Genome {
    pub fn new(inputs: u32, outputs: u32, crossover: bool) -> Self {
        let mut genome = Self {
            inputs,
            outputs,
            nodes: Vec::with_capacity((inputs + outputs + 1) as usize),
            conns: Vec::with_capacity(((inputs + 1) * outputs) as usize),
            fitness: 0.,
            species_id: 0,
        };

        let mut dy = 1. / (inputs + 1) as f64;
        let mut dy_curr = dy;

        for i in 1..=(inputs + 1) {
            genome.nodes.push(Node::new(i, 0., dy_curr));
            dy_curr += dy;
        }

        dy = 1. / (outputs + 1) as f64;
        dy_curr = dy;

        for i in (inputs + 2)..(inputs + outputs + 2) {
            genome.nodes.push(Node::new(i, 1., dy_curr));
            dy_curr += dy;
        }

        if crossover {
            return genome;
        }

        let mut rng = thread_rng();
        let mut ctr = 1;
        for i in 0..(inputs + 1) {
            let from = genome.nodes[i as usize].innov;
            for o in (inputs + 1) as usize..genome.nodes.len() {
                let to = genome.nodes[o].innov;
                genome
                    .conns
                    .push(Connection::new(ctr, from, to, rng.gen::<f64>(), true));

                ctr += 1;
            }
        }

        genome
    }

    pub fn set_species(&mut self, id: u32) {
        self.species_id = id;
    }

    pub fn feed_forward(&mut self, input: &Vec<f64>) -> Result<Vec<f64>, &'static str> {
        if input.len() != self.inputs as usize {
            return Err("Provided input size doesn't match Genome input size");
        }

        let mut node_vals = HashMap::<u32, f64>::new();

        let mut i = 1;
        for val in input {
            node_vals.insert(i, *val);
            i += 1;
        }

        node_vals.insert(self.inputs + 1, 1.);

        for node in self.nodes.iter() {
            let from_val = *node_vals.get(&node.innov).unwrap();

            let feed_forward_val = Node::activate(from_val, node.x);

            for conn in self.conns.iter().filter(|&c| c.from == node.innov) {
                let to_val = node_vals.entry(conn.to).or_insert(0.);
                *to_val += feed_forward_val * conn.weight;
            }
        }

        Ok(((self.inputs + 2)..(self.inputs + self.outputs + 2))
            .map(|v| *node_vals.get(&v).unwrap())
            .collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_genome() {
        let mut gen = Genome::new(3, 2, false);

        for conn in &mut gen.conns {
            conn.weight = 1.;
        }

        assert_eq!(gen.feed_forward(&vec![1., 1., 1.]).unwrap(), vec![4., 4.]);
    }
}
