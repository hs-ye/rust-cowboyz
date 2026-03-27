## Root Cause Analysis

After investigating the code, I've identified the root cause of the vertical expansion bug:

### Problem
The solar map component is missing CSS styles for `.solar-map-container`, `.solar-map-canvas`, and `.map-legend` classes. Without these styles:

1. **Canvas resize feedback loop**: The canvas sets its internal dimensions (via `canvas.set_width()`/`set_height()`) based on the parent's bounding rect. Without CSS constraints, this creates a feedback loop where:
   - Canvas grows → Parent container expands to fit → Resize effect triggers → Canvas grows more

2. **Legend layout shifts**: The legend element has no styling, causing it to expand/contract based on content, which affects the parent container size

3. **Mouse move triggers re-render**: The `on_canvas_mousemove` handler updates `hovered_planet` signal on EVERY mouse move, causing reactive re-renders that can trigger the resize effect

### Specific Code Issues

**File: `src/ui/solar_map.rs` (lines 385-403)**
```rust
Effect::new(move |_| {
    let canvas = match canvas_ref.get() {
        Some(c) => c,
        None => return,
    };
    let parent = canvas.parent_element();
    if let Some(parent) = parent {
        let rect = parent.get_bounding_client_rect();
        let width = rect.width().max(400.0);
        let height = rect.height().max(300.0);
        canvas.set_width(width as u32);  // Sets drawing buffer size
        canvas.set_height(height as u32);
        render_canvas();
    }
});
```

This effect runs when `canvas_ref` changes, but the canvas dimensions are being set from the parent's size without CSS constraints.

**Missing CSS in `index.html`:**
- No `.solar-map-container` styles
- No `.solar-map-canvas` styles  
- No `.map-legend` styles

### Proposed Fix

1. **Add CSS styles to `index.html`** to constrain the map container:
```css
.solar-map-container {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    overflow: hidden;
}

.solar-map-canvas {
    flex: 1;
    width: 100%;
    min-height: 0; /* Critical for flex child */
    cursor: crosshair;
}

.map-legend {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem 1rem;
    padding: 0.75rem;
    background: rgba(0, 0, 0, 0.3);
    border-top: 1px solid var(--border-color);
    font-size: 0.8rem;
    max-height: 80px;
    overflow-y: auto;
}

.legend-title {
    font-weight: 600;
    color: var(--accent-cyan);
    margin-right: 0.5rem;
}

.legend-item {
    display: flex;
    align-items: center;
    gap: 0.35rem;
}

.legend-color {
    width: 10px;
    height: 10px;
    border-radius: 2px;
}

.legend-marker {
    color: var(--accent-green);
}

.player-indicator {
    margin-left: auto;
}
```

2. **Optimize mouse move handler** in `solar_map.rs` to prevent unnecessary updates:
```rust
let on_canvas_mousemove = move |event: web_sys::MouseEvent| {
    // ... existing code ...
    
    // Only update if changed to prevent unnecessary re-renders
    let current = hovered_planet.get();
    if current != found_planet {
        hovered_planet.set(found_planet);
    }
};
```

### Files to Modify
- `index.html` - Add missing CSS styles (lines ~900+)
- `src/ui/solar_map.rs` - Optimize mouse move handler (line ~330)

### Estimated Effort
**Small (1-2 hours)** - CSS addition and minor optimization
