# One Million Crabs - The Game

> *Explorers race frantically as they attempt to escape a meteor shower. Will they be able to meet their AI lover?*

## Overview

Welcome to One Million Crabs (OMC)'s galaxy visualizer! This repo houses the front-end of the 2025/26 Advanced 
Programming course project, built with Bevy. Explorers try to navigate the galaxy autonomously (or manually, if you 
think you can do better than our AI!) to avoid asteroids and generate as many resources as possible to generate even
more things. Who will survive the longest? Who will create the most resources? Who will make the most AI partners? 
Take it for a spin and find out!

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

### Event communication

- Game events occur on a fixed tick basis, like an RTS game
- Between each tick, the orchestrator pools all of the events on an event queue
- The event queue is then passed on to the main game function to trigger the necessary actions
- Events are handled in chunks thanks to observers, which allows for concurrent event handling

### Advanced rust featuers

- Generic functions to reduce code duplication for shared logic
- Custom traits for components with similar behavior
- Closures for functional and reusable UI code

---

Built by Davide Da Col for the 2025/26 Advanced Programming course @ UniTN