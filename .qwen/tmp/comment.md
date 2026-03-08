## Fix Applied ✅

**Root Cause Analysis:**
The web UI was showing a blank screen because of three issues:

1. **Missing `leptos/csr` feature**: Leptos 0.6 requires the `csr` (Client-Side Rendering) feature to be explicitly enabled. The original code only had `dep:leptos` without the feature flag.

2. **WASM entry point not being called**: In WASM targets, `main()` doesn't auto-run like native binaries. The `#[wasm_bindgen(start)]` attribute was needed on the entry function.

3. **Library vs Binary approach**: Using a binary target (`[[bin]]`) for WASM caused confusion. The library approach (`crate-type = ["cdylib"]`) with `#[wasm_bindgen(start)]` is the correct pattern for Leptos WASM apps.

**Changes Made:**
- `Cargo.toml`: Added `leptos/csr` to web feature
- `src/lib.rs`: Added `start()` function with `#[wasm_bindgen(start)]` attribute
- `src/ui/web.rs`: Created new module with App component
- `index.html`: Updated to use `data-target-name` for library build
- `src/main_web.rs`: Cleaned up (kept for reference but not used)

**Verification:**
- Build succeeds with `trunk build`
- Web UI displays correctly at http://localhost:8080
- All components render: header, solar map, player/ship panels, inventory, market

**Commit:** `2e3e60a` on branch `fix/89-web-ui-blank-screen`

---

**Lessons Learned:**
- Always check browser console first for WASM errors
- Leptos 0.6+ specifically needs `csr` feature (not `web` or `hydrate`)
- `#[wasm_bindgen(start)]` is required for WASM auto-execution
- Created debugging skill `.qwen/skills/rust-wasm-frontend-debug/` for future issues
