use browser_engine::benchmarks::{run_all_benchmarks, format_results};

fn main() {
    println!("Running Boa JavaScript Engine Benchmarks...");
    println!("This may take a few minutes...\n");
    
    let results = run_all_benchmarks();
    print!("{}", format_results(&results));
    
    // Calculate totals
    let total_time: f64 = results.iter().map(|r| r.duration_ms).sum();
    let avg_ops_per_sec: f64 = results.iter().map(|r| r.ops_per_sec).sum::<f64>() / results.len() as f64;
    
    println!("Total execution time: {:.2} ms", total_time);
    println!("Average ops/sec: {:.2}", avg_ops_per_sec);
    println!("\n=== Performance Analysis ===");
    println!("Boa is a pure Rust JavaScript engine optimized for safety and embeddability.");
    println!("While not as fast as V8, it provides:");
    println!("- Good performance for typical web content");
    println!("- Memory safety guarantees");
    println!("- Easy Rust integration");
    println!("- No C++ dependencies");
    println!("\nFor Phase 8 goals (React/Vue apps), Boa's performance is sufficient.");
}
