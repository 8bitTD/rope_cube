//use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use super::stage_map::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct GoalCollision {
    pub px :f32,
    pub py: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BlockCollision {
    pub px: f32,
    pub py: f32,
    pub sx: f32,
    pub sy: f32,
    pub degree: f32,
}

#[derive(Debug)]
pub struct Stage {
    pub blocks: Vec<BlockCollision>,
    pub goal: GoalCollision,
}
impl Stage{
    pub fn new_stage(stage_count: usize) -> Stage{
        match stage_count {
            1 => get_stage1(),
            2 => get_stage2(),
            3 => get_stage3(),
            4 => get_stage4(),
            5 => get_stage5(),
            6 => get_stage6(),
            7 => get_stage7(),
            _ => get_stage1(),
        }
    }
    pub fn new(gpx: f32, gpy: f32) -> Stage{
        Stage { 
            blocks: Vec::new(), 
            goal: GoalCollision{
                px: gpx, 
                py: gpy
            }
        }
    }
    pub fn add_block(mut self, px: f32, py: f32, sx: f32, sy: f32, degree: f32) -> Stage{
        let bc = BlockCollision{
            px: px,
            py: py,
            sx: sx,
            sy: sy,
            degree: degree,
        };
        self.blocks.push(bc);
        self
    }
}