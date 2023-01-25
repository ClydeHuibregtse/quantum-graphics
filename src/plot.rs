use ndarray::Array1;
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