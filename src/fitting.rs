use linfa::prelude::*;
use linfa_linear::LinearRegression; //cargo add linfa-linear -> for linear fits
use ndarray::{Array1, Array2};
use polars::prelude::*;
use plotters::prelude::*;
use std::error::Error;

pub fn fit_and_plot(df: &DataFrame) -> Result<(), Box<dyn Error>> {
    // Extract columns
    let x = df.column("x")?.f64()?;
    let y = df.column("y")?.f64()?;

    // Prepare ndarray input
    let features: Vec<f64> = x
        .into_iter()
        .filter_map(|opt| opt)
        .collect();
    let targets: Vec<f64> = y
        .into_iter()
        .filter_map(|opt| opt)
        .collect();

    let n = features.len();
    let x_array = Array2::from_shape_vec((n, 1), features)?;
    let y_array = Array1::from_vec(targets);

    // Train model
    let dataset = DatasetBase::new(x_array.view(), y_array.view());
    let model = LinearRegression::default().fit(&dataset)?;

    // Predict
    let y_pred = model.predict(&dataset);

    // Plot results

    // base colours
    let base = RGBColor(30, 30, 46);
    let text = RGBColor(205, 214, 244);
    let grid = RGBColor(108, 112, 134);

    let root = BitMapBackend::new("fit.png", (800, 600)).into_drawing_area();
    root.fill(&base)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Linear Fit", ("sans-serif", 30).into_font().color(&text))
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..10.0, 0.0..10.0)?;

    chart.configure_mesh()
    .axis_style(&text)
    .light_line_style(&grid)
    .label_style(("sans-serif", 20).into_font().color(&text))
    .draw()?;

    // Draw original points
    for (x, y) in x_array.column(0).iter().zip(y_array.iter()) {
        chart.draw_series(std::iter::once(Circle::new((*x, *y), 4, RGBColor(137, 220, 235).filled())))?;
    }

    // Draw predicted line
    let line: Vec<(f64, f64)> = x_array
        .column(0)
        .iter()
        .zip(y_pred.iter())
        .map(|(&x, &y)| (x, y))
        .collect();

    chart.draw_series(LineSeries::new(line, RED.stroke_width(3)))?;

    println!("✅ Saved fit plot to fit.png");
    Ok(())
}