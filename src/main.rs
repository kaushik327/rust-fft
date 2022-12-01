use std::i16;
use hound;
use plotly::{
    Plot, Scatter,
    layout::{Axis, Layout},
    common::{Title},
};
use num_complex::Complex;
use std::f64::consts::PI;

pub fn read_wav(filename: &str) -> Vec<i16> {
    let mut reader = hound::WavReader::open(filename).unwrap();
    let samples: Vec<i16> = reader.samples::<i16>().map(|x| x.unwrap()).collect();
    // println!("{:?}", samples);
    return samples;
}

pub fn display_samples(samples: Vec<i16>) {
    let mut plot = Plot::new();
    let time_scale = (0..samples.len()).map(|x| x as f32 / 44100.0).collect();
    let trace = Scatter::new(time_scale, samples);
    plot.add_trace(trace);
    let layout = Layout::new()
        .title(Title::new("Audio"))
        .x_axis(Axis::new().title(Title::new("Time (s)")))
        .y_axis(Axis::new().title(Title::new("Amplitude (i16)")));
    plot.set_layout(layout);
    plot.write_html("output/out.html");
    // Creates html file in output folder containing the graph.
    // Definitely switching to a different crate in the future.
}

pub fn i16_fft(x: Vec<i16>) -> Vec<i16> {
    let complex_vec = x.iter().map(|x| Complex::<f64>::new(*x as f64, 0.0)).collect();
    let fft = fft(complex_vec);
    return fft.iter().map(|x| x.re as i16).collect();
}

pub fn fft(x: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    // radix-2 Cooley-Tukey FFT; recursive algorithm, O(nlogn).
    // Assumes x.len() is a power of 2.
    let N = x.len();
    if N == 1 { return x.to_vec() }
    let mut x_even = Vec::new();
    let mut x_odd = Vec::new();
    for k in (0..N).step_by(2) {
        x_even.push(x[k]);
        x_odd.push(x[k+1]);
    }
    let X_even = fft(x_even);
    let X_odd = fft(x_odd);
    let mut ans = Vec::new();
    for k in 0..N/2 {
        let factor = Complex::<f64>::new(0.0, -2.0*PI*(k as f64)/(N as f64)).exp();
        ans.push(X_even[k] + factor * X_odd[k]);
    }
    for k in 0..N/2 {
        let factor = -Complex::<f64>::new(0.0, -2.0*PI*(k as f64)/(N as f64)).exp();
        ans.push(X_even[k] + factor * X_odd[k])
    }
    return ans;
}


fn main() {
    // let samples = read_wav("/home/vagrant/src/rust-fft/audio/clap.wav");
    // display_samples(samples);

    // Test data from https://bookdown.org/rdpeng/timeseriesbook/the-fast-fourier-transform-fft.html#example-a-simple-fft
    let vec = vec![-1.545448388, -0.528393243, -1.086758791, -0.000111512];
    let complex_vec = vec.iter().map(|x| Complex::<f64>::new(*x, 0.0)).collect();
    let fft = fft(complex_vec);
    println!("{:?}", fft);
}