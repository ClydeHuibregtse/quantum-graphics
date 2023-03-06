
mod field;
mod state;
mod plot;

use crate::field::{Field, C_};
use crate::plot::{plot1D, plot2D};
use std::f32::consts::PI;
use itertools_num::linspace;
use ndarray::{Array};

fn main() {
    let (x0, xf) = (-1.0, 1.0);
    let field = Field::init(xf);
    let N = 100;
    let axis = Array::from_vec(linspace(x0, xf, N).collect());

    let es = vec![PI.powi(2) / 8.0, 4.0 * PI.powi(2) / 8.0, 9.0 * PI.powi(2) / 8.0];
    for e in es {

        let res = field.solve_step_1D(Box::new(|x: f32| 0.0), e as f32);
        
        plot1D(&axis, &res, format!("plots/{}.png", e).as_str()).unwrap();
    }

    let res = field.solve_step_2D(Box::new(|x: f32| 0.0), 4.0 * PI.powi(2) / 8.0);

    plot2D(&axis, &res, format!("plots/2D.png").as_str()).unwrap();

}

#[cfg(test)]
mod tests {
    use crate::field::{Field, C_, normalize};
    use itertools_num::linspace;
    use ndarray::{Array, array, Array1, s};
    use num::complex::Complex;
    use std::f32::consts::PI;
    use crate::plot::plot1D;


    #[test]
    fn field_basics() {
        let field = Field::init(1.0);

        let res = field.solve_step_1D(Box::new(|x: f32| 0.0), 1.0);
        // field.plot1D(&res);
    }

    #[test]
    fn field_step() {
        let field = Field::init(2.0);
        
    }

    #[test]
    fn state_sim() {
    }

    #[test]
    fn normalize_trivial() {
        let h = 1.0;
        // empty arrays should throw bad options

        let a1: Array<Complex<f32>, _>= Array::zeros(10);
        assert!(normalize(&a1, h).is_none());

        let a2: Array<Complex<f32>, _>= Array::zeros((10, 10));
        assert!(normalize(&a2, h).is_none());

        let a3: Array<Complex<f32>, _>= Array::zeros((10, 10, 2));
        assert!(normalize(&a3, h).is_none());

    }

    #[test]
    fn normalize_single_value() {
        let h = 1.0;
        // Single value gets renormalized such that their integral is 1
        
        let mut a1: Array<Complex<f32>, _>= Array::zeros(10);
        a1[5] = C_(0.5, 0.0);
        assert_eq!(normalize(&a1, h).unwrap()[[5]], C_(1.0, 0.0));

        let mut a2: Array<Complex<f32>, _>= Array::zeros((10, 10));
        a2[[5,5]] = C_(0.5, 0.0);
        assert_eq!(normalize(&a2, h).unwrap()[[5,5]], C_(1.0, 0.0));

        let mut a3: Array<Complex<f32>, _>= Array::zeros((10, 10, 2));
        a3[[5,5,1]] = C_(0.5, 0.0);
        assert_eq!(normalize(&a3, h).unwrap()[[5,5,1]], C_(1.0, 0.0));

    }

    fn gaussian(x: Array1<f32>) -> Complex<f32>
    {
        let k = x.ndim() as f32;
        C_((2.0 * PI).powf(-k / 2.0) * (-0.5 * x.dot(&x) ).exp(), 0.0)
    }


    #[test]
    fn normalize_gaussian() {

        let N = 1000;
        let (x0, xf) = (-4.0, 4.0);
        let h = (xf - x0)/ N as f32;
        let axis = Array::from_vec(linspace(x0, xf, N).collect());

        // Univariate gaussian should renormalize to itself
        let g1 = axis.map(|z|  gaussian(array![*z]));

        let g1_norm = normalize(&g1, h).unwrap();
        let mse = (g1_norm - g1).map(|x| x.re.powi(2)).sum();

        assert!(mse < 1e-4);

        // Bivariate gaussian should renormalize to itself
        let N = 1000;
        let (x0, xf) = (-10.0, 10.0);
        let h = (xf - x0)/ N as f32;
        let axis = Array::from_vec(linspace(x0, xf, N).collect());
        let mut g2 = Array::zeros((N, N));
        for i in 0..N {
            for j in 0..N {
                g2[[i, j]] = gaussian(array![axis[i], axis[j]]);
            }
        }

        let g2_norm = normalize(&g2, h).unwrap();
        let mse = (&g2_norm - &g2).map(|x| x.re.powi(2)).sum();
        plot1D(&axis, &g2.slice_move(s![500, ..]), "plots/g2.png");
        plot1D(&axis, &g2_norm.slice_move(s![500, ..]), "plots/g2_norm.png");
        assert!(mse < 1e-4);

    }

}