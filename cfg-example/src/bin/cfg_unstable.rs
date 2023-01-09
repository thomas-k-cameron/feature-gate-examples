fn main() {
    cfg_unstable();
}

#[cfg(cfg_unstable)]
fn cfg_unstable() {
    println!("cfg_unstable");
}