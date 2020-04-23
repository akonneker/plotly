use plotly::surface::{Lighting, PlaneContours, PlaneProject, SurfaceContours};
use plotly::{Layout, Plot, Surface};

fn spectral_surface_plot() {
    let mut x: Vec<f64> = Vec::new();
    let mut y: Vec<f64> = Vec::new();
    let mut z: Vec<Vec<f64>> = Vec::new();
    let n = 100;

    for i in 0..n {
        x.push(i as f64 / n as f64);
        y.push(i as f64 / n as f64);
    }

    for xi in 0..n {
        let mut iz: Vec<f64> = Vec::new();
        for yi in 0..n {
            let xf = (xi as f64) / n as f64;
            let yf = (yi as f64) / n as f64;

            let cz =
                (2.0 * xf * std::f64::consts::PI).sin() * (4.0 * yf * std::f64::consts::PI).cos();
            iz.push(cz);
        }
        z.push(iz);
    }

    let trace = Surface::new(z)
        .x(x)
        .y(y)
        .visible(true)
        .hide_surface(false)
        .lighting(Lighting::default())
        .contours(
            SurfaceContours::default().z(PlaneContours::default()
                .show(true)
                .use_colormap(true)
                .project(PlaneProject::default().z(true))),
        );
    let mut plot = Plot::default();
    plot.set_layout(Layout::default());
    plot.add_trace(trace);
    plot.show();
}

fn main() -> std::io::Result<()> {
    spectral_surface_plot();
    Ok(())
}
