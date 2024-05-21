# Evolutionary Runner

Evolutionary Runner is a project that aims to simulate the process of evolution in a fun and interactive way. The project creates virtual creatures that learn to run and evolve based on how far they can go within a selected amount of time.

## Project Overview

The project is written in Rust and uses the SDL2 library for rendering. The creatures are complex structures made of joints and muscles that allow them to move. The creatures are controlled by neural networks that are trained using a genetic algorithm. The creatures that can go the furthest within the time limit will be considered the most successful and will pass their traits on to the next generation.

## Simulation Engine

The simulation engine, "`Evongin`", is responsible for running the simulation and displaying the creatures. The engine is built on top of the SDL2 library. It is a separate crate that can be used to create other simulations.

You can find the source code for the simulation engine [here](https://github.com/JoshuaVandaele/Evongin).

## Getting Started

### Prerequisites

- Rust - [Installation Guide](https://www.rust-lang.org/tools/install)
- SDL2 - [Installation Guide](https://github.com/Rust-SDL2/rust-sdl2#sdl20-development-libraries)

### Installation

- Clone the repository

```bash
git clone https://github.com/JoshuaVandaele/evolutionary-runner.git
```

- Navigate to the project directory

```bash
cd evolutionary-runner
```

- Run the project

```bash
cargo run --release
```

## Usage

After running the project, you will see the creatures start to move. The creatures that can go the furthest within the time limit will be considered the most successful and will pass their traits on to the next generation.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.
