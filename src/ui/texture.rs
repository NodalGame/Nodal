pub mod texture {

    #[derive(Eq, PartialEq, Clone, Copy)]
    pub enum Texture {
        Node,

        CdtnBranchEqual,
        CdtnLeaf,

        CdtnDegreeEqual,

        SetRuleDisconnected,
        SetRuleLeaf,

        SetRuleHomomorphism,

        SetRuleXor,
        SetRuleScope,

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

        SetTileBoundedVertical,
        SetTileBoundedHorizontal,
        SetTileBoundedBottomLeft,
        SetTileBoundedBottomRight,
        SetTileBoundedTopLeft,
        SetTileBoundedTopRight,

        SetRuleBox,

        BgSet0,
        BgSet1,
        BgSet2,
        BgSet3,

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
            Self::Missing
        }
    }

    impl Texture {
        pub fn path(&self) -> &str {
            match self {
                Texture::Node => "../assets/textures/sprites/NODE_EMPTY.png",

                Texture::CdtnBranchEqual => "../assets/textures/sprites/CDTN_BRANCH_EQUAL.png",
                Texture::CdtnLeaf => "../assets/textures/sprites/CDTN_LEAF.png",

                Texture::CdtnDegreeEqual => "../assets/textures/sprites/CDTN_DEGREE_EQUAL.png",

                Texture::SetRuleDisconnected => {
                    "../assets/textures/sprites/SETRULE_DISCONNECTED.png"
                }
                Texture::SetRuleLeaf => "../assets/textures/sprites/SETRULE_LEAF.png",

                Texture::SetRuleHomomorphism => {
                    "../assets/textures/sprites/SETRULE_HOMOMORPHISM.png"
                }

                Texture::SetRuleXor => "../assets/textures/sprites/SETRULE_XOR.png",
                Texture::SetRuleScope => "../assets/textures/sprites/SETRULE_SCOPE.png",

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

                Texture::SetTileBoundedVertical => {
                    "../assets/textures/background/SET_TILE_BOUNDED_VERTICAL.png"
                }
                Texture::SetTileBoundedHorizontal => {
                    "../assets/textures/background/SET_TILE_BOUNDED_HORIZONTAL.png"
                }
                Texture::SetTileBoundedBottomLeft => {
                    "../assets/textures/background/SET_TILE_BOUNDED_BOTTOM_LEFT.png"
                }
                Texture::SetTileBoundedBottomRight => {
                    "../assets/textures/background/SET_TILE_BOUNDED_BOTTOM_RIGHT.png"
                }
                Texture::SetTileBoundedTopLeft => {
                    "../assets/textures/background/SET_TILE_BOUNDED_TOP_LEFT.png"
                }
                Texture::SetTileBoundedTopRight => {
                    "../assets/textures/background/SET_TILE_BOUNDED_TOP_RIGHT.png"
                }

                Texture::SetRuleBox => "../assets/textures/sprites/SET_RULE_BOX.png",

                Texture::BgSet0 => "../assets/textures/background/BG_SET_0.png",
                Texture::BgSet1 => "../assets/textures/background/BG_SET_1.png",
                Texture::BgSet2 => "../assets/textures/background/BG_SET_2.png",
                Texture::BgSet3 => "../assets/textures/background/BG_SET_3.png",

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
