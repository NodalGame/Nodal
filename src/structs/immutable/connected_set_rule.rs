pub mod connected_set_rule {
    use bevy::{math::Vec2, color::Color, sprite::Sprite};
    use serde::{Deserialize, Serialize};

    use crate::{
        logic::connected_rule_checks::connected_rule_checks::are_homomorphic,
        structs::immutable::{
            game_set::game_set::GameSet,
            solution::solution::Solution,
        },
        CDTN_RULE_SPRITE_SIZE, COLOR_RULE_ORANGE_UNSAT, COLOR_RULE_RED_UNSAT,
        COLOR_RULE_YELLOW_UNSAT,
    };

    /// ConnectedSetRule implies connectivity between rules across
    /// different sets that share the same rule class. For example, a
    /// homomorphic rule on two sets would imply connectivity between
    /// the graphs of the two sets, while if they were different class the
    /// rule would not apply.
    #[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, Hash)]
    pub enum ConnectedSetRule {
        /// This set must be homomorphic to every other set with a rule
        /// of the same rule class.
        Homomorphic(RuleClass),
    }

    impl ConnectedSetRule {
        pub fn rule_class(&self) -> &RuleClass {
            match self {
                ConnectedSetRule::Homomorphic(rule_class) => rule_class,
            }
        }

        pub fn sprite(&self) -> Sprite {
            Sprite {
                custom_size: Some(Vec2::new(CDTN_RULE_SPRITE_SIZE, CDTN_RULE_SPRITE_SIZE)),
                color: self.rule_class().color().clone(),
                ..Default::default()
            }
        }

        pub fn is_satisfied(&self, sets: Vec<&GameSet>, solution: &Solution) -> bool {
            match self {
                ConnectedSetRule::Homomorphic(_rule_class) => are_homomorphic(sets, solution),
            }
        }
    }

    /// RuleClass is the class of the ConnectedSetRule.
    #[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, Hash)]
    pub enum RuleClass {
        Yellow,
        Orange,
        Red,
    }

    impl RuleClass {
        pub fn color(&self) -> &Color {
            match self {
                RuleClass::Yellow => &COLOR_RULE_YELLOW_UNSAT,
                RuleClass::Orange => &COLOR_RULE_ORANGE_UNSAT,
                RuleClass::Red => &COLOR_RULE_RED_UNSAT,
            }
        }
    }
}
