use macroquad::audio::{
    load_sound, play_sound, play_sound_once, stop_sound, PlaySoundParams, Sound,
};

pub enum Cues {
    MusicMenu,
    MusicGame,
    SfxCrash,
    SfxUfo,
    SfxClick,
    SfxHo,
}

pub struct SoundEngine {
    music_menu: Sound,
    music_game: Sound,
    sfx_crash: Sound,
    sfx_ufo: Sound,
    sfx_click: Sound,
    sfx_ho: Vec<Sound>,
    ho_head: usize,
    ufo_cue: bool,
    current_score: u32,
}

impl SoundEngine {
    pub async fn new() -> Self {
        let music_menu = load_sound("res/audio/music_menu.ogg").await.unwrap();
        let music_game = load_sound("res/audio/music_game.ogg").await.unwrap();
        let sfx_crash = load_sound("res/audio/sfx_crash.ogg").await.unwrap();
        let sfx_ufo = load_sound("res/audio/sfx_ufo.ogg").await.unwrap();
        let sfx_click = load_sound("res/audio/sfx_click.ogg").await.unwrap();
        let sfx_ho = vec![
            load_sound("res/audio/sfx_ho1.ogg").await.unwrap(),
            load_sound("res/audio/sfx_ho2.ogg").await.unwrap(),
            load_sound("res/audio/sfx_ho3.ogg").await.unwrap(),
            load_sound("res/audio/sfx_ho4.ogg").await.unwrap(),
            load_sound("res/audio/sfx_ho5.ogg").await.unwrap(),
            load_sound("res/audio/sfx_ho6.ogg").await.unwrap(),
        ];

        Self {
            music_menu,
            music_game,
            sfx_crash,
            sfx_ufo,
            sfx_click,
            sfx_ho,
            ho_head: 0,
            ufo_cue: false,
            current_score: 0,
        }
    }

    pub fn play(&mut self, cue: Cues) {
        match cue {
            Cues::MusicMenu => {
                play_sound(
                    &self.music_menu,
                    PlaySoundParams {
                        looped: true,
                        volume: 0.7,
                    },
                );
            }
            Cues::MusicGame => {
                play_sound(
                    &self.music_game,
                    PlaySoundParams {
                        looped: true,
                        volume: 0.7,
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
                            volume: 0.2,
                        },
                    );
                }
            }
            Cues::SfxClick => play_sound_once(&self.sfx_click),
            Cues::SfxHo => {
                play_sound(
                    &self.sfx_ho[self.ho_head],
                    PlaySoundParams {
                        looped: false,
                        volume: 0.7,
                    },
                );
                self.ho_head += 1;
                if self.ho_head > self.sfx_ho.len() - 1 {
                    self.ho_head = 0;
                }
            }
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
            Cues::SfxHo => {}
        }
    }

    pub fn trigger_ho(&mut self, score: u32) {
        if self.current_score != score {
            self.current_score = score;
            self.play(Cues::SfxHo);
        }
    }
}
