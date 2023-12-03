# Day 1

## Input

    two1nine  
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixtee

## Output
Extract the first and last digits, either in digit or word form ("one", "two", "three", etc), to produce the following numeric outputs. 

    29
    83
    13
    24
    42
    14
    76

Pay attention to cases such `eightwothree` is understood as `eight`, `two` & `three` hence results to `83` 

## Approach
While moving fwd the input array, look backwards for patterns e.g.

    e^
    ei^
    eig^
    eigh^
    eight^ <-- matched "eight"
    eightw^
    eightwo^ <-- matched "two"
