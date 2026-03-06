use crate::tracker::Tracker;

mod tracker;
fn main() {
    let outer_tracker = Tracker::new("outer tracker".to_owned());
    println!(
        "main: {} created. count: {}",
        outer_tracker.name, outer_tracker.count
    );
    inspect(&outer_tracker);
    println!(
        "main: {} after inspect, still exists, not moved.",
        outer_tracker.name
    );

    let ref1 = &outer_tracker;
    let ref2 = &outer_tracker;
    println!("main: ref1 of {} (count={})", ref1.name, ref1.count);
    println!("main: ref2 of {} (count={})", ref2.name, ref2.count);

    let mut mutable_tracker = Tracker::new("mutable tracker".to_owned());
    mutate(&mut mutable_tracker);
    println!(
        "main: tracker {} has now count={}",
        mutable_tracker.name, mutable_tracker.count
    );
}

fn inspect(tracker: &Tracker) {
    println!(
        "inspect: {} borrowed. count: {}",
        tracker.name, tracker.count
    );
}

fn mutate(mutable_tracker: &mut Tracker) {
    println!(
        "mutate: before incrementing, {} (count={})",
        mutable_tracker.name, mutable_tracker.count
    );
    mutable_tracker.count += 1;
    println!(
        "mutate: after incrementing, {} (count={})",
        mutable_tracker.name, mutable_tracker.count
    );
}
