use std::i16;
use hound;
use num_complex::Complex;
use std::f32::consts::PI;
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;
use plotlib::style::{PointStyle, LineStyle};

pub fn read_wav(filename: &str) -> Vec<i16> {
    let mut reader = hound::WavReader::open(filename).unwrap();
    let samples: Vec<i16> = reader.samples::<i16>().map(|x| x.unwrap()).collect();
    return samples;
}

pub fn display_samples(samples: Vec<i16>) {
    let data = samples.iter().enumerate().map(|(x, y)| ((x as f64)/44100.0, *y as f64)).collect();

    // We create our scatter plot from the data
    let s1: Plot = Plot::new(data)
        .point_style(
            PointStyle::new()
                .size(0.0),
        ).line_style(
            LineStyle::new()
                .colour("#663399")
        ); // and a custom colour

    // The 'view' describes what set of data is drawn
    let v = ContinuousView::new()
        .add(s1)
        .x_range(0.0, (samples.len() as f64)/44100.0)
        .y_range(i16::MIN as f64, i16::MAX as f64)
        .x_label("Time")
        .y_label("Amplitude");

    // A page with a single view is then saved to an SVG file
    Page::single(&v).save("output/audio.svg").unwrap();
}

pub fn fft(x: Vec<i16>) -> Vec<Complex<f32>> {
    // radix-2 Cooley-Tukey FFT; recursive algorithm.
    // Assumes x.len() is a power of 2.
    let length = x.len();
    if length == 1 {
        return vec![Complex::new(x[0] as f32, 0.0)];
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
        fft.push(fft_even[k] + fft_odd[k] * Complex::new(0.0, -2.0*PI*(k as f32)/(length as f32)).exp());
    }
    fft.push(fft_even[0] - fft_odd[0]);
    for k in (1..length/2).rev() {
        fft.push(fft[k].conj());
    }
    return fft;
}


fn main() {
    let samples = read_wav("/home/vagrant/src/rust-fft/audio/clap.wav");

    display_samples(samples);

    // let fft = fft(vec![1, 6, 2, 7, 3, 4, 6, 1, 2, 7, 4, 9, 1, 8, 4, 2]);
    // let formatted: Vec<Vec<f32>> = fft.iter().map(|x| vec![x.re, x.im]).collect();
    // println!("{:?}", formatted);
    
    // Prints the vector of complex numbers like [real, imaginary]
    // https://scistatcalc.blogspot.com/2013/12/fft-calculator.html
}