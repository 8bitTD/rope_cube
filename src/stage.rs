//use bevy::prelude::*;

#[derive(Debug)]
pub struct GoalCollision {
    pub px :f32,
    pub py: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct BlockCollision {
    pub px: f32,
    pub py: f32,
    pub sx: f32,
    pub sy: f32,
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
            _ => get_stage3(),
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
    pub fn add_block(mut self, px: f32, py: f32, sx: f32, sy: f32) -> Stage{
        let bc = BlockCollision{
            px: px,
            py: py,
            sx: sx,
            sy: sy
        };
        self.blocks.push(bc);
        self
    }
}

pub fn get_stage1() -> Stage{
    Stage::new(0.0, -900.0)
        .add_block(0.0, 210.0, 760.0, 10.0)
        .add_block(0.0, -390.0, 500.0, 10.0)
        .add_block(-770.0, -480.0, 10.0, 700.0)
        .add_block(770.0, -480.0, 10.0, 700.0)
        .add_block(0.0, -1170.0, 760.0, 10.0)
}

pub fn get_stage2() -> Stage{
    Stage::new(1000.0, 830.0)//ゴール
        .add_block(440.0, 990.0, 640.0, 10.0)//上
        .add_block(440.0, -210.0, 640.0, 10.0)//下
        .add_block(-210.0, 390.0, 10.0, 610.0)//左
        .add_block(1090.0, 390.0, 10.0, 610.0)//右
        .add_block(360.0,190.0,420.0, 10.0)
        .add_block(580.0,550.0,500.0, 10.0)
}
pub fn get_stage3() -> Stage{
    Stage::new(0.0, 130.0)//ゴール
        .add_block(-150.0, 1000.0, 1000.0, 10.0)//上
        .add_block(-150.0, -1000.0, 1000.0, 10.0)//下
        .add_block(-1140.0, 0.0, 10.0, 1000.0)//左
        .add_block(840.0, 0.0, 10.0, 1000.0)//右
        .add_block(-40.0, -400.0, 170.0, 10.0)
        .add_block(-160.0, 50.0, 300.0, 10.0)
        .add_block(-470.0, -470.0, 10.0, 530.0)
        .add_block(140.0, 0.0, 10.0, 800.0)
        .add_block(-310.0, 750.0, 10.0, 250.0)
        .add_block(-390.0, 300.0, 530.0, 10.0)
        .add_block(-910.0, 0.0, 10.0, 700.0)
}