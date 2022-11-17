use std::i16;
use hound;
use plotly::{
    Plot, Scatter,
    layout::{Axis, Layout},
    common::{Title},
};

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


fn main() {
    let samples = read_wav("/home/vagrant/src/rust-fft/audio/clap.wav");
    display_samples(samples);
}