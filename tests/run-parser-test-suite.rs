 Here is how I would convert the C code to Rust:

```rust
use std::{fs::File, io::Read};
use yaml_rust::{Yaml, YamlLoader};

fn print_escaped(s: &str) {
    for c in s.chars() {
        match c {
            '\\' => print!("\\\\"),
            '\0' => print!("\\0"),
            '\x08' => print!("\\b"),
            '\n' => print!("\\n"),
            '\r' => print!("\\r"), 
            '\t' => print!("\\t"),
            _ => print!("{}", c),
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut flow = None;
    let mut input = std::io::stdin();

    let mut i = 1;
    while i < args.len() {
        if args[i] == "--flow" {
            i += 1;
            let flow_str = &args[i];
            if flow_str == "keep" {
                flow = Some(false);
            } else if flow_str == "on" {
                flow = Some(true);
            } else if flow_str == "off" {
                flow = None; 
            } else {
                eprintln!("Invalid flow style");
                return;
            }
        } else if args[i] == "--help" || args[i] == "-h" {
            println!("Usage: libyaml-parser [--flow (on|off|keep)] [<input-file>]");
            return;
        } else {
            input = File::open(&args[i]).unwrap();
        }
        i += 1;
    }

    let docs = YamlLoader::load_from_reader(input).unwrap();

    for doc in docs {
        println!("+DOC");
        match doc {
            Yaml::Hash(map) => {
                println!("+MAP");
                for (k, v) in map {
                    print!("=VAL ");
                    print_escaped(k);
                    print!(": ");
                    print_escaped(v);
                    println!("");
                }
                println!("-MAP");
            }
            Yaml::Array(seq) => {
                println!("+SEQ");
                for v in seq {
                    print!("=VAL ");
                    print_escaped(&v.to_string());
                    println!("");
                }
                println!("-SEQ");
            }
            _ => (),
        }
        println!("-DOC");
    }
}
```

The key differences:

- Use the yaml-rust crate to parse YAML instead of libyaml
- Print events instead of using the YAML event API
- Handle arguments and file opening manually instead of with getopt
- Use Rust strings instead of yaml_char_t
- No need to manually delete events
- Use idiomatic Rust style (enums instead of ints for flow style)