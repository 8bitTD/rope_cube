use rand::distributions::{Distribution, Uniform};
use bevy::prelude::*;

pub enum FacialState{
    Normal,
    Smile,
}

#[derive(Component)]
pub struct FacialRoot{
    pub state: FacialState,
    pub state_timer: f32,
}
impl Default for FacialRoot{
    fn default() -> Self{
        Self { 
            state: FacialState::Normal,
            state_timer: 0.0,
        }
    }
}

#[derive(Component)]
pub struct FacialNormal{
    pub blink_period_timer: f32,
    pub blink_timer: f32,
}

impl Default for FacialNormal{
    fn default() -> Self{
        Self { 
            blink_period_timer: 2.0, 
            blink_timer: 0.0, 
        }
    }
}
impl FacialNormal{
    pub fn period_timer_reset(&mut self){
        let range = Uniform::new(1.0,10.0);
        let mut rng = rand::thread_rng();
        let val = range.sample(&mut rng);
        self.blink_period_timer = val;
    }

}
#[derive(Component)]
pub struct FacialNormalEyes;

#[derive(Component)]
pub struct FacialSmile{
    pub is_smile_jump: bool,
}
impl Default for FacialSmile{
    fn default() -> Self{
        Self { is_smile_jump:false }
    }
}

#[derive(Component)]
pub struct FacialParts;