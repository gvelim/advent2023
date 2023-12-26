fn main() {

}

#[cfg(test)]
mod test {
    #[test]
    fn test_parse() {

        let damaged = "#.#.###";
        let record = [1,1,3usize];

        let diter = damaged.split('.');
        let mut riter = record.iter();

        println!("{:?}",diter
            .inspect(|d| print!("{:?}->",d) )
            .map(|p|
                riter.next().is_some_and(|v| p.len().eq(v))
            )
            .inspect(|d| println!("{:?}",d) )
            .all(|m| m)
        );
    }

}