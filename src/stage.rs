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
            _ => get_stage5(),
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

pub fn get_stage1() -> Stage{
    Stage::new(0.0, -900.0)
        .add_block(0.0, 210.0, 760.0, 10.0, 0.0)
        .add_block(0.0, -390.0, 500.0, 10.0, 0.0)
        .add_block(-770.0, -480.0, 10.0, 700.0, 0.0)
        .add_block(770.0, -480.0, 10.0, 700.0, 0.0)
        .add_block(0.0, -1170.0, 760.0, 10.0, 0.0)
}

pub fn get_stage2() -> Stage{
    Stage::new(1000.0, 830.0)//ゴール
        .add_block(440.0, 990.0, 640.0, 10.0, 0.0)//上
        .add_block(440.0, -210.0, 640.0, 10.0, 0.0)//下
        .add_block(-210.0, 390.0, 10.0, 610.0, 0.0)//左
        .add_block(1090.0, 390.0, 10.0, 610.0, 0.0)//右
        .add_block(360.0,190.0,420.0, 10.0, 0.0)
        .add_block(580.0,550.0,500.0, 10.0, 0.0)
}
pub fn get_stage3() -> Stage{
    Stage::new(30.0, -920.0)//ゴール
        .add_block(-170.0, 300.0, 400.0, 10.0, 0.0)//上
        .add_block(-560.0, -130.0, 10.0, 440.0, 0.0)//左
        .add_block(220.0, -130.0, 10.0, 440.0, 0.0)//右
        .add_block(-20.0, -170.0, 240.0, 10.0, 0.0)//中下
        .add_block(-290.0, -570.0, 280.0, 10.0, 0.0)//左下
        .add_block(150.0, -570.0, 80.0, 10.0, 0.0)//右下
        .add_block(-260.0, -80.0, 10.0, 200.0, 0.0)//縦障害物上
        .add_block(-160.0, -450.0, 10.0, 120.0, 0.0)//縦障害物下
        .add_block(00.0, -760.0, 10.0, 200.0, 0.0)//ゴール左
        .add_block(60.0, -760.0, 10.0, 200.0, 0.0)//ゴール右
        .add_block(30.0, -950.0, 30.0, 10.0, 0.0)//ゴール下
}
pub fn get_stage4() -> Stage{
    Stage::new(0.0, 130.0)//ゴール
        .add_block(-150.0, 1000.0, 1000.0, 10.0, 0.0)//上
        .add_block(-150.0, -1000.0, 1000.0, 10.0, 0.0)//下
        .add_block(-1140.0, 0.0, 10.0, 1000.0, 0.0)//左
        .add_block(840.0, 0.0, 10.0, 1000.0, 0.0)//右
        .add_block(-40.0, -400.0, 170.0, 10.0, 0.0)
        .add_block(-160.0, 50.0, 300.0, 10.0, 0.0)
        .add_block(-470.0, -470.0, 10.0, 530.0, 0.0)
        .add_block(140.0, 0.0, 10.0, 800.0, 0.0)
        .add_block(-310.0, 750.0, 10.0, 250.0, 0.0)
        .add_block(-390.0, 300.0, 530.0, 10.0, 0.0)
        .add_block(-910.0, 0.0, 10.0, 700.0, 0.0)
}

pub fn get_stage5() -> Stage{
    Stage::new(0.0, -200.0)//ゴール
        .add_block(0.0, 100.0, 1000.0, 10.0, 0.0)//上
        .add_block(0.0, -1900.0, 1000.0, 10.0, 0.0)//下
        .add_block(-1000.0, -900.0, 10.0, 1010.0, 0.0)//左
        .add_block(1000.0, -900.0, 10.0, 1010.0, 0.0)//右
        
        .add_block(-350.0, -180.0, 10.0, 350.0, 95.0)//左ギミック
        .add_block(-660.0, -420.0, 10.0, 350.0, 80.0)
        .add_block(-190.0, -920.0, 10.0, 750.0, 5.0)
        .add_block(-500.0, -750.0, 10.0, 350.0, 65.0)
        .add_block(-750.0, -950.0, 10.0, 200.0, 90.0)
        .add_block(-520.0, -1170.0, 10.0, 400.0, 120.0)
        .add_block(-650.0, -1650.0, 10.0, 400.0, 120.0)
        //.add_block(-130.0, -1320.0, 20.0, 430.0, 185.0)

        .add_block(400.0, -200.0, 10.0, 400.0, 80.0)//右ギミック
        .add_block(660.0, -500.0, 10.0, 350.0, 75.0)
        .add_block(200.0, -900.0, 10.0, 800.0, 0.0)
        .add_block(530.0, -800.0, 10.0, 330.0, 105.0)
        .add_block(810.0, -1150.0, 10.0, 190.0, 75.0)
        .add_block(390.0, -1150.0, 10.0, 190.0, 105.0)
        .add_block(700.0, -1450.0, 10.0, 300.0, 60.0)

        .add_block(-100.0, -350.0, 10.0, 40.0, 90.0)//ゴール下
        .add_block(100.0, -550.0, 10.0, 40.0, 90.0)
        .add_block(0.0, -800.0, 10.0, 10.0, 90.0)
}