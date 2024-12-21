use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum BlockStateValue {
    String(String),
    Bool(bool),
    Int(i32),
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct BlockTransitionalState {
    name: String,
    states: HashMap<String, BlockStateValue>,
}

pub trait WorldBlockTrait: Clone + PartialEq<Self> {
    type UserState;
    fn from_transition(value: BlockTransitionalState, state: &mut Self::UserState) -> Self;
    fn into_transition(self, state: &mut Self::UserState) -> BlockTransitionalState;

    fn get_id(&self) -> &str;
    fn air(state: &mut Self::UserState) -> Self;
    fn from_other(other: &Self) -> Self {
        other.clone()
    }
}

#[cfg(feature = "default-impl")]
pub mod default_impl {
    use super::*;
    use std::fmt::Formatter;
    use std::marker::PhantomData;

    pub struct WorldBlock<UserState> {
        id: String,
        pub states: HashMap<String, BlockStateValue>,
        _state_marker: PhantomData<UserState>,
    }

    impl<T> WorldBlock<T> {
        pub fn new(id: String) -> Self {
            Self {
                id,
                states: HashMap::new(),
                _state_marker: PhantomData,
            }
        }
    }

    impl<'a, UserState> PartialEq<Self> for WorldBlock<UserState> {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id && self.states == other.states
        }
    }

    impl<UserState> WorldBlockTrait for WorldBlock<UserState> {
        type UserState = UserState;

        fn from_transition(value: BlockTransitionalState, _: &mut Self::UserState) -> Self {
            Self {
                id: value.name.clone(),
                states: value.states.clone(),
                _state_marker: PhantomData,
            }
        }

        fn into_transition(self, _: &mut Self::UserState) -> BlockTransitionalState {
            BlockTransitionalState {
                name: self.id,
                states: self.states,
            }
        }

        fn get_id(&self) -> &str {
            &self.id
        }

        fn air(_: &mut Self::UserState) -> Self {
            Self {
                id: String::from("minecraft:air"),
                states: HashMap::new(),
                _state_marker: PhantomData,
            }
        }
    }

    impl<T> std::fmt::Debug for WorldBlock<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "WorldBlock(ID: {}, States: {:?})", self.id, self.states)
        }
    }

    impl<T> std::clone::Clone for WorldBlock<T> {
        fn clone(&self) -> Self {
            Self {
                id: self.id.clone(),
                states: self.states.clone(),
                _state_marker: PhantomData,
            }
        }
    }
}
