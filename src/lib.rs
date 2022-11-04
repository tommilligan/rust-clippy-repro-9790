use std::collections::HashMap;

pub fn example() {
    let mut parent = HashMap::new();
    let grandchild = parent.entry(2).or_insert(Vec::new());
    grandchild.push(-1);
}
