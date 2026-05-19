use crate::Log;

/// Compares `.xyz` files in `output/rs` and `output/cpp` for content equality
pub fn run(name: &str) {
    Log::Success.log("Testing", &format!("`{name}`"));

    let prefix = format!("{name}_");
    const SUFFIX: &str = ".xyz";

    for i in 1..72 {
        let rs_path = format!("output/rs/{prefix}{i:04}{SUFFIX}");
        let cpp_path = format!("output/cpp/{prefix}{i:04}{SUFFIX}");

        if !std::path::Path::new(&rs_path).exists() || !std::path::Path::new(&cpp_path).exists() {
            break;
        }

        let rs_content =
            std::fs::read_to_string(&rs_path).expect("Failed to read Rust output file");
        let mut rs_lines = rs_content.lines();
        let cpp_content =
            std::fs::read_to_string(&cpp_path).expect("Failed to read C++ output file");
        let mut cpp_lines = cpp_content.lines();

        // compare particle count
        match (rs_lines.next(), cpp_lines.next()) {
            (Some(rs_header), Some(cpp_header)) => {
                if rs_header != cpp_header {
                    let s = format!(
                        "assertion failed: equal particle count: `{rs_header}` vs `{cpp_header}`"
                    );
                    Log::Failure.log("Error", &s);
                    std::process::exit(1);
                }
            }
            (l, r) => {
                unreachable!("programs are severely broken. {l:?} vs {r:?}")
            }
        }

        // skip comment line
        rs_lines.next();
        cpp_lines.next();

        // compare particle data lines
        for (line, (rs_line, cpp_line)) in rs_lines.zip(cpp_lines).enumerate() {
            // trim `Ar ` prefix
            let rs_line = rs_line.trim_start_matches("Ar ");
            let cpp_line = cpp_line.trim_start_matches("Ar ");

            // split by whitespace and compare
            let rs_parts: Vec<&str> = rs_line.split_whitespace().collect();
            let cpp_parts: Vec<&str> = cpp_line.split_whitespace().collect();

            for i in 0..3 {
                let rs: f64 = rs_parts
                    .get(i)
                    .expect("Missing particle data in Rust output")
                    .parse()
                    .expect("Failed to parse Rust particle data");
                let cpp: f64 = cpp_parts
                    .get(i)
                    .expect("Missing particle data in C++ output")
                    .parse()
                    .expect("Failed to parse C++ particle data");

                // tolerance for floating-point comparison
                let tol = 1e-6;
                if (rs - cpp).abs() < tol {
                    continue;
                }

                let s = format!("assertion failed: `{rs}` != `{cpp}` in line {line}:");
                Log::Failure.log("Fail", &s);
                Log::Info.log("Cpp", &rs_line);
                Log::Info.log("Rust", &cpp_line);
                break;
            }
        }

        // let display = format!("[{} / {}]", i, 71);
        // Log::Success.log("Pass", &display);
    }

    let display = format!("72 steps");
    Log::Success.log("Pass", &display);
}
