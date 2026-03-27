## Bug Report: Planets Not Visible on Solar System Map

### Problem Summary
The solar system map displays a placeholder instead of the actual interactive planet map. Players cannot see or click on planets to select travel destinations.

### Root Cause Analysis

After investigation, I've identified the root causes:

1. **SolarMap Component Not Integrated**: The `SolarMap` component exists in `src/ui/solar_map.rs` (fully implemented with HTML5 Canvas rendering, orbital mechanics, click handling, and hover effects), but it is **NOT imported or used** in `src/ui/web.rs`.

2. **Placeholder HTML in web.rs**: The current `web.rs` has a placeholder div instead of the SolarMap component:
   ```rust
   <div class="map-placeholder">
       <div class="sun"></div>
       <p>"Solar system map will be displayed here"</p>
   </div>
   ```

3. **PR #113 Has Merge Conflicts**: PR #113 (which attempts to integrate SolarMap) has merge conflicts and cannot be merged. It also tries to modify `src/main_web.rs` which was deleted in commit 445a99d.

4. **Epic #93 Prematurely Marked Complete**: The epic was marked complete but the SolarMap integration was never actually finished.

### SolarMap Component Status
The SolarMap component (`src/ui/solar_map.rs`) is **fully implemented** with:
- ✅ HTML5 Canvas rendering
- ✅ Orbital position calculations based on current turn
- ✅ Planet rendering with color-coding by type
- ✅ Sun with glow effect at center
- ✅ Star field background
- ✅ Click handling for planet selection
- ✅ Hover effects for visual feedback
- ✅ Player location indicator (green triangle)
- ✅ Selection ring for selected planet
- ✅ Map legend showing planet types

### Recommended Fix Approach: Option B (New Integration)

**Decision**: Create a fresh integration PR rather than fixing PR #113.

**Rationale**:
- PR #113 has merge conflicts and modifies a deleted file (`main_web.rs`)
- The integration should be in `web.rs`, not `main_web.rs`
- Cleaner to start fresh with a focused integration task
- SolarMap component is already fully implemented - just needs to be wired up

### Implementation Plan

See implementation ticket #118 for detailed steps.

### Acceptance Criteria
- [ ] SolarMap component renders in the map panel
- [ ] Planets are displayed at their correct orbital positions
- [ ] Planets are clickable for selection
- [ ] Selected planet is visually highlighted
- [ ] Player location is indicated on the map
- [ ] Map updates when turn advances

### Related
- Epic: #93 
- ADR: 0003-web-ui-view.md
- Component: `src/ui/solar_map.rs`
- Current UI: `src/ui/web.rs`
