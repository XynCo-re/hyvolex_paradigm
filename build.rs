use std::process::Command;
use std::env;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Basic rerun triggers
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");
    
    // Check if we're in a development environment
    if env::var("CARGO_CFG_DEBUG_ASSERTIONS").is_ok() {
        validate_dependencies()?;
    }
    
    Ok(())
}

fn validate_dependencies() -> Result<(), Box<dyn std::error::Error>> {
    // Check rustc version
    let rustc = Command::new("rustc")
        .arg("--version")
        .output()
        .map_err(|e| format!("Failed to get rustc version: {}", e))?;
    
    if !rustc.status.success() {
        println!("cargo:warning=Failed to verify rustc version");
    }
    
    // Check for outdated dependencies
    let output = Command::new("cargo")
        .args(["outdated", "--workspace", "--exit-code", "1"])
        .output()
        .map_err(|e| format!("Failed to check dependencies: {}", e))?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !stderr.contains("error: no such command") {
            println!("cargo:warning=Outdated dependencies found. Run 'cargo outdated' for details");
        }
    }
    
    // Verify Cargo.lock is up to date
    if !Path::new("Cargo.lock").exists() {
        let status = Command::new("cargo")
            .arg("generate-lockfile")
            .status()
            .map_err(|e| format!("Failed to generate lockfile: {}", e))?;
            
        if !status.success() {
            println!("cargo:warning=Failed to generate Cargo.lock");
        }
    }
    
    // Run clippy checks with detailed output
    let clippy = Command::new("cargo")
        .args([
            "clippy",
            "--all-features",
            "--message-format=json",
            "--",
            "-D",
            "warnings"
        ])
        .output()
        .map_err(|e| format!("Failed to run clippy: {}", e))?;
    
    if !clippy.status.success() {
        let stderr = String::from_utf8_lossy(&clippy.stderr);
        println!("cargo:warning=Clippy checks failed:");
        for line in stderr.lines() {
            println!("cargo:warning={}", line);
        }
    }
    
    // Verify formatting if rustfmt is available
    let rustfmt = Command::new("cargo")
        .args(["fmt", "--", "--check"])
        .output()
        .map_err(|e| format!("Failed to check formatting: {}", e))?;
        
    if !rustfmt.status.success() {
        println!("cargo:warning=Code formatting issues found. Run 'cargo fmt' to fix");
    }
    
    Ok(())
}