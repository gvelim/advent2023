# Day 4

## Input
Each card has two lists of numbers separated by a vertical bar (|): a list of winning numbers and then a list of numbers you have.
```
        <-- Winning --> <----- Numbers --------> 
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
```
## Part 1: Output
Figure out which of the numbers you have appear in the list of winning numbers. First match makes the card worth one point and each match after the first doubles the point value of that card. Sum them all up.
```
Card { 1, {53, 48, 9, 83, 86, 17, 6, 31} } --> [83, 86, 17, 48] --> Score: 8
Card { 2, {32, 17, 19, 30, 68, 24, 61, 82} } -->  [61, 32] --> Score: 2
Card { 3, {69, 82, 63, 72, 16, 21, 1, 14} } -->  [1, 21] --> Score: 2
Card { 4, {59, 83, 54, 51, 58, 76, 84, 5} } -->  [84] --> Score: 1
Card { 5, {22, 93, 36, 88, 70, 30, 82, 12} } -->  []
Card { 6, {77, 23, 10, 35, 11, 36, 67, 74} } -->  []
Part 1 Sum: 13
```
## Part 2: Output
There's no such thing as "points". You win copies of the scratchcards below the winning card equal to the number of matches. So, if card 10 were to have 5 matching numbers, you would win one copy each of cards 11, 12, 13, 14, and 15.
```
Card { 1, {17, 9, 83, 86, 6, 31, 48, 53} } --> Wins: 4 (copies: 2..5) --> Total Copies: 1
Card { 2, {32, 19, 24, 61, 30, 68, 17, 82} } --> Wins: 2 (copies: 3..4) --> Total Copies: 2
Card { 3, {1, 63, 69, 82, 72, 21, 16, 14} } --> Wins: 2 (copies: 4..5) --> Total Copies: 4
Card { 4, {84, 58, 83, 54, 76, 51, 5, 59} } --> Wins: 1 (copies: 5..5) --> Total Copies: 8
Card { 5, {93, 12, 82, 22, 30, 36, 70, 88} } --> Wins: 0 --> Total Copies: 14
Card { 6, {36, 67, 10, 74, 23, 35, 77, 11} } --> Wins: 0 --> Total Copies: 1
Part2 Sum: 30
```
## Approach
### Part 1
Using sets of unique numbers we can extract the intersection easily, then use the 2 to the power of number of wins. In this case we use the `HashSet` data structure
### Part 2
A `HashMap` data structure can be initiated with (key,value) pair as of (Card ID, Number of Copies). As we process each card in sequential order, we increment the values of the subsequent card ID, e.g. if we process `card id:1` then we increase by `card wins` times the copy values for card keys `2, 3, 4, 5` 