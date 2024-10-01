use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fmt::{Debug, Display, Formatter},
};

use petgraph::{dot::Dot, prelude::NodeIndex, Graph};

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn dotfile(input: String) -> String {
    let mut items: HashMap<String, HashSet<String>> = HashMap::new();

    for line in input.lines() {
        let Some((name, value)) = line.split_once(':') else {
            continue;
        };
        let name = name.trim().to_string();
        let value: HashSet<String> = value
            .split(',')
            .map(|v| v.trim())
            .map(ToString::to_string)
            .collect();
        match items.entry(name.clone()) {
            Entry::Occupied(mut entry) => {
                for item in value {
                    entry.get_mut().insert(item);
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(value);
            }
        }
    }

    let mut thoughts = Graph::<String, Undisplayable>::new();

    let mut nodes: HashMap<String, NodeIndex> = HashMap::new();
    for (key, values) in &items {
        if !nodes.contains_key(key) {
            let key_node = thoughts.add_node(key.clone());
            nodes.insert(key.clone(), key_node);
        }
        for topic in values {
            if !nodes.contains_key(topic) {
                let topic_node = thoughts.add_node(topic.clone());
                nodes.insert(topic.clone(), topic_node);
            }
        }
    }

    for (k, tags) in items {
        let key = nodes.get(&k).unwrap_or_else(|| panic!("Unknown key {k}"));

        let connect_to = tags
            .iter()
            .map(|v| nodes.get(v).unwrap_or_else(|| panic!("Unknown key {v}")));

        for connection in connect_to {
            thoughts.add_edge(*key, *connection, Undisplayable);
        }
    }
    Dot::new(&thoughts).to_string()
}

#[derive(Debug)]
struct Undisplayable;

impl Display for Undisplayable {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}
