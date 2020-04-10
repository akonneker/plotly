use plotly::{Candlestick, Plot};
use rand_distr::{Distribution, Normal};

fn geometric_brownian_motion(s_0: f64, dt: f64, n: usize, drift: f64, diffusion: f64) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let dist = Normal::new(0.0, 1.0).unwrap();
    let mut v = Vec::<f64>::with_capacity(n);
    v.push(s_0);
    let drift_factor = 1.0 + drift * dt;
    let diffusion_factor = diffusion * dt.sqrt();
    for idx in 1..n {
        let rv = drift_factor + diffusion_factor * dist.sample(&mut rng);
        let prod: f64 = rv * v[idx - 1];
        v.push(prod);
    }
    v
}

fn simple_candlestick_chart() {
    let x = vec![
        "2017-01-04",
        "2017-01-05",
        "2017-01-06",
        "2017-01-09",
        "2017-01-10",
        "2017-01-11",
        "2017-01-12",
        "2017-01-13",
        "2017-01-17",
        "2017-01-18",
        "2017-01-19",
        "2017-01-20",
        "2017-01-23",
        "2017-01-24",
        "2017-01-25",
        "2017-01-26",
        "2017-01-27",
        "2017-01-30",
        "2017-01-31",
        "2017-02-01",
        "2017-02-02",
        "2017-02-03",
        "2017-02-06",
        "2017-02-07",
        "2017-02-08",
        "2017-02-09",
        "2017-02-10",
        "2017-02-13",
        "2017-02-14",
        "2017-02-15",
    ];
    let open = vec![
        115.849_998, 115.919_998, 116.779_999, 117.949_997, 118.769_997, 118.739_998, 118.900_002,
        119.110_001, 118.339_996, 120.0, 119.400_002, 120.449_997, 120.0, 119.550_003, 120.419_998,
        121.669_998, 122.139_999, 120.93, 121.150_002, 127.029_999, 127.980_003, 128.309_998, 129.130_005,
        130.539_993, 131.350_006, 131.649_994, 132.460_007, 133.080_002, 133.470_001, 135.520_004,
    ];
    let high = vec![
        116.510_002, 116.860_001, 118.160_004, 119.43, 119.379_997, 119.93, 119.300_003, 119.620_003,
        120.239_998, 120.5, 120.089_996, 120.449_997, 120.809_998, 120.099_998, 122.099_998, 122.440_002,
        122.349_998, 121.629_997, 121.389_999, 130.490_005, 129.389_999, 129.190_002, 130.5, 132.089_996,
        132.220_001, 132.449_997, 132.940_002, 133.820_007, 135.089_996, 136.270_004,
    ];
    let low = vec![
        115.75, 115.809_998, 116.470_001, 117.940_002, 118.300_003, 118.599_998, 118.209_999, 118.809_998,
        118.220_001, 119.709_999, 119.370_003, 119.730_003, 119.769_997, 119.5, 120.279_999, 121.599_998,
        121.599_998, 120.660_004, 120.620_003, 127.010_002, 127.779_999, 128.160_004, 128.899_994,
        130.449_997, 131.220_001, 131.119_995, 132.050_003, 132.75, 133.25, 134.619_995,
    ];
    let close = vec![
        116.019_997, 116.610_001, 117.910_004, 118.989_998, 119.110_001, 119.75, 119.25, 119.040_001,
        120.0, 119.989_998, 119.779_999, 120.0, 120.080_002, 119.970_001, 121.879_997, 121.940_002,
        121.949_997, 121.629_997, 121.349_998, 128.75, 128.529_999, 129.080_002, 130.289_993, 131.529_999,
        132.039_993, 132.419_998, 132.119_995, 133.289_993, 135.020_004, 135.509_995,
    ];

    let trace1 = Candlestick::new(x, open, high, low, close);

    let mut plot = Plot::default();
    plot.add_trace(trace1);
    plot.show();
}

fn gbm_simple_candlestick_chart() {
    let n = 3_000;
    let x = (0..n).collect();
    let mid = geometric_brownian_motion(100.0, 1.0 / 365.0, n, 0.15, 0.5);
    let mut open = Vec::<f64>::with_capacity(n);
    let mut high = Vec::<f64>::with_capacity(n);
    let mut low = Vec::<f64>::with_capacity(n);
    let mut close = Vec::<f64>::with_capacity(n);
    for mid_point in mid.iter() {
        let up: bool = rand::random();
        if up {
            open.push(0.98 * *mid_point);
            close.push(1.02 * *mid_point);
        } else {
            open.push(1.02 * *mid_point);
            close.push(0.98 * *mid_point);
        }

        low.push(0.92 * *mid_point);
        high.push(1.09 * *mid_point);
    }

    let trace = Candlestick::new(x, open, high, low, close);
    let mut plot = Plot::default();
    plot.add_trace(trace);
    plot.show();
}

fn main() -> std::io::Result<()> {
    simple_candlestick_chart();
    gbm_simple_candlestick_chart();
    Ok(())
}
