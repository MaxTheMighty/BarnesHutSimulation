
# Barnes-Hut Simulation

The Barnes-Hut simulation is an efficient algorithm for approximating N-body problems, reducing the computational complexity from O(n²) to O(n log n). This implementation, written entirely in Rust, showcases the power and performance of this approach.

## Features

- **Efficient N-body Simulation**: Utilizes the Barnes-Hut algorithm for fast approximation of gravitational interactions.
- **Rust Implementation**: Leverages Rust's performance and safety features for robust and efficient code.
- **Interactive Visualization**: Provides a real-time visual representation of the simulation.
- **Customizable Parameters**: Allows users to adjust simulation parameters for different scenarios.

## Demo

[![Barnes-Hut Simulation Demo](http://img.youtube.com/vi/k-Y4igthvrI/0.jpg)](https://youtu.be/k-Y4igthvrI)

Click the image above to watch the simulation in action!

## Quick Start

1. Ensure you have Rust installed on your system.
2. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/barnes-hut-simulation.git
   cd barnes-hut-simulation
   ```
3. Run the simulation: 
   ```bash
   cargo run --bin draw_quadtree --release
   ```

## Controls
  ```
  P: Pause/Unpause the simulation
  Spacebar: Toggle tree visualization
  ```

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

## License
This project is licensed under the MIT License.

## Acknowledgements
- Josh Barnes and Piet Hut for their groundbreaking work on the Barnes-Hut algorithm.
- The Rust community for their excellent documentation and support.

