use super::stage::*;

pub fn get_stage1() -> Stage{
    Stage::new(0.0, -900.0)
        .add_block(0.0, 210.0, 760.0, 10.0, 0.0)
        .add_block(0.0, -390.0, 500.0, 10.0, 0.0)
        .add_block(-770.0, -480.0, 10.0, 700.0, 0.0)
        .add_block(770.0, -480.0, 10.0, 700.0, 0.0)
        .add_block(0.0, -1170.0, 760.0, 10.0, 0.0)
}
pub fn get_stage2() -> Stage{
    Stage::new(-106.0, 906.0)
        .add_block(-105.0, 980.0, 365.0, 10.0, 0.0)
        .add_block(-119.0, -147.0, 360.0, 10.0, 0.0)
        .add_block(-185.0, 141.0, 10.0, 405.0, -45.0)
        .add_block(525.0, 139.0, 10.0, 410.0, -45.0)
        .add_block(-180.0, 697.0, 10.0, 400.0, 45.0)
        .add_block(535.0, 701.0, 10.0, 400.0, 45.0)
}
pub fn get_stage3() -> Stage{
    Stage::new(-360.0, -1925.0)
        .add_block(-100.0, 50.0, 300.0, 10.0, 0.0)
        .add_block(200.0, -316.0, 10.0, 376.0, 0.0)
        .add_block(50.0, -691.0, 160.0, 10.0, 0.0)
        .add_block(-100.0, -877.0, 10.0, 190.0, 0.0)
        .add_block(-408.0, -700.0, 10.0, 760.0, 0.0)
        .add_block(55.0, -1057.0, 156.0, 10.0, 0.0)
        .add_block(200.0, -1528.0, 10.0, 481.0, 0.0)
        .add_block(-102.0, -2000.0, 300.0, 10.0, 0.0)
        .add_block(-253.0, -1458.0, 165.0, 10.0, 0.0)
        .add_block(-98.0, -1650.0, 10.0, 193.0, 0.0)
        .add_block(-246.0, -1833.0, 153.0, 10.0, 0.0)
        .add_block(-409.0, -1917.0, 10.0, 93.0, 0.0)
}
pub fn get_stage4() -> Stage{
    Stage::new(30.0, -920.0)
        .add_block(-170.0, 300.0, 400.0, 10.0, 0.0)
        .add_block(-560.0, -130.0, 10.0, 440.0, 0.0)
        .add_block(220.0, -130.0, 10.0, 440.0, 0.0)
        .add_block(-20.0, -170.0, 240.0, 10.0, 0.0)
        .add_block(-290.0, -570.0, 280.0, 10.0, 0.0)
        .add_block(150.0, -570.0, 80.0, 10.0, 0.0)
        .add_block(-260.0, -80.0, 10.0, 200.0, 0.0)
        .add_block(-160.0, -450.0, 10.0, 120.0, 0.0)
        .add_block(0.0, -760.0, 10.0, 200.0, 0.0)
        .add_block(60.0, -760.0, 10.0, 200.0, 0.0)
        .add_block(30.0, -950.0, 30.0, 10.0, 0.0)
}
pub fn get_stage5() -> Stage{
    Stage::new(0.0, 181.0)
        .add_block(-86.0, 696.0, 690.0, 10.0, 0.0)
        .add_block(-86.0, -278.0, 690.0, 10.0, 0.0)
        .add_block(-775.0, 209.0, 10.0, 497.0, 0.0)
        .add_block(608.0, 209.0, 10.0, 497.0, 0.0)
        .add_block(-66.0, 50.0, 211.0, 10.0, 0.0)
        .add_block(-271.0, -113.0, 10.0, 173.0, 0.0)
        .add_block(145.0, 187.0, 10.0, 340.0, 0.0)
        .add_block(-232.0, 573.0, 10.0, 126.0, 0.0)
        .add_block(-232.0, 300.0, 373.0, 10.0, 0.0)
        .add_block(-610.0, 191.0, 10.0, 307.0, 0.0)
}
pub fn get_stage6() -> Stage{
    Stage::new(0.0, -246.0)
        .add_block(0.0, 100.0, 1000.0, 10.0, 0.0)
        .add_block(0.0, -1571.0, 1000.0, 10.0, 0.0)
        .add_block(-1000.0, -734.0, 10.0, 845.0, 0.0)
        .add_block(1000.0, -734.0, 10.0, 845.0, 0.0)
        .add_block(-713.0, -492.0, 10.0, 292.0, 80.0)
        .add_block(-205.0, -768.0, 10.0, 595.0, 5.0)
        .add_block(-510.0, -838.0, 10.0, 350.0, 65.0)
        .add_block(-770.0, -1303.0, 10.0, 345.0, 140.0)
        .add_block(-5.0, -185.0, 802.0, 10.0, 0.0)
        .add_block(200.0, -826.0, 10.0, 645.0, 0.0)
        .add_block(810.0, -815.0, 10.0, 190.0, 75.0)
        .add_block(390.0, -815.0, 10.0, 190.0, 105.0)
        .add_block(720.0, -1287.0, 10.0, 320.0, 60.0)
        .add_block(-100.0, -350.0, 10.0, 40.0, 90.0)
        .add_block(8.0, -550.0, 10.0, 66.0, 90.0)
        .add_block(0.0, -800.0, 10.0, 85.0, 0.0)
}
pub fn get_stage7() -> Stage{
    Stage::new(-321.0, 975.0)
        .add_block(-292.0, 1060.0, 530.0, 10.0, 0.0)
        .add_block(-305.0, -150.0, 523.0, 10.0, 0.0)
        .add_block(-818.0, 460.0, 10.0, 610.0, 0.0)
        .add_block(228.0, 452.0, 10.0, 612.0, 0.0)
        .add_block(-128.0, 357.0, 10.0, 510.0, 0.0)
        .add_block(-228.0, 585.0, 10.0, 477.0, 0.0)
        .add_block(-432.0, 118.0, 200.0, 10.0, 0.0)
        .add_block(-652.0, 467.0, 181.0, 10.0, -20.0)
        .add_block(-422.0, 772.0, 211.0, 10.0, -20.0)
}
