use polars::prelude::*; //cargo add polars --features lazy,ndarray -> for dataframes
use plotters::prelude::*; //cargo add plotters -> for plotting
use std::error::Error;

/// Manual k-means clustering. Returns cluster assignment for each point.
fn kmeans(points: &[(f64, f64)], k: usize, max_iter: usize) -> Vec<usize> {
    let n = points.len();

    // Pick initial centroids from evenly spaced indices (deterministic)
    let mut centroids: Vec<(f64, f64)> = (0..k)
        .map(|i| points[i * n / k])
        .collect();

    let mut assignments = vec![0usize; n];

    for _ in 0..max_iter {
        let mut changed = false;

        // Assign each point to nearest centroid
        for (i, &(px, py)) in points.iter().enumerate() {
            let nearest = centroids.iter().enumerate()
                .min_by(|(_, (cx, cy)), (_, (cx2, cy2))| {
                    let d1 = (px - cx).powi(2) + (py - cy).powi(2);
                    let d2 = (px - cx2).powi(2) + (py - cy2).powi(2);
                    d1.partial_cmp(&d2).unwrap()
                })
                .unwrap().0;
            if assignments[i] != nearest {
                assignments[i] = nearest;
                changed = true;
            }
        }

        if !changed {
            break;
        }

        // Recompute centroids
        for c in 0..k {
            let (sum_x, sum_y, count) = points.iter().zip(assignments.iter())
                .filter(|(_, a)| **a == c)
                .fold((0.0, 0.0, 0usize), |(sx, sy, cnt), (&(px, py), _)| {
                    (sx + px, sy + py, cnt + 1)
                });
            if count > 0 {
                centroids[c] = (sum_x / count as f64, sum_y / count as f64);
            }
        }
    }

    assignments
}

pub fn plot_dataframe(df: &DataFrame) -> Result<(), Box<dyn Error>> {
    let x_col = df.column("x")?.f64()?;
    let y_col = df.column("y")?.f64()?;

    let points: Vec<(f64, f64)> = x_col.into_iter()
        .zip(y_col)
        .filter_map(|(ox, oy)| Some((ox?, oy?)))
        .collect();

    let assignments = kmeans(&points, 3, 100);

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
        .label_style(("sans-serif", 20).into_font().color(&text))
        .draw()?;

    for (&(x, y), &cluster) in points.iter().zip(assignments.iter()) {
        chart.draw_series(std::iter::once(Circle::new((x, y), 5, cluster_colours[cluster].filled())))?;
    }

    println!("✅ Saved to clusters.png");
    Ok(())
}
