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
    ufo_cue: bool,
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
            ufo_cue: false,
        }
    }

    pub fn play(&mut self, cue: Cues) {
        match cue {
            Cues::MusicMenu => {
                play_sound(
                    &self.music_menu,
                    PlaySoundParams {
                        looped: true,
                        volume: 0.9,
                    },
                );
            }
            Cues::MusicGame => {
                play_sound(
                    &self.music_game,
                    PlaySoundParams {
                        looped: true,
                        volume: 0.9,
                    },
                );
            }
            Cues::SfxCrash => play_sound_once(&self.sfx_crash),
            Cues::SfxUfo => {
                if !self.ufo_cue {
                    self.ufo_cue = true;
                    play_sound(
                        &self.sfx_ufo,
                        PlaySoundParams {
                            looped: true,
                            volume: 0.9,
                        },
                    );
                }
            }
            Cues::SfxClick => play_sound_once(&self.sfx_click),
        }
    }

    pub fn stop(&mut self, cue: Cues) {
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
                self.ufo_cue = false;
                stop_sound(&self.sfx_ufo);
            }
            Cues::SfxClick => {
                stop_sound(&self.sfx_click);
            }
        }
    }
}
