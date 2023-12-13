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
## Part 1: Output
Starting at `AAA`, follow the left/right instructions. How many steps are required to reach `ZZZ`?
```
```
## Part 2: Output
```
```
## Approach
```
```
