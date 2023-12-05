use std::ops::{RangeInclusive};

fn main() {

}
#[derive(Debug)]
struct EngineSchematic {
    len: usize,
    schematic: String
}

#[derive(Debug)]
struct PartNumber {
    value: u32,
    range: RangeInclusive<usize>
}

mod test {
    use super::*;

    static INPUT: &str =
        "467..114..\n\
        ...*......\n\
        ..35..633.\n\
        ......#...\n\
        617*......\n\
        .....+.58.\n\
        ..592.....\n\
        ......755.\n\
        ...$.*....\n\
        .664.598..";
    // "467..114.....*........35..633.......#...617*...........+.58...592...........755....$.*.....664.598.."
    #[test]
    fn test_parse_engine_schematic() {
        let input = INPUT.lines();
        let mut len = 0;
        let es = EngineSchematic {
            schematic: input.inspect(|l| {len = l.len();}).flat_map(|d| d.chars()).collect::<String>(),
            len,
        };

        let EngineSchematic {len, schematic} = es;

        let mut pns: Vec<PartNumber> = vec![];
        let mut smb: Vec<(usize,char)> = vec![];
        let mut buf = vec![];

        schematic.char_indices()
            .for_each(|c| {
                match c.1 {
                    '.' => {
                        if !buf.is_empty() {
                            let number = buf.iter().map(|(_,c)| c).collect::<String>();
                            pns.push(PartNumber {
                                value: u32::from_str_radix(number.as_str(), 10).expect("Ops!"),
                                range: (buf[0].0 ..= buf.last().expect("Ops").0),
                            });
                            buf.clear();
                        }
                    }
                    '0'..='9' => buf.push(c),
                    _ => smb.push(c),
                }
            });
        println!("{:?}\n{:?}", pns, smb);

        let sum = pns.iter()
            .filter(|pn| {
                smb.iter().any(|(pos,_)| {
                    pn.range.contains(&(pos + 1))
                        || pn.range.contains(&(pos - 1))
                        || pn.range.contains(&(pos + len-1))
                        || pn.range.contains(&(pos + len))
                        || pn.range.contains(&(pos + len+1))
                        || pn.range.contains(&(pos - len+1))
                        || pn.range.contains(&(pos - len))
                        || pn.range.contains(&(pos - len-1))
                })
            })
            .inspect(|pn| print!("F::{:?}", pn))
            .map(|pn| pn.value)
            .sum::<u32>();

        assert_eq!(sum,4361)
    }



}