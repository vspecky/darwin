use crate::connection::Connection;
use crate::genome::Genome;
use crate::history::History;
use crate::node::Node;
use crate::settings::Settings;
use crate::species::Species;

use std::collections::HashMap;
use std::vec::{IntoIter, Vec};

struct Population {
    sets: Settings,
    species: Vec<Species>,
    hist: History,
    best_fitness: f64,
    best_genome: Option<Genome>,
    next_species_id: u32,
    generations: u64,
}

impl Population {
    pub fn new(sets: Settings) -> Self {
        let inputs = sets.inputs;
        let outputs = sets.outputs;
        let pop_size = sets.pop_size;
        let mut pop = Self {
            sets: sets,
            species: Vec::new(),
            hist: History::new(inputs, outputs),
            best_fitness: 0.,
            best_genome: None,
            next_species_id: 1,
            generations: 0,
        };

        pop.reset();

        pop
    }

    fn reset(&mut self) {
        self.species.clear();
        self.best_fitness = 0.;
        self.best_genome = None;
        self.next_species_id = 1;
        self.generations = 0;
        self.hist = History::new(self.sets.inputs, self.sets.outputs);

        let mut genomes = Vec::<Genome>::with_capacity(self.sets.pop_size as usize);

        for _ in 0..self.sets.pop_size {
            let genome = Genome::new(self.sets.inputs, self.sets.outputs, false);
            genomes.push(genome);
        }

        self.speciate_population(genomes);
    }

    pub fn next_generation(&mut self) -> IntoIter<&mut Genome> {
        self.kill_bad_species();
        self.species.iter_mut().for_each(|s| s.fitness_sharing());

        let total_avg_fitness = self.species.iter().fold(0., |acc, s| acc + s.avg_fitness);

        let pop_size = self.sets.pop_size;

        self.species.iter_mut().for_each(|s| {
            s.assigned_offspring = (s.avg_fitness / total_avg_fitness * pop_size as f64) as usize;
            s.cull_lower_half();
        });

        self.species.retain(|s| s.assigned_offspring > 0);

        let mut progeny = Vec::<Genome>::with_capacity(pop_size as usize);

        for species in &self.species {
            let mut new_offspring = species.assigned_offspring;

            if species.genomes.len() > 3 {
                let mut champ = species.genomes[0].clone();
                champ.fitness = 0.;
                progeny.push(champ);
                new_offspring -= 1;
            }

            for mut child in species.produce_offspring(new_offspring, &self.sets) {
                child.mutate(&mut self.hist, &self.sets);
                progeny.push(child);
            }
        }

        self.species.sort_unstable_by(|a, b| {
            b.genomes[0]
                .fitness
                .partial_cmp(&a.genomes[0].fitness)
                .unwrap()
        });

        let this_champ = self.species[0].genomes[0].clone();

        if this_champ.fitness > self.best_fitness {
            self.best_fitness = this_champ.fitness;
            self.best_genome = Some(this_champ.clone());
        }

        if progeny.len() < pop_size as usize {
            while progeny.len() < pop_size as usize {
                let mut another_child = this_champ.clone();
                another_child.mutate(&mut self.hist, &self.sets);
                progeny.push(another_child);
            }
        }

        self.species.iter_mut().for_each(|s| s.set_representative());
        self.speciate_population(progeny);

        self.get_citizens()
    }

    fn kill_bad_species(&mut self) {
        self.species.retain(|s| s.genomes.len() > 0);
        self.species.iter_mut().for_each(|s| {
            s.sort_genomes();
            s.update_stagnancy();
        });

        let allowed_stagnancy = self.sets.allowed_stagnancy;

        self.species.retain(|s| s.stagnancy < allowed_stagnancy);
    }

    pub fn get_citizens(&mut self) -> IntoIter<&mut Genome> {
        let mut vec = Vec::<&mut Genome>::new();

        for species in self.species.iter_mut() {
            vec.append(&mut species.genomes.iter_mut().collect::<Vec<&mut Genome>>());
        }

        vec.into_iter()
    }

    fn speciate_population(&mut self, pop: Vec<Genome>) {
        for species in &mut self.species {
            species.genomes.clear();
        }

        'outer: for genome in pop {
            for species in &mut self.species {
                if species.can_accomodate(&genome, &self.sets) {
                    species.add_genome(genome);
                    continue 'outer;
                }
            }

            let new_spec = Species::new(genome, self.next_species_id);
            self.next_species_id += 1;
            self.species.push(new_spec);
        }
    }
}
