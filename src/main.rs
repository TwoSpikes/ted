use cursive::CursiveExt;

fn main() {
    let mut app = ::cursive::Cursive::default();
    app.add_layer(::cursive::views::Dialog::text("Hello, world!"));
    app.run();
}
