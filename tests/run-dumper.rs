 Here is the C code converted to Rust:

```rust
use yaml_rust::{yaml, YamlLoader};
use std::fs::File;
use std::io::{Read, Write};

const BUFFER_SIZE: usize = 65536;
const MAX_DOCUMENTS: usize = 16;

fn copy_document(document_to: &mut yaml::Document, document_from: &yaml::Document) -> bool {
    for node in &document_from.nodes {
        match node {
            yaml::Node::Scalar(scalar) => {
                if !document_to.add_scalar(scalar.tag.clone(), scalar.value.clone(), scalar.style) {
                    return false;
                }
            }
            yaml::Node::Sequence(sequence) => {
                if !document_to.add_sequence(sequence.tag.clone(), sequence.style) {
                    return false;
                }
            }
            yaml::Node::Mapping(mapping) => {
                if !document_to.add_mapping(mapping.tag.clone(), mapping.style) {
                    return false;
                }
            }
        }
    }

    for node in &document_from.nodes {
        match node {
            yaml::Node::Sequence(sequence) => {
                for item in &sequence.items {
                    if !document_to.append_sequence_item(item.clone()) {
                        return false;
                    }
                }
            }
            yaml::Node::Mapping(mapping) => {
                for pair in &mapping.pairs {
                    if !document_to.append_mapping_pair(pair.key.clone(), pair.value.clone()) {
                        return false;
                    }
                }
            }
            _ => {}
        }
    }

    true
}

fn compare_nodes(document1: &yaml::Document, index1: usize, 
                 document2: &yaml::Document, index2: usize, level: u32) -> bool {

    if level > 1000 {
        return false;
    }

    let node1 = document1.get_node(index1);
    let node2 = document2.get_node(index2);

    if node1.is_none() || node2.is_none() {
        return false;
    }

    let node1 = node1.unwrap();
    let node2 = node2.unwrap();

    if node1.node_type() != node2.node_type() {
        return false;
    }

    if node1.tag != node2.tag {
        return false;
    }

    match (node1, node2) {
        (yaml::Node::Scalar(node1), yaml::Node::Scalar(node2)) => {
            if node1.value != node2.value {
                return false;
            }
        }
        (yaml::Node::Sequence(node1), yaml::Node::Sequence(node2)) => {
            if node1.items.len() != node2.items.len() {
                return false;
            }
            for i in 0..node1.items.len() {
                if !compare_nodes(document1, node1.items[i], document2, node2.items[i], level + 1) {
                    return false;
                }
            }
        }
        (yaml::Node::Mapping(node1), yaml::Node::Mapping(node2)) => {
            if node1.pairs.len() != node2.pairs.len() {
                return false;
            }
            for i in 0..node1.pairs.len() {
                if !compare_nodes(document1, node1.pairs[i].key, document2, node2.pairs[i].key, level + 1) {
                    return false;
                }
                if !compare_nodes(document1, node1.pairs[i].value, document2, node2.pairs[i].value, level + 1) {
                    return false;
                }
            }
        }
        _ => return false,
    }

    true
}

fn compare_documents(document1: &yaml::Document, document2: &yaml::Document) -> bool {

    if document1.version_directive != document2.version_directive {
        return false;
    }

    if document1.tag_directives.len() != document2.tag_directives.len() {
        return false;
    }
    for i in 0..document1.tag_directives.len() {
        if document1.tag_directives[i] != document2.tag_directives[i] {
            return false;
        }
    }

    if document1.nodes.len() != document2.nodes.len() {
        return false;
    }

    if !document1.nodes.is_empty() {