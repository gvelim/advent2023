# Day 7: Camel Cards

## Problem Overview
You're given a list of poker-like hands and corresponding bids. Each hand needs to be ranked by strength, and then you calculate total winnings based on these rankings.

## Input Example
```
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
```

Each hand consists of five cards labeled `A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2`. Card strength follows this order with `A` being highest.

Hand types from strongest to weakest:
1. **Five of a kind**: All five cards have the same label (e.g., `AAAAA`)
2. **Four of a kind**: Four cards with the same label (e.g., `AA8AA`)
3. **Full house**: Three of one label, two of another (e.g., `23332`)
4. **Three of a kind**: Three cards with the same label (e.g., `TTT98`)
5. **Two pair**: Two pairs of different labels (e.g., `23432`)
6. **One pair**: One pair of the same label (e.g., `A23A4`)
7. **High card**: All cards have different labels (e.g., `23456`)

When two hands have the same type, you compare cards in order (1st, 2nd, etc.) until finding a difference.

## Solution Approach

### Step 1: Modeling the Problem Domain

First, we define domain objects to represent our card game:

```rust
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub(crate) enum HandType {
    HighCard = 0,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

pub(crate) struct Hand {
    pub(crate) layout: String,
    pub(crate) hands_type: HandType,
    pub(crate) ord_layout: String,
    pub(crate) cards: std::rc::Rc<[(char, u8)]>,
    joker_pos: Option<usize>
}
```

**Design Insight**: Using enums with derive macros for comparison operations simplifies hand ranking logic significantly. The `#[derive]` attribute automatically implements traits like `Ord` for us, which enables direct comparison between hand types.

**Field Explanations**:
- `layout`: Original string representation of the hand
- `hands_type`: Classified type of the hand (pair, flush, etc.)
- `ord_layout`: A transformed version of the layout for efficient ordering
- `cards`: Frequency analysis of cards, shared via reference counting
- `joker_pos`: Position of joker in the frequency array (if present)

### Step 2: Hand Classification Algorithm

The core of the solution is classifying hands by type. We use frequency analysis:

```rust
pub(crate) fn get_type(&self) -> HandType {
    let mut unique_cards = self.cards.len() as u32;
    let mut freq = self.cards[0].1;

    // if we have joker position && and is not a 'JJJJJ' case
    if self.joker_pos.is_some() && unique_cards > 1 {
        unique_cards -= 1;
        freq += self.cards[self.joker_pos.unwrap()].1;
    }

    match unique_cards {
        1 => HandType::FiveOfAKind,
        2 if freq == 4 => HandType::FourOfAKind,
        2 => HandType::FullHouse,
        3 if freq == 3 => HandType::ThreeOfAKind,
        3 => HandType::TwoPair,
        4 => HandType::OnePair,
        _ => HandType::HighCard
    }
}
```

**Algorithm Insight**: This function uses two key parameters:
1. `unique_cards`: Number of unique cards (after accounting for jokers)
2. `freq`: Frequency of the most common card (plus jokers if applicable)

The `match` expression with guard clauses (`if freq == 4`) creates very readable code for complex classification rules. For example, a hand with 2 unique cards could be either Four of a Kind or Full House, and the frequency helps distinguish them.

### Step 3: Parsing and Frequency Analysis

To classify hands, we first analyze card frequencies:

```rust
pub(crate) fn parse(input: &str, card_order: [char; 13], joker: Option<char>) -> Hand {
    // Create mapping for ordering transformation
    let ord_card = card_order.iter()
        .zip([ '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E' ])
        .map(|(&i,o)| (i,o) )
        .collect::<HashMap<char,char>>();

    let mut joker_pos = None;
    // Count frequencies and transform for ordering
    let (cards, ord_layout) = input.chars()
        .fold((HashMap::with_capacity(5), String::with_capacity(5)),
            |(mut cards, mut ord_layout), card| {
                *cards.entry(card).or_insert(0) += 1;
                ord_layout.push(ord_card[&card]);
                (cards, ord_layout)
            });

    // extract the HashMap onto an array
    let mut cards = cards.into_iter().collect::<Vec<_>>();
    // reverse sort the array by order of card freq
    cards.sort_by_key(|(_, freq)| *freq);
    cards.reverse();

    // Additional joker handling...

    // Create and return the hand
    let mut hand = Hand {
        layout: String::from(input),
        ord_layout,
        hands_type: HandType::HighCard,
        cards: cards.into(),
        joker_pos
    };
    hand.hands_type = hand.get_type();
    hand
}
```

**Functional Programming Insight**:
- We use `fold` to concisely build both frequency map and ordering representation in a single pass
- `HashMap::with_capacity(5)` pre-allocates memory for efficiency
- The `cards.sort_by_key(|(_, freq)| *freq)` sorts cards by frequency
- `cards.into()` converts the Vec into an `Rc<[T]>` which shares ownership efficiently

The `ord_layout` transformation maps each card to a character that preserves the correct ordering for string comparison, making card-by-card comparison simpler later.

### Step 4: Handling Jokers (Part 2)

For part 2, the `J` card becomes a joker that can represent any card to maximize hand strength:

