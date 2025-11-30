# Telos 🦀

** Telos is a high-performance, modular simulation engine written in Rust.

Designed with a focus on **Data-Oriented Design** and strict architectural boundaries, this project serves two purposes: a functional game engine with a TUI (Terminal User Interface) and a headless environment for training Reinforcement Learning (RL) agents.

## Architectural Design (OOP Principles)

The project strictly follows the **Model-View-Controller (MVC)** pattern adapted for Rust's ownership model, ensuring decoupling and high cohesion.

### 1. Separation of Concerns
* **Model** : Contains pure data structures (`WorldState`, `Entity`, `Map`). It has no dependencies on the rendering logic or the game loop. It represents the "Truth" of the simulation.
* **Controller** : The `GameEngine` struct encapsulates all business logic. It validates actions, mutates the state, and enforces rules. It acts as a gatekeeper to the data.
* **View** : A rendering layer based on `ratatui`. It observes the Model through read-only interfaces and renders the state without modifying it.

### 2. Polymorphism & Traits
The UI system utilizes Rust traits to achieve polymorphism. The `MenuState` trait defines a contract for any game mode (`EditorMode`, `GameMode`) to render its interface. This allows the render loop to process different modes generically without knowing their concrete implementation (Dependency Inversion Principle).

### 3. State Machine Pattern
The application flow is controlled by a Finite State Machine (FSM) implemented in `ApplicationState`. It manages transitions between Menu, Editor, and Game states, ensuring predictable application behavior and preventing invalid states.

## 🌟 Key Features

* **Zero-Copy TUI:** Optimized rendering pipeline using `ratatui` for immediate visual feedback in any terminal environment.
* **Editor Mode:** A fully featured map editor utilizing the Command pattern for tool interactions (`BuildTool`).
* **AI System:** A modular AI subsystem based on Pathfinding algorithms (BFS/A*), capable of tactical decision-making (Move vs. Attack evaluation).
* **RL-Ready Architecture:** The `GameEngine` is decoupled from the UI, allowing for "headless" execution. This enables running thousands of parallel simulations for training ML agents (Gym-like interface).

## Tech Stack

* **Language:** Rust (2021 Edition) - Chosen for memory safety and zero-cost abstractions.
* **Rendering:** `ratatui`, `crossterm`.
* **Math:** `glam` (SIMD-optimized vector math).
* **Serialization:** `serde` (JSON persistence).
* **Error Handling:** `color-eyre` for robust error reporting.

## Installation & Usage

Ensure you have the **Rust Toolchain** installed.

```bash
# Clone the repository
git clone [https://github.com/roovman/Telos.git](https://github.com/roovman/Telos.git)
cd Telos

# Run in Release mode 
cargo run --release
```

## Controls

### Game Mode (Simulation)
* **Navigate:** Cursor.
* **1..4:** Select Tools (Select, Move, Attack, Skill).
* **T:** Next Phase (Passes turn to AI or ends the current phase).
* **Q:** Return to Main Menu.

### Editor Mode (Content Creation)
* **W:** Wall Tool.
* **F:** Floor Tool.
* **U:** Unit Spawner.
* **T, H, E, D:** Quick Edit attributes (Team, HP, Energy, Damage).
* **S:** Save Map to `map.json`.

## Future Roadmap: Reinforcement Learning

The engine is built to support a custom Gym environment for RL experiments:

* **Observation Space:** Tensor representation of the Map and Entity states.
* **Action Space:** Discrete set of valid moves and attacks.
* **Reward Function:** Customizable scoring based on survival time and enemies defeated.
* **Parallel Training:** Utilizing Rust's async capabilities to run batched simulations for rapid agent convergence.

## 📄 License

This project is open-sourced software licensed under the [MIT license](LICENSE).
Feel free to use, modify, and distribute this code for educational or research purposes.
---
*Developed by Roman, Kyiv Polytechnic Institute.*
