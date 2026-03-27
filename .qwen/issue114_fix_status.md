## Fix Implemented

I've implemented the fix for this critical bug. Here's what was done:

### Changes Made

**1. Added missing CSS styles to `index.html`** (lines 949-1004)
- `.solar-map-container`: Flex column layout with constrained dimensions
- `.solar-map-canvas`: `min-height: 0` and `display: block` to prevent expansion
- `.map-legend`: Constrained height with `flex-shrink: 0` and `max-height: 80px`

**2. Optimized mouse move handler in `src/ui/solar_map.rs`** (line 377-379)
- Added check to only update `hovered_planet` signal when value actually changes
- Prevents unnecessary reactive re-renders on every mouse move event

### Key Fix Details

The root cause was a **resize feedback loop**:
1. Canvas was setting its internal dimensions based on parent size
2. Without CSS constraints, canvas display size affected parent container
3. Parent growth triggered resize effect, which expanded canvas further
4. Mouse movement caused re-renders that exacerbated the issue

The fix adds:
- `min-height: 0` on canvas (critical for flex children)
- `flex-shrink: 0` on legend to prevent it from growing
- `overflow: hidden` on container to clip any expansion
- Signal change detection to reduce re-render frequency

### Verification
- Code compiles successfully with `cargo check --features web`
- No breaking changes to existing functionality

### Next Steps
- Build and deploy to test environment
- Verify planets are now clickable and map stays stable during mouse movement
