use crate::connection::Connection;
use crate::genome::Genome;
use crate::history::History;
use crate::node::Node;
use crate::settings::Settings;
use crate::species::Species;

use std::vec::Vec;

struct Population {
    sets: Settings,
    citizens: Vec<Genome>,
    species: Vec<Species>,
    best_fitness: f64,
    best_genome: Option<Genome>,
    next_species_id: u32,
    generations: u64,
}
