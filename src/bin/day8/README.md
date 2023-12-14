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
## Approach
1. We parse the Network input into a `HashMap` that holds `(Key: Node Name, Value: (Left Node, Right Node))`, for example line `11A = (11B, XXX)` turns in `(Key:"11A", Value:("11B","XXX"))`.
2. We parse directions into a cyclical `Iterator`, that is, when it gives the last item, the next one will be the first again i.e. `"LRLR".chars().cycle()` will continuously provide the next direction.


For part 1, and in order to traverse the network, we can create a Network `Iterator` that takes (a) a **starting `node`** as `current` and (b) the **directions `Iterator`**. The Network `Iterator` will always produce the next `node` using 
1. the `HashMap::get(current) -> (left node,right node)`
2. and take the next instruction from the direction iterator 
3. to decide whether `current` will take the `left node` or `right node` value.
4. then repeat step 1 until the `current` == `target node`

   
For part 2, Brute forcing the solution could take up to trillions iterations!! However we can observe that each run follows the exact same path hence is repeating itself, in other words, succesively reaches its goal at a fix period, let's say every 20 steps. 

Hence which each parallel run having its' own fixed period, the total number of steps for achieving the part 2 goal is equal to the **Least Common Multiple** of all fixed periods. Hence the solution is to first find the fixed periods per run and then extract the LCM value.
```
"AAA" -> repeats every 20093 steps
"CVA" -> repeats every 22357 steps
"LDA" -> repeats every 16697 steps
"LHA" -> repeats every 14999 steps
"RHA" -> repeats every 17263 steps
"VGA" -> repeats every 20659 steps

Part 2: Least Common Multiple = 22103062509257
```
