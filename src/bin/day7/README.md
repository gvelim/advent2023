# Day 7
## Input
You get a list of hands & bids pairs, and your goal is to order them based on the strength of each hand. 
```
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
```
A hand consists of five cards labeled one of `A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2`. The relative strength of each card follows this order, where `A` is the highest and `2` is the lowest
Every hand is exactly one type. From strongest to weakest, they are:

* **Five of a kind**, where all five cards have the same label: `AAAAA`
* **Four of a kind**, where four cards have the same label and one card has a different label: `AA8AA`
* **Full house**, where three cards have the same label, and the remaining two cards share a different label: `23332`
* **Three of a kind**, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: `TTT98`
* **Two pair**, where two cards share one label, two other cards share a second label, and the remaining card has a third label: `23432`
* **One pair**, where two cards share one label, and the other three cards have a different label from the pair and each other: `A23A4`
* **High card**, where all cards' labels are distinct: `23456`

If two hands have the same type, a second ordering rule takes effect. Start by comparing the first card in each hand. If these cards are different, the hand with the stronger first card is considered stronger. If the first card in each hand have the same label, however, then move on to considering the second card in each hand. If they differ, the hand with the higher second card wins; otherwise, continue with the third card in each hand, then the fourth, then the fifth
## Part 1: Output
Find the rank of every hand in your set and calculate the total winnings of this set of hands by adding up the result of multiplying each hand's bid with its rank
```
Rank 1 - ("32T3K", OnePair) => "32T3K"
Rank 2 - ("KTJJT", TwoPair) => "KTJJT"
Rank 3 - ("KK677", TwoPair) => "KK677"
Rank 4 - ("T55J5", ThreeOfAKind) => "T55J5"
Rank 5 - ("QQQJA", ThreeOfAKind) => "QQQJA

765 * 1 + 220 * 2 + 28 * 3 + 684 * 4 + 483 * 5 = 6440
```
## Part 2: Output
Now, `J` cards are jokers - wildcards that can act like whatever card would make the hand the strongest type possible.

To balance this, `J` cards are now the weakest individual cards, weaker even than `2`. The other cards stay in the same order: `A, K, Q, T, 9, 8, 7, 6, 5, 4, 3, 2, J`.

Using the new joker rule, find the rank of every hand in your set. What are the new total winnings?
```
Rank 1 - ("32T3K", OnePair) => "32T3K"
Rank 2 - ("KK677", TwoPair) => "KK677"
Rank 3 - ("T55J5", FourOfAKind) => "T55J5"
Rank 4 - ("QQQJA", FourOfAKind) => "QQQJA"
Rank 5 - ("KTJJT", FourOfAKind) => "KTJJT"

765 * 1 + 28 * 2 + 684 * 3 + 483 * 4 + 220 * 5 = 5905
```
## Approach
To find the type of hand, use a `Hashmap` to extract the frequency per character in the hand, then convert the `Hashmap` to a `Vector` and reverse sort it. Now you should have in reverse order all the unique cards and card frequency i.e. `[('A',2),('J',2),('2',1)]`

Hence, the types are derived from two values (a) number of unique cards; `array.len()` & (b) the highest card freq; `array[0].freq`
```
match array.len() {
    1 => HandType::FiveOfAKind,
    2 if array[0].freq ==4 => HandType::FourOfAKind,
    2 => HandType::FullHouse,
    3 if array[0].freq ==3 => HandType::ThreeOfAKind,
    3 => HandType::TwoPair,
    4 => HandType::OnePair,
    _ => HandType::HighCard
}
```
The Joker card affects the two key parameters like this
1. `number of unique cards` is reduced by one since the Joker isn't a unique card
2. `new highest card freq` = highest card freq + Joker card frequency

Watch out for unique hand cases like 
1. `JJ123`, hence if the Joker is the most frequent card, you have to pick the next in order card that has the highest card frequency after the joker one.
2. `JJJJJ` no joker logic applies in this case 