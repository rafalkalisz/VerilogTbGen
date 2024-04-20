use std::{env, usize};
use std::fs::File;
use std::io::{self, BufRead, LineWriter, Write};

#[derive(PartialEq, Debug, Clone)]
enum PortDirection {
    INPUT,
    OUTPUT,
}

#[derive(Debug, Clone)]
struct ModulePort {
    port_direction: PortDirection,
    port_length: usize,
    port_name: String,
}

impl ModulePort {
    pub fn new(line: String) -> Self {
        let mut line = line;
        if !line.ends_with(',') {
            line.push(',');
        }
        let parts: Vec<&str> = line.split_whitespace().collect();

        let port_direction = match parts.get(0) {
            Some(&"input") => PortDirection::INPUT,
            Some(&"output") => PortDirection::OUTPUT,
            _ => PortDirection::INPUT,
        };

        let (port_length, port_name) = match parts.len() {
            2 => (Some(1), parts[1].trim_end_matches(',')),
            3 => {
                let brackets = parts[1].trim_matches(|c| c == '[' || c == ']');
                let mut xy = brackets.split(':');
                let x = xy.next().and_then(|s| s.parse::<usize>().ok()).unwrap_or(1);
                let name = parts[2].trim_end_matches(',');
                (Some(x), name)
            },
            4 => {
                let brackets = parts[2].trim_matches(|c| c == '[' || c == ']');
                let mut xy = brackets.split(':');
                let x = xy.next().and_then(|s| s.parse::<usize>().ok()).unwrap_or(1);
                let name = parts[3].trim_end_matches(',');
                (Some(x), name)
            }
            _ => (Some(1), ""),
        };
        Self { port_direction: port_direction, port_length: port_length.unwrap(), port_name: port_name.to_string()} 
    }

    pub fn get_testbench_line(self) -> String {
        let mut line = "    ".to_string(); // Evil indent
        match self.port_direction {
            PortDirection::INPUT => line += "reg ",
            PortDirection::OUTPUT => line += "wire ",
        }
        if self.port_length != 1 {
            line = line + "[" + self.port_length.to_string().as_str() + ":0] ";
        }
        line = line + self.port_name.to_string().as_str() + ";\n";

        line.to_string()
        
    }
}

fn main() {

    let args:Vec<String> = env::args().collect();
    let input_file_name = &args[1];

    let input_file = File::open(input_file_name).unwrap();
    let reader = io::BufReader::new(input_file);

    let output_file_name = input_file_name.trim_end_matches(".v").to_string() + "_tb.v";
    let mut module_ports: Vec<ModulePort> = Vec::new();

    let lines_iter = reader.lines().map(|line| line.unwrap());
    let mut module_name: String = String::new();
    for line in lines_iter {
        if line.trim().starts_with("module") {
            let parts:Vec<&str> = line.split_whitespace().collect();
            module_name = parts[1].trim_end_matches('(').to_string();
        } else if line.trim().starts_with("input") || line.trim().starts_with("output") {
            module_ports.push(ModulePort::new(line));
        }
    }

    let output_file = File::create(output_file_name).expect("Unable to create file");
    let mut output_file = LineWriter::new(output_file);
    let testbench_module_line = String::new() + "module " + module_name.as_str() + "_tb;\n\n";
    output_file.write_all(testbench_module_line.as_bytes()).expect("Unable to write to file");
    for module_port in module_ports.clone() {
        output_file.write_all(module_port.get_testbench_line().as_bytes()).unwrap();
    }
    let dut_line = String::new() + "\n    " + module_name.as_str() + " DUT (\n";
    output_file.write_all(dut_line.as_bytes()).unwrap();
    let mut module_port_it = module_ports.iter().peekable();
    while let Some(module_port) = module_port_it.next()  {
        let mut port_map_line = String::new() + "      ." + module_port.port_name.as_str() + "(" + module_port.port_name.as_str() + ")";
        if module_port_it.peek().is_none() {
            port_map_line += "\n";
        } else {
            port_map_line += ",\n";
        }
        output_file.write_all(port_map_line.as_bytes()).unwrap();
    }
    output_file.write_all(b"    );\n\n").unwrap();
    output_file.write_all(b"endmodule\n").unwrap();

}