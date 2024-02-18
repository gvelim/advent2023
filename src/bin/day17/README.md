# Day 17: Clumsy Crucible
## Input
You land near the gradually-filling pool of lava at the base of your new **lavafall**. Lavaducts will eventually carry the lava throughout the city, but to make use of it immediately, Elves are loading it into large **crucibles** on wheels. The crucibles are top-heavy and pushed by hand. Unfortunately, the crucibles become very difficult to steer at high speeds, and so it can be hard to go in a straight line for very long.

You'll need to find the best way to get the crucible **from the lava pool to the machine parts factory**. To do this, you need to **minimize heat loss** while choosing a route that doesn't require the crucible to go in a straight line for too long

Fortunately, the Elves here have a map (your puzzle input) that uses traffic patterns, ambient temperature, and hundreds of other parameters to calculate exactly how much heat loss can be expected for a crucible entering any particular city block.

For example:
```
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
```
## Part 1
Each city block is marked by a single digit that represents the **amount of heat loss if the crucible enters that block**. The starting point, the lava pool, is the top-left city block; the destination, the machine parts factory, is the bottom-right city block. (Because you already start in the top-left block, you don't incur that block's heat loss unless you leave that block and then return to it.)

Because it is difficult to keep the top-heavy crucible going in a straight line for very long, **it can move at most three blocks** in a single direction before it must turn 90 degrees left or right. The crucible also can't reverse direction; after entering each city block, it may only turn left, continue straight, or turn right.

One way to minimize heat loss is this path:
```
▶ 2/0  :0 |▶ 4/4  :1 |▶ 1/5  :2 |  3/0  :0 |  4/0  :0 |▲ 3/23 :1 |▶ 2/25 :1 |▶ 3/28 :2 |▶ 1/29 :3 |  1/0  :0 |  3/0  :0 |  2/0  :0 |  3/0  :0 |
  3/0  :0 |  2/0  :0 |▼ 1/6  :1 |▶ 5/11 :1 |▶ 4/15 :2 |▶ 5/20 :3 |  3/0  :0 |  5/0  :0 |▼ 3/32 :1 |▶ 5/37 :1 |  6/0  :0 |  2/0  :0 |  3/0  :0 |
  3/0  :0 |  2/0  :0 |  5/0  :0 |  5/0  :0 |  2/0  :0 |  4/0  :0 |  5/0  :0 |  6/0  :0 |  5/0  :0 |▼ 4/41 :1 |▶ 2/43 :1 |  5/0  :0 |  4/0  :0 |
  3/0  :0 |  4/0  :0 |  4/0  :0 |  6/0  :0 |  5/0  :0 |  8/0  :0 |  5/0  :0 |  8/0  :0 |  4/0  :0 |  5/0  :0 |▼ 4/47 :1 |  5/0  :0 |  2/0  :0 |
  4/0  :0 |  5/0  :0 |  4/0  :0 |  6/0  :0 |  6/0  :0 |  5/0  :0 |  7/0  :0 |  8/0  :0 |  6/0  :0 |  7/0  :0 |▼ 5/52 :2 |▶ 3/55 :1 |  6/0  :0 |
  1/0  :0 |  4/0  :0 |  3/0  :0 |  8/0  :0 |  5/0  :0 |  9/0  :0 |  8/0  :0 |  7/0  :0 |  9/0  :0 |  8/0  :0 |  4/0  :0 |▼ 5/60 :1 |  4/0  :0 |
  4/0  :0 |  4/0  :0 |  5/0  :0 |  7/0  :0 |  8/0  :0 |  7/0  :0 |  6/0  :0 |  9/0  :0 |  8/0  :0 |  7/0  :0 |  7/0  :0 |▼ 6/66 :2 |  6/0  :0 |
  3/0  :0 |  6/0  :0 |  3/0  :0 |  7/0  :0 |  8/0  :0 |  7/0  :0 |  7/0  :0 |  9/0  :0 |  7/0  :0 |  9/0  :0 |  6/0  :0 |▼ 5/71 :3 |▶ 3/74 :1 |
  4/0  :0 |  6/0  :0 |  5/0  :0 |  4/0  :0 |  9/0  :0 |  6/0  :0 |  7/0  :0 |  9/0  :0 |  8/0  :0 |  6/0  :0 |  8/0  :0 |  8/0  :0 |▼ 7/81 :1 |
  4/0  :0 |  5/0  :0 |  6/0  :0 |  4/0  :0 |  6/0  :0 |  7/0  :0 |  9/0  :0 |  9/0  :0 |  8/0  :0 |  6/0  :0 |  4/0  :0 |  5/0  :0 |▼ 3/84 :2 |
  1/0  :0 |  2/0  :0 |  2/0  :0 |  4/0  :0 |  6/0  :0 |  8/0  :0 |  6/0  :0 |  8/0  :0 |  6/0  :0 |  5/0  :0 |  5/0  :0 |◀ 6/93 :1 |▼ 3/87 :3 |
  2/0  :0 |  5/0  :0 |  4/0  :0 |  6/0  :0 |  5/0  :0 |  4/0  :0 |  8/0  :0 |  8/0  :0 |  8/0  :0 |  7/0  :0 |  7/0  :0 |▼ 3/96 :1 |  5/0  :0 |
  4/0  :0 |  3/0  :0 |  2/0  :0 |  2/0  :0 |  6/0  :0 |  7/0  :0 |  4/0  :0 |  6/0  :0 |  5/0  :0 |  5/0  :0 |  5/0  :0 |▼ 3/99 :2 |▶ 3/102:1 |
```
This path never moves more than three consecutive blocks in the same direction and incurs a heat loss of only `102`.

