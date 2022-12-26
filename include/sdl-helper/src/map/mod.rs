use std::path::Path;

use tiled;
use crate::Camera;
use crate::FontManager;
use crate::TextureManager;
use crate::font_manager;

mod tile;
mod layer;

use tile::*;
use layer::*;


/// Used for drawing [tiled] maps
pub struct Map {
    tiled_map: tiled::Map,
    tiles : Vec<Tile>,
    layers : Vec<Layer>,
}

impl Map {
    pub fn new<'sdl, TexType>(filename: &str, tex_manager : &'sdl mut TextureManager<TexType>, font_manager: &'sdl mut FontManager<TexType>) -> Result<Self, String> {
        let mut map = Self {
            tiled_map: tiled::Map::new(filename).unwrap(),
            tiles: Vec::new(),
            layers: Vec::new(),
        };

        map.layers.resize(
            map.tiled_map.layers.len() + map.tiled_map.img_layers.len() + map.tiled_map.obj_groups.len(),
            Layer::blank()
        );

        map.load_tilesets(tex_manager)?;
        map.set_map_draws();
        map.set_img_layers(tex_manager)?;
        map.set_obj_group_layers(font_manager)?;
        
        map.clear_blank_layers();
        
        Ok(map)
    }

    pub fn draw(&self, cam: &mut Camera) {
        for l in self.layers.iter() {
            for t in l.tile_draws.iter() {
                cam.draw(t);
            }
            match l.image_draw {
                Some(g) => cam.draw(&g),
                None => (),
            }
        }
    }

    fn load_tilesets<'sdl, TexType>(&mut self, tex_manager : &'sdl mut TextureManager<TexType>) -> Result<(), String> {
        self.tiles.resize(self.tiled_map.total_tiles as usize, Tile::new());
        // blank tile
        self.tiles[0].rect.w = self.tiled_map.tile_width as f64;
        self.tiles[0].rect.h = self.tiled_map.tile_height as f64;
        for ts in self.tiled_map.tilesets.iter() {
            load_tileset(&mut self.tiles, ts, tex_manager.load(&Path::new(&ts.image_path))?)?;
        }
        Ok(())
    }

    fn set_map_draws(&mut self) {
        for l in self.tiled_map.layers.iter() {
            self.layers[l.info.layer_position as usize] = Layer::new_tile_layer(&l, &self.tiles);
        }
    }

    fn set_img_layers<'sdl, TexType>(&mut self, tex_manager : &'sdl mut TextureManager<TexType>) -> Result<(), String> {
        for l in self.tiled_map.img_layers.iter() {
            self.layers[l.info.layer_position as usize] = Layer::new_image_layer(
                l, tex_manager.load(Path::new(&(self.tiled_map.path.clone() + &l.image_path)))?
            )
        }
        Ok(())
    }

    fn set_obj_group_layers<'sdl, TexType>(&mut self, font_manager : &'sdl mut FontManager<TexType>) -> Result<(), String> {
        for l in self.tiled_map.obj_groups.iter() {
            self.layers[l.info.layer_position as usize] = Layer::new_object_layer(
                l,
                font_manager
            )?;
        }

        Ok(())
    }

    fn clear_blank_layers(&mut self) {
        let mut i = 0;
        while i < self.layers.len() {
            if self.layers[i].is_blank() {
                self.layers.remove(i);
                i -= 1;
            }
            i += 1;
        }
    }
}
