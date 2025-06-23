mod collatz_seq;

fn main() {
    println!(
        "Collatz Sequence Length: {}",
        collatz_seq::collatz_length(11)
    ); // should be 15
}
