# Day 3

## Input
The engine schematic consists of a visual representation of the engine. Numbers adjacent to a symbol, even diagonally, are "part numbers".
```
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
```
## Part 1: Output
Find all the part numbers and produce their SUM()
```
467 + 35 + 633 + 617 + 592 + 755 + 664 + 598 = 4361
```
## Part 2: Output
A gear is any `*` symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of multiplying those two numbers together. Add up all the gear ratios
```
467 * 35 + 755 * 598 = 467835

```
## Approach
1. Keep the length of the first line (offset) and convert the two dimensions to a single dimension; single string
2. Extract all numbers from the string in a struct (value, pos(start,end))
3. Extract all symbols from the string in a struct (value, pos)
4. Per number, iterate over the symbols and check if
   1. `Symbol.pos -/+ 1` falls within `Number.pos(star,end)` ? == Adjacent to number
   2. `Symbol.pos -/+ offset` falls within `Number.pos(star,end)` ? == Under/Over the number 
   3. `Symbol.pos -/+ offset +/- 1` falls within `Number.pos(star,end)` ?  == diagonal to the number

```
...123...   ==> ...123........*... (offset: 9)   
.....*...            +---9----+

Offset analysis
================
..abbbc..   ==> ..abbbc....d123d....efffg.. (offset: 9)
..d123d..         ||  |    + 1 -    |  ||
..efffg..         ||  +      8      -  ||
                  |+++       9       ---|
                  +          10         -
```
