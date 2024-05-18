pub mod connected_node_condition {
    use bevy::{math::Vec2, render::color::Color, sprite::Sprite};
    use serde::{Deserialize, Serialize};

    use crate::{
        logic::connected_condition_checks::connected_condition_checks::is_degree_equal,
        objects::immutable::{
            game_node::game_node::GameNode,
            solution::{self, solution::Solution},
        },
        CDTN_RULE_SPRITE_SIZE, COLOR_CDTN_BLUE_UNSAT, COLOR_CDTN_GREEN_UNSAT,
        COLOR_CDTN_PURPLE_UNSAT,
    };

    /// ConnectedNodeCondition applies a condition to a single node in a puzzle
    /// in relation to all other nodes of the same class, "connecting" them without
    /// needing to be within the same set.
    #[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, Hash)]
    pub enum ConnectedNodeCondition {
        /// This node's degree (number of node connections) must be equal to every other
        /// node with this condition of the same ConditionClass.
        DegreeEqual(ConditionClass),
    }

    impl ConnectedNodeCondition {
        pub fn condition_class(&self) -> &ConditionClass {
            match self {
                ConnectedNodeCondition::DegreeEqual(condition_class) => condition_class,
            }
        }

        pub fn sprite(&self) -> Sprite {
            Sprite {
                custom_size: Some(Vec2::new(CDTN_RULE_SPRITE_SIZE, CDTN_RULE_SPRITE_SIZE)),
                color: self.condition_class().color().clone(),
                ..Default::default()
            }
        }

        /// Takes the connected node condition, nodes with that condition of matching class, and a solution, then
        /// returns if it is satisfied or not for those nodes (reflexive).
        pub fn is_satisfied(&self, nodes: Vec<&GameNode>, solution: &Solution) -> bool {
            match self {
                ConnectedNodeCondition::DegreeEqual(condition_class) => {
                    is_degree_equal(nodes, solution)
                }
            }
        }
    }

    /// ConditionClass is the class of the ConnectedNodeCondition.
    #[derive(Serialize, Deserialize, Clone, Copy, Debug, Eq, PartialEq, Hash)]
    pub enum ConditionClass {
        Blue,
        Purple,
        Green,
    }

    impl ConditionClass {
        pub fn color(&self) -> &Color {
            match self {
                ConditionClass::Blue => &COLOR_CDTN_BLUE_UNSAT,
                ConditionClass::Purple => &COLOR_CDTN_PURPLE_UNSAT,
                ConditionClass::Green => &COLOR_CDTN_GREEN_UNSAT,
            }
        }
    }
}
