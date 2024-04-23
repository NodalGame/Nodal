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
                Texture::ClassRed => "../assets/textures/sprites/NODE_RED.png",
                Texture::ClassBlue => "../assets/textures/sprites/NODE_BLUE.png",
                Texture::ClassYellow => "../assets/textures/sprites/NODE_YELLOW.png",

                Texture::CdtnUniversal => "../assets/textures/sprites/CDTN_UNIVERSAL.png",
                Texture::CdtnBranchEqual => "../assets/textures/sprites/CDTN_BRANCH_EQUAL.png",
                Texture::CdtnLeaf => "../assets/textures/sprites/CDTN_LEAF.png",
                Texture::CdtnLinked => "../assets/textures/sprites/CDTN_LINKED.png",

                Texture::LineHorizontal => "../assets/textures/sprites/LINE_HORIZONTAL.png",
                Texture::LineVertical => "../assets/textures/sprites/LINE_VERTICAL.png",
                Texture::LineDiagonalBottomLeftTopRight => {
                    "../assets/textures/sprites/LINE_DIAGONAL_BL_TR.png"
                }
                Texture::LineDiagonalTopLeftBottomRight => {
                    "../assets/textures/sprites/LINE_DIAGONAL_TL_BR.png"
                }

                Texture::SetTileVertical => "../assets/textures/background/SET_TILE_VERTICAL.png",
                Texture::SetTileHorizontal => {
                    "../assets/textures/background/SET_TILE_HORIZONTAL.png"
                }
                Texture::SetTileBottomLeft => {
                    "../assets/textures/background/SET_TILE_BOTTOM_LEFT.png"
                }
                Texture::SetTileBottomRight => {
                    "../assets/textures/background/SET_TILE_BOTTOM_RIGHT.png"
                }
                Texture::SetTileTopLeft => "../assets/textures/background/SET_TILE_TOP_LEFT.png",
                Texture::SetTileTopRight => "../assets/textures/background/SET_TILE_TOP_RIGHT.png",

                Texture::BgTileNode => "../assets/textures/background/BG_TILE_NODE.png",
                Texture::BgTileSideLeft => "../assets/textures/background/BG_TILE_SIDE_LEFT.png",
                Texture::BgTileSideRight => "../assets/textures/background/BG_TILE_SIDE_RIGHT.png",
                Texture::BgTileSideBottom => {
                    "../assets/textures/background/BG_TILE_SIDE_BOTTOM.png"
                }
                Texture::BgTileSideTop => "../assets/textures/background/BG_TILE_SIDE_TOP.png",
                Texture::BgTileBottomLeft => {
                    "../assets/textures/background/BG_TILE_BOTTOM_LEFT.png"
                }
                Texture::BgTileBottomRight => {
                    "../assets/textures/background/BG_TILE_BOTTOM_RIGHT.png"
                }
                Texture::BgTileTopLeft => "../assets/textures/background/BG_TILE_TOP_LEFT.png",
                Texture::BgTileTopRight => "../assets/textures/background/BG_TILE_TOP_RIGHT.png",
                Texture::BgTileBetweenHorizontal => {
                    "../assets/textures/background/BG_TILE_BETWEEN_HORIZONTAL.png"
                }
                Texture::BgTileBetweenVertical => {
                    "../assets/textures/background/BG_TILE_BETWEEN_VERTICAL.png"
                }
                Texture::BgTileBetweenCross => {
                    "../assets/textures/background/BG_TILE_BETWEEN_CROSS.png"
                }

                Texture::BtnCheckAnswer => "../assets/textures/buttons/BTN_CHECK_ANSWER.png",
                Texture::BtnClearLines => "../assets/textures/buttons/BTN_CLEAR_LINES.png",
                Texture::BtnGoBack => "../assets/textures/buttons/BTN_GO_BACK.png",

                Texture::Missing => "",
            }
        }
    }
}
