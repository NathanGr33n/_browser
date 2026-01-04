use boa_engine::{Context, Source};
use std::time::Instant;

/// Benchmark result with timing and memory info
#[derive(Debug)]
pub struct BenchmarkResult {
    pub name: String,
    pub duration_ms: f64,
    pub iterations: usize,
    pub ops_per_sec: f64,
}

impl BenchmarkResult {
    pub fn new(name: String, duration_ms: f64, iterations: usize) -> Self {
        let ops_per_sec = (iterations as f64 / duration_ms) * 1000.0;
        Self {
            name,
            duration_ms,
            iterations,
            ops_per_sec,
        }
    }
}

/// Run a benchmark with the given JavaScript code
fn run_benchmark(name: &str, code: &str, iterations: usize) -> BenchmarkResult {
    // Warmup
    for _ in 0..10 {
        let mut context = Context::default();
        let _ = context.eval(Source::from_bytes(code));
    }
    
    // Actual benchmark - create fresh context each iteration to avoid duplicate declarations
    let start = Instant::now();
    for _ in 0..iterations {
        let mut context = Context::default();
        context.eval(Source::from_bytes(code)).expect("Benchmark code should execute");
    }
    let duration = start.elapsed();
    
    BenchmarkResult::new(
        name.to_string(),
        duration.as_secs_f64() * 1000.0,
        iterations,
    )
}

/// Array operations benchmark
pub fn benchmark_array_operations() -> BenchmarkResult {
    let code = r#"
        const arr = [];
        for (let i = 0; i < 1000; i++) {
            arr.push(i);
        }
        arr.map(x => x * 2);
        arr.filter(x => x % 2 === 0);
        arr.reduce((a, b) => a + b, 0);
    "#;
    
    run_benchmark("Array Operations", code, 100)
}

/// Object creation and property access benchmark
pub fn benchmark_object_operations() -> BenchmarkResult {
    let code = r#"
        const objects = [];
        for (let i = 0; i < 100; i++) {
            objects.push({
                id: i,
                name: "Item " + i,
                value: i * 2,
                nested: {
                    data: i * 3
                }
            });
        }
        objects.forEach(obj => {
            obj.computed = obj.value + obj.nested.data;
        });
    "#;
    
    run_benchmark("Object Operations", code, 100)
}

/// Function calls and closures benchmark
pub fn benchmark_function_calls() -> BenchmarkResult {
    let code = r#"
        function fibonacci(n) {
            if (n <= 1) return n;
            return fibonacci(n - 1) + fibonacci(n - 2);
        }
        
        function makeCounter() {
            let count = 0;
            return function() {
                return ++count;
            };
        }
        
        fibonacci(15);
        const counter = makeCounter();
        for (let i = 0; i < 100; i++) {
            counter();
        }
    "#;
    
    run_benchmark("Function Calls", code, 100)
}

/// String operations benchmark
pub fn benchmark_string_operations() -> BenchmarkResult {
    let code = r#"
        let str = "Hello";
        for (let i = 0; i < 100; i++) {
            str += " World";
        }
        str.split(" ").join("-");
        str.toUpperCase();
        str.toLowerCase();
        str.substring(0, 50);
    "#;
    
    run_benchmark("String Operations", code, 100)
}

/// DOM-like operations benchmark (simulated)
pub fn benchmark_dom_like_operations() -> BenchmarkResult {
    let code = r#"
        class Node {
            constructor(type, value) {
                this.type = type;
                this.value = value;
                this.children = [];
                this.parent = null;
            }
            
            appendChild(child) {
                child.parent = this;
                this.children.push(child);
            }
            
            removeChild(child) {
                const index = this.children.indexOf(child);
                if (index > -1) {
                    this.children.splice(index, 1);
                    child.parent = null;
                }
            }
            
            querySelector(type) {
                if (this.type === type) return this;
                for (const child of this.children) {
                    const result = child.querySelector(type);
                    if (result) return result;
                }
                return null;
            }
        }
        
        const root = new Node("div", "root");
        for (let i = 0; i < 50; i++) {
            const child = new Node("div", "child" + i);
            root.appendChild(child);
            for (let j = 0; j < 5; j++) {
                const grandchild = new Node("span", "gc" + j);
                child.appendChild(grandchild);
            }
        }
        
        root.querySelector("span");
        root.children.forEach(child => {
            child.value = "updated";
        });
    "#;
    
    run_benchmark("DOM-like Operations", code, 100)
}

/// Real-world pattern: TodoMVC-like operations
pub fn benchmark_todomvc_pattern() -> BenchmarkResult {
    let code = r#"
        class TodoStore {
            constructor() {
                this.todos = [];
                this.nextId = 1;
            }
            
            addTodo(title) {
                this.todos.push({
                    id: this.nextId++,
                    title: title,
                    completed: false
                });
            }
            
            toggleTodo(id) {
                const todo = this.todos.find(t => t.id === id);
                if (todo) todo.completed = !todo.completed;
            }
            
            removeTodo(id) {
                this.todos = this.todos.filter(t => t.id !== id);
            }
            
            getActive() {
                return this.todos.filter(t => !t.completed);
            }
            
            getCompleted() {
                return this.todos.filter(t => t.completed);
            }
        }
        
        const store = new TodoStore();
        for (let i = 0; i < 50; i++) {
            store.addTodo("Task " + i);
        }
        
        for (let i = 1; i <= 50; i += 2) {
            store.toggleTodo(i);
        }
        
        store.getActive();
        store.getCompleted();
        
        for (let i = 1; i <= 10; i++) {
            store.removeTodo(i);
        }
    "#;
    
    run_benchmark("TodoMVC Pattern", code, 100)
}