```rust
// if we are dealing with a Joker case
joker.is_some_and(|joker| {
    // find Joker's freq order and store position
    joker_pos = cards.iter().position(|(card, _)| joker.eq(card));
    // if it is 1st and not the only card in the hand; we deal with JJ123 cases
    cards.len() > 1 && joker_pos.eq(&Some(0))
})
.then(|| {
    // move to the last place & update its position
    cards.rotate_left(1);
    joker_pos = Some(cards.len()-1);
    Some(())
});
```

**Algorithm Insight**:
- `is_some_and` checks if joker exists and applies the following condition
- `position` finds where the joker is in our frequency list
- If jokers are the most frequent card (e.g., `JJ123`), we need to move them to the end
- `rotate_left(1)` shifts the entire array, placing the first element at the end

This clever handling ensures that jokers boost the next most frequent card rather than themselves, handling edge cases correctly.

### Step 5: Hand Comparison Logic

After classification, we need to compare hands of the same type:

```rust
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hands_type.cmp(&other.hands_type) {
            Ordering::Equal =>
                self.ord_layout.cmp(&other.ord_layout),
            comparison => comparison
        }
    }
}
```

**Design Insight**:
- Implementing the `Ord` trait enables hands to be sorted with `sort()`
- First comparison is by hand type (pair, flush, etc.)
- If hand types are equal, we compare the `ord_layout` strings
- Using the transformed `ord_layout` means we can use Rust's built-in string comparison instead of implementing card-by-card comparison manually

The `std::cmp::Ordering` enum represents comparison results as `Less`, `Equal`, or `Greater`.

### Step 6: Putting It All Together

The main function orchestrates the process:

```rust
static CAMEL_ORDER_PART1: [char; 13] = [ '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A' ];
static CAMEL_ORDER_PART2: [char; 13] = [ 'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A' ];

fn main() {
    let input = std::fs::read_to_string("./src/bin/day7/input.txt").expect("Ops!");

    let run_part = |camel_order, joker| {
        let mut hands = input.lines()
            .map(|line|{
                let mut split = line.split_ascii_whitespace();
                (
                    Hand::parse(split.next().expect("Ops!"), camel_order, joker),
                    split.next().unwrap().parse::<u32>().expect("Ops!")
                )
            })
            .collect::<Vec<_>>();

        hands.sort();
        hands.iter()
            .enumerate()
            .map(|(i,(_,bid))| (i as u32+1) * *bid)
            .sum::<u32>()
    };

    println!("Part 1: {}", run_part(CAMEL_ORDER_PART1, None));
    println!("Part 2: {}", run_part(CAMEL_ORDER_PART2, Some('J')));
}
```

**Functional Programming Insight**:
- `run_part` is a higher-order function that parameterizes the solution
- `static` arrays define the card ordering for each part
- `lines()`, `map()`, `collect()` demonstrate Rust's iterator-based approach
- `enumerate()` adds rank information (starting from 0, so we add 1)
- The final calculation uses `sum()` to accumulate the total

By parameterizing with `camel_order` and `joker`, we avoid code duplication between Part 1 and Part 2, following the DRY (Don't Repeat Yourself) principle.

## Special Cases and Edge Conditions

1. **All Jokers Case**: When all cards are jokers (`JJJJJ`), they're already the best possible hand (Five of a Kind). The code handles this with:
   ```rust
   if self.joker_pos.is_some() && unique_cards > 1 {
   ```

2. **Jokers as Most Frequent**: When jokers are the most frequent card (e.g., `JJ123`), we need to ensure they boost the next most frequent card:
   ```rust
   cards.len() > 1 && joker_pos.eq(&Some(0))
   ```

3. **Card Ordering**: In Part 2, while jokers are the most powerful for hand type, they're the weakest for card-by-card comparison:
   ```rust
   static CAMEL_ORDER_PART2: [char; 13] = [ 'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A' ];
   ```

## Performance Considerations

- Using `Rc` (reference counting) for card frequency data avoids unnecessary cloning while allowing shared ownership
- Pre-allocating data structures with capacity hints: `HashMap::with_capacity(5)`
- Single-pass algorithms for parsing and frequency analysis with `fold`
- The `ord_layout` transformation enables efficient string comparison instead of custom comparators
- Reusing the same function for both parts with different parameters

## Example Walkthrough

For the hand `T55J5` in Part 2:
1. Frequency analysis: `[('5', 3), ('T', 1), ('J', 1)]` (sorted by frequency)
2. With joker rule, it counts as: `unique_cards = 2, freq = 3 + 1 = 4` (Four of a Kind)
3. In Part 1, this would just be Three of a Kind: `unique_cards = 3, freq = 3`

For `KTJJT` in Part 2:
1. Frequency analysis: `[('T', 2), ('J', 2), ('K', 1)]`
2. Special handling for jokers as frequent cards: rotate to `[('T', 2), ('K', 1), ('J', 2)]`
3. With joker rule: `unique_cards = 2, freq = 2 + 2 = 4` (Four of a Kind)
4. In Part 1: `unique_cards = 3, freq = 2` (Two Pair)

## Results Summary

Using the test input:
- Part 1: 6440
- Part 2: 5905

This solution demonstrates elegant use of Rust's type system, functional programming patterns, and careful edge case handling to solve a deceptively complex card game problem.
