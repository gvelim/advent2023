# Day 19: Aplenty

## Input

The Elves of Gear Island are thankful for your help and send you on your way. They even have a hang glider that someone stole from Desert Island; since you're already going that direction, it would help them a lot if you would use it to get down there and return it to them.

As you reach the bottom of the relentless avalanche of machine parts, you discover that they're already forming a formidable heap. Don't worry, though - a group of Elves is already here organizing the parts, and they have a system.

To start, each part is rated in each of four categories:
```
x: Extremely cool looking
m: Musical (it makes a noise when you hit it)
a: Aerodynamic
s: Shiny
```
Then, each part is sent through a series of workflows that will ultimately accept or reject the part. Each workflow has a name and contains a list of rules; each rule specifies a condition and where to send the part if the condition is true. The first rule that matches the part being considered is applied immediately, and the part moves on to the destination described by the rule. (The last rule in each workflow has no condition and always applies if reached.)

Consider the workflow ```ex{x>10:one,m<20:two,a>30:R,A}```. This workflow is named ex and contains four rules. If workflow ex were considering a specific part, it would perform the following steps in order:

- Rule "`x>10:one`": If the part's x is more than 10, send the part to the workflow named one.
- Rule "`m<20:two`": Otherwise, if the part's m is less than 20, send the part to the workflow named two.
- Rule "`a>30:R`": Otherwise, if the part's a is more than 30, the part is immediately **rejected** (R).
- Rule "`A`": Otherwise, because no other rules matched the part, the part is immediately **accepted** (A).

If a part is sent to another workflow, it immediately switches to the start of that workflow instead and never returns. If a part is **accepted** (sent to A) or **rejected** (sent to R), the part immediately stops any further processing.

## Part 1

The system works, but it's not keeping up with the torrent of weird metal shapes. The Elves ask if you can help sort a few parts and give you the list of workflows and some part ratings (your puzzle input). For example:

```
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
```

The workflows are listed first, followed by a blank line, then the ratings of the parts the Elves would like you to sort. All parts begin in the workflow named in. In this example, the five listed parts go through the following workflows:

- {x=787,m=2655,a=1222,s=2876}: in -> qqz -> qs -> lnx -> A
- {x=1679,m=44,a=2067,s=496}: in -> px -> rfg -> gd -> R
- {x=2036,m=264,a=79,s=2244}: in -> qqz -> hdj -> pv -> A
- {x=2461,m=1339,a=466,s=291}: in -> px -> qkq -> crn -> R
- {x=2127,m=1623,a=2188,s=1013}: in -> px -> rfg -> A

Ultimately, three parts are **accepted**. Adding up the x, m, a, and s rating for each of the accepted parts gives `7540` for the part with `x=787`, `4623` for the part with `x=2036`, and `6951` for the part with `x=2127`. Adding all of the ratings for **all** of the accepted parts gives the sum total of `19114`.

Sort through all of the parts you've been given; **what do you get if you add together all of the rating numbers for all of the parts that ultimately get accepted**?

## Part 2
Even with your help, the sorting process **still** isn't fast enough.

One of the Elves comes up with a new plan: rather than sort parts individually through all of these workflows, maybe you can figure out in advance which combinations of ratings will be accepted or rejected.

Each of the four ratings (x, m, a, s) can have an integer value ranging from a minimum of `1` to a maximum of `4000`. Of **all possible distinct combinations of ratings**, your job is to figure out which ones will be **accepted**.

In the above example, there are `167409079868000` distinct combinations of ratings that will be accepted.

Consider only your list of workflows; the list of part ratings that the Elves wanted you to sort is no longer relevant. **How many distinct combinations of ratings will be accepted by the Elves' workflows?**

## Approach
### Workflow and Rules
We are told a `Workflow` is a collection of rules. A `Rule` is a (`Condition`,`Action`) pair **or** just an `Action`. A `Condition` consists of 3 parts, and is followed by an `Action` that is triggered when the `Condition` evaluates to `true`. Action values are "Accept", "Reject" or "continue to another workflow".
```
Rule: {
  ([part variable] [Operant] [Value] : [Action]) | ([Action])
}
e.g. m < 2000 : A, x > 100 : R , s > 100 : xyz, A, R, xyz
```
We use the below enums and structs in order to capture the above `Rule` definition:
```rust
type Unit = usize;

enum PartVar { X = 0, M, A, S }

enum Operant { GT, LT }

struct Condition {
    var: PartVar,
    operant: Operant,
    value: Unit,
}

enum Action {
    WorkFlow(Rc<str>),
    Accept,
    Reject,
}

pub(crate) enum Rule {
    ConAct(Condition, Action),
    Act(Action),
}
```
Subsequently a `Worklflow` becomes a structure that holds its `name` along with a collection of `rules`
```rust
struct Workflow {
    rules: Rc<[Rule]>,
    name: Rc<str>,
}
```
### Part 1: Sorting System and part processing
Each part of fed through a `SortingSystem` that has a collection of workflows used to process a part and either accept or reject it. The below struct holds a collection of workflows in a `HashMap`
```rust
SortingSystem {
    map: HashMap<Rc<str>, Workflow>
}
```
The SortingSystem applies the following processing logic:
* If a part is sent to another workflow, it immediately switches to the start of that workflow instead and never returns.
* If a part is accepted (sent to A) or rejected (sent to R), the part immediately stops any further processing.

