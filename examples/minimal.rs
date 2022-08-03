fn main() {
    let now = utcnow::utcnow().unwrap();
    println!("{}.{:09} = {:?}", now.as_secs(), now.subsec_nanos(), now);
}
