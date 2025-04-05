# Day 4: Scratchcards

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

### Core Design
The solution revolves around efficiently parsing card data and identifying matching numbers. We use a modular approach with three key components:
1. A `Numbers` struct to represent sets of numbers
2. A `Card` struct to hold the card ID and numbers
3. Processing logic for both parts of the puzzle

### Step 1: Data Representation
We create a custom `Numbers` struct that wraps a `HashSet<u32>` to store unique numbers:

```rust
pub(crate) struct Numbers(pub(crate) HashSet<u32>);
```

This allows us to use set operations (particularly intersection) for finding matching numbers efficiently.

### Step 2: Parsing Input
We implement the `FromStr` trait for our structures to convert string input to structured data with robust error handling:

```rust
impl FromStr for Numbers {
    // Convert string of numbers into a HashSet
}

impl FromStr for Card {
    // Parse card ID and numbers from input line
}
```

### Part 1
Using the `HashSet` data structure, we can find matching numbers through set intersection. For each card:
1. Find the intersection between the card's numbers and winning numbers
2. Count the matches
3. Calculate the score using 2^(matches-1)
4. Sum all scores

```rust
let part1 = Rounds::parse_rounds(input.as_str())
    .map(|(card, numbers)| card.winning_numbers(&numbers).count())
    .filter(|&size| size > 0)
    .map(|size| 2_u32.pow((size - 1) as u32))
    .sum::<u32>();
```

### Part 2
A `HashMap` data structure is used to track copies of each card, with (Card ID, Number of Copies) as the (key, value) pairs. As we process each card in sequential order:

1. Initialize a HashMap with all cards having 1 copy
2. For each card, determine how many matching numbers it has
3. For the next N cards (where N is the number of matches), add copies equal to the current card's copies
4. Sum the total number of cards

```rust
let mut part2 = Rounds::parse_rounds(input.as_str())
    .map(|(card,_)| (card.id,1))
    .collect::<HashMap<u32,u32>>();

let part2_sum = Rounds::parse_rounds(input.as_str())
    .map(|(card, numbers)| {
        let winning_numbers = card.winning_numbers(&numbers).count() as u32;
        (card,winning_numbers)
    })
    .map(|(card, wins)| {
        let card_copies = *part2.get(&card.id).unwrap();
        // Add copies to subsequent cards
        card_copies
    })
    .sum::<u32>();
```

This approach allows us to efficiently track and update the copies of cards as we process them in order.

### Key Data Structures and Algorithms

1. **HashSet**: Used for efficient set operations like intersection
2. **HashMap**: Used to track card copies for Part 2
3. **Iterators**: Used extensively for functional, declarative code
4. **Error handling**: Custom error types with appropriate conversions
5. **FromStr trait**: For parsing input strings into structured data

By combining these components, we've created a modular, efficient solution to the scratchcards puzzle.
