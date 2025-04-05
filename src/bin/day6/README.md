# Day 6: Wait For It

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

## Solution Analysis

### Understanding the Problem

In this problem, we need to find how many different ways we can beat a record distance in a race by deciding how long to hold a button at the start. The key relationship is:

- Holding the button for `charge` milliseconds gives the boat a speed of `charge` mm/ms
- The boat then travels for `(duration - charge)` milliseconds at that speed
- Total distance traveled = `(duration - charge) * charge`

### Mathematical Intuition

Looking at the formula for distance traveled, we can observe that this is a quadratic function that forms a parabola. For a successful race, we need:

```
(duration - charge) * charge > record
```

This can be rearranged to:
```
-charge² + duration*charge - record > 0
```

Or in standard quadratic form:
```
charge² - duration*charge + record < 0
```

### Step 1: Modeling the Race

First, let's create a data structure to represent a race:

```rust
#[derive(Debug,PartialEq)]
pub(crate) struct Race {
    pub(crate) duration: u64,
    pub(crate) record: u64
}
```

And a way to calculate the distance for a given charge time:

```rust
pub(crate) fn distance_travelled(charge: u64, duration: u64) -> u64 {
    (duration - charge) * charge
}
```

### Step 2: Finding Winning Charge Times (Brute Force Approach)

One way to solve this is to check every possible charge time and count the winning ones:

```rust
pub(crate) fn _winning_charge_times(&self) -> impl Iterator<Item=(u64, u64)> + '_ {
    self._trial_charge_times().filter(|&(_,dist)| dist > self.record)
}

pub(crate) fn _trial_charge_times(&self) -> impl Iterator<Item=(u64, u64)> + '_ {
    (0..=self.duration).map(|charge|
        ( charge, Boat::distance_travelled(charge,self.duration) )
    )
}
```

This approach works for small race durations but becomes inefficient for larger ones.

### Step 3: Mathematical Optimization

Using the quadratic formula, we can directly calculate the charge times that give exactly the record distance:

```
charge² - duration*charge + record = 0
```

The solutions to this equation are:
```
charge = (duration ± √(duration² - 4*record)) / 2
```

These solutions represent the boundaries where the distance equals the record. Any charge time between these bounds will beat the record.

```rust
pub(crate) fn find_lower_winning_charge(&self) -> u64 {
    let charge = (self.duration - u64::isqrt(u64::pow(self.duration,2) - 4*self.record)) / 2;
    self.find_winning_charge_between( (charge - 1) ..= (charge + 1) )
}

pub(crate) fn find_upper_winning_charge(&self) -> u64 {
    let charge = (self.duration + u64::isqrt(u64::pow(self.duration,2) - 4*self.record)) / 2;
    self.find_winning_charge_between( ((charge - 1) ..= (charge + 1)).rev() )
}
```

However, due to integer arithmetic and rounding errors, we need to check a few values around our calculated bounds to find the exact boundaries:

```rust
fn find_winning_charge_between(&self, mut time_range: impl Iterator<Item = u64>) -> u64 {
    let mut output = 0;
    time_range.any(|charge| {
        output = charge;
        self.record < Boat::distance_travelled(charge, self.duration)
    });
    output
}
```

This approach is visualized below:
```
          v -- True bounds -- v
----------*-------------------*-----------
         + -- lower, upper  -- +         <- approximate calculation due to rounding error
       |>>>| (lower-2..=lower+2)         <- direction lower -> upper
  (upper-2..=upper+2).rev()  |<<<|        <- direction upper -> lower
```

### Step 4: Parsing the Input

For Part 1, we parse the input into multiple races:

```rust
pub(crate) fn parse_races(input: &str) -> impl Iterator<Item=Race> + '_ {
    let mut split = input.split('\n');
    let time = split.next().unwrap().split(':').last().unwrap().split_ascii_whitespace();
    let dist = split.next().unwrap().split(':').last().unwrap().split_ascii_whitespace();
    time.zip(dist)
        .map(|(charge,dist)|
            (
                u64::from_str(charge).expect("duration:Ops!"),
                u64::from_str(dist).expect("best_dist:Ops!")
            ).into()
        )
}
```

For Part 2, we need to interpret the numbers differently:

```rust
pub(crate) fn parse_whole_numbers(input: &str) -> Result<Race,ParseIntError> {
    let mut split = input.split('\n');
    let time = split.next().unwrap().split(':').last().unwrap()
        .split_ascii_whitespace().flat_map(|c| c.chars()).collect::<String>();
    let dist = split.next().unwrap().split(':').last().unwrap()
        .split_ascii_whitespace().flat_map(|c| c.chars()).collect::<String>();

    Ok(Race {
        duration: u64::from_str(time.as_str())?,
        record: u64::from_str(dist.as_str())?
    })
}
```

### Step 5: Putting It All Together

To find the total number of ways to beat each record, we calculate:
```
total_ways = upper_bound - lower_bound + 1
```

This gives us the number of integer charge times that result in a winning race.

For Part 1, we multiply these counts across all races:
```rust
let product = races
    .map(|race|
        (race.find_upper_winning_charge(), race.find_lower_winning_charge())
    )
    .map(|(ub,lb)| ub-lb+1)
    .product::<u64>();
```

For Part 2, we apply the same logic to our single, much longer race:
```rust
let lb = race.find_lower_winning_charge();
let ub = race.find_upper_winning_charge();
println!("Part 2: Bounds {:?} -> {} - {:?}", (lb,ub), ub-lb+1, t.elapsed());
```

This mathematical approach allows us to efficiently solve both parts of the problem, even with very large race durations in Part 2.
