use bevy::{platform::collections::HashMap, prelude::*};

#[derive(Clone, Default, Debug, Reflect, Eq, Hash, PartialEq)]
#[reflect(Default)]
pub enum BoardKeys {
    #[default]
    Unknown,
    Health,
    Test,
}

#[derive(Default, Reflect, Debug, Clone)]
#[reflect(Default)]
pub enum BoardValueTypes {
    #[default]
    Unknown,
    Int,
    Float,
    String,
    Bool,
}

#[derive(Component, Clone, Default, Debug, Reflect)]
pub struct ThinkBoard {
    main_goal: bool,
    bonus: u32,

    bool_map: HashMap<BoardKeys, bool>,
    float_map: HashMap<BoardKeys, f32>,
    int_map: HashMap<BoardKeys, i32>,
}

impl ThinkBoard {
    pub fn get_bool(&self, key: &BoardKeys) -> Option<bool> {
        self.bool_map.get(key).copied()
    }
    pub fn set_bool(&mut self, key: BoardKeys, value: bool) {
        self.bool_map.insert(key, value);
    }

    pub fn get_float(&self, key: &BoardKeys) -> Option<f32> {
        self.float_map.get(key).copied()
    }
    pub fn set_float(&mut self, key: BoardKeys, value: f32) {
        self.float_map.insert(key, value);
    }
    pub fn get_int(&self, key: &BoardKeys) -> Option<i32> {
        self.int_map.get(key).copied()
    }
    pub fn set_int(&mut self, key:  BoardKeys, value: i32) {
        self.int_map.insert(key, value);
    }
    // pub fn set_value(&mut self, key: &BoardKeys, value_type: &BoardValueTypes, value: Option<()>) {
    //     match value_type {
    //         BoardValueTypes::Bool => self.set_bool(key, value.unwrap()),
    //         BoardValueTypes::Float => self.set_float(key, value.unwrap()),
    //         BoardValueTypes::Int => self.set_int(key, value.unwrap()),
    //         _ => {}
    //     }
    // }
    // pub fn get_value(&self, key: &BoardKeys, value_type: &BoardValueTypes) -> Option<()> {
    //     match value_type {
    //         BoardValueTypes::Bool => self.get_bool(key),
    //         BoardValueTypes::Float => self.get_float(key),
    //         BoardValueTypes::Int => self.get_int(key),
    //         _ => None,
    //     }
    // }
}
