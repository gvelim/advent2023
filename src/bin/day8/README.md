# Day 8

## Input
You are given a "map" on how to navigate the desert. The map contains
* a list of left/right instructions, and
* the rest of the document seem to describe some kind of network of labeled nodes

```
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
```
Assume we need to navigate from `AAA` to `ZZZ`. Starting with `AAA`, you need to look up the next element based on the next left/right instruction in your input. In this example, start with `AAA` and go right (R) by choosing the right element of `AAA`, `CCC`. Then, `L` means to choose the left element of `CCC`, `ZZZ`. By following the left/right instructions, you reach `ZZZ` in `2` steps

## Part 1
Starting at `AAA`, follow the left/right instructions. How many steps are required to reach `ZZZ`?

```
Input:

LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)

Output:

"BBB"
"AAA"
"BBB"
"AAA"
"BBB"
"ZZZ"

6 steps
```

## Part 2
Start concurently, at every node that ends with `A` and follow all of the paths at the same time until they all simultaneously end up at nodes that end with `Z`. How many steps are required to reach the place where all nodes are ending with `Z`?

```
Input:

LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)

Output:

["11A", "22A"]
["11B", "22B"]
["11Z", "22C"] <-- only one 'Z', carry on
["11B", "22Z"] <-- only one 'Z', carry on
["11Z", "22B"] <-- only one 'Z', carry on
["11B", "22C"]
["11Z", "22Z"] <-- Both nodes ending in 'Z'; finished

6 Steps
```

## Solution Approach

### Understanding the Problem

This puzzle challenges us to navigate through a network by following directional instructions. We need to build a data structure that efficiently represents the network and create a mechanism for traversing it according to the given rules.

### Step 1: Data Structure Design

First, we need a suitable representation for our network. A `HashMap` is perfect for this task as it provides O(1) lookup time for the next node based on the current node name:

```rust
pub(crate) struct Network {
    pub(crate) net: HashMap<Rc<str>, (Rc<str>, Rc<str>)>,
}
```

Each entry maps a node name to a tuple containing its left and right connections. Using `Rc<str>` instead of `String` allows for efficient memory management through reference counting, avoiding unnecessary string duplication.

### Step 2: Parsing the Input

The input consists of two parts: the navigation instructions and the network structure. We parse these separately:

```rust
fn parse(input: &str) -> (&str, Rc<Network>) {
    let mut split = input.split("\n\n");
    (
        split.next().unwrap(),  // Navigation instructions
        Rc::new(
            split.next().unwrap()
                .parse::<Network>()
                .unwrap_or_else(|e| panic!("{}",e))
        )
    )
}
```

The network parsing involves converting each line into a node entry:

```rust
impl FromStr for Network {
    // Convert lines like "AAA = (BBB, CCC)" into HashMap entries
    // where key is "AAA" and value is ("BBB", "CCC")
}
```

### Step 3: Creating a Network Iterator

To efficiently traverse the network, we implement a custom iterator that follows the left/right instructions:

```rust
pub(crate) struct NetworkIter<I> where I: Iterator<Item=char> {
    net: Rc<Network>,
    key: Rc<str>,       // Current node
    turns: I            // Direction iterator
}

impl<I> Iterator for NetworkIter<I> where I: Iterator<Item=char> {
    type Item = Rc<str>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.turns.next() {
            Some('L') => self.net.net.get(&self.key).map(|(l,_)| l.clone()),
            Some('R') => self.net.net.get(&self.key).map(|(_,r)| r.clone()),
            _ => unreachable!()
        }
        .inspect(|next| self.key = next.clone())
    }
}
```

This iterator takes the current position in the network and the next direction, then returns the next node while updating the current position.

### Step 4: Solving Part 1

For Part 1, we simply count the steps until we reach the target node 'ZZZ':

```rust
let steps = net.clone()
    .iter("AAA", turns.chars().cycle())
    .take_while(|node| !(node as &str).eq("ZZZ"))
    .count() + 1;
```

We use `.cycle()` to create an endless iterator of directions, allowing us to reuse the instruction set as needed.

### Step 5: Solving Part 2 - The Cycle Detection Insight

Part 2 presents a more complex challenge. A brute force approach would be impractical due to the potentially enormous number of steps required. However, we can leverage an important mathematical insight:

1. Each path starting from a node ending with 'A' will eventually reach a node ending with 'Z'
2. Due to the deterministic nature of the network and instructions, these paths form cycles
3. The number of steps needed for all paths to simultaneously end at 'Z' nodes is the least common multiple (LCM) of their individual cycle lengths

```rust
let a_nodes = net.net
    .keys()
    .filter(|s| s.ends_with('A'))
    .collect::<std::rc::Rc<[_]>>();

let steps = a_nodes.iter()
    .map(|node| {
        // Find cycle length for each starting node
        net.clone()
            .iter(node, turns.chars().cycle())
            .take_while(|node| !node.ends_with("Z"))
            .count() + 1
    })
    .reduce(num::integer::lcm)  // Calculate LCM of all cycle lengths
    .unwrap();
```

This elegant approach transforms what could have been a trillion-step simulation into a much more manageable calculation.

### Performance Considerations

Several design choices enhance performance:
1. Using `HashMap` for O(1) node lookups
2. Employing `Rc<str>` to avoid string duplication
3. Leveraging iterators for memory-efficient traversal
4. Applying mathematical principles (LCM) to solve the synchronization problem

For our example puzzle input, this results in finding the LCM of six different cycle lengths:
```
"AAA" -> 20093 steps
"CVA" -> 22357 steps
"LDA" -> 16697 steps
"LHA" -> 14999 steps
"RHA" -> 17263 steps
"VGA" -> 20659 steps

Part 2 solution: 22,103,062,509,257 steps
```

This solution demonstrates how combining proper data structures, efficient algorithms, and mathematical insights can solve seemingly intractable problems in computational puzzles.
