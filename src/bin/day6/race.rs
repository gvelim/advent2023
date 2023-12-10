use std::num::ParseIntError;
use std::str::FromStr;

pub(crate) struct Boat;
impl Boat {
    pub(crate) fn distance_travelled(charge: u64, duration: u64) -> u64 {
        // what is the min charge to win the race?
        // (duration - x)*x = Winning_distance
        // => x^2 - duration * x + Winning_distance = 0
        (duration - charge) * charge
    }
}

#[derive(Debug,PartialEq)]
pub(crate) struct Race {
    pub(crate) duration: u64,
    pub(crate) record: u64
}

impl  Race {
    pub(crate) fn trial_charge_times(&self) -> impl Iterator<Item=(u64, u64)> + '_ {
        (0..=self.duration).map(|charge|
            ( charge, Boat::distance_travelled(charge,self.duration) )
        )
    }
    pub(crate) fn winning_charge_times(&self) -> impl Iterator<Item=(u64, u64)> + '_ {
        self.trial_charge_times().filter(|&(_,dist)| dist > self.record)
    }

    pub(crate) fn find_lower_winning_charge(&self) -> u64 {
        let charge = (self.duration - u64::isqrt(u64::pow(self.duration,2) - 4*self.record)) / 2;
        let mut range = (charge - 2) ..= (charge + 2);
        let mut output = 0;
        range.any(|charge| {
            output = charge;
            self.record < Boat::distance_travelled(charge as u64, self.duration)
        });
        output as u64
    }

    pub(crate) fn find_upper_winning_charge(&self) -> u64 {
        let charge = (self.duration + u64::isqrt(u64::pow(self.duration,2) - 4*self.record)) / 2;
        let mut range = ((charge - 2) ..= (charge + 2)).rev();
        let mut output = 0;
        range.any(|charge| {
            output = charge;
            self.record < Boat::distance_travelled(charge as u64, self.duration)
        });
        output as u64
    }

    pub(crate) fn parse_races(input: &str) -> impl Iterator<Item=Race> + '_{
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
    pub(crate) fn parse_whole_numbers(input: &str) -> Result<Race,ParseIntError> {
        let mut split = input.split('\n');
        let time = split.next().unwrap().split(':').last().unwrap()
            .split_ascii_whitespace().map(|c| c.chars()).flatten().collect::<String>();
        let dist = split.next().unwrap().split(':').last().unwrap()
            .split_ascii_whitespace().map(|c| c.chars()).flatten().collect::<String>();

        Ok(Race {
            duration: u64::from_str(time.as_str())?,
            record: u64::from_str(dist.as_str())?
        })
    }
}

impl From<(u64,u64)> for Race {
    fn from(value: (u64, u64)) -> Self {
        let (duration, record) = value;
        Race { duration, record }
    }
}
