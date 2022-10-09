pub struct Pet {
    name: String,
    age: i8,
    hunger: i8,
    fitness: i8,
    children: Vec<Pet>,
}

impl Pet {
    pub fn is_alive(&self) -> bool {
        self.hunger < 10 && self.fitness > 0 && self.age < 30
    }
    pub fn adopt_child(&mut self, child: Pet) {
        self.children.push(child);
    }
}
