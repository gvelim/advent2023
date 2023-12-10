use std::str::FromStr;

struct Boat;
impl Boat {
    fn distance_travelled(charge: u32, duration: u32) -> u32 {
        (duration - charge) * charge
    }
}

#[derive(Debug,PartialEq)]
pub(crate) struct Race {
    duration: u32,
    record: u32
}

impl  Race {
    pub(crate) fn trial_charge_times(&self) -> impl Iterator<Item=(u32, u32)> + '_ {
        (0..=self.duration).map(|charge|
            ( charge, Boat::distance_travelled(charge,self.duration) )
        )
    }
    pub(crate) fn winning_charge_times(&self) -> impl Iterator<Item=(u32, u32)> + '_ {
        self.trial_charge_times().filter(|&(_,dist)| dist > self.record)
    }

    pub(crate) fn parse_races(input: &str) -> impl Iterator<Item=Race> + '_{
        let mut split = input.split('\n');
        let time = split.next().unwrap().split(':').last().unwrap().split_ascii_whitespace();
        let dist = split.next().unwrap().split(':').last().unwrap().split_ascii_whitespace();
        time.zip(dist)
            .map(|(charge,dist)|
                (
                    u32::from_str(charge).expect("duration:Ops!"),
                    u32::from_str(dist).expect("best_dist:Ops!")
                ).into()
            )
    }
}

impl From<(u32,u32)> for Race {
    fn from(value: (u32, u32)) -> Self {
        let (duration, record) = value;
        Race { duration, record }
    }
}
