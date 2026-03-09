//! Solar System Map Component
//!
//! Renders the solar system using HTML5 Canvas with orbital positions.
//! Planets are positioned based on current turn orbital calculations.

#[cfg(feature = "web")]
use crate::simulation::orbits::Position;
#[cfg(feature = "web")]
use crate::simulation::planet_types::PlanetType;
#[cfg(feature = "web")]
use leptos::html::Canvas;
#[cfg(feature = "web")]
use leptos::RwSignal;
#[cfg(feature = "web")]
use leptos::Effect;
#[cfg(feature = "web")]
use leptos::SignalGet;
#[cfg(feature = "web")]
use leptos::SignalSet;
#[cfg(feature = "web")]
use leptos::NodeRef;
#[cfg(feature = "web")]
use leptos::view;
#[cfg(feature = "web")]
use leptos::IntoView;
#[cfg(feature = "web")]
use leptos::component;
#[cfg(feature = "web")]
use wasm_bindgen::{JsValue, JsCast};
#[cfg(feature = "web")]
use web_sys::CanvasRenderingContext2d;

/// Represents a planet with its display properties for the map
#[derive(Clone, Debug)]
pub struct MapPlanet {
    pub id: String,
    pub name: String,
    pub orbit_radius: u32,
    pub orbit_period: u32,
    #[cfg(feature = "web")]
    pub position: Position,
    #[cfg(feature = "web")]
    pub planet_type: PlanetType,
}

/// Color mapping for planet types
#[cfg(feature = "web")]
pub fn get_planet_color(planet_type: &PlanetType) -> &'static str {
    match planet_type {
        PlanetType::Agricultural => "#4CAF50",    // Green
        PlanetType::MegaCity => "#9C27B0",        // Purple
        PlanetType::Mining => "#FF9800",          // Orange
        PlanetType::PirateSpaceStation => "#F44336", // Red
        PlanetType::ResearchOutpost => "#2196F3", // Blue
        PlanetType::Industrial => "#607D8B",      // Grey
        PlanetType::FrontierColony => "#795548",  // Brown
    }
}

/// Size mapping for planet types (radius in pixels)
#[cfg(feature = "web")]
pub fn get_planet_size(planet_type: &PlanetType) -> f64 {
    match planet_type {
        PlanetType::Agricultural => 12.0,
        PlanetType::MegaCity => 14.0,
        PlanetType::Mining => 10.0,
        PlanetType::PirateSpaceStation => 8.0,
        PlanetType::ResearchOutpost => 9.0,
        PlanetType::Industrial => 11.0,
        PlanetType::FrontierColony => 10.0,
    }
}

/// Calculate pixel position from orbital position
/// Returns (x, y) coordinates relative to canvas center
#[cfg(feature = "web")]
fn calculate_orbital_position(
    orbit_radius: u32,
    orbital_position: u32,
    orbit_period: u32,
    scale: f64,
    center_x: f64,
    center_y: f64,
) -> (f64, f64) {
    if orbit_period == 0 {
        return (center_x, center_y);
    }

    // Calculate angle based on orbital position (0 to 2π)
    let angle = (orbital_position as f64 / orbit_period as f64) * 2.0 * std::f64::consts::PI;

    // Calculate position with offset of -π/2 to start at top
    let x = center_x + (orbit_radius as f64 * scale) * angle.cos();
    let y = center_y + (orbit_radius as f64 * scale) * (angle + std::f64::consts::FRAC_PI_2).sin();

    (x, y)
}

