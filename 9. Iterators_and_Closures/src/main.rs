use crate::{
    alert::{Alert, get_mock_alerts},
    alert_level::AlertLevel,
};

mod alert;
mod alert_level;

fn main() {
    let alerts = get_mock_alerts();
    // iter
    println!("\n***** iter *****");
    for alert in alerts.iter() {
        alert.print_alert();
    }

    // filter & closure
    println!("\n***** filter & closure *****");
    let critical_alerts_closure =
        |alert: &&Alert| matches!(alert.get_alert_level(), AlertLevel::Critical);

    for alert in alerts.iter().filter(critical_alerts_closure) {
        alert.print_alert();
    }

    // map
    println!("\n***** map & closure *****");
    for summary in alerts.iter().map(|alert| alert.get_summary()) {
        println!("{}", summary);
    }

    // find
    println!("\n***** find *****");
    let unresolved_critical_alert = alerts.iter().find(|alert| {
        matches!(alert.get_alert_level(), AlertLevel::Critical) && !alert.get_is_resolved()
    });
    match unresolved_critical_alert {
        Some(alert) => {
            println!("Below Critical alert is not yet resolved!!!");
            println!("{}", alert.get_summary());
        }
        None => {
            println!("There are no critical unresolved alerts! Thats a good sign :)");
        }
    }

    // any & all
    println!("\n***** any & all *****");
    println!(
        "Any critical alerts?\n{}",
        alerts
            .iter()
            .any(|alert| matches!(alert.get_alert_level(), AlertLevel::Critical))
    );
    println!(
        "All alerts resolved?\n{}",
        alerts.iter().all(|alert| alert.get_is_resolved())
    );

    // fold
    println!("\n***** fold *****");
    let sum_closure = |acc: u8, alert: &Alert| acc + alert.get_severity_score();
    let sum = alerts.iter().fold(0, sum_closure);
    println!("The sum of all severity scores from all alerts is {}", sum);
}
