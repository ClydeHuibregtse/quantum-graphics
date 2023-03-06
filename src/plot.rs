use ndarray::{Array1, Array2};
use num::complex::Complex;
use plotters::prelude::*;


pub fn plot1D(xaxis:&Array1<f32>, state: &Array1<Complex<f32>>, filename: &str) -> Result<(), Box<dyn std::error::Error>>{

    let root = BitMapBackend::new(filename, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let ymin = -3.0f32;
    let ymax = 3.0f32;

    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(xaxis[0]..xaxis[xaxis.len() - 1], ymin..ymax)?;

    chart
        .configure_mesh()
        .disable_mesh()
        .draw()?;
    
    chart
        .draw_series(LineSeries::new(
            (0..xaxis.len()).map(|xi| (xaxis[xi], state[xi].re)),
            &RED,
        ))?
        .label("Field")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}


fn pdf(x: f32, y: f32) -> f32 {
    const SDX: f32 = 0.1;
    const SDY: f32 = 0.1;
    const A: f32 = 5.0;
    let x = x as f32 / 10.0;
    let y = y as f32 / 10.0;
    A * (-x * x / 2.0 / SDX / SDX - y * y / 2.0 / SDY / SDY).exp()
}

// pub fn get_value_at_loc(state: x: f32, y: f32) -> Complex<f32> {


// }

pub fn plot2D(axis: &Array1<f32>, state: &Array2<Complex<f32>>, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(filename, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .build_cartesian_3d(axis[0]..axis[axis.len() - 1], -1.0..1.0f32, axis[0]..axis[axis.len() - 1])?;
    
    chart.with_projection(|mut p| {
        p.pitch = 0.5;
        p.scale = 0.7;
        p.into_matrix() // build the projection matrix
    });

    chart
        .configure_axes()
        .light_grid_style(BLACK.mix(0.15))
        .max_light_lines(3)
        .draw()?;

    chart.draw_series(
        SurfaceSeries::xoz(
            axis.iter().map(|x| *x ),
            axis.iter().map(|x| *x ),
            |x, y| {
                state[
                    [
                        axis.iter().position(|x_prime| *x_prime == x).unwrap(),
                        axis.iter().position(|y_prime| *y_prime == y).unwrap()
                    ]
                ].re
            },
        )
        .style_func(&|&v| {
            (&HSLColor(240.0 / 360.0 - 240.0 / 360.0 * v as f64, 1.0, 0.7)).into()
        }),
    )?;

    root.present()?;
    Ok(())
}