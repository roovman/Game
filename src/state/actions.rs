use crate::map::position::MapPosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildTool {
    Wall,
    Floor, 
    Unit, 
}

#[derive(Debug, Clone)]
pub enum Action {
    // --- Global ---
    BackToMenu, 
    QuitApp, 

    // --- Menu ---
    MenuSelect(MenuSelection),

    // --- Editor Mode ---
    CycleBuildTool,
    EditorClick { pos: MapPosition },
    SaveMap,

    // --- Game Mode ---
    GameClick { pos: MapPosition },
}

#[derive(Debug, Clone)]
pub enum MenuSelection {
    EnterBuildMode,
    EnterPlayMode,
    LoadLatest, 
}