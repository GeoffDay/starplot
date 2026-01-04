//! This code borrowed from the egui::plot example to implement custom gestures to pan and zoom in the plot
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(unused)]

use std::arch::x86_64;

use eframe::egui::{self, DragValue, Event, Vec2};
use egui::scroll_area::State;
use egui_plot::{Legend, Line, PlotPoints};

const GRAVITY: f64 = 6.674e-11; // Newtons Gravitational constant 6.674×10−11 m3⋅kg−1⋅s−2


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


fn main() -> Result<(), eframe::Error> {
   
    let mut star_vec_a: Vec<Star> = Vec::new();   	// Create an empty vector of Stars

        star_vec_a.push(Star {name: String::from("Fred"), active: true, pos_x: 10.0, pos_y: 50.0, pos_z: -25.0, vel_x: 5.0, vel_y: 3.0, vel_z: 10.0, mass: 10.0});
        star_vec_a.push(Star {name: String::from("Wilma"), active: true, pos_x: -100.0, pos_y: 50.0, pos_z: -5.0, vel_x: -5.0, vel_y: -4.0, vel_z: 2.0, mass: 31.0});
        star_vec_a.push(Star {name: String::from("Barny"), active: true, pos_x: 50.0, pos_y: -10.0, pos_z: 25.0, vel_x: 5.0, vel_y: -3.0, vel_z: -7.0, mass: 20.0});
        star_vec_a.push(Star {name: String::from("Betty"), active: true, pos_x: -25.0, pos_y: 50.0, pos_z: -25.0, vel_x: 5.0, vel_y: 3.0, vel_z: 10.0, mass: 10.0});
        star_vec_a.push(Star {name: String::from("BammBamm"), active: true, pos_x: 114.0, pos_y: 50.0, pos_z: -2.0, vel_x: 5.0, vel_y: 3.0, vel_z: 10.0, mass: 3.0});
        star_vec_a.push(Star {name: String::from("Pebbles"), active: true, pos_x: 30.0, pos_y: 30.0, pos_z: 25.0, vel_x: -5.0, vel_y: 3.0, vel_z: 10.0, mass: 8.0});
    
    let mut star_vec_b: Vec<Star> = star_vec_a.clone();             // make a copy of a which will take changes
    
	let sliced_star_vec = &star_vec_a[..];
	let other_star_vec = &star_vec_b[..];
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
        
        
        star_vec_b[i].vel_x = sliced_star_vec[i].vel_x - force_i * Star::disx(&sliced_star_vec[i], &sliced_star_vec[j]);
        star_vec_b[i].vel_y = sliced_star_vec[i].vel_y - force_i * Star::disy(&sliced_star_vec[i], &sliced_star_vec[j]);
        star_vec_b[i].vel_z = sliced_star_vec[i].vel_z - force_i * Star::disz(&sliced_star_vec[i], &sliced_star_vec[j]);
        // star_vec_b[i].vel_x = star_vec_b[i].vel_x + force * sliced_star_vec_a[i].mass * 
        star_vec_b[j].vel_x = sliced_star_vec[j].vel_x - force_j * Star::disx(&sliced_star_vec[i], &sliced_star_vec[j]);
        star_vec_b[j].vel_y = sliced_star_vec[j].vel_y - force_j * Star::disy(&sliced_star_vec[i], &sliced_star_vec[j]);
        star_vec_b[j].vel_z = sliced_star_vec[j].vel_z - force_j * Star::disz(&sliced_star_vec[i], &sliced_star_vec[j]);
    }
    
 
   
    

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Plot",
        options,
        Box::new(|_cc| Box::<PlotExample>::default()),
    )


}


struct PlotExample {
    lock_x: bool,
    lock_y: bool,
    ctrl_to_zoom: bool,
    shift_to_horizontal: bool,
    zoom_speed: f32,
    scroll_speed: f32,
    num_stars: u32,
}

impl Default for PlotExample {
    fn default() -> Self {
        Self {
            lock_x: false,
            lock_y: false,
            ctrl_to_zoom: false,
            shift_to_horizontal: false,
            zoom_speed: 1.0,
            scroll_speed: 1.0,
            num_stars: 10,
        }
    }
}

