pub mod texture {
    pub enum Texture {
        NodeEmpty,
        BgTileSideLeft,
        BgTileSideRight,
        BgTileSideBottom,
        BgTileSideTop,
        BgTileBottomLeft,
        BgTileBottomRight,
        BgTileTopLeft,
        BgTileTopRight,
        BgTileBetweenHorizontal,
        BgTileBetweenVertical,
        BgTileBetweenCross,
    }

impl Default for Texture {
    fn default() -> Self {
        Self::BgTileSideRight
    }
}
    
    impl Texture {
        pub fn path(&self) -> &str {
            match self {
                Texture::NodeEmpty => "textures/sprites/NODE_EMPTY.png",
                Texture::BgTileSideLeft => "textures/background/BG_TILE_SIDE_LEFT.png",
                Texture::BgTileSideRight => "textures/background/BG_TILE_SIDE_RIGHT.png",
                Texture::BgTileSideBottom => "textures/background/BG_TILE_SIDE_BOTTOM.png",
                Texture::BgTileSideTop => "textures/background/BG_TILE_SIDE_TOP.png",
                Texture::BgTileBottomLeft => "textures/background/BG_TILE_BOTTOM_LEFT.png",
                Texture::BgTileBottomRight => "textures/background/BG_TILE_BOTTOM_RIGHT.png",
                Texture::BgTileTopLeft => "textures/background/BG_TILE_TOP_LEFT.png",
                Texture::BgTileTopRight => "textures/background/BG_TILE_TOP_RIGHT.png",
                Texture::BgTileBetweenHorizontal => "textures/background/BG_TILE_BETWEEN_HORIZONTAL.png",
                Texture::BgTileBetweenVertical => "textures/background/BG_TILE_BETWEEN_VERTICAL.png",
                Texture::BgTileBetweenCross => "textures/background/BG_TILE_BETWEEN_CROSS.png",
            }
        }
    }
}