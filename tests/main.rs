use virtual_pet::Pet;

#[test]
fn pet_gets_a_name() {
    let test_pet = Pet::new("Testy".to_string());
    assert_eq!(test_pet.name, "Testy")
}

#[test]
fn pet_is_alive() {
    let test_pet = Pet::new("Testy".to_string());

    assert_eq!(true, test_pet.is_alive());
}

#[test]
fn pet_can_die() {
    let mut test_pet = Pet::new("Testy".to_string());
    test_pet.grow_up();
    test_pet.grow_up();
    test_pet.grow_up();
    test_pet.grow_up();

    assert_eq!(test_pet.is_alive(), false)
}

#[test]
fn pet_fitness_increases_when_not_max() {
    let mut test_pet = Pet::new("Testy".to_string());
    test_pet.grow_up();
    test_pet.walk();

    assert_eq!(test_pet.fitness, 10);
}

#[test]
fn pet_hunger_decreases_when_fed() {
    let mut test_pet = Pet::new("Testy".to_string());
    test_pet.grow_up();
    test_pet.feed();
    assert_eq!(test_pet.hunger, 0)
}

#[test]
fn pet_grows_up(){
    let mut test_pet = Pet::new("Testy".to_string());
    test_pet.grow_up();
    test_pet.grow_up();

    assert_eq!(test_pet.age, 2);
    assert_eq!(test_pet.hunger, 6);
    assert_eq!(test_pet.fitness, 4);
}
