fn main() {
    let now = utcnow::utcnow().unwrap();
    println!("{now} = {now:?}", now = now);
}
