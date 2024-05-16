use std::collections::HashMap;
use rand::prelude::ThreadRng;
use super::animals::*;
use super::island::*;
use super::graphics::*;

pub struct Simulation<'a> {
    pub island: Island<'a>,
    pub graphics: Graphics,

    animals: HashMap<Species, Vec<u32>>,
    placement: HashMap<(usize, usize), HashMap<Species, Vec<u32>>>,
}

impl Simulation<'_> {
    pub fn new<'a>(geography: Vec<&'a str>, rng: &'a mut ThreadRng, path: &'static str) ->
                                                                                       Simulation<'a> {
        let isl = Island::new(geography, rng);
        let mut animals = HashMap::new();
        let mut placement = HashMap::new();

        for species in [Species::Herbivore, Species::Carnivore].iter() {
            animals.insert(*species, Vec::new());
        }

        for x in 0..isl.geography.len() {
            for y in 0..isl.geography[0].len() {
                let mut species = HashMap::new();
                for _species in [Species::Herbivore, Species::Carnivore].iter() {
                    species.insert(*_species, Vec::new());
                }
                placement.insert((x, y), species);
            }
        }

        Simulation {
            island: isl,
            graphics: Graphics { path },
            animals,
            placement,
        }
    }

    pub fn add_population(&mut self, population: Vec<((usize, usize), Species, u16)>) {
        self.island.add_population(population);
    }

    pub fn simulate(&mut self, years: u16, graph: bool) {

        self.metrics();

        for _ in 0..years {
            self.island.yearly_cycle();
            self.metrics();
        }

        if graph { self.graph(); }
    }

    fn metrics(&mut self) {
        let (n_species, n_cell) = self.island.animals();
        n_species.iter().for_each(|(species, n)| {
            self.animals
                .get_mut(species).expect("Species error.")
                .push(*n);
        });
        n_cell.iter().for_each(|((x, y), species)| {
            species.iter().for_each(|(species, n)| {
                self.placement
                    .get_mut(&(*x, *y)).expect("Placement error.")
                    .get_mut(species).expect("Species error.")
                    .push(*n);
            });
        });
    }

    pub fn reset(&mut self) {
        self.animals.clear();
        self.placement.iter_mut().for_each(|(_, species)| {
            species.clear();
        });
    }

    pub fn graph(&self) {
        self.graphics.graph(&self.animals);
    }
}