Directing the crucible from the lava pool to the machine parts factory, but not moving more than three consecutive blocks in the same direction, **what is the least heat loss it can incur?**
## Part 2
Elves are going to upgrade to **ultra crucibles**. Once an ultra crucible starts moving in a direction, it needs to move **a minimum of four blocks** in that direction before it can turn (or even before it can stop at the end). However, it will eventually start to get wobbly: an ultra crucible can move **a maximum of ten consecutive blocks** without turning

In the above example, an ultra crucible could follow this path to minimize heat loss:
```
▶ 2/0  : 0 |▶ 4/4  : 1 |▶ 1/5  : 2 |▶ 3/8  : 3 |▶ 4/12 : 4 |▶ 3/15 : 5 |▶ 2/17 : 6 |▶ 3/20 : 7 |▶ 1/21 : 8 |  1/0  : 0 |  3/0  : 0 |  2/0  : 0 |  3/0  : 0 |
  3/0  : 0 |  2/0  : 0 |  1/0  : 0 |  5/0  : 0 |  4/0  : 0 |  5/0  : 0 |  3/0  : 0 |  5/0  : 0 |▼ 3/24 : 1 |  5/0  : 0 |  6/0  : 0 |  2/0  : 0 |  3/0  : 0 |
  3/0  : 0 |  2/0  : 0 |  5/0  : 0 |  5/0  : 0 |  2/0  : 0 |  4/0  : 0 |  5/0  : 0 |  6/0  : 0 |▼ 5/29 : 2 |  4/0  : 0 |  2/0  : 0 |  5/0  : 0 |  4/0  : 0 |
  3/0  : 0 |  4/0  : 0 |  4/0  : 0 |  6/0  : 0 |  5/0  : 0 |  8/0  : 0 |  5/0  : 0 |  8/0  : 0 |▼ 4/33 : 3 |  5/0  : 0 |  4/0  : 0 |  5/0  : 0 |  2/0  : 0 |
  4/0  : 0 |  5/0  : 0 |  4/0  : 0 |  6/0  : 0 |  6/0  : 0 |  5/0  : 0 |  7/0  : 0 |  8/0  : 0 |▼ 6/39 : 4 |▶ 7/46 : 1 |▶ 5/51 : 2 |▶ 3/54 : 3 |▶ 6/60 : 4 |
  1/0  : 0 |  4/0  : 0 |  3/0  : 0 |  8/0  : 0 |  5/0  : 0 |  9/0  : 0 |  8/0  : 0 |  7/0  : 0 |  9/0  : 0 |  8/0  : 0 |  4/0  : 0 |  5/0  : 0 |▼ 4/64 : 1 |
  4/0  : 0 |  4/0  : 0 |  5/0  : 0 |  7/0  : 0 |  8/0  : 0 |  7/0  : 0 |  6/0  : 0 |  9/0  : 0 |  8/0  : 0 |  7/0  : 0 |  7/0  : 0 |  6/0  : 0 |▼ 6/70 : 2 |
  3/0  : 0 |  6/0  : 0 |  3/0  : 0 |  7/0  : 0 |  8/0  : 0 |  7/0  : 0 |  7/0  : 0 |  9/0  : 0 |  7/0  : 0 |  9/0  : 0 |  6/0  : 0 |  5/0  : 0 |▼ 3/73 : 3 |
  4/0  : 0 |  6/0  : 0 |  5/0  : 0 |  4/0  : 0 |  9/0  : 0 |  6/0  : 0 |  7/0  : 0 |  9/0  : 0 |  8/0  : 0 |  6/0  : 0 |  8/0  : 0 |  8/0  : 0 |▼ 7/80 : 4 |
  4/0  : 0 |  5/0  : 0 |  6/0  : 0 |  4/0  : 0 |  6/0  : 0 |  7/0  : 0 |  9/0  : 0 |  9/0  : 0 |  8/0  : 0 |  6/0  : 0 |  4/0  : 0 |  5/0  : 0 |▼ 3/83 : 5 |
  1/0  : 0 |  2/0  : 0 |  2/0  : 0 |  4/0  : 0 |  6/0  : 0 |  8/0  : 0 |  6/0  : 0 |  8/0  : 0 |  6/0  : 0 |  5/0  : 0 |  5/0  : 0 |  6/0  : 0 |▼ 3/86 : 6 |
  2/0  : 0 |  5/0  : 0 |  4/0  : 0 |  6/0  : 0 |  5/0  : 0 |  4/0  : 0 |  8/0  : 0 |  8/0  : 0 |  8/0  : 0 |  7/0  : 0 |  7/0  : 0 |  3/0  : 0 |▼ 5/91 : 7 |
  4/0  : 0 |  3/0  : 0 |  2/0  : 0 |  2/0  : 0 |  6/0  : 0 |  7/0  : 0 |  4/0  : 0 |  6/0  : 0 |  5/0  : 0 |  5/0  : 0 |  5/0  : 0 |  3/0  : 0 |▼ 3/94 : 8 |```
In the above example, an ultra crucible would incur the minimum possible heat loss of `94`.

Directing the ultra crucible from the lava pool to the machine parts factory, **what is the least heat loss it can incur**?

## Approach
