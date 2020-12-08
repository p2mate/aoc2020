use itertools::Itertools;
use std::collections::HashMap;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone)]
struct BagRule {
    inner: HashMap<String, usize>,
}
#[derive(Debug, Clone)]
struct BagRules {
    rules: HashMap<String, BagRule>,
}

impl BagRules {
    fn find_inner(&self, start_colour: &str, colour: &str) -> bool {
        let r = self.rules.get(start_colour).unwrap();

        if r.inner.contains_key(colour) {
            return true;
        } else {
            for inner_colour in r.inner.iter() {
                if self.find_inner(&inner_colour.0, colour) {
                    return true;
                }
            }
        }

        false
    }
    fn find(&self, colour: &str) -> Vec<String> {
        let mut result = Vec::new();
        for (name, _) in self.rules.iter() {
            if self.find_inner(name, colour) {
                result.push(name.clone());
            }
        }
        result
    }

    fn count_bags(&self, colour: &str) -> usize {
        self.rules
            .get(colour)
            .unwrap()
            .inner
            .iter()
            .fold(0, |acc, (name, count)| {
                acc + count * (self.count_bags(name) + 1)
            })
    }
    fn new<T: AsRef<Path>>(name: T) -> io::Result<Self> {
        let mut bag_rules = HashMap::new();
        for l in BufReader::new(std::fs::File::open(name)?).lines() {
            let line = l?;
            let line = line.split(' ');
            let name = line.clone().take(2).collect::<String>();
            let mut inner = HashMap::new();
            for i in line.skip(4).chunks(4).into_iter() {
                let rule = i.collect::<Vec<_>>();
                match rule[0] {
                    "no" => break,
                    x => {
                        let count = x.parse::<usize>().unwrap();
                        let name = rule[1].to_string() + rule[2];
                        inner.insert(name, count);
                    }
                }
            }
            bag_rules.insert(name.clone(), BagRule { inner });
        }

        Ok(BagRules { rules: bag_rules })
    }
}
fn main() -> io::Result<()> {
    let filename = std::env::args().skip(1).next().unwrap();
    let bag_rules = BagRules::new(filename)?;
    println!("{}", bag_rules.find("shinygold").len());
    println!(
        "{} bags required for shiny gold",
        bag_rules.count_bags("shinygold")
    );

    Ok(())
}
