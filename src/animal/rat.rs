use pvector::PVector;
use animal::Animal;

impl Animal {
    pub fn new_rat() -> Animal{
        let mut ret = Animal::new();
        ret.velocity = 0.5;
        ret
    }
    
    pub fn next_states_rats(cats: &Vec<Animal>, rats: &Vec<Animal>) -> Vec<Animal> {
        let mut ret = Animal::eat_rats(cats, rats);
        ret = ret
            .into_iter()
            .map(|rat| rat.run_away(cats))
            .collect();
        Animal::life_manage(&ret)
    }
    
    pub fn after_eat(&self, rats: Vec<Animal>) -> Vec<Animal> {
     rats
            .into_iter()
            .filter(|rat| !self.is_within(rat, 1.0))
            .collect()
    }
    
    fn run_away(&self, preyers: &Vec<Animal>) -> Animal {
        let next_velocity = self
            .as_velocity()
            .add(self.run_away_vector(preyers))
            .normalize()
            .mult(self.velocity);
        self
            .apply_velocity(next_velocity)
            .move_self()
    }
    
    fn run_away_vector(&self, preyers: &Vec<Animal>) -> PVector {
        let near_preyer = self.collect_near_pvectors(preyers, 10.0);
        
        if near_preyer.len() <= 0 {
            return PVector::zero();
        }
        
        self
            .calculate_direction(near_preyer)
    }
}
