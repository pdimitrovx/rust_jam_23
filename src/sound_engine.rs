use macroquad::audio::{
    load_sound, play_sound, play_sound_once, stop_sound, PlaySoundParams, Sound,
};

pub enum Cues {
    MusicMenu,
    MusicGame,
    SfxCrash,
    SfxUfo,
    SfxClick,
}

pub struct SoundEngine {
    music_menu: Sound,
    music_game: Sound,
    sfx_crash: Sound,
    sfx_ufo: Sound,
    sfx_click: Sound,
}

impl SoundEngine {
    pub async fn new() -> Self {
        let music_menu = load_sound("res/audio/music_menu.ogg").await.unwrap();
        let music_game = load_sound("res/audio/music_game.ogg").await.unwrap();
        let sfx_crash = load_sound("res/audio/sfx_crash.ogg").await.unwrap();
        let sfx_ufo = load_sound("res/audio/sfx_ufo.ogg").await.unwrap();
        let sfx_click = load_sound("res/audio/sfx_click.ogg").await.unwrap();

        Self {
            music_menu,
            music_game,
            sfx_crash,
            sfx_ufo,
            sfx_click,
        }
    }

    pub fn play(&self, cue: Cues) {
        match cue {
            Cues::MusicMenu => {
                play_sound(
                    &self.music_menu,
                    PlaySoundParams {
                        looped: true,
                        volume: 0.0,
                    },
                );
            }
            Cues::MusicGame => {
                play_sound(
                    &self.music_game,
                    PlaySoundParams {
                        looped: true,
                        volume: 1.0,
                    },
                );
            }
            Cues::SfxCrash => play_sound_once(&self.sfx_crash),
            Cues::SfxUfo => play_sound_once(&self.sfx_ufo),
            Cues::SfxClick => play_sound_once(&self.sfx_click),
        }
    }

    pub fn stop(&self, cue: Cues) {
        match cue {
            Cues::MusicMenu => {
                stop_sound(&self.music_menu);
            }
            Cues::MusicGame => {
                stop_sound(&self.music_game);
            }
            Cues::SfxCrash => {
                stop_sound(&self.sfx_crash);
            }
            Cues::SfxUfo => {
                stop_sound(&self.sfx_ufo);
            }
            Cues::SfxClick => {
                stop_sound(&self.sfx_click);
            }
        }
    }
}
