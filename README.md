# Trimmed Mean Calculator 

### A simple Rust command line program that calculates the trimmed mean. The program also outputs the arithmetic mean which serves for the user to see the differences between the two estimates of location. The program will output a YAML and a plot for the results.

##  Prereqs
    1. Rustc >= 1.65.0
    2. serde >= 1.0
    3. serde_yaml >= 0.9.21
    4. plotters >= 0.3.3
    5. chrono >= 0.4.26

## Installation
1. Clone the repo
2. cd into the clonned repo

## Running the program
1. Run ```cargo run``` (be patient) in the program dir
2. Give a single value, press enter to supply the next one
3. Press enter on an empty line to finish supplying values
4. Give the number of the values you want to remove, and press enter
5. The program will supply the trimmed mean, the yaml with the io-s, and the plot for the trimmed mean

## Happy Rusting
Should you need my help to run the program or want to know more about it, let me know: shpendaliu@pm.me

### Final note
The program may be buggy, because this is my first ever Rust program. I will extend this in the future because it has potential, especially when learning about Rust multithreaded programming.