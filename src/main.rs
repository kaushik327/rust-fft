use hound;
use num_complex::Complex;
use std::f64::consts::PI;
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;
use plotlib::style::{PointStyle, LineStyle};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;


pub fn read_wav(filename: &str) -> Vec<i16> {
    let full_filename = format!("{}/audio/{}.wav", std::env::current_dir().unwrap().display(), filename);
    println!("{}", full_filename);
    let mut reader = hound::WavReader::open(full_filename).unwrap();
    let samples: Vec<i16> = reader.samples::<i16>().map(|x| x.unwrap()).collect();
    return samples;
}

pub fn display_samples(samples: &Vec<i16>, filename: &str) {
    let data = samples.iter().enumerate().map(|(x, y)| ((x as f64)/44100.0, *y as f64)).collect();
    let graph: Plot = Plot::new(data)
        .point_style(
            PointStyle::new()
                .size(0.0),
        ).line_style(
            LineStyle::new()
                .colour("#ff0000")
        );
    let v = ContinuousView::new()
        .add(graph)
        .x_range(0.0, (samples.len() as f64)/44100.0)
        .y_range(i16::MIN as f64, i16::MAX as f64)
        .x_label("Time (s)")
        .y_label("Amplitude");
    Page::single(&v).save(filename).unwrap();
}

pub fn display_fft(fft: &Vec<Complex<f64>>, filename: &str) {
    let length = fft.len();

    let data = fft.iter()
                  .enumerate()
                  .map(|(x, y)| ((x as f64) * 44100.0 * 2.0 / (length as f64), y.norm() * 2.0 / (length as f64)))
                  .take(length / 4 + 1)
                  .collect();
    let graph: Plot = Plot::new(data)
        .point_style(
            PointStyle::new()
                .size(0.0),
        ).line_style(
            LineStyle::new()
                .colour("#ff0000")
        );
    let v = ContinuousView::new()
        .add(graph)
        .x_range(0.0, 44100.0 / 2.0)
        .y_range(0.0, i16::MAX as f64)
        .x_label("Frequency (Hz)")
        .y_label("Amplitude");
    Page::single(&v).save(filename).unwrap();
}

pub fn fft(x: &Vec<i16>) -> Vec<Complex<f64>> {
    // radix-2 Cooley-Tukey FFT; recursive algorithm.
    // Assumes x.len() is a power of 2.
    let length = x.len();
    if length == 1 { return vec![Complex::new(x[0] as f64, 0.0)]; }
    let mut x_even = Vec::new();
    let mut x_odd = Vec::new();
    for k in (0..length).step_by(2) {
        x_even.push(x[k]);
        x_odd.push(x[k+1]);
    }

    let (tx_even, rx_even): (Sender<Vec<Complex<f64>>>, Receiver<Vec<Complex<f64>>>) = mpsc::channel();
    thread::spawn(move || {
        tx_even.send(fft(&x_even)).unwrap();
    });
    let (tx_odd, rx_odd): (Sender<Vec<Complex<f64>>>, Receiver<Vec<Complex<f64>>>) = mpsc::channel();
    thread::spawn(move || {
        tx_odd.send(fft(&x_odd)).unwrap();
    });
    let fft_even = rx_even.recv().unwrap();
    let fft_odd = rx_odd.recv().unwrap();

    let mut fft = Vec::new();
    fft.push(fft_even[0] + fft_odd[0]);
    for k in 1..length/2 {
        fft.push(fft_even[k] + fft_odd[k] * Complex::new(0.0, -2.0*PI*(k as f64)/(length as f64)).exp());
    }
    fft.push(fft_even[0] - fft_odd[0]);
    for k in (1..length/2).rev() {
        fft.push(fft[k].conj());
    }
    return fft;
}

fn main() {
    println!("Add your .wav file to the /audio directory, and enter its filename (without extension) below. Press [enter] when done.");
    let mut filename = String::new();
    std::io::stdin().read_line(&mut filename).unwrap();
    filename.pop();


    let samples = read_wav(&filename)[..4096].to_vec();
    display_samples(&samples, "output/audio.svg");

    let fft = fft(&samples);
    display_fft(&fft, "output/fft.svg");

    // https://scistatcalc.blogspot.com/2013/12/fft-calculator.html
}