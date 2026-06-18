# One Million Crabs - The Game

> *Explorers race frantically as they attempt to escape a meteor shower. Will they be able to meet their AI lover?*

## Overview

Welcome to One Million Crabs (OMC)'s galaxy visualizer! This repo houses the front-end of the 2025/26 Advanced 
Programming course project, built with Bevy. Explorers try to navigate the galaxy autonomously (or manually, if you 
think you can do better than our AI!) to avoid asteroids and generate as many resources as possible to generate even
more things. Who will survive the longest? Who will create the most resources? Who will make the most AI partners? 
Take it for a spin and find out!

![project screenshot](https://i.ibb.co/z35DJ4y/rsz-omc-screen.jpg)

## Run the project

The project is built in Rust and as such it can be run with Cargo. simply:

1. Clone the project
```bash
git clone https://github.com/Advance-Programming-2025/omc-gui.git
```
2. Build and run
```bash
cargo run
```

3. Optional - if you want to use Bevy's dynamic linking feature:
```bash
# reduce iterative compilation time, at the cost of a slower first build
cargo build --features dyn_link
cargo run
```

## Project overview

### The ECS system

Bevy is an Entity Component System (ECS). This means that the main components of the game are:
- Entities, each distinguished thing that lives inside of the game world
- Components, objects that represent a particular characteristic of an entity (its health, inventory, etc)
- Systems, functions that regulate the logic of the game world 

This system is highly optimized for a high number of entities, so performance is maintained even in large and densely
connected galaxies.

### Features

- **Galaxy setup** — Pick a topology file or generate a random galaxy with configurable planet count and sunray/asteroid ratio
- **Scalable galaxy** — Go big or go home - planets and explorers scale their size dynamically based on the amount of planets
- **Manual override** — Take direct control of any explorer's movement and actions
- **Nuke & Blind** — Destroy every planet at once or blanket the galaxy in sunrays
- **Dynamic menus** — Left panel for inspecting planets/explorers; right panel with Start/Pause, log output, and ratio controls
- **Sound effects** — Audio feedback for planet destruction and explorer movement
- **Game-over detection** — When all planets are destroyed, a summary screen appears

The guide for correctly structured topology files can be found in the README for [omc-galaxy](https://github.com/Advance-Programming-2025/omc-galaxy) 

### Event communication

- Game events occur on a fixed tick basis, like an RTS game
- Between each tick, the orchestrator pools all of the events on an event queue
- The event queue is then passed on to the main game function to trigger the necessary actions
- Events are handled in chunks thanks to observers, which allows for concurrent event handling

### Advanced Rust featuers

- Generic functions to reduce code duplication for shared logic
- Custom traits for components with similar behavior
- Closures for functional and reusable UI code

---

Built by Davide Da Col for the 2025/26 Advanced Programming course @ UniTN