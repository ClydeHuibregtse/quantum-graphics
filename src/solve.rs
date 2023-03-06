

use crate::field::{Field, C_};
use num::complex::Complex;

pub struct State {
    fields: Vec<Field>,
}

impl State {

    // pub fn new() -> State {
    //     State {
    //         fields: vec![Field::init(2)],
    //         H: Matrix::new(2, 2, vec![C_(0.0, 0.0), C_(1.0, 0.0), C_(1.0, 0.0), C_(0.0, 0.0)])
    //     }
    // }

    // fn _sim_step(&mut self) {

    //     let field = &self.fields.last().unwrap();
    //     self.fields.push(field.step(&self.H));
    // }

    // pub fn sim(&mut self) -> &Vec<Field>{
    //     for iter in 0..100 {
    //         self._sim_step();
    //     }
    //     &self.fields
    // }
}