/// SolarMap component using HTML5 Canvas
#[cfg(feature = "web")]
#[component]
pub fn SolarMap(
    planets: Vec<MapPlanet>,
    current_turn: u32,
    player_location: String,
    selected_planet: Option<String>,
    on_planet_select: Option<Box<dyn Fn(String)>>,
) -> impl IntoView {
    // Canvas element reference using Leptos html module
    let canvas_ref: NodeRef<Canvas> = NodeRef::new();

    // Track hover state for visual feedback using RwSignal
    let hovered_planet = RwSignal::new(Option::<String>::None);

    // Calculate scale based on canvas size and max orbit radius
    let calculate_scale = |width: f64, height: f64, max_orbit: u32| -> f64 {
        if max_orbit == 0 {
            return 15.0;
        }
        let min_dimension = width.min(height) / 2.0;
        // Leave some padding (80%)
        (min_dimension * 0.8) / (max_orbit as f64 + 20.0)
    };

    // Get max orbit radius from planets
    let max_orbit = planets.iter()
        .map(|p| p.orbit_radius)
        .max()
        .unwrap_or(0);

    // Clone for use in closures
    let planets_for_render = planets.clone();
    let planets_for_click = planets.clone();
    let planets_for_hover = planets.clone();
    let selected_planet_for_render = selected_planet.clone();
    let player_location_for_render = player_location.clone();

    // Render the canvas
    let render_canvas = move || {
        let canvas = match canvas_ref.get() {
            Some(c) => c,
            None => return,
        };

        let context = match canvas.get_context("2d") {
            Ok(Some(ctx)) => ctx,
            _ => return,
        };
        let ctx: CanvasRenderingContext2d = context.unchecked_into();

        let width = canvas.width() as f64;
        let height = canvas.height() as f64;
        let center_x = width / 2.0;
        let center_y = height / 2.0;
        let scale = calculate_scale(width, height, max_orbit);

        // Clear canvas with space background
        ctx.set_fill_style(&JsValue::from_str("#0a0a1a"));
        ctx.fill_rect(0.0, 0.0, width, height);

        // Draw star field background
        draw_star_field(&ctx, width, height);

        // Draw orbital rings
        ctx.set_stroke_style(&JsValue::from_str("#1a1a3a"));
        ctx.set_line_width(1.0);

        let unique_orbits: Vec<u32> = planets_for_render.iter()
            .map(|p| p.orbit_radius)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        for orbit_radius in unique_orbits {
            let radius = orbit_radius as f64 * scale;
            ctx.begin_path();
            ctx.arc(center_x, center_y, radius, 0.0, 2.0 * std::f64::consts::PI).unwrap();
            ctx.stroke();
        }

        // Draw the sun at center
        ctx.set_fill_style(&JsValue::from_str("#FFD700"));
        ctx.begin_path();
        ctx.arc(center_x, center_y, 20.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
        ctx.fill();

        // Draw sun glow
        let gradient = ctx.create_radial_gradient(center_x, center_y, 10.0, center_x, center_y, 40.0);
        if let Ok(gradient) = gradient {
            let gradient: web_sys::CanvasGradient = gradient.unchecked_into();
            let _ = gradient.add_color_stop(0.0, "rgba(255, 215, 0, 0.8)");
            let _ = gradient.add_color_stop(0.5, "rgba(255, 165, 0, 0.3)");
            let _ = gradient.add_color_stop(1.0, "rgba(255, 100, 0, 0.0)");
            ctx.set_fill_style(&gradient.unchecked_into());
            ctx.begin_path();
            ctx.arc(center_x, center_y, 40.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
            ctx.fill();
        }

        // Draw planets
        for planet in &planets_for_render {
            // Calculate position at current turn
            let position = crate::simulation::orbits::calculate_orbit_position(
                planet.orbit_period,
                current_turn,
            );

            let (x, y) = calculate_orbital_position(
                planet.orbit_radius,
                position.orbital_position,
                planet.orbit_period,
                scale,
                center_x,
                center_y,
            );

            let color = get_planet_color(&planet.planet_type);
            let size = get_planet_size(&planet.planet_type);
            let is_selected = selected_planet_for_render.as_ref() == Some(&planet.id);
            let is_hovered = hovered_planet.get().as_ref() == Some(&planet.id);
            let is_player_location = player_location_for_render == planet.id;

            // Draw selection ring
            if is_selected {
                ctx.set_stroke_style(&JsValue::from_str("#FFFFFF"));
                ctx.set_line_width(3.0);
                ctx.begin_path();
                ctx.arc(x, y, size + 6.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
                ctx.stroke();
            }

            // Draw hover ring
            if is_hovered && !is_selected {
                ctx.set_stroke_style(&JsValue::from_str("rgba(255, 255, 255, 0.5)"));
                ctx.set_line_width(2.0);
                ctx.begin_path();
                ctx.arc(x, y, size + 4.0, 0.0, 2.0 * std::f64::consts::PI).unwrap();
                ctx.stroke();
            }

            // Draw planet body
            ctx.set_fill_style(&JsValue::from_str(color));
            ctx.begin_path();
            ctx.arc(x, y, size, 0.0, 2.0 * std::f64::consts::PI).unwrap();
            ctx.fill();

            // Draw player indicator if this is player's location
            if is_player_location {
                // Draw a triangle indicator
                ctx.set_fill_style(&JsValue::from_str("#00FF00"));
                ctx.begin_path();
                ctx.move_to(x, y - size - 10.0);
                ctx.line_to(x - 6.0, y - size - 20.0);
                ctx.line_to(x + 6.0, y - size - 20.0);
                ctx.close_path();
                ctx.fill();
            }

            // Draw planet name
            ctx.set_fill_style(&JsValue::from_str("#CCCCCC"));
            ctx.set_font("12px Arial");
            ctx.set_text_align("center");
            let _ = ctx.fill_text(&planet.name, x, y + size + 16.0);
        }
    };

    // Effect to re-render when props change
    let render_canvas_for_effect = render_canvas.clone();
    Effect::new(move |_| {
        // Track dependencies to trigger re-render
        let _ = current_turn;
        let _ = selected_planet.clone();
        let _ = player_location.clone();

        // Re-render canvas
        render_canvas_for_effect();
    });

    // Handle canvas click
    let on_canvas_click = move |event: web_sys::MouseEvent| {
        let canvas = match canvas_ref.get() {
            Some(c) => c,
            None => return,
        };

        let rect = canvas.get_bounding_client_rect();
        let x = event.client_x() as f64 - rect.left();
        let y = event.client_y() as f64 - rect.top();

        let width = canvas.width() as f64;
        let height = canvas.height() as f64;
        let center_x = width / 2.0;
        let center_y = height / 2.0;
        let scale = calculate_scale(width, height, max_orbit);

        // Check if click is on any planet
        for planet in &planets_for_click {
            let position = crate::simulation::orbits::calculate_orbit_position(
                planet.orbit_period,
                current_turn,
            );

            let (px, py) = calculate_orbital_position(
                planet.orbit_radius,
                position.orbital_position,
                planet.orbit_period,
                scale,
                center_x,
                center_y,
            );

            let size = get_planet_size(&planet.planet_type);
            let click_radius = size + 10.0; // Add some padding for easier clicking

            let dx = x - px;
            let dy = y - py;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance <= click_radius {
                // Call the selection callback if provided
                if let Some(ref callback) = on_planet_select {
                    callback(planet.id.clone());
                }
                return;
            }
        }
    };

    // Handle mouse move for hover effects
    let on_canvas_mousemove = move |event: web_sys::MouseEvent| {
        let canvas = match canvas_ref.get() {
            Some(c) => c,
            None => return,
        };

        let rect = canvas.get_bounding_client_rect();
        let x = event.client_x() as f64 - rect.left();
        let y = event.client_y() as f64 - rect.top();

        let width = canvas.width() as f64;
        let height = canvas.height() as f64;
        let center_x = width / 2.0;
        let center_y = height / 2.0;
        let scale = calculate_scale(width, height, max_orbit);

        // Check if mouse is over any planet
        let mut found_planet: Option<String> = None;

        for planet in &planets_for_hover {
            let position = crate::simulation::orbits::calculate_orbit_position(
                planet.orbit_period,
                current_turn,
            );

            let (px, py) = calculate_orbital_position(
                planet.orbit_radius,
                position.orbital_position,
                planet.orbit_period,
                scale,
                center_x,
                center_y,
            );

            let size = get_planet_size(&planet.planet_type);
            let hover_radius = size + 10.0;

            let dx = x - px;
            let dy = y - py;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance <= hover_radius {
                found_planet = Some(planet.id.clone());
                break;
            }
        }

        hovered_planet.set(found_planet);
    };

    // Handle mouse leave
    let on_canvas_mouseleave = move |_| {
        hovered_planet.set(None);
    };

    // Set up resize observer effect
    Effect::new(move |_| {
        let canvas = match canvas_ref.get() {
            Some(c) => c,
            None => return,
        };

        // Set initial canvas size based on container
        let parent = canvas.parent_element();
        if let Some(parent) = parent {
            let rect = parent.get_bounding_client_rect();
            let width = rect.width().max(400.0);
            let height = rect.height().max(300.0);

            canvas.set_width(width as u32);
            canvas.set_height(height as u32);

            // Initial render
            render_canvas();
        }
    });

    view! {
        <div class="solar-map-container">
            <canvas
                ref=canvas_ref
                class="solar-map-canvas"
                on:click=on_canvas_click
                on:mousemove=on_canvas_mousemove
                on:mouseleave=on_canvas_mouseleave
            />
            <div class="map-legend">
                <span class="legend-title">"Planets:"</span>
                <div class="legend-item">
                    <span class="legend-color" style="background: #4CAF50"></span>
                    <span>"农业 Agricultural"</span>
                </div>
                <div class="legend-item">
                    <span class="legend-color" style="background: #FF9800"></span>
                    <span>"采矿 Mining"</span>
                </div>
                <div class="legend-item">
                    <span class="legend-color" style="background: #607D8B"></span>
                    <span>"工业 Industrial"</span>
                </div>
                <div class="legend-item">
                    <span class="legend-color" style="background: #9C27B0"></span>
                    <span>"都市 Mega City"</span>
                </div>
                <div class="legend-item">
                    <span class="legend-color" style="background: #2196F3"></span>
                    <span>"科研 Research"</span>
                </div>
                <div class="legend-item">
                    <span class="legend-color" style="background: #F44336"></span>
                    <span>"海盗 Pirate"</span>
                </div>
                <div class="legend-item">
                    <span class="legend-color" style="background: #795548"></span>
                    <span>"前沿 Frontier"</span>
                </div>
                <div class="legend-item player-indicator">
                    <span class="legend-marker">"▲"</span>
                    <span>"玩家位置 Player Location"</span>
                </div>
            </div>
        </div>
    }
}

/// Draw a simple star field background
#[cfg(feature = "web")]
fn draw_star_field(ctx: &CanvasRenderingContext2d, width: f64, height: f64) {
    // Use a seeded random for consistent star positions
    // For simplicity, we'll use a fixed pattern based on coordinates
    let star_count = ((width * height) / 2000.0) as i32;

    for i in 0..star_count {
        let x = ((i * 7919 % 1000) as f64 / 1000.0) * width;
        let y = ((i * 6271 % 1000) as f64 / 1000.0) * height;
        let size = if i % 3 == 0 { 1.5 } else { 1.0 };
        let brightness = 0.3 + (i % 7) as f64 / 10.0;

        ctx.set_fill_style(&JsValue::from_str(&format!("rgba(255, 255, 255, {:.1})", brightness)));
        ctx.begin_path();
        ctx.arc(x, y, size, 0.0, 2.0 * std::f64::consts::PI).unwrap();
        ctx.fill();
    }
}

#[cfg(all(test, feature = "web"))]
mod tests {
    use super::*;

    #[test]
    fn test_planet_colors() {
        assert_eq!(get_planet_color(&PlanetType::Agricultural), "#4CAF50");
        assert_eq!(get_planet_color(&PlanetType::Mining), "#FF9800");
        assert_eq!(get_planet_color(&PlanetType::Industrial), "#607D8B");
    }

    #[test]
    fn test_planet_sizes() {
        assert!(get_planet_size(&PlanetType::MegaCity) > get_planet_size(&PlanetType::Mining));
    }

    #[test]
    fn test_orbital_position_calculation() {
        // Test that position wraps correctly
        // At position 0 with orbit_radius=10, scale=10, center=(100,100):
        // angle = 0, so x = 100 + 100 * cos(0) = 200, y = 100 + 100 * sin(π/2) = 200
        let (x, y) = calculate_orbital_position(10, 0, 10, 10.0, 100.0, 100.0);
        assert!((x - 200.0).abs() < 0.01);
        assert!((y - 200.0).abs() < 0.01);
    }
}