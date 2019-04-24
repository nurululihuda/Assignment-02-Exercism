pub struct Triangle {
    a: u64,
    b: u64,
    c: u64,
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        // unimplemented!("Construct new Triangle from following sides: {:?}. Return None if the sides are invalid.", sides);
        let v = sides.to_vec();
        let t = Triangle{a: sides[0], b: sides[1], c: sides[2]};
        if v[0] > 0 && v[1] > 0 && v[2] > 0 && Triangle::is_triangle(&t) == true {
            Some(Triangle {a: sides[0], b: sides[1], c: sides[2]})
        } else {
            None
        }       
    }

    pub fn is_triangle(&self) -> bool {
        self.a + self.b > self.c && self.a + self.c > self.b && self.b + self.c > self.a
    }

    pub fn is_equilateral(&self) -> bool {
        // unimplemented!("Determine if the Triangle is equilateral.");
        self.a == self.b && self.b == self.c
    }

    pub fn is_scalene(&self) -> bool {
        // unimplemented!("Determine if the Triangle is scalene.");
        self.a != self.b && self.a != self.c && self.b != self.c
    }

    pub fn is_isosceles(&self) -> bool {
        // unimplemented!("Determine if the Triangle is isosceles.");        
        (self.a == self.b && self.b != self.c) || (self.a != self.b && self.a == self.c) || (self.a != self.b && self.b == self.c)
    }
}
