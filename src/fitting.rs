use polars::prelude::*;
use plotters::prelude::*;
use std::error::Error;

pub fn fit_and_plot(df: &DataFrame) -> Result<(), Box<dyn Error>> {
    // Extract columns
    let x_col = df.column("x")?.f64()?;
    let y_col = df.column("y")?.f64()?;

    let x: Vec<f64> = x_col.into_iter().filter_map(|opt| opt).collect();
    let y: Vec<f64> = y_col.into_iter().filter_map(|opt| opt).collect();

    let n = x.len() as f64;

    // Least-squares linear regression: y = m*x + b
    let sum_x: f64 = x.iter().sum();
    let sum_y: f64 = y.iter().sum();
    let sum_xy: f64 = x.iter().zip(y.iter()).map(|(xi, yi)| xi * yi).sum();
    let sum_x2: f64 = x.iter().map(|xi| xi * xi).sum();

    let m = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x * sum_x);
    let b = (sum_y - m * sum_x) / n;

    let y_pred: Vec<f64> = x.iter().map(|xi| m * xi + b).collect();

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
    for (&xi, &yi) in x.iter().zip(y.iter()) {
        chart.draw_series(std::iter::once(Circle::new((xi, yi), 4, RGBColor(137, 220, 235).filled())))?;
    }

    // Draw predicted line
    let line: Vec<(f64, f64)> = x.iter().zip(y_pred.iter()).map(|(&xi, &yi)| (xi, yi)).collect();

    chart.draw_series(LineSeries::new(line, RED.stroke_width(3)))?;

    println!("✅ Saved fit plot to fit.png");
    Ok(())
}
