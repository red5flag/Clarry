
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

const PROJECT_ROOT: &str = "/home/red/Downloads/Clarry-dev2/Clarry-dev2";
const EXPORT_FILE: &str = "min-exp.txt";
const OUTPUT_FILE: &str = "min-out.txt";

const EXCLUDED_DIRS: &[&str] = &[
    "src/static",
    "target",
    "models",
    ".git",
    "node_modules",
];

const INCLUDED_EXTENSIONS: &[&str] = &[
    "rs", "toml", "md", "yml", "yaml", "json", "txt", "pest",
];

const EXCLUDED_FILES: &[&str] = &[
    "Cargo.lock",
    ".gitignore",
    ".gitattributes",
    "run_dev.rs",
    "run_fix.rs",
    "run_qwen.rs",
    "run_comm.rs",
    "min-out.txt",
    "min-exp.txt",
    "README.md",
];

fn main() {
    println!("🚀 Starting Code Export and Error Check");
    println!("==========================================\n");

    if !Path::new(PROJECT_ROOT).exists() {
        eprintln!("❌ Error: Project root does not exist: {}", PROJECT_ROOT);
        std::process::exit(1);
    }

    let _ = File::create(EXPORT_FILE);
    let _ = File::create(OUTPUT_FILE);

    println!("📦 Exporting codebase to {}...", EXPORT_FILE);
    export_codebase(EXPORT_FILE);
    println!("✅ Codebase exported successfully.\n");

    println!("🔍 Running cargo commands...\n");

    let errors = [
        ("cargo check", run_cargo_command("cargo check")),
        ("cargo clippy", run_cargo_command("cargo clippy")),
    ];

    for (cmd, _) in &errors {
        println!("   ✅ {} complete", cmd);
    }

    println!("\n📝 Writing errors to {}...", OUTPUT_FILE);
    write_errors_to_file(OUTPUT_FILE, &errors);
    println!("✅ Error check complete.\n");

    println!("📊 Summary:");
    println!("   - Project root: {}", PROJECT_ROOT);
    println!("   - Exported to: {}", EXPORT_FILE);
    println!("   - Commands run: cargo check, cargo clippy");
    println!("   - Output: {}", OUTPUT_FILE);
    println!("\n✨ Done!");
}

fn export_codebase(output_file: &str) {
    let mut output = File::create(output_file).expect("Failed to create export file");
    writeln!(output, "=== Codebase Export ===").unwrap();
    walk_directory(Path::new(PROJECT_ROOT), &mut output);
}

fn walk_directory(dir: &Path, output: &mut File) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
    entries.sort_by_key(|e| e.path());

    for entry in entries {
        let path = entry.path();
        let relative_path = path.strip_prefix(PROJECT_ROOT).unwrap_or(&path);
        let path_str = relative_path.to_string_lossy();

        if path.is_dir() {
            if EXCLUDED_DIRS.iter().any(|excluded| path_str.starts_with(excluded)) {
                continue;
            }
            walk_directory(&path, output);
        } else if path.is_file() {
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

            if EXCLUDED_FILES.contains(&file_name) {
                continue;
            }

            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

            if !INCLUDED_EXTENSIONS.contains(&ext) {
                continue;
            }

            writeln!(output, "\n{}", "=".repeat(80)).unwrap();
            writeln!(output, "FILE: {}", path_str).unwrap();
            writeln!(output, "{}\n", "=".repeat(80)).unwrap();

            if let Ok(contents) = fs::read_to_string(&path) {
                writeln!(output, "{}", contents).unwrap();
            } else {
                writeln!(output, "[Error reading file]").unwrap();
            }
        }
    }
}

fn run_cargo_command(command: &str) -> String {
    let mut parts = command.split_whitespace();
    let cmd_name = parts.next().unwrap_or("cargo");
    let args: Vec<&str> = parts.collect();

    let output = Command::new(cmd_name)
        .current_dir(PROJECT_ROOT)
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    match output {
        Ok(result) => {
            let stdout = String::from_utf8_lossy(&result.stdout);
            let stderr = String::from_utf8_lossy(&result.stderr);
            let status = result.status.code().unwrap_or(-1);

            format!(
                "Command: {}\nExit code: {}\n\n--- STDOUT ---\n{}\n\n--- STDERR ---\n{}",
                command, status, stdout, stderr
            )
        }
        Err(e) => format!("Command: {}\nError: {}", command, e),
    }
}

fn write_errors_to_file(filename: &str, errors: &[(&str, String)]) {
    let mut file = File::create(filename).expect("Failed to create output file");
    writeln!(file, "=== Cargo Command Errors ===").unwrap();

    for (command, output) in errors {
        writeln!(file, "{}", "=".repeat(80)).unwrap();
        writeln!(file, "COMMAND: {}", command).unwrap();
        writeln!(file, "{}", "=".repeat(80)).unwrap();
        writeln!(file, "{}", output).unwrap();
        writeln!(file).unwrap();
    }
}
