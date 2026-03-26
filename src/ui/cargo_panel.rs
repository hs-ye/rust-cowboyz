//! Cargo Panel Component
//!
//! Displays cargo hold status with a visual progress bar showing:
//! - Current used capacity vs total capacity
//! - Color-coded zones (green 0-50%, yellow 51-80%, red 81-100%)
//! - State labels ("Cargo Empty" at 0%, "CARGO FULL" at 100%)

#[cfg(feature = "web")]
use leptos::*;

/// CargoPanel Component
///
/// Displays a horizontal progress bar showing cargo hold status with:
/// - Color zones: Green (0-50%), Yellow (51-80%), Red (81-100%)
/// - Numeric overlay: "used/total units" format
/// - State labels: "Cargo Empty" (grey) at 0%, "CARGO FULL" (red, bold) at 100%
///
/// # Arguments
/// * `current_used` - Current cargo units used (can be a signal for reactivity)
/// * `capacity` - Total cargo capacity (can be a signal for reactivity)
#[cfg(feature = "web")]
#[component]
pub fn CargoPanel(
    current_used: u32,
    capacity: u32,
) -> impl IntoView {
    // Calculate fill percentage
    let fill_percentage = if capacity > 0 {
        (current_used as f64 / capacity as f64) * 100.0
    } else {
        0.0
    };

    // Determine color zone class based on fill percentage
    let color_zone_class = create_memo(move |_| {
        if fill_percentage <= 50.0 {
            "bg-green-500"
        } else if fill_percentage <= 80.0 {
            "bg-yellow-500"
        } else {
            "bg-red-500"
        }
    });

    // Determine if cargo is empty (0%)
    let is_empty = create_memo(move |_| current_used == 0);

    // Determine if cargo is full (100%)
    let is_full = create_memo(move |_| current_used >= capacity && capacity > 0);

    // Get inline style for the fill width
    let fill_style = create_memo(move |_| {
        format!("width: {}%", fill_percentage)
    });

    view! {
        <div class="cargo-status-container">
            <div class="cargo-progress-bar">
                <div
                    class="cargo-progress-fill"
                    class:is-empty=move || is_empty.get()
                    class:is-full=move || is_full.get()
                    class:zone-green=move || color_zone_class.get() == "bg-green-500"
                    class:zone-yellow=move || color_zone_class.get() == "bg-yellow-500"
                    class:zone-red=move || color_zone_class.get() == "bg-red-500"
                    style=fill_style
                >
                    <span class="cargo-numeric-overlay">
                        {move || format!("{}/{} units", current_used, capacity)}
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
