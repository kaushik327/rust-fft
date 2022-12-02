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

pub fn fft(x: Vec<i16>) -> Vec<Complex<f64>> {
    // radix-2 Cooley-Tukey FFT; recursive algorithm.
    // Assumes x.len() is a power of 2.
    let length = x.len();
    if length == 1 {
        return vec![Complex::new(x[0] as f64, 0.0)];
    }
    let mut x_even = Vec::new();
    let mut x_odd = Vec::new();
    for k in (0..length).step_by(2) {
        x_even.push(x[k]);
        x_odd.push(x[k+1]);
    }
    let fft_even = fft(x_even);
    let fft_odd = fft(x_odd);
    let mut fft = Vec::new();
    fft.push(fft_even[0] + fft_odd[0]);
    for k in 1..length/2 {
        fft.push(fft_even[k] + fft_odd[k] * Complex::new(0.0, -2.0*PI*(k as f64)/(length as f64)).exp());
    }
    fft.push(fft_even[0] - fft_odd[0]);
    for k in (1..length/2).rev() {
        fft.push(fft[k].conj())
    }
    return fft;
}


fn main() {
    // let samples = read_wav("/home/vagrant/src/rust-fft/audio/clap.wav");
    // display_samples(samples);

    // https://scistatcalc.blogspot.com/2013/12/fft-calculator.html
    let fft = fft(vec![1, 6, 2, 7, 3, 4, 6, 1, 2, 7, 4, 9, 1, 8, 4, 2]);
    let formatted: Vec<Vec<f64>> = fft.iter().map(|x| vec![x.re, x.im]).collect();
    println!("{:?}", formatted);
    // Prints the vector of complex numbers like [real, imaginary]
}