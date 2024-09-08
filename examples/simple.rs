use std::collections::HashMap;
use std::hash::Hash;

fn count_items<T>(iter: impl IntoIterator<Item = T>) -> HashMap<T::Owned, usize>
where
    T: ToOwned,
    T::Owned: Hash + Eq,
{
    let mut counts = HashMap::new();
    for x in iter {
        *counts.entry(x.to_owned()).or_default() += 1;
    }
    counts
}

fn main() {
    println!("{:?}", count_items("abracadabra".chars()));
}
