use crate::archive::arch::arch_file as arc;
use rand::Rng;
mod archive;
mod clean;
mod player;

fn main() {
    arc("file.txt");

    let movie: &str = "Terminator";
    let song: &str = "Killer";
    player::play_movie(movie);
    player::play_audio(song);

    clean::clean::perform_clean();
    clean::clean::files::clean_files();

    let mut rng = rand::thread_rng();
    let a: i32 = rng.gen();
    println!("{}", a);
}
