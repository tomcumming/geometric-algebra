use std::collections::BTreeSet;
use std::str::FromStr;

pub type VectorIndex = usize;

#[derive(Debug, PartialEq)]
pub struct Element(BTreeSet<VectorIndex>);

pub fn try_parse_element(name: &str) -> Option<Vec<VectorIndex>> {
    let mut iter = name.chars();
    if let Some('e') = iter.next() {
        let number_part: String = iter.take_while(|c| c.is_digit(10)).collect();
        if !number_part.is_empty() && (number_part == "0" || !number_part.starts_with('0')) {
            let idx = usize::from_str(&number_part).expect("Could not parse usize vector base");
            let rest = &name[number_part.len() + 1..];
            if rest.is_empty() {
                Some(vec![idx])
            } else {
                let mut idxs = vec![idx];
                idxs.append(&mut try_parse_element(rest)?);
                Some(idxs)
            }
        } else {
            None
        }
    } else {
        None
    }
}
