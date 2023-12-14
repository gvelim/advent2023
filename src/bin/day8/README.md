# Day 8
## Input
You are given a "map" on how to navigate the desert. The contains a list of left/right instructions, and the rest of the document seem to describe some kind of network of labeled nodes
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
Starting with `AAA`, you need to look up the next element based on the next left/right instruction in your input. In this example, start with `AAA` and go right (R) by choosing the right element of `AAA`, `CCC`. Then, `L` means to choose the left element of `CCC`, `ZZZ`. By following the left/right instructions, you reach `ZZZ` in `2` steps
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
Start at every node that ends with `A` and follow all of the paths at the same time until they all simultaneously end up at nodes that end with `Z`
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
## Approach
1. We parse the Network input into a `HashMap` that holds `(Key: Node Name, Value: (Left Node, Right Node))`, for example line `11A = (11B, XXX)` turns in `(Key:"11A", Value:("11B","XXX"))`
2. We parse directions into a cyclical `Iterator`, that is, when it gives the last item, the next one will be the first again i.e. `"LRLR".chars().cycle()` will continuously provide the next direction.
3. For part 1, and in order to traverse the network, the `Network` provides a Network `Iterator` with inputs (a) **starting `node`** and (b) the **directions `Iterator`**. The network `Iterator` will always produce the next `node` 
4. For part 2, similarly to (3) above however we use a different parallel Iterator with input (a) a list of starting `nodes` and (b) directions `Iterator`. The parallel `Iterator` will then produce the list of `nodes` following, which in turn can be assessed whether they meet the end condition.
