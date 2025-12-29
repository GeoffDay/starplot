
#![allow(unused)]

#[derive(Debug, Clone)]
struct Star {
  name: String,
  active: bool,
  pos_x: f64,
  pos_y: f64,
  pos_z: f64,
  vel_x: f64,
  vel_y: f64,
  vel_z: f64,
  mass: f64,
}


impl Star {
    fn distance(&self, other: &Star) -> f64 {
        ((self.pos_x - other.pos_x).powi(2) + (self.pos_y - other.pos_y).powi(2) + (self.pos_z - other.pos_z).powi(2)).sqrt()
    }

    fn force(&self, other: &Star) -> f64 {
         // F = Gravity * (mass1 * mass2) / distance^2
        ((self.mass * other.mass) * GRAVITY) / Star::distance(&self, &other)
    }
    
    fn disx(&self, other: &Star) -> f64 {
        self.pos_x - other.pos_x
    }
    
    fn disy(&self, other: &Star) -> f64 {
        self.pos_y - other.pos_y
    }
    
    fn disz(&self, other: &Star) -> f64 {
        self.pos_z - other.pos_z
    }
}

const GRAVITY: f64 = 9.803;

fn main() {
    
    let mut star_vec_a: Vec<Star> = Vec::new();  // Create an empty vector of Stars

        star_vec_a.push(Star {name: String::from("Fred"), active: true, pos_x: 10.0, pos_y: 50.0, pos_z: -25.0, vel_x: 5.0, vel_y: 3.0, vel_z: 10.0, mass: 10.0});
        star_vec_a.push(Star {name: String::from("Wilma"), active: true, pos_x: -100.0, pos_y: 50.0, pos_z: -5.0, vel_x: -5.0, vel_y: -4.0, vel_z: 2.0, mass: 31.0});
        star_vec_a.push(Star {name: String::from("Barny"), active: true, pos_x: 50.0, pos_y: -10.0, pos_z: 25.0, vel_x: 5.0, vel_y: -3.0, vel_z: -7.0, mass: 20.0});
        star_vec_a.push(Star {name: String::from("Betty"), active: true, pos_x: -25.0, pos_y: 50.0, pos_z: -25.0, vel_x: 5.0, vel_y: 3.0, vel_z: 10.0, mass: 10.0});
        star_vec_a.push(Star {name: String::from("BammBamm"), active: true, pos_x: 114.0, pos_y: 50.0, pos_z: -2.0, vel_x: 5.0, vel_y: 3.0, vel_z: 10.0, mass: 3.0});
        star_vec_a.push(Star {name: String::from("Pebbles"), active: true, pos_x: 30.0, pos_y: 30.0, pos_z: 25.0, vel_x: -5.0, vel_y: 3.0, vel_z: 10.0, mass: 8.0});

    let mut star_vec_b: Vec<Star> = star_vec_a.clone();             // make a copy of a which will take changes
    
let sliced_star_vec = &star_vec_a[..];
let mut other_star_vec = &mut star_vec_b[..];
let mut force:f64 = 0.0;

    for (i, j) in (0..sliced_star_vec.len()).flat_map(|i| (0..i).map(move |j| (i,j))) {
        // dbg!(i,j,&sliced_star_vec_a[i].name, &sliced_star_vec_a[j].name);
        // dbg!(Star::distance(&sliced_star_vec_a[i], &sliced_star_vec_a[j])); 
        // dbg!(Star::force(&sliced_star_vec_a[i], &sliced_star_vec_a[j])); 
        let mut force_i:f64 = Star::force(&sliced_star_vec[i], &sliced_star_vec[j]);
        let mut force_j:f64 = force_i.clone() / &sliced_star_vec[j].mass;
        force_i = force_i / &sliced_star_vec[i].mass;
        dbg!(force_i);
        dbg!(force_j);
        
        other_star_vec[i].vel_x = sliced_star_vec[i].vel_x - force_i * Star::disx(&sliced_star_vec[i], &sliced_star_vec[j]);
        other_star_vec[i].vel_y = sliced_star_vec[i].vel_y - force_i * Star::disy(&sliced_star_vec[i], &sliced_star_vec[j]);
        other_star_vec[i].vel_z = sliced_star_vec[i].vel_z - force_i * Star::disz(&sliced_star_vec[i], &sliced_star_vec[j]);
        
        other_star_vec[j].vel_x = sliced_star_vec[j].vel_x - force_j * Star::disx(&sliced_star_vec[i], &sliced_star_vec[j]);
        other_star_vec[j].vel_y = sliced_star_vec[j].vel_y - force_j * Star::disy(&sliced_star_vec[i], &sliced_star_vec[j]);
        other_star_vec[j].vel_z = sliced_star_vec[j].vel_z - force_j * Star::disz(&sliced_star_vec[i], &sliced_star_vec[j]);
        // star_vec_b[i].vel_x = star_vec_b[i].vel_x + force * sliced_star_vec_a[i].mass * 
    }
    
    
 }
   
