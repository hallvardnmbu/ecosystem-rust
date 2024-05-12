use std::collections::HashMap;
use ordered_float::OrderedFloat;
use rand::prelude::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;
use crate::animals::Species::{Carnivore, Herbivore};
use super::animals::*;

pub const ALPHA: f32 = 0.1;
pub const V_MAX: u16 = 800;

pub struct Island<'a> {
    pub year: u16,
    pub geography: Vec<&'a [u8]>,

    pub cells: HashMap<(usize, usize), Cell>,
    pub inhabited: Vec<(usize, usize)>,

    pub rng: &'a mut ThreadRng,
}

impl Island<'_> {
    pub fn new<'a>(geography: Vec<&'a str>, rng: &'a mut ThreadRng) -> Island<'a> {

        // Change `geography` into vector of bytes, and check that edges are 'W'.
        let geography: Vec<&[u8]> = geography
            .iter()
            .map(|row| {
                let row = row.as_bytes();
                assert_eq!(*row.first().unwrap(), 'W' as u8, "Edges must be of 'W'");
                assert_eq!(*row.last().unwrap(), 'W' as u8, "Edges must be of 'W'");
                row
            } )
            .collect();
        if !geography.first().unwrap().iter().all(|&b| b == 'W' as u8) {
            panic!("Edges must be of 'W'!")
        }
        if !geography.last().unwrap().iter().all(|&b| b == 'W' as u8) {
            panic!("Edges must be of 'W'!")
        }

        let cells: HashMap<(usize, usize), Cell> = geography
            .iter().enumerate()
            .flat_map(|(i, row)| {
                row.iter().enumerate()
                    .map(move |(j, _el)| {
                        let f_max = match _el {
                            b'W' => 0u16,
                            b'H' => 300u16,
                            b'L' => 800u16,
                            b'M' => 0u16,
                            _ => panic!("Unknown terrain type!"),
                        };
                        ((i, j), Cell {
                            f_max,
                            fodder: f_max,
                            animals: HashMap::from([
                                (Herbivore, Vec::new()),
                                (Carnivore, Vec::new())
                            ])
                        })
                    })
            })
            .collect();
        let inhabited = Vec::new();

        Island {
            year: 0,
            geography, cells, inhabited,
            rng
        }
    }

    // Input: vec![((x, y), Species, n), ...]
    // Where (x, y) is the coordinate
    // and Species, n the Species and number of individuals.
    pub fn add_population(&mut self, population: Vec<((usize, usize), Species, u16)>) {
        for (coordinate, species, amount) in population {
            let cell = self.cells.get_mut(&coordinate).expect("Expected Cell.");
            for _ in 0..amount {
                let mut animal = Animal {
                    species: species,
                    age: 0,
                    weight: birthweight(species, self.rng),
                    fitness: 0.0,
                };
                animal.calculate_fitness();
                cell.animals.get_mut(&species).expect("Expected animals.").push(animal);
            }
        }
        self.update_inhabited();
    }

    fn procreate(&mut self) {
        let probability = HashMap::from([
            (Herbivore, Parameters::HERBIVORE.procreate),
            (Carnivore, Parameters::CARNIVORE.procreate),
        ]);

        for coordinate in self.inhabited.iter() {
            let mut babies = HashMap::from([
                (Herbivore, Vec::new()),
                (Carnivore, Vec::new()),
            ]);
            let cell = self.cells
                .get_mut(coordinate)
                .expect("Expected Cell.");
            let species_keys: Vec<_> = cell.animals.keys().cloned().collect();

            for species in species_keys {
                let factor: f32 = match species {
                    Herbivore => Parameters::HERBIVORE.gamma * cell.animals[&species].len() as f32,
                    Carnivore => Parameters::CARNIVORE.gamma * cell.animals[&species].len() as f32,
                };

                for animal in cell.animals.get_mut(&species).expect("Expected animal.") {
                    if animal.weight < probability[&species] {
                        continue
                    } else if self.rng.gen::<f32>() >= animal.fitness * factor {
                        continue
                    }
                    let babyweight = animal.birthweight(self.rng);
                    if !animal.lose_weight_birth(babyweight) {
                        continue
                    }
                    babies.get_mut(&species).expect("Expected babies.").push({
                        let mut baby = Animal {
                            species: species.clone(),
                            age: 0,
                            weight: babyweight,
                            fitness: 0.0
                        };
                        baby.calculate_fitness();
                        baby
                    })
                }
            }
            for (species, babes) in babies.iter_mut() {
                cell.animals
                    .get_mut(species).expect("Expected animals")
                    .append(babes);
            }
        }
    }

    fn feed(&mut self) {
        for coordinate in self.inhabited.iter() {
            let cell = self.cells.get_mut(coordinate).expect("Expected Cell");
            cell.grow_fodder();

            // Herbivores:
            let mut herbivores = cell.animals
                .get_mut(&Herbivore).expect("Expected Herbivores")
                .clone();
            herbivores.sort_by_key(|herbivore| OrderedFloat(herbivore.fitness));

            for herbivore in herbivores.iter_mut().rev() {
                cell.fodder -= herbivore.graze(cell.fodder);
            }

            // Carnivores:
            let carnivores = cell.animals
                .get_mut(&Carnivore).expect("Expected Carnivores");
            carnivores.shuffle(self.rng);
            for carnivore in carnivores.iter_mut() {
                let _ = carnivore.predation(self.rng, &mut herbivores);
            }

            cell.animals.insert(Herbivore, herbivores);
        }
    }

    fn migrate(&mut self) {
        let mut migrating: Vec<(usize, (usize, usize), Species)> = Vec::new();

        for coordinate in self.inhabited.iter() {
            let cell = self.cells.get_mut(coordinate).expect("Expected Cell");
            for (species, animals) in cell.animals.iter_mut() {
                let mu = match species {
                    Herbivore => Parameters::HERBIVORE.mu,
                    Carnivore => Parameters::CARNIVORE.mu,
                };
                for (idx, animal) in animals.iter_mut().enumerate() {
                    if self.rng.gen::<f32>() > mu * animal.fitness {
                        continue
                    }

                    migrating.push((idx, *coordinate, *species));
                }
            }
        }


        migrating.sort_by(|a, b| a.0.cmp(&b.0));
        for (idx, coordinate, species) in migrating.iter().rev() {
            let new_cell = self.new_cell(&coordinate, &species);
            match new_cell {
                None => continue,
                Some(new_coordinate) => {
                    let animal = self.cells.get_mut(&coordinate).unwrap().animals.get_mut(&species).unwrap().remove(*idx);
                    self.cells.get_mut(&new_coordinate).unwrap().animals.get_mut(&species).unwrap().push(animal);
                }
            }
        }
        self.update_inhabited();
        // migrating.sort_by(|a, b| a.0.cmp(&b.0));
        // println!("{:?}", migrating);
        // // for (idx, from_cell, to_cell) in migrating.iter().rev() {
        // //     let animal = self.cells[from_cell.0][from_cell.1].animals.0.remove(*idx);
        // //     self.cells[to_cell.0][to_cell.1].animals.0.push(animal);
        // // }
    }

    fn new_cell(&mut self, (x, y): &(usize, usize), species: &Species) -> Option<(usize, usize)> {

        let (stride, hunger) = match species {
            Herbivore => (Parameters::HERBIVORE.stride, Parameters::HERBIVORE.hunger),
            Carnivore => (Parameters::CARNIVORE.stride, Parameters::CARNIVORE.hunger),
        };

        let mut possibilities: Vec<(usize, usize)> = Vec::new();

        for i in x.checked_sub(stride).unwrap_or(0)..x.checked_add(stride).unwrap_or(0)+1 {
            for j in y.checked_sub(stride).unwrap_or(0)..y.checked_add(stride).unwrap_or(0)+1 {
                if (i, j) == (*x, *y) {
                    continue
                }
                if i <= 0
                    || i >= self.geography.len() as u8 as usize
                    || j <= 0
                    || j >= self.geography[0].len() as u8 as usize {
                    continue
                }
                if self.geography[i][j].eq(&b'W') {
                    continue
                }
                let diff_i = i.checked_sub(*x).unwrap_or(0);
                let diff_j = j.checked_sub(*y).unwrap_or(0);
                if diff_i.pow(2) + diff_j.pow(2) > stride.pow(2) {
                    continue
                }
                possibilities.push((i, j));
            }
        }

        let mut propensity: Vec<f32> = Vec::new();
        for idx in possibilities.iter() {
            let fodder = match species {
                Herbivore => {
                    self.cells[idx].fodder as f32
                },
                Carnivore => {
                    let mut fodder = 0.0f32;
                    for animal in self.cells
                        .get_mut(idx).expect("Expected Cell")
                        .animals.get_mut(&Herbivore).expect("Expected Herbivore") {
                        fodder += animal.weight;
                    }
                    fodder
                },
            };
            let population = self.cells[idx].animals[species].len() as u16;
            let abundance = fodder /
                ((population + 1) * hunger)
                    .max(population + 1)
                    .max(hunger)
                    .max(1) as f32;
            propensity.push(abundance);
        }
        if propensity.len() == 0 {
            return None
        }

        let propensity_sum: f32 = propensity.iter().sum();
        let new_pos_idx = ((self.rng.gen::<f32>() * propensity.len() as f32).ceil() - 1.0) as usize;

        let probability: f32;
        if propensity_sum == 0.0f32 {
            probability = 0.5;
        } else {
            probability = propensity[new_pos_idx] / propensity_sum;
        }

        if self.rng.gen::<f32>() < probability {
            return Some(possibilities[new_pos_idx])
        };
        return None
    }

    fn update_inhabited(&mut self) {
        self.inhabited.clear();
        for (coordinates, cell) in self.cells.iter_mut() {
            if cell.animals[&Herbivore].len() > 0
                || cell.animals[&Carnivore].len() > 0 {
                self.inhabited.push(*coordinates);
            }
        }
    }

    fn ageing(&mut self) {
        for coordinate in self.inhabited.iter() {
            for animals in self.cells
                .get_mut(coordinate).expect("Expected Cell")
                .animals.values_mut() {
                for animal in animals {
                    animal.aging();
                }
            }
        }
    }

    fn weight_loss(&mut self) {
        for coordinate in self.inhabited.iter() {
            for animals in self.cells.get_mut(coordinate).expect("Expected Cell").animals.values_mut() {
                for animal in animals {
                    animal.lose_weight_year();
                }
            }
        }
    }

    fn death(&mut self) {
        for coordinate in self.inhabited.iter() {
            // let cell = &mut self.cells.get_mut(coordinate).expect("Expected Cell");

            for (species, animals) in self.cells.get_mut(coordinate).expect("Expected Cell").animals.iter_mut() {
                animals.retain(|animal| {
                    let mut animal = animal.clone();
                    animal.calculate_fitness();
                    let omega = match species {
                        Herbivore => Parameters::HERBIVORE.omega,
                        Carnivore => Parameters::CARNIVORE.omega,
                    };
                    !(animal.weight <= 0.0f32 && self.rng.gen::<f32>() >= omega * (1.0f32 - animal.fitness))
                });
            }
        }
    }

    pub fn yearly_cycle(&mut self) {
        self.procreate();
        self.feed();
        self.migrate();
        self.ageing();
        self.weight_loss();
        self.death();

        self.year += 1;
    }

    // pub fn animals(&mut self) {
    //
    // }
}

// #[derive(Debug, Clone)]
#[derive(Debug)]
pub struct Cell {
    pub f_max: u16,
    pub fodder: u16,
    pub animals: HashMap<Species, Vec<Animal>>,
}

impl Cell {
    pub const ALPHA: f32 = 0.1;
    pub const V_MAX: u16 = 800;

    fn grow_fodder(&mut self) {
        if self.f_max == 0 {
            return
        }
        self.fodder = self.fodder.min(
            self.fodder + Cell::V_MAX * (
                1.0
                    - Cell::ALPHA
                    * (self.f_max - self.fodder) as f32
                    / self.f_max as f32
            ) as u16);
    }
}