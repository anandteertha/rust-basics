use crate::log_entry::LogEntry;
mod log_entry;

fn main() {
    let input = "database:connected
cache:degraded
auth:disconnected
payments:connected
search:degraded
orders:ssabbaajj";

    let mut entries: Vec<LogEntry> = Vec::new();
    println!("\nEntries");
    for line in input.lines() {
        match LogEntry::parse(line) {
            Ok(entry) => {
                entry.print_entry();
                entries.push(entry);
            }
            Err(err) => println!("{}", err),
        }
    }
    println!("\nproblematic entries:");
    for entry in &entries {
        if entry.is_problematic() {
            entry.print_entry();
        }
    }
    println!("\nsevere entries:");
    for entry in &entries {
        if entry.is_severe() {
            entry.print_entry();
        }
    }
}
