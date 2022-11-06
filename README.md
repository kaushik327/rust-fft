# CS 128H Final Project: Fast Fourier Transforms with Rust


### Group Members

Vedang Bhargava (vedangb2), Jesse Lee (jessel3), Kaushik Varadharajan (kv22)

### Introduction

This Rust project will implement the Fast Fourier Transform and use it to visualize audio. The FFT is an incredibly important algorithm in computer science, math, and even music, so we thought it would be enlightening to recreate it ourselves. 

### Technical Overview

First, we will need to implement the FFT algorithm itself. This will probably involve calculations using multiple threads for efficiency. The end product should be a frequency-amplitude graph generated from an inputted time-amplitude graph. (This is our goal for checkpoint 1.)

Then, we can add the file input system. This will read .wav files from the project directory (or maybe a URL) and put them into the previously constructed FFT algorithm. Finally, we will work on output. The frequency-amplitude graph will be exported from the code as an image, or a video if we have the time to implement that. The end goal is an audio visualizer, just like one from a digital audio workstation. (Both of these are our goal for checkpoint 2.)

### Possible challenges

The FFT is a complex algorithm, so we will first need to understand the mathematics behind it to implement it in Rust. Also, it is a lot of calculation, so we will need to manage multithreading and sending data from thread to thread. Also, file input and output will have to be done with external crates, which will take time to learn.

### References

We got the project idea from the project instructions doc, as well as from numerous YouTube videos on the FFT.
