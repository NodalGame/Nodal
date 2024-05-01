pub mod connected_set_rule {
    use bevy::{math::Vec2, render::color::Color, sprite::Sprite};
    use serde::{Deserialize, Serialize};

    use crate::CDTN_RULE_SPRITE_SIZE;

    /// ConnectedSetRule implies connectivity between rules across
    /// different sets that share the same rule class. For example, a
    /// homomorphism rule on two sets would imply connectivity between
    /// the graphs of the two sets, while if they were different class the
    /// rule would not apply.
    #[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
    pub enum ConnectedSetRule {
        /// This set must be a homomorphism of every other set with a rule
        /// of the same rule class.
        Homomorphism(RuleClass),
    }

    impl ConnectedSetRule {
        pub fn rule_class(&self) -> &RuleClass {
            match self {
                ConnectedSetRule::Homomorphism(rule_class) => rule_class,
            }
        }

        pub fn sprite(&self) -> Sprite {
            Sprite {
                custom_size: Some(Vec2::new(CDTN_RULE_SPRITE_SIZE, CDTN_RULE_SPRITE_SIZE)),
                color: self.rule_class().color().clone(),
                ..Default::default()
            }
        }
    }

    /// RuleClass is the class of the ConnectedSetRule.
    #[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, Hash)]
    pub enum RuleClass {
        Yellow,
        Orange,
        Red,
    }

    impl RuleClass {
        pub fn color(&self) -> &Color {
            match self {
                RuleClass::Yellow => &Color::YELLOW,
                RuleClass::Orange => &Color::ORANGE,
                RuleClass::Red => &Color::RED,
            }
        }
    }
}
