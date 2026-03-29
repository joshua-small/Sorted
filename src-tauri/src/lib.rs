mod solver;

#[tauri::command]
fn random_puzzle(settings: solver::PuzzleSettings) -> solver::PuzzleState {
    solver::random_puzzle(settings)
}

#[tauri::command]
fn random_puzzle_seeded(settings: solver::PuzzleSettings, seed_hex: String) -> Result<solver::PuzzleState, String> {
    solver::random_puzzle_seeded(settings, seed_hex)
}

#[tauri::command]
fn check_state(state: solver::PuzzleState, single_mode: bool) -> solver::StateCheck {
    solver::check_state(state, single_mode)
}

#[tauri::command]
fn solve(state: solver::PuzzleState, single_mode: bool) -> solver::SolverResult {
    solver::solve(state, single_mode)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![random_puzzle, random_puzzle_seeded, check_state, solve])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
