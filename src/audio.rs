extern crate baal;


use super::Vec3;
use std::collections::{HashSet, HashMap};

#[derive(Debug, Clone, PartialEq)]
pub struct AudioConfiguration {
    pub effect_directory: String,
    pub music_directory: String,

    pub global_volume: f32,

    pub effect_names: HashSet<String>,
    pub music_names: HashSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedConfiguration {
    pub effect_names: HashMap<String, usize>,
    pub music_names: HashMap<String, usize>,
}

impl LoadedConfiguration {
    pub fn play_effect(&self, name:&String, at:Vec3) -> bool {
        match self.effect_names.get(name) {
            Some(idx) => {
                let v3 : [f64; 3] = at.into();
                baal::effect::play(*idx,&v3); 
                true
            },
            None => false
        }
    }
}

pub fn destroy(loaded_configuration: LoadedConfiguration) {
    baal::close();
}

pub fn init_audio(audio_configuration:AudioConfiguration) -> Result<LoadedConfiguration, baal::InitError> {  
    // take our sets, force them in to an order
    let effect_names : Vec<String> = audio_configuration.effect_names.into_iter().collect();
    let music_names : Vec<String> = audio_configuration.music_names.into_iter().collect();

    // record the ordering
    let eff_names : HashMap<String, usize> = effect_names.iter().enumerate().map ( |(a, b)| (b.clone(), a) ).collect(); // .into_iter()
    let mus_names : HashMap<String, usize> = music_names.iter().enumerate().map ( |(a, b)| (b.clone(), a) ).collect(); // .into_iter()

    // add ogg extensions
    let effect_names_with_ogg : Vec<(String, u32)> = effect_names.iter().map(|name|{ 
        let mut my_str = name.clone();
        my_str.push_str(".ogg");
        (my_str, 1)
    }).collect();
    let music_names_with_ogg : Vec<String> = music_names.iter().map(|name|{
        let mut my_str = name.clone();
        my_str.push_str(".ogg");
        my_str
    }).collect();

    let setting = baal::Setting {
        channels: 2,
        sample_rate: 44100.,
        frames_per_buffer: 64,

        effect_dir: audio_configuration.effect_directory,
        music_dir: audio_configuration.music_directory,

        global_volume: 0.5,
        music_volume: 0.5,
        effect_volume: 0.5,

        distance_model: baal::effect::DistanceModel::Linear(10.,110.),

        music_loop: true,

        music_transition: baal::music::MusicTransition::Instant,

        effect: effect_names_with_ogg,
        music: music_names_with_ogg,

        check_level: baal::CheckLevel::Always,
    };

    baal::init(&setting).map(|_| {
        LoadedConfiguration { effect_names: eff_names, music_names: mus_names }
    })
}









