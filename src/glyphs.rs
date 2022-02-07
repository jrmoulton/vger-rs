
use crate::atlas::Atlas;
use crate::defs::*;
use std::collections::HashMap;
use rect_packer::Rect;

#[derive(Copy, Clone, Debug)]
pub struct GlyphInfo {
    pub size: u32,
    pub rect: Rect,
    pub metrics: fontdue::Metrics,
}

pub struct GlyphCache {
    atlas: Atlas,
    pub font: fontdue::Font,
    info: HashMap<(char, u32), GlyphInfo>
}

impl GlyphCache {
    pub fn new(device: &wgpu::Device) -> Self {
        
        let font = include_bytes!("fonts/Anodina-Regular.ttf") as &[u8];

        Self {
            atlas: Atlas::new(device),
            font: fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap(),
            info: HashMap::new(),
        }
    }

    pub fn get_glyph(&mut self, c: char, size: u32) -> GlyphInfo {

        // Do we already have a glyph?
        match self.info.get( &(c, size) ) {
            Some(info) => *info,
            None => {
                let (metrics, data) = self.font.rasterize(c, size as f32);

                let rect = self.atlas.add_region(&data, metrics.width as u32, metrics.height as u32).unwrap();

                let info = GlyphInfo {
                    size,
                    rect,
                    metrics
                };

                self.info.insert( (c, size), info);
                info
            }
        }
    }
}