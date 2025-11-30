// src/state/actions.rs

use crate::map::position::MapPosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuSelection {
    EnterBuildMode,
    EnterPlayMode,
    LoadLatest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildTool {
    Wall,
    Floor,
    Unit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameTool {
    Select,
    Move,
    Attack,
    Skill,
}

#[derive(Debug, Clone)]
pub enum Action {
    QuitApp,
    BackToMenu,
    MenuSelect(MenuSelection),
    
    // Editor Actions
    CycleBuildTool,
    SaveMap,
    EditorClick { pos: MapPosition },
    EditorMenuClick { screen_x: i32, screen_y: i32 },
    
    // Editor Input & Hotkeys
    EditorType(char),     
    EditorKeyPress(char),  
    EditorBackspace,
    EditorConfirm,
    EditorCancel,

    // Game Actions
    GameClick { pos: MapPosition },
    GameMenuClick { screen_x: i32, screen_y: i32 },
    GameKeyPress(char), 
}