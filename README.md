# Cube

The program implemented in this repository tries to brute-force all possible outcomes to the soma cube game:

[![Youtube Video Soma Cube](https://img.youtube.com/vi/gCYqQJPU5Tc/0.jpg)](https://www.youtube.com/watch?v=gCYqQJPU5Tc)

## Usage

1. Install the rustup toolchain
2. Clone this repository
3. Build and run the program:

    cargo run --release

4. See the output in the cubes.txt textfile

## Format of the output

- Each possibility to solve the cube is marked with a headline (# Cube) 
- The output is split into 3 layers (1.; 2.; 3.)
- Each of the seven parts has its own number 1-7
- The parts can span over multiple layers

Sample:
```
# Cube 0
1. 5 3 3  2. 3 3 2  3. 4 2 2 
   5 5 1     7 7 1     4 6 2 
   5 7 1     4 7 6     4 6 6
```
