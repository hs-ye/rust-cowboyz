//! Cargo Panel Component
//!
//! Displays cargo hold status with a visual progress bar showing:
//! - Current used capacity vs total capacity
//! - Color-coded zones (green 0-50%, yellow 51-80%, red 81-100%)
//! - State labels ("Cargo Empty" at 0%, "CARGO FULL" at 100%)
//! - Dynamic preview of projected cargo from pending trades

#[cfg(feature = "web")]
use leptos::*;

/// CargoPanel Component
///
/// Displays a horizontal progress bar showing cargo hold status with:
/// - Color zones: Green (0-50%), Yellow (51-80%), Red (81-100%)
/// - Numeric overlay: "used/total units" format
/// - State labels: "Cargo Empty" (grey) at 0%, "CARGO FULL" (red, bold) at 100%
/// - Dynamic preview: Shows projected cargo level from pending trades
///
/// # Arguments
/// * `current_used` - Callback returning current cargo units used
/// * `capacity` - Callback returning total cargo capacity
/// * `cargo_change` - Memo containing cargo change from pending trades
///                    (positive = buying, negative = selling)
/// * `projected_cargo` - Memo containing projected cargo after trade
#[cfg(feature = "web")]
#[component]
pub fn CargoPanel(
    current_used: impl Fn() -> u32 + Clone + 'static,
    capacity: impl Fn() -> u32 + Clone + 'static,
    cargo_change: Memo<i32>,
    projected_cargo: Memo<u32>,
) -> impl IntoView {
    // Clone capacity for use in multiple memos
    let capacity_clone1 = capacity.clone();
    let capacity_clone2 = capacity.clone();
    let capacity_clone3 = capacity.clone();
    let capacity_clone4 = capacity.clone();

    // Calculate fill percentage for projected cargo
    let fill_percentage = create_memo(move |_| {
        let projected = projected_cargo.get();
        let cap = capacity_clone1();
        if cap > 0 {
            (projected as f64 / cap as f64) * 100.0
        } else {
            0.0
        }
    });

    // Determine color zone class based on projected fill percentage
    let color_zone_class = create_memo(move |_| {
        let pct = fill_percentage.get();
        if pct <= 50.0 {
            "zone-green"
        } else if pct <= 80.0 {
            "zone-yellow"
        } else {
            "zone-red"
        }
    });

    // Determine if projected cargo is empty (0%)
    let is_empty = create_memo(move |_| projected_cargo.get() == 0);

    // Determine if projected cargo is full (100%)
    let is_full = create_memo(move |_| {
        let projected = projected_cargo.get();
        let cap = capacity_clone2();
        projected >= cap && cap > 0
    });

    // Get inline style for the fill width (animated)
    let fill_style = create_memo(move |_| {
        format!("width: {}%", fill_percentage.get())
    });

    // Format the cargo display with projection
    let cargo_display = create_memo(move |_| {
        let current = current_used();
        let projected = projected_cargo.get();
        let cap = capacity_clone3();
        let change = cargo_change.get();
        
        if change == 0 {
            // No change - just show current cargo
            format!("{}/{} units", current, cap)
        } else {
            // Show change with arrow notation
            format!("{}/{} → {}/{} units", current, cap, projected, cap)
        }
    });

    // Determine if there's a cargo warning
    let cargo_warning = create_memo(move |_| {
        let projected = projected_cargo.get();
        let cap = capacity_clone4();
        let change = cargo_change.get();
        
        if change == 0 {
            None
        } else if projected < 0 {
            Some("Cannot sell more than you have!")
        } else if projected > cap {
            Some("Exceeds cargo capacity!")
        } else {
            None
        }
    });

    view! {
        <div class="cargo-status-container">
            <div class="cargo-progress-bar">
                <div
                    class="cargo-progress-fill"
                    class:is-empty=move || is_empty.get()
                    class:is-full=move || is_full.get()
                    class:zone-green=move || color_zone_class.get() == "zone-green"
                    class:zone-yellow=move || color_zone_class.get() == "zone-yellow"
                    class:zone-red=move || color_zone_class.get() == "zone-red"
                    style=fill_style
                >
                    <span class="cargo-numeric-overlay">
                        {move || cargo_display.get()}
                    </span>
                </div>
            </div>
            <div class="cargo-state-labels">
                <span
                    class="cargo-label cargo-label-empty"
                    class:hidden=move || !is_empty.get()
                >
                    "Cargo Empty"
                </span>
                <span
                    class="cargo-label cargo-label-full"
                    class:hidden=move || !is_full.get()
                >
                    "CARGO FULL"
                </span>
                // Show warning if applicable
                {move || {
                    cargo_warning.get().map(|warning| {
                        view! {
                            <span class="cargo-label cargo-label-warning">
                                {warning}
                            </span>
                        }
                    })
                }}
            </div>
        </div>
    }
}

/// Helper function to calculate fill percentage
/// Used for testing and external calculations
pub fn calculate_fill_percentage(current_used: u32, capacity: u32) -> f64 {
    if capacity > 0 {
        (current_used as f64 / capacity as f64) * 100.0
    } else {
        0.0
    }
}

/// Helper function to determine color zone based on fill percentage
/// Returns: "green" for 0-50%, "yellow" for 51-80%, "red" for 81-100%
pub fn get_color_zone(fill_percentage: f64) -> &'static str {
    if fill_percentage <= 50.0 {
        "green"
    } else if fill_percentage <= 80.0 {
        "yellow"
    } else {
        "red"
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_calculate_fill_percentage_normal() {
        assert_eq!(calculate_fill_percentage(25, 50), 50.0);
        assert_eq!(calculate_fill_percentage(0, 50), 0.0);
        assert_eq!(calculate_fill_percentage(50, 50), 100.0);
        assert_eq!(calculate_fill_percentage(35, 50), 70.0);
        assert_eq!(calculate_fill_percentage(45, 50), 90.0);
    }

    #[test]
    fn test_calculate_fill_percentage_zero_capacity() {
        assert_eq!(calculate_fill_percentage(0, 0), 0.0);
        assert_eq!(calculate_fill_percentage(5, 0), 0.0);
    }

    #[test]
    fn test_get_color_zone_green() {
        assert_eq!(get_color_zone(0.0), "green");
        assert_eq!(get_color_zone(25.0), "green");
        assert_eq!(get_color_zone(50.0), "green");
    }

    #[test]
    fn test_get_color_zone_yellow() {
        assert_eq!(get_color_zone(51.0), "yellow");
        assert_eq!(get_color_zone(65.0), "yellow");
        assert_eq!(get_color_zone(80.0), "yellow");
    }

    #[test]
    fn test_get_color_zone_red() {
        assert_eq!(get_color_zone(81.0), "red");
        assert_eq!(get_color_zone(90.0), "red");
        assert_eq!(get_color_zone(100.0), "red");
    }

    #[test]
    fn test_boundary_conditions() {
        // Test exact boundaries
        assert_eq!(get_color_zone(50.0), "green");
        assert_eq!(get_color_zone(50.1), "yellow");
        assert_eq!(get_color_zone(80.0), "yellow");
        assert_eq!(get_color_zone(80.1), "red");
    }
}
