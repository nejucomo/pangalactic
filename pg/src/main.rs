use pangalactic;

fn main() {
    println!("=== {} ===", env!("CARGO_PKG_NAME"));
    let mut args = std::env::args().skip(1);
    let guestpath = args.next().unwrap();
    assert_eq!(None, args.next());

    pangalactic::execute_path(guestpath);
}
