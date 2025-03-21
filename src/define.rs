pub mod common{
    pub const TOOLNAME: &str = "rope_cube";

}

pub mod system{
    pub const FPS: f32 = 60.0;
}

pub mod assets{
    pub const DEFAULTFONT: &str = "fonts/NotoSansJP-Bold.ttf";

    pub const BGM: &str = "bgm/maou_bgm_8bit29.mp3";
    //pub const BGM: &str = "bgm/maou_bgm_8bit25.mp3";
    pub const BGMENDING: &str = "bgm/ending.mp3";

    pub const SOUNDJUMP: &str = "sound/jump.mp3";
    pub const SOUNDGRAB: &str = "sound/grab.wav";
    pub const SOUNDDEATH: &str = "sound/death.wav";
}

pub mod value{
    pub const VOLUME: f32 = 0.05; //0.05
    pub const RAPIERDEBUGRENDERPLUGINENABLED: bool = false;
    pub const FADETIME: f32 = 1.0;
    pub const DEFAULTTEXTSTAGEALPHA: f32 = 3.0;
    pub const STARTSTAGE: usize = 1;
    pub const MAXSTAGE: usize = 4;
    pub const ENDINGTEXTMOVE: f32 = 130.0;
    pub const ISDEBUG: bool = false;
}