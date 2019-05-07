use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;


pub type GameTextureCreator = TextureCreator<WindowContext>;
pub type GameTexture<'a> = Texture<'a>;
