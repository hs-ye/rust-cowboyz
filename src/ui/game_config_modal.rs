//! Game Configuration Modal Component
//!
//! A modal dialog for configuring new game settings including:
//! - Difficulty selection (Easy/Normal/Hard)
//! - Starting credits based on difficulty
//! - Turn limit configuration
//! - Warning when starting new game

#[cfg(feature = "web")]
use leptos::view;
#[cfg(feature = "web")]
use leptos::IntoView;
#[cfg(feature = "web")]
use leptos::component;
#[cfg(feature = "web")]
use leptos::Callback;
#[cfg(feature = "web")]
use leptos::Callable;
#[cfg(feature = "web")]
use leptos::RwSignal;
#[cfg(feature = "web")]
use leptos::SignalGet;
#[cfg(feature = "web")]
use leptos::SignalSet;
#[cfg(feature = "web")]
use crate::game_state::GameDifficulty;
#[cfg(feature = "web")]
use wasm_bindgen::JsCast;

#[cfg(not(feature = "web"))]
use crate::game_state::GameDifficulty;

/// Signal to control modal visibility
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GameConfigModalState {
    pub is_open: bool,
}

impl GameConfigModalState {
    pub fn new() -> Self {
        GameConfigModalState { is_open: false }
    }
}

/// Game configuration data
#[derive(Debug, Clone, PartialEq)]
pub struct GameConfig {
    pub difficulty: GameDifficulty,
    pub turn_limit: u32,
    pub starting_credits: u32,
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig {
            difficulty: GameDifficulty::Normal,
            turn_limit: GameDifficulty::Normal.turn_limit(),
            starting_credits: GameDifficulty::Normal.starting_money(),
        }
    }
}

