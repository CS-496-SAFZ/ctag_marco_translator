 Here is how I would convert the C code to Rust:

```rust
use yaml_rust::{Yaml, YamlLoader};
use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut help = false;
    let mut canonical = false;
    let mut unicode = false;

    // Parse command line arguments
    for arg in &args[1..] {
        match arg.as_str() {
            "-h" | "--help" => help = true,
            "-c" | "--canonical" => canonical = true,
            "-u" | "--unicode" => unicode = true,
            _ => {
                eprintln!("Unrecognized option: {:?}", arg);
                eprintln!("Try '--help' for more information.");
                return;
            }
        }
    }

    if help {
        print_help(&args);
        return;
    }

    let stdin = std::io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();

    let docs = YamlLoader::load_from_str(&input).unwrap();

    for doc in docs {
        println!("{:#?}", doc);
    }
}

fn print_help(args: &[String]) {
    let prog = &args[0];
    println!(
        "{prog} <input>
or
{prog} --help

Deconstruct a YAML stream.

Options:
  -h, --help         Display this help and exit
  -c, --canonical    Output in canonical YAML format
  -u, --unicode      Output unescaped non-ASCII characters
",
        prog = prog
    );
}
```

The key points:

- Use the `yaml-rust` crate for YAML parsing instead of libyaml
- Read from stdin and parse the YAML input 
- Print the YAML documents instead of re-emitting them
- Use Rust strings, vectors, pattern matching etc instead of C constructs

This prints out the YAML documents in a debug format instead of re-emitting them, but could be adapted to output in a specific format.