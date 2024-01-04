# Day 13: Point of Incidence

## Input
You note down the patterns of ash (.) and rocks (#) that you see as you walk (your puzzle input); perhaps by carefully analyzing these patterns, you can figure out where the mirrors are!

For example:
```
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
```
## Part 1
To find the reflection in each pattern, you need to find a perfect reflection across either a horizontal line between two rows or across a vertical line between two columns

For example, in the first pattern, the reflection is across a vertical line between two columns; arrows on each of the two columns point at the line between the columns:

```
123456789
    ><   
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.
    ><   
123456789
```
The second pattern reflects across a horizontal line instead:

```
1 #...##..# 1
2 #....#..# 2
3 ..##..### 3
4v#####.##.v4
5^#####.##.^5
6 ..##..### 6
7 #....#..# 7
```

To **summarize** your pattern notes, add up **the number of columns** to the left of each vertical line of reflection; to that, also **add 100 multiplied by the number of rows** above each horizontal line of reflection.

Find the line of reflection in each of the patterns in your notes. **What number do you get after summarizing all of your notes?**
```
  <---4--->
"#.##.|.##."
"..#.#|#.#."
"##...|...#"
"##...|...#"
"..#.#|#.#."
"..##.|.##."
"#.#.#|#.#."
MAX( Horizontal:[None], Vertical:[position:5, radius:4] ) -> Result: 5
   
"#...##..#"
"#....#..#" <
"..##..###" |
"#####.##." |
----------- 3
"#####.##." |
"..##..###" |
"#....#..#" <
MAX( Horizontal[position:4, radius:3], Vertical[None] ) -> Result 4 * 100

Sum: 405
```
## Part 2
You resume walking through the valley of mirrors and - SMACK! - run directly into one. Hopefully nobody was watching, because that must have been pretty embarrassing. Upon closer inspection, you discover that every mirror has exactly one smudge: **exactly one** `.` or `#` should be the opposite type

In each pattern, you'll need to locate and fix the smudge that causes a different reflection line to be valid. (The old reflection line won't necessarily continue being valid after the smudge is fixed.)

In each pattern, fix the smudge and find the different line of reflection. What number do you get after summarizing the new reflection line in each pattern in your notes?
```
"..##..##." <
"..#.##.#." |
"##......#" |
----------- 3
"##......#" |
"..#.##.#." |
"..##..##." <
"#.#.##.#."
 MIN( Horizontal[position:3, radius:3], Vertical[position:5, radius:4] ) -> Result 3 * 100 = 300
 
"#....#..#" <
----------- 1
"#....#..#" <
"..##..###"
"#####.##."
"#####.##."
"..##..###"
"#....#..#"
 MIN( Horizontal[position:1, radius:1], Vertical[None] ) -> Result 1 * 100 = 100
 
 Total = 400
```
## Approach
```
```
