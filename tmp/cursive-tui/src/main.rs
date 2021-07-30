fn main() {
    let mut siv = cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    siv.run();
}
