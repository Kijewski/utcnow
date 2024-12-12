//! A trivial example that prints the current UNIX time.

fn main() {
    let now = utcnow::utcnow().unwrap();
    println!("{now} = {now:?}", now = now);
}
