Day 2: Cube Conundrum

## Understanding the Problem

### Input
The Elf will reach into the bag, grab a handful of random cubes, show them to you, and then put them back in the bag. He'll do this a few times per game.

```
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
```

### Part 1
Sum the IDs of those games that are feasible given a bag contains only 12 red cubes, 13 green cubes, and 14 blue cubes.

Expected output:
```
Game 1
Game 2
Game 5
======
Sum  8
```

### Part 2
For each game to be feasible, what would be the lowest required number of cubes per color that could have been in the bag?

Expected output:
```
Game 1 - 48 <- numbers of red, green, and blue cubes multiplied together
Game 2 - 12
Game 3 - 1560
Game 4 - 630
Game 5 - 36
==============
Sum      2286
```

## Solution Approach

### Intuition
To solve this problem, we need to:
1. Parse the input to understand each game and its runs (the handfuls of cubes shown)
2. For Part 1: Check if each game is feasible with the given constraint
3. For Part 2: Find the minimum number of cubes needed for each game

### Step 1: Defining the Data Model
We'll use two primary structures:
- `Run`: Represents a handful of cubes (one observation)
- `Game`: Contains multiple runs and the game's ID

Let's start with the `Run` structure, which stores the count of each color:

```rust
#[derive(Debug, Default, PartialEq)]
pub struct Run {
    pub(crate) red: u32,
    pub(crate) green: u32,
    pub(crate) blue: u32
}
```

This design allows us to easily track and compare cube counts. The `Default` trait gives us a convenient way to initialize a `Run` with zero values.

### Step 2: Parsing Input
The input consists of games, each containing multiple runs. Parsing this structured input requires:

1. Splitting the game ID from its runs
2. Parsing each run into color-count pairs
3. Building the complete game structure

For parsing the runs, we implement `FromStr` for the `Run` structure:

```rust
impl FromStr for Run {
    type Err = RunError;

    /// convert " 3 blue, 4 red"," 1 red, 2 green, 6 blue", "2 green"
    /// to [(Blue,3),(Red,4)], etc
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // Implementation that handles color name and value parsing
        // ...
    }
}
```

The implementation splits the input by commas, extracts color names and values, and builds a `Run` structure.

### Step 3: Game Logic
Now we implement the two key operations:

1. Checking if a game is feasible (Part 1):
```rust
pub(crate) fn is_feasible(&self, run: &Run) -> bool {
    self.runs
        .iter()
        .all(|r| r.is_feasible(run))
}
```

2. Calculating the power of minimum cube set (Part 2):
```rust
pub(crate) fn power(&self) -> u32 {
    self.max.power()
}
```

Where `self.max` is a `Run` containing the maximum values seen for each color across all runs in a game.

### Step 4: Smart Solution Optimization
Notice how we compute the maximum values during game creation:

```rust
gsplit
    .next().unwrap()
    .split(';')
    .map(|run| run.parse::<Run>())
    .inspect(|run| {
        if let Ok(run) = run {
            red = max(red, run.red);
            blue = max(blue, run.blue);
            green = max(green, run.green);
        }
    })
    .collect::<Result<Rc<_>,_>>()
```

This approach calculates the maximum values for each color during parsing, avoiding a separate pass through the data later.

### Step 5: Error Handling
We defined a custom error type `RunError` to provide meaningful errors during parsing:

```rust
#[derive(Debug, PartialEq)]
pub enum RunError {
    InvalidColourValue,
    MissingColourValue,
    InvalidColourName,
    MissingColourName,
}
```

This makes debugging easier and provides clear feedback about what went wrong.

### Step 6: Main Program Structure
Finally, our main program ties everything together:

```rust
fn main() {
    let input = std::fs::read_to_string("src/bin/day2/input.txt").unwrap_or_else(|e| panic!("{e}"));
    let rref = Run { red: 12, blue: 14, green: 13 };

    let games = input
        .lines()
        .map(|game| game.parse::<Game>()
            .map_err(|e| panic!("{} -> {:?}",e,game))
            .unwrap()
        )
        .collect::<std::rc::Rc<_>>();

    // Part 1
    let sum = games.iter()
        .filter(|game| game.is_feasible(&rref))
        .map(|game| game.id)
        .sum::<u32>();

    // Part 2
    let sum = games.iter()
        .map(|game| game.power())
        .sum::<u32>();
}
```

## Key Design Decisions

1. **Using `Rc<[Run]>`**: We use reference counting with `Rc` to efficiently store and share the runs within a game, avoiding unnecessary copying.

2. **Separation of Concerns**: We separate the parsing logic (`FromStr` implementations) from the game logic (feasibility checking and power calculation).

3. **Iterative Processing**: Both parts leverage Rust's iterator patterns for clean, functional-style code.

4. **Error Handling Strategy**: Custom error types provide detailed information during parsing, making debugging easier.

By building our solution in these logical steps, we create a maintainable, efficient program that clearly addresses the puzzle requirements.
