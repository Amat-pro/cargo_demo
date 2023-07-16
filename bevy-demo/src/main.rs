mod rhythm;
mod sprite_sheet;

fn main() {
    println!("run rhythm start ...");
    rhythm::run_rhythm();
    println!("run rhythm end ...");

    println!("run start ...");
    sprite_sheet::run_sprite_sheet();
    println!("run end ...");
}
