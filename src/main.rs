use std::f32::consts::PI;
use std::i16;
use hound;

pub fn read_wav(filename: &str) {
    let mut reader = hound::WavReader::open(filename).unwrap();
    
    let samples: Vec<i16> = reader.samples::<i16>().map(|x| x.unwrap()).collect();
    
    println!("{:?}", samples)
    return samples;
}


fn main() {
    println!("Hello, world!");
    read_wav("/home/vagrant/src/rust-fft/audio/clap.wav");
}