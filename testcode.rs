// fn main() {
//     let array = [1, 2, 3, 4];

//     for (i, j) in (0..array.len()).flat_map(|i| (0..i).map(move |j| (i,j))) {
//         dbg!(i,j,array[i]*array[j]);
//     }
// }



#![allow(unused)]

#[derive(Debug, Clone)]
struct Star {
  name: String,
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
        ((self.mass * other.mass) * GRAVITY) / ((self.pos_x - other.pos_x).powi(2) + (self.pos_y - other.pos_y).powi(2) + (self.pos_z - other.pos_z).powi(2))
    }
}

const GRAVITY: f64 = 9.803;

fn main() {
    
    let mut star_vec: Vec<Star> = Vec::new();  // Create an empty vector of Stars

        star_vec.push(Star {name: String::from("Fred"), pos_x: 10.0, pos_y: 50.0, pos_z: -25.0, vel_x: 5.0, vel_y: 3.0, vel_z: 10.0, mass: 10.0});
        star_vec.push(Star {name: String::from("Wilma"), pos_x: -100.0, pos_y: 50.0, pos_z: -5.0, vel_x: -5.0, vel_y: -4.0, vel_z: 2.0, mass: 31.0});
        star_vec.push(Star {name: String::from("Barny"), pos_x: 50.0, pos_y: -10.0, pos_z: 25.0, vel_x: 5.0, vel_y: -3.0, vel_z: -7.0, mass: 20.0});
        star_vec.push(Star {name: String::from("Betty"), pos_x: -25.0, pos_y: 50.0, pos_z: -25.0, vel_x: 5.0, vel_y: 3.0, vel_z: 10.0, mass: 10.0});
        star_vec.push(Star {name: String::from("BammBamm"), pos_x: 114.0, pos_y: 50.0, pos_z: -2.0, vel_x: 5.0, vel_y: 3.0, vel_z: 10.0, mass: 3.0});
        star_vec.push(Star {name: String::from("Pebbles"), pos_x: 30.0, pos_y: 30.0, pos_z: 25.0, vel_x: -5.0, vel_y: 3.0, vel_z: 10.0, mass: 8.0});


let sliced_star_vec = &star_vec[..];
let other_star_vec = &star_vec[..];

    
    //         // Iterate over elements
    for n in sliced_star_vec {
         println!("first_star: {:?}", n.name);
    }
    
    for m in other_star_vec {
         println!("second_star: {:?}", m.name);
    }
    

    for (i, j) in (0..sliced_star_vec.len()).flat_map(|i| (0..i).map(move |j| (i,j))) {
        dbg!(i,j,&sliced_star_vec[i].name, &other_star_vec[j].name);
        dbg!(Star::distance(&sliced_star_vec[i], &other_star_vec[j])); 
        dbg!(Star::force(&sliced_star_vec[i], &other_star_vec[j])); 
                dbg!(Star::force(&sliced_star_vec[i], &sliced_star_vec[j]));
    }
    
    
 }
   
