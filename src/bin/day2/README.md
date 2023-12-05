# Day 2

## Input
The Elf will reach into the bag, grab a handful of random cubes, show them to you, and then put them back in the bag. He'll do this a few times per game.

    Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

## Part 1: Output
Sum the IDs of those games who are feasible given a bag contains only 12 red cubes, 13 green cubes, and 14 blue cubes?

    Game 1
    Game 2
    Game 5
    ======
    Sum  8

## Part 2: Output
For each game to be feasible, what would be the lowest required number of cubes per color that could have been in the bag ? 

    Game 1 - 48 <- numbers of red, green, and blue cubes multiplied together
    Game 2 - 12
    Game 3 - 1560
    Game 4 - 630
    Game 5 - 36
    ==============
    Sum      2286

## Approach
Use the below Data structure to (a) reject all games that has at least a run with  higher number of cubes, (b) calculate max red, green, blue cubes per game

    Games
        +-- Game
            + max( red, green, blue )
            + Runs
               +---- (red, green, blue)
               +---- (red, green, blue)
               ...
               +---- (red, green, blue)
        +-- Game
            + Max( 4, 2, 6 )
            + Runs
               +---- (4, 0, 3)
               +---- (1, 2, 6)
               ...
               +---- (0, 2, 0)
        ...