Which is implemented by the below functions
```rust
impl Condition {
...
    fn validate(&self, part: Part) -> bool {
        match (&self.var, &self.operant) {
            (PartVar::X, Operant::GT) => part.x > self.value,
            (PartVar::X, Operant::LT) => part.x < self.value,
            (PartVar::M, Operant::GT) => part.m > self.value,
            (PartVar::M, Operant::LT) => part.m < self.value,
            (PartVar::S, Operant::GT) => part.s > self.value,
            (PartVar::S, Operant::LT) => part.s < self.value,
            (PartVar::A, Operant::GT) => part.a > self.value,
            (PartVar::A, Operant::LT) => part.a < self.value,
        }
    }
...
}

impl Rule {
    fn validate(&self, part: Part) -> Option<Action> {
        match self {
            Rule::ConAct(c, a) if c.validate(part) => Some(a.clone()),
            Rule::Act(a) => Some(a.clone()),
            _ => None
        }
    }
}

impl Workflow {
...
    fn validate(&self, part: Part) -> Option<Action> {
        // The first rule that matches the part being considered is applied immediately,
        // and the part moves on to the destination described by the rule
        self.iter()
            .filter_map(|rule| rule.validate(part))
            .next()
    }
}

impl SortingSystem {
    fn process_part(&self, part: Part, workflow: &str) -> Option<Action> {
        let mut wf = self.map
            .get(workflow)
            .expect("SortingSystem::process() - Starting workflow unknown!!");

        while let Some(Action::WorkFlow(next)) = wf.validate(part) {
            wf = self.map
                .get(&next)
                .expect("SortingSystem::process() - redirected to non-existent Workflow");
        }
        wf.validate(part)
    }
...
}
```
Therefore, extracting the `sum()` of all accepted parts can be realised with the following implementation
```rust
let sum = parts.iter()
    .filter(|&&part|
        system.process_part(part, "in") == Some(Action::Accept)
    )
    .map(|part| part.sum())
    .sum::<Unit>();
```
### Part 2: Distinct Combinations
To find the unique combinations, we make the following observations
1. We apply a **tree-like search**
2. with `Actions` forming the main **nodes**
3. and `Conditions` form the tree **branches**.
4. **Terminal** nodes formed by the actions `Accept` and `Reject`
5. **Transition** nodes formed by actions of type `Workflow`

Therefore, when processing a
1. **terminal node**, with
    1. `Accept`, we return the `product()` of ranges arrising for that specific search path
    2. `Reject`, we return `0`
2. **transition node**,
    1. we iterate over the workflow's rules and against the node's input ranges
    2. process each rule sequencially, with each rule **reducing/consuming** the relevant range by the **conditional amount**
    For example, applying rule `x<100:R` on the range `1..4001` will result to (a) 1..99 range be rejected, (b) leaving range 100..4001 for processing by the next rule

The below example explains the above observations
```
in{s<1000:R,s<2000:abc,A}
abc{x<100:A,m<200:A,R}
                              in
                    [1..5,1..5,1..5,1..5]
                       x    m    a    s
                              |
                             112
            +-----------------+-------------------+
            | s<2             | s<4               |
            R                abc                  A
      [ , , ,1..2]       [ , , ,2..4]        [ , , ,4..5]
            0                 48             4 * 4 * 4 * 1
                              |
            +-----------------+-------------------+
            | x<2             | m<2               |
            A                 A                   R
    [1..2, , ,2..4]   [2..5,1..2, ,2..4]   [2..5,1..2, ,2..4]
     1 * 4 * 4 * 2       2 * 1 * 4 * 2            0
           32                 16                  0
```

Therefore, we enhance `Condition` with a function `partition()` that, given (a) an input `Range` and (b) condition, it returns a pair of `(Target, Residual)` ranges.
```rust
impl Condition {
...
    fn partition(&self, rng: &Range<Unit>) -> (Range<Unit>,Range<Unit>) {
        if rng.contains(&self.value) {
            match self.operant {
                Operant::GT => (self.value+1..rng.end, rng.start..self.value+1),
                Operant::LT => (rng.start..self.value, self.value..rng.end ),
            }
        } else {
            panic!("Condition::partition - value out of input range")
        }
    }
}
```
For example, when the above function is called with an input range `1..4001` and with curretn Rule `m<2000:A`, the `partition()` function will produce a (target,residual) range pair, with values
1. **target** range `1..1999` that will be `Accepted`
2. **residual** range `2000..4000` for use with the next workflow rule

We can now implement a **tree-like search** function that (a) traverses the tree, (b) extracts and (c) sums up the products of the ranges accepted.
```rust
impl SortingSystem {
...
    fn total_combinations(&self, wf: &str, rngs: &[Range<Unit>; 4]) -> Unit {
        let mut residual = rngs.clone();

        self.map
            .get(wf)
            .expect("System::total_combinations - Workflow name doesn't exist")
            .iter()
            // process rule against input ranges
            .map(|rule| {
                // current ranges becomes the target
                let mut target = residual.clone();
                // Process rule into "Action" & "target" part ranges
                match rule {
                    // Process Conditional rule into "target" and "remaining" ranges
                    Rule::ConAct(c, a) => {
                        let part = c.part() as usize;
                        // partition part range and update "target" and "remaining" accordingly
                        (target[part], residual[part]) = c.partition(&residual[part]);
                        (a, target)
                    },
                    // Pass-through action and target part ranges
                    Rule::Act(a) => (a, target),
                }
            })
            // process action against target range
            .map(|(a,target)|{
                // Process Action given "target" part ranges
                match a {
                    Action::WorkFlow(next_wf) => self
                        .total_combinations(next_wf, &target),
                    Action::Accept => target
                        .iter()
                        .map(|r| r.len() as Unit)
                        .product(),
                    Action::Reject => 0,
                }
            })
            .sum::<Unit>()
    }
}
```
