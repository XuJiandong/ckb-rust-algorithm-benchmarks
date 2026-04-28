use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};

fn main() {
    let input = read_input();

    let binary_sizes = parse_ls_output(&input);
    let cycles = parse_debug_output(&input);

    update_readme(&binary_sizes, &cycles);
}

fn read_input() -> String {
    let args: Vec<String> = std::env::args().collect();
    let mut input = String::new();
    if args.len() > 1 {
        input = fs::read_to_string(&args[1]).expect("Failed to read input file");
    } else {
        io::stdin()
            .read_to_string(&mut input)
            .expect("Failed to read stdin");
    }
    input
}

fn parse_ls_output(input: &str) -> HashMap<String, String> {
    // Match: permissions links owner group size month day time filename
    // Example: -rwxr-xr-x 1 runner docker 73K Apr 28 10:00 p256-test
    let re = Regex::new(
        r"^[-dl]\S+\s+\d+\s+\S+\s+\S+\s+(\d+\.?\d*[KMGT]?)\s+\w{3}\s+\d+\s+\S+\s+(\S+)",
    )
    .unwrap();
    let mut sizes = HashMap::new();
    for line in input.lines() {
        if let Some(caps) = re.captures(line) {
            let size = caps[1].to_string();
            let filename = caps[2].to_string();
            sizes.insert(filename, size);
        }
    }
    sizes
}

fn parse_debug_output(input: &str) -> HashMap<String, String> {
    // Match: cost of <algo> verifying cycles: <value> M
    let re = Regex::new(r"cost of (.+?) verifying cycles: ([\d.]+) M").unwrap();
    let mut cycles = HashMap::new();
    for line in input.lines() {
        if let Some(caps) = re.captures(line) {
            cycles.insert(caps[1].to_string(), caps[2].to_string());
        }
    }
    cycles
}

fn update_readme(binary_sizes: &HashMap<String, String>, cycles: &HashMap<String, String>) {
    let path = "README.md";
    let content = fs::read_to_string(path).unwrap_or_else(|_| {
        eprintln!("Error: README.md not found in current directory");
        std::process::exit(1);
    });

    let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

    // Locate the table
    let section_idx = lines
        .iter()
        .position(|l| l.trim() == "## Benchmark Matrix")
        .unwrap_or_else(|| {
            eprintln!("Error: '## Benchmark Matrix' section not found");
            std::process::exit(1);
        });

    let header_idx = lines[section_idx..]
        .iter()
        .position(|l| l.starts_with("| Algorithm |"))
        .map(|i| section_idx + i)
        .unwrap_or_else(|| {
            eprintln!("Error: table header not found");
            std::process::exit(1);
        });

    // separator is header_idx + 1; rows start at header_idx + 2
    let first_row = header_idx + 2;
    let mut last_row = first_row;
    while last_row < lines.len() && lines[last_row].starts_with('|') {
        last_row += 1;
    }

    // Parse existing rows into a map keyed by (curve, additional_info)
    let mut new_lines = lines.clone();
    let re = Regex::new(r"^\|\s*([^|]+?)\s*\|\s*([^|]+?)\s*\|\s*([^|]+?)\s*\|\s*([^|]+?)\s*\|$")
        .unwrap();

    for i in first_row..last_row {
        if let Some(caps) = re.captures(&new_lines[i]) {
            let curve = caps[1].trim();
            let old_cycles = caps[2].trim().to_string();
            let old_size = caps[3].trim().to_string();
            let additional = caps[4].trim();

            let new_cycles = lookup_cycles(cycles, curve, additional).unwrap_or(old_cycles);
            let new_size = lookup_size(binary_sizes, curve, additional).unwrap_or(old_size);

            let pad_left = |s: &str, w: usize| {
                let s = s.trim();
                if s.len() >= w {
                    s.to_string()
                } else {
                    format!("{: <w$}", s, w = w)
                }
            };
            let pad_center = |s: &str, w: usize| {
                let s = s.trim();
                if s.len() >= w {
                    s.to_string()
                } else {
                    let left = (w - s.len()) / 2;
                    let right = w - s.len() - left;
                    format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))
                }
            };

            new_lines[i] = format!(
                "| {} | {} | {} | {} |",
                pad_left(curve, 11),
                pad_center(&new_cycles, 13),
                pad_center(&new_size, 11),
                pad_left(additional, 22)
            );
        }
    }

    let output = new_lines.join("\n") + "\n";
    fs::write(path, output).unwrap_or_else(|e| {
        eprintln!("Error writing README.md: {}", e);
        std::process::exit(1);
    });
}

fn lookup_cycles(
    cycles: &HashMap<String, String>,
    curve: &str,
    additional: &str,
) -> Option<String> {
    let debug_key = match (curve, additional) {
        ("p256", _) => "p256",
        ("k256", "No precomputed table") => "k256",
        ("k256", "Recovery") => "k256(recovery)",
        ("RSA-2048", _) => "rsa-2048",
        ("ed25519", _) => "ed25519",
        ("schnorr", _) => "schnorr",
        ("sp1 verifier", _) => "sp1(zkVM)",
        ("ml-dsa-44", _) => "ml-dsa (MlDsa44)",
        ("ml-dsa-65", _) => "ml-dsa (MlDsa65)",
        ("ml-dsa-87", _) => "ml-dsa (MlDsa87)",
        _ => return None,
    };
    cycles.get(debug_key).map(|v| format!("{}M Cycles", v))
}

fn lookup_size(
    sizes: &HashMap<String, String>,
    curve: &str,
    additional: &str,
) -> Option<String> {
    let binary_key = match (curve, additional) {
        ("p256", _) => "p256-test",
        ("k256", "No precomputed table") => "k256-test",
        ("k256", "Recovery") => "k256-recovery-test",
        ("RSA-2048", _) => "rsa-test",
        ("ed25519", _) => "ed25519-test",
        ("schnorr", _) => "schnorr-test",
        ("sp1 verifier", _) => "sp1-test",
        ("ml-dsa-44", _) | ("ml-dsa-65", _) | ("ml-dsa-87", _) => "ml-dsa-test",
        _ => return None,
    };
    sizes.get(binary_key).map(|v| format!("{} Bytes", v))
}
