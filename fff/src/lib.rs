pub mod pbp;
pub mod rosters;

// This would NOT be acceptable to avoid SQL injection
// but we don't really have a DB driver to hand parameterized queries to.
// I'm assuming the Spice API effectively disallows SQL injection (and writes in general)
// so this just handles input parameters a bit more sanely
fn safe_spice(input: &str) -> String {
    let sanitized: String = input
        .chars()
        .filter(|c| !c.is_control())
        .map(|c| match c {
            '\'' => "''".to_string(),
            '\\' => "\\\\".to_string(),
            _ => c.to_string(),
        })
        .collect();

    sanitized
}
