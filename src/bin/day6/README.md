# Day 6

## Input
You are given the below input which translates as
* The first race lasts 7ms. The record distance in this race is 9mm.
* The second race lasts 15ms. The record distance in this race is 40mm.
* The third race lasts 30ms. The record distance in this race is 200mm.
```
Time:      7  15   30
Distance:  9  40  200
```
## Part 1: Output
Your toy boat has a starting speed of zero millimeters per millisecond. For each whole millisecond you spend at the beginning of the race holding down the button, the boat's speed increases by one millimeter per millisecond i.e.
* Hold the button for 1 millisecond at the start of the race. Then, the boat will travel at a speed of 1 millimeter per millisecond for 6 milliseconds, reaching a total distance traveled of 6 millimeters.
* Hold the button for 2 milliseconds, giving the boat a speed of 2 millimeters per millisecond. It will then get 5 milliseconds to move, reaching a total distance of 10 millimeters, etc


Determine the number of ways you can beat the record in each race;
```
Race { duration: 7, record: 9 }-> 4
Race { duration: 15, record: 40 }-> 8
Race { duration: 30, record: 200 }-> 9

Total : 4 * 8 * 9 = 288 times
```
## Part 2: Output
Ignore the spaces between the numbers on each line, hence Duration `7`,`15`,`30` becomes Duration `71530`. Similarly for record numbers.

How many ways can you beat the record in this one much longer race?
```
Lower Duration = 14
Upper Duration = 71516
Total: 71516 - 14 + 1 = 71503 ways!
```
## Approach
The below quadratic formula defines the bounds where `charge` values give `run` distances always greater than `record` distance
```
( run_limit - x ) * x = record_distance
=> -x^2 + run_limit * x - record_distance = 0
=> x^2 - run_limit * x + record_distance = 0
 
Where x takes charge values 0..n
```


