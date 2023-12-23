
#[derive(Debug,Clone,PartialEq)]
pub(crate) struct Galaxy {
    pub(crate) pos: (usize, usize)
}

impl Galaxy {
    pub(crate) fn shift_by(&mut self, delta: (usize, usize)) {
        self.pos.0 += delta.0;
        self.pos.1 += delta.1;
    }
    pub(crate) fn distance_to(&self, dst: &Galaxy) -> usize {
        // Using the Manhattan distance formula
        dst.pos.0.abs_diff(self.pos.0) + dst.pos.1.abs_diff(self.pos.1)
    }
}

#[cfg(test)]
mod test {
    use crate::universe::Universe;

    #[test]
    fn test_galaxy_distance() {
        let input = std::fs::read_to_string("src/bin/day11/sample.txt").expect("Ops!");
        let mut universe = input.parse::<Universe>().expect("Failed to parse Universe!");

        let cluster = &universe.expand(2).clusters;

        assert_eq!(9, cluster[4].distance_to(&cluster[8]));
        assert_eq!(15, cluster[0].distance_to(&cluster[6]));
        assert_eq!(17, cluster[2].distance_to(&cluster[5]));
        assert_eq!(5, cluster[7].distance_to(&cluster[8]));
    }
}