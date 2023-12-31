use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Workflow {
    name: String,
    flow: Vec<(String, cmp::Ordering, usize, String)>,
    default: String,
}

impl Workflow {
    fn parse(line: &str) -> Self {
        let line = line
            .replace('{', ",")
            .replace('}', "")
            .split(',')
            .map(|s| s.to_owned())
            .collect::<Vec<_>>();
        let name = line[0].to_string();
        let flow = line[1..line.len() - 1]
            .iter()
            .map(|l| {
                let (category, compare, rest) = l.split_once('<').map_or_else(
                    || {
                        l.split_once('>')
                            .map(|(c, v)| (c, cmp::Ordering::Greater, v))
                            .unwrap()
                    },
                    |(c, v)| (c, cmp::Ordering::Less, v),
                );

                let (value, target) = rest.split_once(':').unwrap();
                (
                    category.to_owned(),
                    compare,
                    value.parse().unwrap(),
                    target.to_string(),
                )
            })
            .collect::<Vec<_>>();
        let default = line[line.len() - 1].to_string();

        Workflow {
            name,
            flow,
            default,
        }
    }
}

/// https://adventofcode.com/2023/day/19
/// Day 19: Aplenty
fn solve(path: &str) -> io::Result<usize> {
    let file = File::open(path)?;
    let mut lines = BufReader::new(file).lines();

    let mut workflows = HashMap::new();
    while let Some(Ok(line)) = lines.next() {
        if line.is_empty() {
            break;
        }
        // parse workflows
        let workflow = Workflow::parse(&line);
        workflows.insert(workflow.name.clone(), workflow);
    }

    let mut rating = 0;
    for x in 1..=4000 { for m in 1..=4000 { for a in 1..=4000 { for s in 1..=4000 {
        let parts: HashMap<String, usize> = HashMap::from_iter([
            ("x".into(), x),
            ("m".into(), m),
            ("a".into(), a),
            ("s".into(), s),
        ]);


        let mut next = "in";
        'outer: while let Some(workflow) = workflows.get(next) {
            let mut matched = false;
            for (c, compare, expected, t) in &workflow.flow {
                let v = parts.get(c.as_str()).unwrap();
                if v.cmp(expected) == *compare {
                    matched = true;
                    match t.as_str() {
                        "A" => {
                            rating += 1;
                            break 'outer;
                        }
                        "R" => break 'outer,
                        _ => {
                            next = t;
                            break;
                        }
                    }
                }
            }

            // default
            if !matched {
                match workflow.default.as_str() {
                    "A" => {
                        rating += 1;
                        break;
                    }
                    "R" => break,
                    _ => next = &workflow.default,
                }
            }
        }
    }}}}

    Ok(rating)
}

fn main() {
    println!("{}", solve("./input/day19.txt").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        assert_eq!(solve("./input/day19.sample.txt").unwrap(), 167409079868000);
    }
}
