

use num::complex::Complex;
use ndarray::*;
use itertools_num::linspace;

pub fn C_(alpha: f32, beta: f32) -> Complex<f32> {
    Complex::new(alpha, beta)
}

/// Integrate over the inteval of `y`, assuming fixed inter-point distance of `h`
pub fn integrate<T>(y: &Array<Complex<f32>, T>, h: f32) -> f32
where T: ndarray::Dimension {

    let integral: f32 = (y * h.powi(y.ndim() as i32)).sum().re ; // Note: is always real
    integral
}




/// Normalizes a complex array to be of unit integral
pub fn normalize<T>(y: &Array<Complex<f32>, T>, h: f32) -> Option<Array<Complex<f32>, T>>
where T: ndarray::Dimension
{
    let denom = integrate(y, h);

    if denom == 0.0 {
        None
    }
    else {
        Some(y / denom)
    }

}

#[derive(Debug)]
pub struct Field {
    xsize: f32,
    ysize: Option<f32>,
    zsize: Option<f32>,

    gran: usize
}

impl Field { 
    pub fn init(xsize: f32) -> Field {
        Field {
            xsize: xsize,
            ysize: None,
            zsize: None,

            gran: 100
        }
    }

    /// Implements the Numberov 1D linear differential equation algorithm
    /// # Arguments
    /// * `v` a function that when given a location `x: f32`, returns a potential
    ///   energy `v(x): f32`
    /// 
    /// # Theory
    /// Differential equations of the form:
    /// 
    /// y'' = -g(x)y(x)
    /// 
    /// can be solved via the relation:
    /// 
    /// y_n = 1 / (1 + h^2/12 * g(x_n)) * (2 * y_{n-1} * (1 - 5*h^2/12 * g(x_{n-1})) - y_{n-2} * (1 + h^2/12 * g(x_{n-2})))
    pub fn solve_step_1D(&self, v: Box<dyn Fn(f32) -> f32>, e: f32) -> Array1<Complex<f32>> {

        // differential offset
        let h = (2.0 * self.xsize) / self.gran as f32;
        let g = Box::new(|x| (e - v(x)));

        let mut out = Array::zeros(self.gran);
        let xaxis: Vec<f32> = linspace::<f32>(-self.xsize, self.xsize, self.gran).collect();
        for (xi, x) in xaxis.iter().enumerate(){

            if *x == self.xsize || *x == -self.xsize {
                // Boundary Condition
                out[[xi]] = C_(0.0, 0.0);
            }
            else if xi == 1 {
                out[[xi]] = C_(h, 0.0);
            }
            else {
                let x_1 = xaxis[xi - 1];
                let x_2 = xaxis[xi - 2];
                let y_1 = out[[xi - 1]];
                let y_2 = out[[xi - 2]];

                out[[xi]] = 
                    (1.0 + h.powi(2) / 12.0 * 2.0 * g(*x)).powi(-1)
                    * (
                        (2.0 * y_1) * (1.0 - 5.0 * h.powi(2) / 12.0 * 2.0 * g(x_1))
                        - (y_2 * (1.0 + h.powi(2) / 12.0 * 2.0 * g(x_2)))
                    )
                
            }
        }

        let out_star = out.map(|z| C_(z.re, -z.im));

        let A = integrate(&(&out_star * &out), h).sqrt();
        
        out / A
    }

}