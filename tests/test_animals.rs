#[cfg(test)]
mod tests {
    use crate::animals;
    use rand::rngs::ThreadRng;
    use rand::thread_rng;

    #[test]
    fn test_animal_gain_weight() {
        let mut animal = animals::Animal {
            weight: 5.0,
            age: 0,
            fitness: None,
        };
        animal.gain_weight(5);
        assert_eq!(animal.weight, 10.0);
    }

    #[test]
    fn test_animal_aging() {
        let mut animal = animals::Animal {
            weight: 5.0,
            age: 0,
            fitness: None,
        };
        animal.aging();
        assert_eq!(animal.age, 1);
    }

    #[test]
    fn test_herbivore_birthweight() {
        let mut rng: ThreadRng = thread_rng();
        let birthweight = animals::Herbivore::birthweight(&mut rng);
        assert!(birthweight > 0.0);
    }

    #[test]
    fn test_herbivore_aging() {
        let mut herbivore = animals::Herbivore {
            animal: animals::Animal {
                weight: 5.0,
                age: 0,
                fitness: None,
            },
        };
        herbivore.aging();
        assert_eq!(herbivore.animal.age, 1);
    }

    #[test]
    fn test_carnivore_birthweight() {
        let mut rng: ThreadRng = thread_rng();
        let birthweight = animals::Carnivore::birthweight(&mut rng);
        assert!(birthweight > 0.0);
    }

    #[test]
    fn test_carnivore_aging() {
        let mut carnivore = animals::Carnivore {
            animal: animals::Animal {
                weight: 5.0,
                age: 0,
                fitness: None,
            },
        };
        carnivore.aging();
        assert_eq!(carnivore.animal.age, 1);
    }
}
