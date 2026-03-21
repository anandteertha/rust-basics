use crate::{file_helper::parse_file, incident::Incident, status::Status};

mod file_helper;
mod incident;
mod severity;
mod status;
fn main() {
    let file_content = parse_file(
        "C:/Users/anand/Development/rust-basics/10. Collections_and_File_IO/src/incidents.txt",
    );
    let mut incidents: Vec<Incident> = Vec::new();
    match file_content {
        Ok(logs) => {
            let mut invalid_count: usize = 0;
            for line in logs.lines() {
                let new_incident = Incident::new(line);
                match new_incident {
                    Ok(incident) => {
                        incidents.push(incident);
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        invalid_count += 1;
                    }
                }
            }
            println!("\nParsed lines: {}", incidents.len());
            println!("skipped lines: {}", invalid_count);
        }
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    }
    println!("\nSummary for Severity:");
    Incident::print_summary(Incident::count_by_severity(&incidents));
    println!("\nSummary for Status:");
    Incident::print_summary(Incident::count_by_status(&incidents));
    println!("\nSummary for Service:");
    Incident::print_summary(Incident::count_by_service(&incidents));
    println!("\nUnresolved log messages:");
    let unresolved_incidents = incidents
        .iter()
        .filter(|incident| matches!(incident.get_status(), Status::Unresolved));
    for unresolved_incident in unresolved_incidents {
        println!("message: {}", &unresolved_incident.get_message());
    }
}
