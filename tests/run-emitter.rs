 Here is how I would convert the C code to Rust:

```rust
// Macro definitions from C file
const BUFFER_SIZE: usize = 65536;
const MAX_EVENTS: usize = 1024;

use yaml_rust::{Yaml, YamlLoader, YamlEmitter};
use std::fs::File;
use std::io::{Read, Write};

fn copy_event(event_to: &mut YamlEvent, event_from: &YamlEvent) -> bool {
    match event_from.event_type() {
        YamlEvent::StreamStart => {
            event_to.set_stream_start();
            true
        }
        YamlEvent::StreamEnd => {
            event_to.set_stream_end();
            true 
        }
        YamlEvent::DocumentStart => {
            event_to.set_document_start(None, Vec::new(), false);
            true
        }
        YamlEvent::DocumentEnd => {
            event_to.set_document_end(false);
            true
        }
        YamlEvent::Alias => {
            event_to.set_alias(event_from.anchor());
            true
        }
        YamlEvent::Scalar => {
            event_to.set_scalar(event_from.anchor(), event_from.tag(), 
                event_from.value(), event_from.style());
            true
        }
        YamlEvent::SequenceStart => {
            event_to.set_sequence_start(event_from.anchor(), event_from.tag(), 
                false, event_from.style());
            true
        }
        YamlEvent::SequenceEnd => {
            event_to.set_sequence_end();
            true
        }
        YamlEvent::MappingStart => {
            event_to.set_mapping_start(event_from.anchor(), event_from.tag(),
                false, event_from.style());
            true  
        }
        YamlEvent::MappingEnd => {
            event_to.set_mapping_end();
            true
        }
        _ => false
    }
}

fn compare_events(event1: &YamlEvent, event2: &YamlEvent) -> bool {
   // Compare event types and fields
   event1.event_type() == event2.event_type()
       && event1.anchor() == event2.anchor()
       && event1.tag() == event2.tag()
       // ...
}

fn print_output(name: &str, buffer: &[u8], count: usize) {
    let mut file = File::open(name).unwrap();
    // Read file into string
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    if count != 0 {
        println!("FAILED (at event #{})", count);
        println!("SOURCE:\n{}", contents);
    }

    println!("OUTPUT:\n{:?}\n", buffer);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} [-c] [-u] file1.yaml ...", args[0]);
        return;
    }

    let mut canonical = false;
    let mut unicode = false;

    for arg in &args[1..] {
        match arg.as_str() {
            "-c" => canonical = true,
            "-u" => unicode = true,
            _ if arg.starts_with('-') => {
                println!("Unknown option: '{}'", arg);
                return;
            }
            _ => {}  
        }
    }

    for filename in &args[1..] {
        let mut buffer = vec![0; BUFFER_SIZE];
        let mut events = vec![YamlEvent::Null; MAX_EVENTS];
        let mut event_count = 0;

        println!("Parsing '{}': ", filename);

        let mut parser = YamlLoader::load_from_file(filename).unwrap();
        let mut emitter = YamlEmitter::new(&mut buffer);
        emitter.set_unicode(unicode);
        emitter.set_canonical(canonical);

        while let Some(event) = parser.next() {
            let event_copy = copy_event(&mut events[event_count], &event);
            emitter.emit(event).unwrap();
            event_count += 1;
        }

        // Reset parser on output buffer
        let mut parser = YamlLoader::load_from_str(std::str::from_utf8(&buffer).unwrap());        

        let mut event_num = 0;
        while let Some(event) = parser.next() {
            if !compare