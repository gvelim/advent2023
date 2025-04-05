# Advent of Code 2023 Solutions

This repository contains my solutions to the Advent of Code 2023 challenges, implemented in Rust. Each day presents unique algorithmic problems that explore different programming concepts and data structures.

## About Advent of Code

[Advent of Code](https://adventofcode.com/) is an annual programming event created by Eric Wastl. It consists of 25 daily programming puzzles released each December, designed to be solvable with any programming language. The puzzles typically increase in difficulty as the days progress.

## Project Structure

Solutions are organized by day in the `src/bin` directory. Each day contains:
- Source code for the solution
- A README explaining the problem and approach
- Input data (if applicable)

To run a specific day's solution:
```
cargo run --bin dayX
```
Where X is the day number (1-25).

## Puzzles and Key Learning Concepts

### Day 1 : [Digit Extraction](./src/bin/day1/README.md)
- Extract first and last digits from text lines including spelled-out numbers.
- _Key concepts: String parsing, regular expressions, iterators_

### Day 2 : [Cube Conundrum](./src/bin/day2/README.md)
- Determine the feasibility of games given bag constraints.
- _Key concepts: Data modeling, parsing, filtering_

### Day 3 : [Gear Ratios](./src/bin/day3/README.md)
- Identify numbers adjacent to symbols in a grid.
- _Key concepts: Grid processing, adjacency detection, 2D→1D mapping_

### Day 4 : [Scratchcards](./src/bin/day4/README.md)
- Calculate points from matching numbers and track card copies.
- _Key concepts: Set operations, hashmap-based counting_

### Day 5 : [If You Give A Seed A Fertilizer](./src/bin/day5/README.md)
- Transform values through a series of range mappings.
- _Key concepts: Range operations, transformation pipelines_

### Day 6: [Wait For It](./src/bin/day6/README.md)
- Determine winning strategies for a toy boat race.
- _Key concepts: Quadratic equations, mathematical optimization_

### Day 7: [Camel Cards](./src/bin/day7/README.md)
- Rank poker-like hands with custom rules.
- _Key concepts: Classification algorithms, custom sorting_

### Day 8: [Haunted Wasteland](./src/bin/day8/README.md)
- Navigate a network following left/right instructions.
- _Key concepts: Graph traversal, cycle detection, LCM_

### Day 9: [Mirage Maintenance](./src/bin/day9/README.md)
- Predict sequence values by analyzing differences.
- _Key concepts: Recursive algorithms, sequence analysis_

### Day 10: [Pipe Maze](./src/bin/day10/README.md)
- Find the longest path in a loop of connected pipes.
- _Key concepts: Graph traversal, flood fill, even-odd rule_

### Day 11: [Cosmic Expansion](./src/bin/day11/README.md)
- Calculate distances between points after space expansion.
- _Key concepts: Coordinate transformation, Manhattan distance_

### Day 12: [Hot Springs](./src/bin/day12/README.md)
- Count valid arrangements of springs with constraints.
- _Key concepts: Dynamic programming, memoization_

### Day 13: [Point of Incidence](./src/bin/day13/README.md)
- Find reflection lines in patterns with imperfections.
- _Key concepts: Pattern recognition, symmetry detection_

### Day 14: [Parabolic Reflector Dish](./src/bin/day14/README.md)
- Calculate load on a platform after tilting rocks.
- _Key concepts: Grid simulation, cycle detection_

### Day 15: [Lens Library](./src/bin/day15/README.md)
- Implement a hash-based focusing system.
- _Key concepts: Custom hash functions, linked lists_

### Day 16: [The Floor Will Be Lava](./src/bin/day16/README.md)
- Track beam paths through mirrors and splitters.
- _Key concepts: Direction-based simulation, cycle detection_

### Day 17: [Clumsy Crucible](./src/bin/day17/README.md)
- Find path with minimal heat loss with directional constraints.
- _Key concepts: Dijkstra's algorithm, state representation_

### Day 18: [Lavaduct Lagoon](./src/bin/day18/README.md)
- Calculate the area enclosed by a trench.
- _Key concepts: Polygon area calculation, Shoelace formula_

### Day 19: [Aplenty](./src/bin/day19/README.md)
- Process parts through a workflow of conditional rules.
- _Key concepts: Rule processing, range partitioning_

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (1.65 or newer recommended)

### Installation
1. Clone this repository
```
git clone https://github.com/yourusername/advent2023.git
cd advent2023
```

2. Build the project
```
cargo build
```

3. Run a specific day
```
cargo run --bin day1
```

## Acknowledgments

- Thanks to [Eric Wastl](http://was.tl/) for creating Advent of Code
- The Rust community for their excellent documentation and support

## License

This project is licensed under the MIT License - see the LICENSE file for details.