/// Game Configuration Modal Component
#[cfg(feature = "web")]
#[component]
pub fn GameConfigModal(
    on_close: Callback<()>,
    on_start: Callback<GameConfig>,
    has_existing_game: bool,
) -> impl IntoView {
    // Local state for form inputs
    let selected_difficulty: RwSignal<GameDifficulty> = RwSignal::new(GameDifficulty::Normal);
    let custom_turn_limit: RwSignal<u32> = RwSignal::new(10u32);
    let show_warning: RwSignal<bool> = RwSignal::new(false);

    // Calculate starting credits based on difficulty
    let starting_credits = move || selected_difficulty.get().starting_money();

    // Calculate turn limit based on difficulty or custom
    let turn_limit = move || {
        match selected_difficulty.get() {
            GameDifficulty::Custom { .. } => custom_turn_limit.get(),
            _ => selected_difficulty.get().turn_limit(),
        }
    };

    // Handle difficulty change
    let on_difficulty_change = move |difficulty: GameDifficulty| {
        selected_difficulty.set(difficulty);
        // Reset custom turn limit when switching away from custom
        if !matches!(difficulty, GameDifficulty::Custom { .. }) {
            custom_turn_limit.set(difficulty.turn_limit());
        }
    };

    // Handle start game button click
    let on_start_click = move |_| {
        if has_existing_game && !show_warning.get() {
            // Show warning first
            show_warning.set(true);
        } else {
            // Start the game
            let config = GameConfig {
                difficulty: selected_difficulty.get(),
                turn_limit: turn_limit(),
                starting_credits: starting_credits(),
            };
            on_start.call(config);
            on_close.call(());
        }
    };

    // Handle cancel
    let on_cancel = move |_| {
        show_warning.set(false);
        on_close.call(());
    };

    view! {
        <div class="modal-overlay" on:click={on_cancel}>
            <div class="modal-content" on:click=|e| e.stop_propagation()>
                <div class="modal-header">
                    <h2>"新游戏配置" </h2>
                    <span class="modal-subtitle">"New Game Configuration"</span>
                    <button class="modal-close" on:click={on_cancel}>"×"</button>
                </div>

                <div class="modal-body">
                    // Warning message for existing game
                    {move || {
                        if has_existing_game {
                            view! {
                                <div class="warning-box">
                                    <span class="warning-icon">"⚠"</span>
                                    <div class="warning-text">
                                        <strong>"警告 Warning"</strong>
                                        <p>"开始新游戏将覆盖当前进度。所有进度将会丢失！"</p>
                                        <p class="hint">"Starting a new game will overwrite your current progress. All progress will be lost!"</p>
                                    </div>
                                </div>
                            }
                        } else {
                            view! { <div class="warning-box hidden"></div> }
                        }
                    }}

                    // Difficulty Selection
                    <div class="form-section">
                        <label class="form-label">
                            "难度 Difficulty"
                        </label>
                        <div class="difficulty-options">
                            <button
                                class="difficulty-btn"
                                class:selected={move || selected_difficulty.get() == GameDifficulty::Easy}
                                on:click={move |_| on_difficulty_change(GameDifficulty::Easy)}
                            >
                                <span class="difficulty-name">"简单 Easy"</span>
                                <span class="difficulty-bonus">"+$2000"</span>
                            </button>
                            <button
                                class="difficulty-btn"
                                class:selected={move || selected_difficulty.get() == GameDifficulty::Normal}
                                on:click={move |_| on_difficulty_change(GameDifficulty::Normal)}
                            >
                                <span class="difficulty-name">"普通 Normal"</span>
                                <span class="difficulty-bonus">"$1000"</span>
                            </button>
                            <button
                                class="difficulty-btn"
                                class:selected={move || selected_difficulty.get() == GameDifficulty::Hard}
                                on:click={move |_| on_difficulty_change(GameDifficulty::Hard)}
                            >
                                <span class="difficulty-name">"困难 Hard"</span>
                                <span class="difficulty-bonus">"$500"</span>
                            </button>
                        </div>
                    </div>

                    // Starting Credits Display
                    <div class="form-section">
                        <label class="form-label">
                            "起始资金 Starting Credits"
                        </label>
                        <div class="credits-display">
                            <span class="credits-value"> {move || format!("${}", starting_credits())}</span>
                            <span class="credits-label">
                                {move || {
                                    match selected_difficulty.get() {
                                        GameDifficulty::Easy => "简单模式加成 Easy mode bonus",
                                        GameDifficulty::Normal => "标准资金 Standard",
                                        GameDifficulty::Hard => "困难模式挑战 Hard mode challenge",
                                        GameDifficulty::Custom { .. } => "自定义 Custom",
                                    }
                                }}
                            </span>
                        </div>
                    </div>

                    // Turn Limit Configuration
                    <div class="form-section">
                        <label class="form-label">
                            "回合限制 Turn Limit"
                        </label>
                        <div class="turn-limit-display">
                            <span class="turn-value"> {move || format!("{} 回合", turn_limit())}</span>
                            <span class="turn-label">
                                {move || {
                                    match selected_difficulty.get() {
                                        GameDifficulty::Easy => "更多回合，更轻松 More turns, easier",
                                        GameDifficulty::Normal => "标准回合数 Standard",
                                        GameDifficulty::Hard => "有限回合，挑战极限 Limited turns, challenge",
                                        GameDifficulty::Custom { .. } => "自定义 Custom",
                                    }
                                }}
                            </span>
                        </div>
                        <div class="turn-slider-container">
                            <input
                                type="range"
                                min="3"
                                max="30"
                                value={turn_limit}
                                on:input={move |e| {
                                    if let Some(target) = e.target() {
                                        if let Some(input) = target.dyn_ref::<web_sys::HtmlInputElement>() {
                                            let value = input.value().parse::<u32>().unwrap_or(10);
                                            custom_turn_limit.set(value);
                                            selected_difficulty.set(GameDifficulty::Custom {
                                                price_volatility: 1.0,
                                                starting_money: selected_difficulty.get().starting_money(),
                                                turn_limit: value,
                                            });
                                        }
                                    }
                                }}
                                class="turn-slider"
                            />
                            <div class="slider-labels">
                                <span>"3"</span>
                                <span>"30"</span>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="modal-footer">
                    <button class="btn btn-secondary" on:click={on_cancel}>
                        "取消 Cancel"
                    </button>
                    <button
                        class="btn btn-primary"
                        class:btn-warning={move || has_existing_game && !show_warning.get()}
                        on:click={on_start_click}
                    >
                        {move || {
                            if has_existing_game && !show_warning.get() {
                                view! { <span>"确认新游戏 Confirm New Game"</span> }
                            } else {
                                view! { <span>"开始游戏 Start Game"</span> }
                            }
                        }}
                    </button>
                </div>
            </div>
        </div>
    }
}

/// Helper function to create modal state signal
#[cfg(feature = "web")]
pub fn create_modal_state() -> (RwSignal<GameConfigModalState>, Callback<()>, Callback<()>) {
    let is_open: RwSignal<GameConfigModalState> = RwSignal::new(GameConfigModalState::new());

    let open = Callback::new(move |_| {
        is_open.set(GameConfigModalState { is_open: true });
    });

    let close = Callback::new(move |_| {
        is_open.set(GameConfigModalState { is_open: false });
    });

    (is_open, open, close)
}