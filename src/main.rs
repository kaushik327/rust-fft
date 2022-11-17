use std::i16;
use hound;
use plotly::{Plot, Scatter};

pub fn read_wav(filename: &str) -> Vec<i16> {
    let mut reader = hound::WavReader::open(filename).unwrap();
    let samples: Vec<i16> = reader.samples::<i16>().map(|x| x.unwrap()).collect();
    // println!("{:?}", samples);
    return samples;
}

pub fn display_samples(samples: Vec<i16>) {
    let mut plot = Plot::new();
    let trace = Scatter::new((0..samples.len()).collect(), samples);
    plot.add_trace(trace);
    plot.write_html("output/out.html");
    // Creates html file in output folder containing the graph.
    // Definitely switching to a different crate in the future.
}


fn main() {
    println!("Hello, world!");
    let samples = read_wav("/home/vagrant/src/rust-fft/audio/clap.wav");
    display_samples(samples);
}