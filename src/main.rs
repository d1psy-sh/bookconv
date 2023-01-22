use bookconv::app::App;

fn main() {
    // read in the first arg as input file
    // use clap for that
    // write the output into a output file same name but html extention
    let mut app = App::new();
    app.run();
}
