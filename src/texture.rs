pub mod texture {

    #[derive(Eq, PartialEq, Clone, Copy)]
    pub enum Texture {
        ClassRed,
        ClassBlue,
        ClassYellow,

        CdtnUniversal,
        CdtnBranchEqual,
        CdtnLeaf,
        CdtnLinked,

        LineHorizontal,
        LineVertical,
        LineDiagonalBottomLeftTopRight,
        LineDiagonalTopLeftBottomRight,

        SetTileVertical,
        SetTileHorizontal,
        SetTileBottomLeft,
        SetTileBottomRight,
        SetTileTopLeft,
        SetTileTopRight,

        BgTileNode,
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

        BtnCheckAnswer,
        BtnClearLines,
        BtnGoBack,

        Missing,
    }

    impl Default for Texture {
        fn default() -> Self {
            Self::BgTileSideRight
        }
    }

    impl Texture {
        pub fn path(&self) -> &str {
            match self {
                Texture::ClassRed => "textures/sprites/NODE_RED.png",
                Texture::ClassBlue => "textures/sprites/NODE_BLUE.png",
                Texture::ClassYellow => "textures/sprites/NODE_YELLOW.png",

                Texture::CdtnUniversal => "textures/sprites/CDTN_UNIVERSAL.png",
                Texture::CdtnBranchEqual => "textures/sprites/CDTN_BRANCH_EQUAL.png",
                Texture::CdtnLeaf => "textures/sprites/CDTN_LEAF.png",
                Texture::CdtnLinked => "textures/sprites/CDTN_LINKED.png",

                Texture::LineHorizontal => "textures/sprites/LINE_HORIZONTAL.png",
                Texture::LineVertical => "textures/sprites/LINE_VERTICAL.png",
                Texture::LineDiagonalBottomLeftTopRight => {
                    "textures/sprites/LINE_DIAGONAL_BL_TR.png"
                }
                Texture::LineDiagonalTopLeftBottomRight => {
                    "textures/sprites/LINE_DIAGONAL_TL_BR.png"
                }

                Texture::SetTileVertical => "textures/background/SET_TILE_VERTICAL.png",
                Texture::SetTileHorizontal => "textures/background/SET_TILE_HORIZONTAL.png",
                Texture::SetTileBottomLeft => "textures/background/SET_TILE_BOTTOM_LEFT.png",
                Texture::SetTileBottomRight => "textures/background/SET_TILE_BOTTOM_RIGHT.png",
                Texture::SetTileTopLeft => "textures/background/SET_TILE_TOP_LEFT.png",
                Texture::SetTileTopRight => "textures/background/SET_TILE_TOP_RIGHT.png",

                Texture::BgTileNode => "textures/background/BG_TILE_NODE.png",
                Texture::BgTileSideLeft => "textures/background/BG_TILE_SIDE_LEFT.png",
                Texture::BgTileSideRight => "textures/background/BG_TILE_SIDE_RIGHT.png",
                Texture::BgTileSideBottom => "textures/background/BG_TILE_SIDE_BOTTOM.png",
                Texture::BgTileSideTop => "textures/background/BG_TILE_SIDE_TOP.png",
                Texture::BgTileBottomLeft => "textures/background/BG_TILE_BOTTOM_LEFT.png",
                Texture::BgTileBottomRight => "textures/background/BG_TILE_BOTTOM_RIGHT.png",
                Texture::BgTileTopLeft => "textures/background/BG_TILE_TOP_LEFT.png",
                Texture::BgTileTopRight => "textures/background/BG_TILE_TOP_RIGHT.png",
                Texture::BgTileBetweenHorizontal => {
                    "textures/background/BG_TILE_BETWEEN_HORIZONTAL.png"
                }
                Texture::BgTileBetweenVertical => {
                    "textures/background/BG_TILE_BETWEEN_VERTICAL.png"
                }
                Texture::BgTileBetweenCross => "textures/background/BG_TILE_BETWEEN_CROSS.png",

                Texture::BtnCheckAnswer => "textures/buttons/BTN_CHECK_ANSWER.png",
                Texture::BtnClearLines => "textures/buttons/BTN_CLEAR_LINES.png",
                Texture::BtnGoBack => "textures/buttons/BTN_GO_BACK.png",

                Texture::Missing => "",
            }
        }
    }
}
