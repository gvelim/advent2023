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
## Part 1
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
Horizontal:[None], Vertical:[position:5, radius:4] -> Result: 5
   
"#...##..#"
"#....#..#" <
"..##..###" |
"#####.##." |
----------- 3
"#####.##." |
"..##..###" |
"#....#..#" <
Horizontal[position:4, radius:3], Vertical[None] -> Result 4 * 100

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
Horizontal[position:3, radius:3], Vertical[position:5, radius:4] -> Result 3 * 100 = 300
 
"#....#..#" <
----------- 1
"#....#..#" <
"..##..###"
"#####.##."
"#####.##."
"..##..###"
"#....#..#"
 Horizontal[position:1, radius:1], Vertical[None] -> Result 1 * 100 = 100
 
 Total = 400
```
## Approach
### Part 1
Below is the definition of a **_perfect_** and _**smudged**_ pattern reflection:
```
Perfect reflection        Smudged reflection is similar to 
relfects either the       a perfect one, but with a
first, last or both       *ONE* flawed reflection
columns or rows           i.e. '2'

  <---4--->                 <---4--->   
"#.##.|.##." = 4          "#.##.|.##." 4
"..#.#|#.#." = 4          "..#.#|#.#." 4
"##...|...#" = 4          "##...|...#" 4
"##...|...#" = 4          "##...|..##" 2 <-- smudged reflection
"..#.#|#.#." = 4          "..#.#|#.#." 4
"..##.|.##." = 4          "..##.|.##." 4
"#.#.#|#.#." = 4          "#.#.#|#.#." 4    
```
Identifying a perfect reflection for a **_single pattern line_** we take the following approach
```
[#|.]##..##. => Index 1, Mirrored: 0 => Abandon, next Index
#[.|#]#..##. => Index 2, Mirrored: 0 => Abandon, next Index
#.[#|#]..##. => Index 3, Mirrored: 1 => found a mirror, not perfect, expand
#[.#|#.].##. => Index 3, Mirrored: 2 => found a mirror, not perfect, expand
[#.#|#..]##. => Index 3, Mirrored: 0 => abandon, next Index
#.#[#|.].##. => Index 4, Mirrored: 0 => Abandon, next Index
#.##.[.|#]#. => Index 5, Mirrored: 0 => Abandon, next Index
#.##..[#|#]. => Index 6, Mirrored: 1 => found a mirror, not perfect, expand
#.##.[.#|#.] => Index 6, Mirrored: 2 => Found a perfect reflecton !!
```
Hence, by applying the above logic to the whole pattern we get
```
For
Index 1            Index 2            Index 3            Index 4             Index 5            Index 6

[#|.]##..##. = 0   #[.|#]#..##. = 0   #.[#|#]..##. = 1   #.#[#|.].##. = 0    #[.##.|.##.] = 4   #.##.[.|#]#. = 0
[. .]#.##.#.       .[. #].##.#.       ..[#|.]##.#. = 0   ..#[. #]#.#.        .[.#.#|#.#.] = 4   ..#.#[# .]#. 
[# #]......#       #[# .].....#       ##[. .]....#       ##.[. .]...#        #[#...|...#] = 4   ##...[. .].#
[# #]......#       #[# .].....#       ##[. .]....#       ##.[. .]...#        #[#...|...#] = 4   ##...[. .].#
[. .]#.##.#.       .[. #].##.#.       ..[# .]##.#.       ..#[. #]#.#.        .[.#.#|#.#.] = 4   ..#.#[# .]#.
[. .]##..##.       .[. #]#..##.       ..[# #]..##.       ..#[# .].##.        .[.##.|.##.] = 4   ..##.[. #]#.
[# .]#.##.#.       #[. #].##.#.       #.[# .]##.#.       #.#[. #]#.#.        #[.#.#|#.#.] = 4   #.#.#[# .]#.

Max Height 1       Max Height 1       Max Height 2       Max Height 1        Max Height 7       Max Height 1
                                                                              ** MATCH **                                    
Perfect Line Mirror at Index 5 with radius 4
```
### Part 2
In order to find a smudged reflection, the above logic has to be adjusted so to accept reflections with **ONLY ONE** flawed radius.

Hence, the above pattern example would have a **smudged reflection** if it had `6` perfect reflections and `1` imperfect
```
Index 5     

#[.##.|.##.] = 4 <-- Perfect reflection
.[.#.#|#.#.] = 4
##..[.|.]#.# = 1 <-- Smudged reflection at radius 2 (1 + 1)
#[#...|...#] = 4
.[.#.#|#.#.] = 4
.[.##.|.##.] = 4
#[.#.#|#.#.] = 4

Max Height  7      
```
Therefore, our scanning algorithm must continue the scan when a reflection flaw is discovered, and later decide whether to accept or reject the **scan results** based on the **radius variation** observed.

As a result and during scanning, we need to measure the **radius' frequency**. We can use an array for this purpose which would look like this for the above example
```
Array Length =  Zero ----- to ---> Pattern Width
            freq[0, 1, 0, 0, 6, 0, 0]
   radius = pos :  '1'      '4'
              
For pattern height = 7, the array reads as:
radius '4' - appeared 6/7 times
radius '1' - appeared 1/7 time
```
Therefore, when an index scan is completed, a smudged reflection is found when the below conditions are true 
1. `freq[radius] == height - 1`
2. `freq[..radius].iter().sum() == 1`

However, computing every index in fully is costly, hence we can abandon a scan when any of the below conditions are true 
1. `freq[0] > 1` - we found more than 1 occurrence of `0` radius
2. `freq[..radius].iter().sum() > 1` - we have more than 1 flaws
