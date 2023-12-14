use std::collections::HashMap;
fn main() {
    let input = if let Ok(s) = std::fs::read_to_string("input.txt") {
        s
    } else {
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
            .to_owned()
    };
    let input = input
        .lines()
        .map(|line| line.split_once(" | ").unwrap());
    let input = input
        .collect::<Vec<(&str, &str)>>();
    let signals = input
        .iter()
        .map(|(sig, _)| sig.split_ascii_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<_>>();
    let indications = input
        .iter()
        .map(|(_, ind)| ind.split_ascii_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<_>>();
    println!("Part 1: {}", part1(&indications));
    decode_display(&signals[0], &indications[0]);
}

fn part1(indications: &Vec<Vec<&str>>) -> usize {
    indications
        .iter()
        .map(|s| {
            s.iter()
                .map(|v| v.len())
                .filter(|c| *c == 2 || *c == 7 || *c == 4 || *c == 3)
                .count()
        })
        .sum()
}

fn decode_display(signal: &Vec<&str>, indication: &Vec<&str>) -> i32 {
    //acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf
    let tmp_s = vec![
        "acedgfb", "cdfbe", "gcdfa", "fbcad", "dac", "cefabd", "eafb", "cagedb", "ab",
    ];
    let tmp_i = vec!["cdfeb", "fcadb", "cdfeb", "cdbaf"];
    let mut mapped: HashMap<char, Vec<char>> = HashMap::new();
    let known = tmp_i
        .iter()
        .filter(|c| c.len() == 2 || c.len() == 7 || c.len() == 4 || c.len() == 3)
        .map(|&c| {
            (
                c,
                tmp_s
                    .iter()
                    .filter(|&s| s.len() == c.len())
                    .map(|c| *c)
                    .collect::<Vec<&str>>(),
            )
        })
        .collect::<HashMap<&str, Vec<&str>>>();

    dbg!(&known);
    for digit in known {}
    0
}

fn map_values(mapped: &mut HashMap<char, Vec<char>>) -> Result<i32, ()> {
    Err(())
}