impl eframe::App for PlotExample {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::SidePanel::left("options").show(ctx, |ui| {
            ui.checkbox(&mut self.lock_x, "Lock x axis").on_hover_text("Check to keep the X axis fixed, i.e., pan and zoom will only affect the Y axisxx");
            ui.checkbox(&mut self.lock_y, "Lock y axis").on_hover_text("Check to keep the Y axis fixed, i.e., pan and zoom will only affect the X axis");
            ui.checkbox(&mut self.ctrl_to_zoom, "Ctrl to zoom").on_hover_text("If unchecked, the behavior of the Ctrl key is inverted compared to the default controls\ni.e., scrolling the mouse without pressing any keys zooms the plot");
            ui.checkbox(&mut self.shift_to_horizontal, "Shift for horizontal scroll").on_hover_text("If unchecked, the behavior of the shift key is inverted compared to the default controls\ni.e., hold to scroll vertically, release to scroll horizontally");
            ui.horizontal(|ui| {
                ui.add(
                    DragValue::new(&mut self.zoom_speed)
                        .clamp_range(0.1..=2.0)
                        .speed(0.1),
                );
                ui.label("Zoom speed").on_hover_text("How fast to zoom in and out with the mouse wheel");
            });
            ui.horizontal(|ui| {
                ui.add(
                    DragValue::new(&mut self.scroll_speed)
                        .clamp_range(0.1..=100.0)
                        .speed(0.1),
                );
                ui.label("Scroll speed").on_hover_text("How fast to pan with the mouse wheel");
            });
            ui.horizontal(|ui| {
                ui.add(
                    DragValue::new(&mut self.num_stars)
                        .clamp_range(2..=100)
                        .speed(0.1),
                );
                ui.label("Number of Stars").on_hover_text("How many stars do you want?");
            });
            ui.allocate_space(ui.available_size()); // put this LAST in your panel/window code
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let (scroll, pointer_down, modifiers) = ui.input(|i| {
                let scroll = i.events.iter().find_map(|e| match e {
                    Event::MouseWheel {
                        unit: _,
                        delta,
                        modifiers: _,
                    } => Some(*delta),
                    _ => None,
                });
                (scroll, i.pointer.primary_down(), i.modifiers)
            });

            ui.label("This example shows how to use raw input events to implement different plot controls than the ones egui provides by default, e.g., default to zooming instead of panning when the Ctrl key is not pressed, or controlling much it zooms with each mouse wheel step.");

            egui_plot::Plot::new("plot")
                .allow_zoom(false)
                .allow_drag(false)
                .allow_scroll(false)
                .legend(Legend::default())
                .show(ui, |plot_ui| {
                    if let Some(mut scroll) = scroll {
                        if modifiers.ctrl == self.ctrl_to_zoom {
                            scroll = Vec2::splat(scroll.x + scroll.y);
                            let mut zoom_factor = Vec2::from([
                                (scroll.x * self.zoom_speed / 10.0).exp(),
                                (scroll.y * self.zoom_speed / 10.0).exp(),
                            ]);
                            if self.lock_x {
                                zoom_factor.x = 1.0;
                            }
                            if self.lock_y {
                                zoom_factor.y = 1.0;
                            }
                            plot_ui.zoom_bounds_around_hovered(zoom_factor);
                        } else {
                            if modifiers.shift == self.shift_to_horizontal {
                                scroll = Vec2::new(scroll.y, scroll.x);
                            }
                            if self.lock_x {
                                scroll.x = 0.0;
                            }
                            if self.lock_y {
                                scroll.y = 0.0;
                            }
                            let delta_pos = self.scroll_speed * scroll;
                            plot_ui.translate_bounds(delta_pos);
                        }
                    }
                    if plot_ui.response().hovered() && pointer_down {
                        let mut pointer_translate = -plot_ui.pointer_coordinate_drag_delta();
                        if self.lock_x {
                            pointer_translate.x = 0.0;
                        }
                        if self.lock_y {
                            pointer_translate.y = 0.0;
                        }
                        plot_ui.translate_bounds(pointer_translate);
                    }

                    let sine_points = PlotPoints::from_explicit_callback(|x| x.sin(), .., 5000);
                    plot_ui.line(Line::new(sine_points).name("Sine"));
                    let square_points = PlotPoints::from_explicit_callback(|x| x.cos(), .., 2500);
                    plot_ui.line(Line::new(square_points).name("Square"));

                });
        });
    }
}



    // // Create a vector with initial values using the vec! macro
    // let another_vec = vec![
    //     MyStruct { id: 1, name: String::from("Alice") },
    //     MyStruct { id: 2, name: String::from("Bob") },
    // ];
  // let star1 = Star { 
    //     name: String::from("fred"),
    //     pos_x: 100.0,
    //     pos_y: 50.0,
    //     pos_z: -25.0,
    //     vel_x: 5.0,
    //     vel_y: -5.0,
    //     vel_z: 3.0,
    //     mass: 10.0,
    // };
    
    //     let star2 = Star { 
    //     name: String::from("wilma"),
    //     pos_x: -100.0,
    //     pos_y: 50.0,
    //     pos_z: -25.0,
    //     vel_x: 5.0,
    //     vel_y: -5.0,
    //     vel_z: -4.0,
    //     mass: 101.0,
    // };
