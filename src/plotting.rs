use polars::prelude::*; //cargo add polars --features lazy,ndarray -> for dataframes
use linfa::prelude::*; //cargo add linfa -> for scientific operations
use linfa_clustering::KMeans; //cargo add linfa-clustering -> for KMeans
use ndarray::{Array1, Array2}; //cargo add ndarray@0.15 -> linfa breaks with 0.16
use plotters::prelude::*; //cargo add plotters -> for plotting
use std::error::Error;

pub fn plot_dataframe(df: &DataFrame) -> Result<(), Box<dyn Error>> {
    // Convert columns to flat Vec<f64>
    let x = df.column("x")?.f64()?; //get data from x column
    let y = df.column("y")?.f64()?; //get data from y colunn

    let flat: Vec<f64> = x.into_iter()
        .zip(y)
        .filter_map(|(ox, oy)| Some((ox?, oy?)))
        .flat_map(|(x, y)| vec![x, y])
        .collect();

    let n = flat.len() / 2;
    let records: Array2<f64> = Array2::from_shape_vec((n, 2), flat)?;
    let targets: Array1<usize> = Array1::zeros(n);

    let dataset = DatasetBase::new(records.view(), targets.view());

    let model = KMeans::params(3).fit(&dataset)?;
    let preds = model.predict(&dataset);

    // Plotting

    // base colours
    let base = RGBColor(30, 30, 46);
    let text = RGBColor(205, 214, 244);
    let grid = RGBColor(108, 112, 134);

    // cool colours for clusters
    let cluster_colours = [
        RGBColor(250, 179, 135), // Peach
        RGBColor(203, 166, 247), // Mauve
        RGBColor(137, 220, 235), // Sky
    ];

    let root = BitMapBackend::new("clusters.png", (800, 600)).into_drawing_area();
    root.fill(&base)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("KMeans Clustering", ("sans-serif", 42).into_font().color(&text))
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..10.0, 0.0..10.0)?;

    chart
        .configure_mesh()
        .axis_style(&text)
        .light_line_style(&grid)
        .label_style(("sans-serif", 20).into_font().color(&text)) //set axis font size here
        .draw()?;
    
    let xvals = df.column("x")?.f64()?.into_no_null_iter();
    let yvals = df.column("y")?.f64()?.into_no_null_iter();

    for ((x, y), &cluster) in xvals.zip(yvals).zip(preds.iter()) {
        chart.draw_series(std::iter::once(Circle::new((x, y), 5, cluster_colours[cluster].filled())))?;
    }

    println!("✅ Saved to clusters.png");
    Ok(())
}