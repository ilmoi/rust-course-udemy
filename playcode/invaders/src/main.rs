use std::error::Error;
use rusty_audio::Audio;

fn main() -> Result <(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "explode.wav");
    audio.add("lose", "lose.wav");
    audio.add("move", "move.wav");
    audio.add("pew", "pew.wav");
    audio.add("startup", "startup.wav");
    audio.add("wind", "wind.wav");
    audio.play("startup");

    //block until all audio done playing
    audio.wait();
    Ok(())

    //abandoned and just watched him do it:)

}
