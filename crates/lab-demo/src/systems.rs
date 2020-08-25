use lab_world::*;
use lab_builder::prelude::*;
use lab_entities::prelude::*;
use crate::*;
use bevy::prelude::*;
use std::rc::Rc;
/*
 &["gravel","wall","floor","tile","gravel_h","brick","brick_door_closed","chair",
            "gravel_v","brick_window","brick_door_open","shelf","brick_window_broken","bed","table","fridge"];*/
mod tiles {
    pub const WALL : &'static str = "wall";
    pub const FLOOR : &'static str = "floor";
    pub const BRICK_DOOR : &'static str = "brick_door_closed";
    pub const BRICK_DOOR_OPEN : &'static str = "brick_door_open";
    pub const BRICK : &'static str = "brick";
    pub const BRICK_WINDOW : &'static str = "brick_window";
    pub const BRICK_WINDOW_OPEN : &'static str = "brick_window_broken";    
    pub const NPC : &'static str = "npc_0";   
    pub const ITEM : &'static str = "item_50";   
    pub const LOCKED_DOOR : &'static str = "locked_door";   
}

/// Adds a simple map using the map builder for the purposes of a demo.

pub fn create_simple_map_system(mut commands: Commands, mut palette: ResMut<TilePalette>) {

    // setup some basic interactions

    if let Some(mut tiles) = palette.components.get_mut(tiles::WALL) {
        // walls are hard
        tiles.hardness = Hardness(1.);
        tiles.tile_attributes.hardness = 1.;
        tiles.tile_attributes.hit_points = 200;
    }
    
    if let Some(mut tiles) = palette.components.get_mut(tiles::BRICK) {
        // brick walls are beefier
        tiles.hardness = Hardness(1.);
        tiles.tile_attributes.hardness = 1.;
        tiles.tile_attributes.hit_points = 800;
    }
    if let Some(mut tiles) = palette.components.get_mut(tiles::BRICK_DOOR) {
        // open doors
        tiles.hardness = Hardness(0.5);
        tiles.tile_attributes.hardness = 1.;
        tiles.tile_attributes.hit_points = 1;
        tiles.interaction = lab_world::Interaction { call: |ctx| {            
            InteractionResult::ChangeTile(TileAttributes { hardness: 0.0, sprite_idx: Some(ctx.tile_palette.unwrap().components.get(tiles::BRICK_DOOR_OPEN).unwrap().sprite.atlas_sprite), ..Default::default()})
        },
            description: "Open Door",}
    }
    if let Some(mut tiles) = palette.components.get(tiles::BRICK_DOOR) {
        // open doors
        let mut new_tile = tiles.clone();

        new_tile.hardness = Hardness(0.5);
        new_tile.tile_attributes.hardness = 1.;
        new_tile.tile_attributes.hit_points = 1;
        new_tile.interaction = lab_world::Interaction { call: |ctx| {    
             
            // poor state tracking right now TODO Refactor and make safer
            let open_sprite = ctx.tile_palette.unwrap().components.get(tiles::BRICK_DOOR_OPEN).unwrap().sprite.atlas_sprite;
            let current_sprite = ctx.sprite_info.unwrap().atlas_sprite;

            if open_sprite == current_sprite {
                return InteractionResult::None;
            }

            if let Some(inventory) = ctx.inventory 
            {

                println!("inventory: {}, current_sprite: {} open_sprite: {}", inventory.items.len(), current_sprite, open_sprite);
                if inventory.items.len() > 0 {
                    println!("Unlocked door");
                    return InteractionResult::ChangeTile(TileAttributes { hardness: 0.0, sprite_idx: Some(open_sprite), ..Default::default()})
                }
            }
            InteractionResult::Block
        },
            description: "Open Door",};

        palette.components.insert("locked_door".to_string(), new_tile);
    }
    if let Some(mut tiles) = palette.components.get_mut(tiles::BRICK_WINDOW) {
        // break windows
        tiles.hardness = Hardness(0.5);
        tiles.tile_attributes.hardness = 1.;
        tiles.tile_attributes.hit_points = 1;
        tiles.interaction = lab_world::Interaction { call: |ctx| {            
            InteractionResult::ChangeTile(TileAttributes { hardness: 0.0, sprite_idx: Some(ctx.tile_palette.unwrap().components.get(tiles::BRICK_WINDOW_OPEN).unwrap().sprite.atlas_sprite), ..Default::default()})
        },
            description: "Break Window",}
    }
    if let Some(mut tiles) = palette.components.get_mut(tiles::ITEM) {
        // break windows
        tiles.interaction = lab_world::Interaction { call: |ctx| {    
            let item = Item { 
                id : 1,
                name: "Test Item".to_string(),
                weight: Weight(0.1),
                item_type: ItemType::Weapon,
                item_slot: ItemSlot::LeftHand
            };
            println!("Interaction with item {:?}", item);
            
            if let Some(mut inventory) = ctx.inventory {
                inventory.items.push(item.clone())
            }
            InteractionResult::PickUp(item)
        },
            description: "Get Item",}
    }
    
    let mut mb = MapBuilder::new(
        Rc::new(palette.clone()), // may have to share the pallete later, so adding resource counting now
        &Location::default()
    );

    &mut mb
            .add_tiles(RelativePosition::RightOf, 5, tiles::WALL.to_string())
            .add_tiles(RelativePosition::Below, 5, tiles::WALL.to_string())
            .add_tiles(RelativePosition::LeftOf, 1, tiles::WALL.to_string())
            .add_tiles(RelativePosition::LeftOf, 1, tiles::BRICK_DOOR.to_string())
            .add_tiles(RelativePosition::LeftOf, 1, tiles::WALL.to_string())
            .add_tiles(RelativePosition::LeftOf, 2, tiles::WALL.to_string())
            .add_tiles(RelativePosition::Above, 5, tiles::WALL.to_string())
            .add_tiles_to_area(&Location(0.,0.,0., WorldLocation::World), Area(6., 6.), tiles::FLOOR.to_string())
            .to_blueprint("basic_house");

    mb
    .add_tiles(RelativePosition::RightOf, 5, tiles::BRICK.to_string())
    .add_tiles(RelativePosition::Below, 5, tiles::BRICK.to_string())
    .add_tiles(RelativePosition::LeftOf, 2, tiles::BRICK_WINDOW.to_string())
    .add_tiles(RelativePosition::LeftOf, 1, tiles::LOCKED_DOOR.to_string())
    .add_tiles(RelativePosition::LeftOf, 1, tiles::BRICK_WINDOW.to_string())
    .add_tiles(RelativePosition::LeftOf, 1, tiles::BRICK.to_string())
    .add_tiles(RelativePosition::Above, 5, tiles::BRICK.to_string())
    .add_tiles_to_area(&Location(0.,0.,0., WorldLocation::World), Area(6., 6.), tiles::FLOOR.to_string())
    .to_blueprint("brick_house");

    mb
    .add_tiles_to_area(&Location::default(), Area(2., 6.), tiles::FLOOR.to_string())
    .to_blueprint("walkway");


    mb
        .add_tiles_from_blueprint("basic_house")
        .add_tiles_from_blueprint("brick_house")
        .add_tiles_from_blueprint("walkway")
        .add_tiles_from_blueprint("basic_house")
        .set_position(Location(16.,0.,3., WorldLocation::World))
        .add_tiles(RelativePosition::Below, 1,  tiles::ITEM.to_string());
        //.add_tiles_from_blueprint("walkway");*/
         //.add_tiles_from_blueprint("basic_house_2");
    


    for comp in mb.iter() {
        commands.spawn(comp.clone()).with_bundle(comp.sprite.to_components(comp.location.into(), Scale(1.)));
    }

    //commands.spawn((Moveable, Location(TILE_SIZE*2.,TILE_SIZE*2.,2.), Visible));
}
