mod collatz_seq;
mod elevator_events;
mod magnitude;
mod transpose;

fn main() {
    collatz_seq::run();
    transpose::run();
    magnitude::run();
    elevator_events::run();
}
