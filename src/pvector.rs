const WIDTH: f64 = 640.0;
const HEIGHT: f64 = 480.0;

#[derive(Clone)]
pub struct PVector {
    pub x: f64,
    pub y: f64,
}

impl PVector {
    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn dist(&self, other: PVector) -> f64 {
        self.offset(other).len()
    }
    
    fn offset_x(self_x: f64, other_x: f64) -> f64 {
        let dist_x = other_x - self_x;
        // self_x < other_x => dist_x > 0
        if self_x < other_x && dist_x < WIDTH - dist_x {
            dist_x
        } else if self_x < other_x {
            dist_x - WIDTH
        } else if -WIDTH - dist_x < dist_x {
            dist_x
        } else {
            WIDTH + dist_x
        }
    }
    
    fn offset_y(self_y: f64, other_y: f64) -> f64 {
        let dist_y = other_y - self_y;
        
        if self_y < other_y && dist_y < HEIGHT - dist_y {
            dist_y
        } else if self_y < other_y {
            dist_y - HEIGHT
        } else if -HEIGHT - dist_y < dist_y {
            dist_y
        } else {
            HEIGHT + dist_y
        }
    }
    
    pub fn offset(&self, other: PVector) -> PVector {
        PVector {
            x: PVector::offset_x(self.x, other.x), 
            y: PVector::offset_y(self.y, other.y) 
        }
    }
    
    pub fn find_near(&self, others: Vec<PVector>, r: f64) -> Vec<PVector> {
        let mut ret: Vec<PVector>= Vec::with_capacity(others.len());
        for other in others {
            if self.dist(other.clone()) < r {
                ret.push(other);
            }
        }
        ret
    }

    fn add(&self, other: PVector) -> PVector {
        PVector{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
    
    pub fn add_all(vectors: Vec<PVector>) -> PVector {
        let mut ret = PVector{ x: 0.0, y: 0.0 };
        for vec in vectors {
            ret = ret.add(vec);
        }
        ret
    }
    
    /*
    fn direction(&self) -> f64 {
            self.x.atan2(self.y)
    }
    */
    
    pub fn normalize(&self) -> PVector {
        let size = self.len();
        PVector{
            x: self.x / size,
            y: self.y / size,
        }
    }
    
    pub fn mult(&self, scalar: f64) -> PVector {
        PVector {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}
