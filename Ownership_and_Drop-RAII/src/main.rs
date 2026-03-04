use crate::tracker::Tracker;

mod tracker;

fn main() {
    let outer_tracker = Tracker::new("Outer tracker".to_owned());
    {
        let inner_tracker = Tracker::new("Inner tracker".to_owned());
    }
    let tracker_to_give_away = Tracker::new("Give away tracker".to_owned());
    move_tracker_to_me(tracker_to_give_away);
    let destroy_tracker = Tracker::new("Destroy Tracker".to_owned());
    std::mem::drop(destroy_tracker);
}

fn move_tracker_to_me(tracker: Tracker) {
    println!("tracker {} is moved to another function", tracker.name);
}