/// JSON parsing and stringification
pub fn benchmark_json_operations() -> BenchmarkResult {
    let code = r#"
        const data = {
            users: [],
            metadata: {
                version: "1.0",
                timestamp: Date.now()
            }
        };
        
        for (let i = 0; i < 100; i++) {
            data.users.push({
                id: i,
                name: "User " + i,
                email: "user" + i + "@example.com",
                active: i % 2 === 0
            });
        }
        
        const json = JSON.stringify(data);
        const parsed = JSON.parse(json);
        parsed.users.filter(u => u.active);
    "#;
    
    run_benchmark("JSON Operations", code, 100)
}

/// Class-based OOP patterns
pub fn benchmark_class_patterns() -> BenchmarkResult {
    let code = r#"
        class Animal {
            constructor(name) {
                this.name = name;
            }
            
            speak() {
                return this.name + " makes a sound";
            }
        }
        
        class Dog extends Animal {
            constructor(name, breed) {
                super(name);
                this.breed = breed;
            }
            
            speak() {
                return this.name + " barks";
            }
            
            getBreed() {
                return this.breed;
            }
        }
        
        const dogs = [];
        for (let i = 0; i < 100; i++) {
            dogs.push(new Dog("Dog" + i, "Labrador"));
        }
        
        dogs.forEach(dog => {
            dog.speak();
            dog.getBreed();
        });
    "#;
    
    run_benchmark("Class Patterns", code, 100)
}

/// Run all benchmarks and return results
pub fn run_all_benchmarks() -> Vec<BenchmarkResult> {
    vec![
        benchmark_array_operations(),
        benchmark_object_operations(),
        benchmark_function_calls(),
        benchmark_string_operations(),
        benchmark_dom_like_operations(),
        benchmark_todomvc_pattern(),
        benchmark_json_operations(),
        benchmark_class_patterns(),
    ]
}

/// Format benchmark results as a table
pub fn format_results(results: &[BenchmarkResult]) -> String {
    let mut output = String::new();
    output.push_str("\n=== Boa JavaScript Engine Benchmarks ===\n\n");
    output.push_str(&format!(
        "{:<30} {:>15} {:>15} {:>20}\n",
        "Benchmark", "Time (ms)", "Iterations", "Ops/sec"
    ));
    output.push_str(&"-".repeat(85));
    output.push_str("\n");
    
    for result in results {
        output.push_str(&format!(
            "{:<30} {:>15.2} {:>15} {:>20.2}\n",
            result.name, result.duration_ms, result.iterations, result.ops_per_sec
        ));
    }
    
    output.push_str("\n");
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_array_operations() {
        let result = benchmark_array_operations();
        assert!(result.duration_ms > 0.0);
        assert_eq!(result.iterations, 100);
        assert!(result.ops_per_sec > 0.0);
    }

    #[test]
    fn test_benchmark_object_operations() {
        let result = benchmark_object_operations();
        assert!(result.duration_ms > 0.0);
        assert_eq!(result.iterations, 100);
    }

    #[test]
    fn test_benchmark_function_calls() {
        let result = benchmark_function_calls();
        assert!(result.duration_ms > 0.0);
        assert_eq!(result.iterations, 100);
    }

    #[test]
    fn test_benchmark_string_operations() {
        let result = benchmark_string_operations();
        assert!(result.duration_ms > 0.0);
        assert_eq!(result.iterations, 100);
    }

    #[test]
    fn test_benchmark_dom_like_operations() {
        let result = benchmark_dom_like_operations();
        assert!(result.duration_ms > 0.0);
        assert_eq!(result.iterations, 100);
    }

    #[test]
    fn test_benchmark_todomvc_pattern() {
        let result = benchmark_todomvc_pattern();
        assert!(result.duration_ms > 0.0);
        assert_eq!(result.iterations, 100);
    }

    #[test]
    fn test_benchmark_json_operations() {
        let result = benchmark_json_operations();
        assert!(result.duration_ms > 0.0);
        assert_eq!(result.iterations, 100);
    }

    #[test]
    fn test_benchmark_class_patterns() {
        let result = benchmark_class_patterns();
        assert!(result.duration_ms > 0.0);
        assert_eq!(result.iterations, 100);
    }

    #[test]
    fn test_run_all_benchmarks() {
        let results = run_all_benchmarks();
        assert_eq!(results.len(), 8);
        for result in results {
            assert!(result.duration_ms > 0.0);
            assert!(result.ops_per_sec > 0.0);
        }
    }

    #[test]
    fn test_format_results() {
        let results = vec![
            BenchmarkResult::new("Test".to_string(), 100.0, 50),
        ];
        let formatted = format_results(&results);
        assert!(formatted.contains("Boa JavaScript Engine Benchmarks"));
        assert!(formatted.contains("Test"));
        assert!(formatted.contains("100.00"));
    }
}
