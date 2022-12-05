use std::i16;
use hound;
use num_complex::Complex;
use std::f32::consts::PI;
use charts::{Chart, ScaleLinear, MarkerType, PointLabelPosition, LineSeriesView};


pub fn read_wav(filename: &str) -> Vec<i16> {
    let mut reader = hound::WavReader::open(filename).unwrap();
    let samples: Vec<i16> = reader.samples::<i16>().map(|x| x.unwrap()).collect();
    return samples;
}

pub fn display_samples(samples: Vec<i16>) {
    let width = 800;
    let height = 600;
    let (top, right, bottom, left) = (90, 40, 50, 60);

    let x = ScaleLinear::new()
        .set_domain(vec![0.0, samples.len() as f32])
        .set_range(vec![0, width - left - right]);

    let y = ScaleLinear::new()
        .set_domain(vec![i16::MIN as f32, i16::MAX as f32])
        .set_range(vec![height - top - bottom, 0]);

    let line_data = samples.iter()
                              .enumerate()
                              .map(|(x, y)| (x as f32, *y as f32))
                              .collect();

    // Create Line series view that is going to represent the data.
    let line_view = LineSeriesView::new()
        .set_x_scale(&x)
        .set_y_scale(&y)
        .set_marker_type(MarkerType::Circle)
        .set_label_position(PointLabelPosition::N)
        .load_data(&line_data).unwrap();

    // Generate and save the chart.
    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_title(String::from("Audio"))
        .add_view(&line_view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label("Time")
        .add_bottom_axis_label("Amplitude")
        .save("/home/vagrant/src/rust-fft/output/scatter-chart.svg").unwrap();
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