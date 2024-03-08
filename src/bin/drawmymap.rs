//这是一个绘画地图用的模块，可以绘制地图并以txt数据保存下来
//应该有的功能：选择地图种类（楼层地图？楼梯侧视图？）
//            选择地图大小（网格行列数）
//            选择地图元素


use bevy::prelude::*;

use hospital_evacuation::map::*;

fn main(){
    App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup,resource_insert)
    .add_systems(Update,load_mymap)
    .run();

}

fn resource_insert(
    mut commands: Commands,
){
    commands.insert_resource({
        Map::new()
    })
}

fn load_mymap(
    asset_server: Res<AssetServer>,
    map: Res<Map>,
    mut commands: Commands,
){}