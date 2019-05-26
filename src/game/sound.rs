use audio::SoundSource;
use ggez::{audio, Context, GameResult};

pub enum SoundType {
    Start,
    Win,
    Lose,
    Move,
    UnableToMove,
    Menu,
}

pub fn play(ctx: &mut Context, sound_type: SoundType) -> GameResult {
    let path: String;
    match sound_type {
        SoundType::Start => path = String::from("/start.ogg"),
        SoundType::Move => path = String::from("/move.wav"),
        SoundType::Menu => path = String::from("/menu.wav"),
        SoundType::Win => path = String::from("/win.wav"),
        SoundType::Lose => path = String::from("/lose.wav"),
        SoundType::UnableToMove => path = String::from("/bump.wav"),
    }

    let mut sound = audio::Source::new(ctx, path)?;
    sound.play_detached()?;
    Ok(())
}
