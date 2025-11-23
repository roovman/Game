use crate::map::position::MapPosition;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildTool {
    Wall,
    Floor, 
    Unit, 
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameTool {
    Select, // Default: Just clicking things to see stats/select them
    Move,   // Force move logic
    Attack, // Force attack logic
    Skill,  // TODO: Super ability
}

// --- NEW ACTION COSTS (Placeholder until we need them) ---
// pub const COST_WALK: u32 = 1;
// pub const COST_ATTACK: u32 = 2;
// --------------------------------------------------------

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

    EditorMenuClick { screen_x: i32, screen_y: i32 }, 
    GameMenuClick { screen_x: i32, screen_y: i32 },

    GameClick { pos: MapPosition }, 
    
}

#[derive(Debug, Clone)]
pub enum MenuSelection {
    EnterBuildMode,
    EnterPlayMode,
    LoadLatest, 
}
