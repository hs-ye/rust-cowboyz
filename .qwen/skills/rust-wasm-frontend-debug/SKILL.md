---
name: rust-wasm-frontend-debug
description: Debug Rust WebAssembly frontend applications using Leptos, Trunk, and WASM. Use when the web UI shows a blank screen, WASM fails to load, or Leptos components don't render. Covers SRI hash errors, feature flag issues, and WASM initialization problems.
---

# Rust WASM Frontend Debugging

Systematic debugging guide for Rust + Leptos + WASM frontend issues.

## Quick Diagnostic (2 minutes)

### 1. Check Browser Console
Ask user: "What errors appear in the browser console (F12)?"

Common errors:
- `None of the "sha384" hashes match` → SRI hash mismatch, rebuild with `trunk build`
- `WASM panicked` → Rust panic, check for unwrap() failures
- `404` on `.wasm` or `.js` files → Build issue, check `dist/` folder

### 2. Verify HTTP Status
```bash
curl -s -o /dev/null -w "%{http_code}" http://localhost:8080/*.wasm
curl -s -o /dev/null -w "%{http_code}" http://localhost:8080/*.js
```
Both should return `200`.

### 3. Check SRI Hashes
```bash
curl -s http://localhost:8080/ | grep -o 'integrity="[^"]*"' | head -2
```
If hashes look wrong or HTML is stale, rebuild: `trunk build`

---

## Systematic Debugging (10 minutes)

### Phase 1: WASM Load Check

```bash
#!/bin/bash
# Save as debug-wasm.sh

echo "=== WASM File Check ==="
ls -la dist/*.wasm dist/*.js 2>/dev/null || echo "No WASM files in dist/"

echo ""
echo "=== HTTP Status ==="
curl -s -o /dev/null -w "WASM: %{http_code}\n" http://localhost:8080/dist/*.wasm
curl -s -o /dev/null -w "JS: %{http_code}\n" http://localhost:8080/dist/*.js

echo ""
echo "=== SRI Hashes in HTML ==="
curl -s http://localhost:8080/ | grep -o 'integrity="[^"]*"' | head -2

echo ""
echo "=== Leptos Features ==="
grep -A5 "\[features\]" Cargo.toml | grep -E "leptos|web"
```

### Phase 2: Feature Flag Verification

Leptos 0.6+ requires `csr` feature for client-side rendering:

```bash
# Check current features
cargo tree -e features | grep leptos

# Should show: leptos v0.6.x with feature "csr"
```

**Fix if missing:**
```toml
[features]
web = ["dep:leptos", "leptos/csr", "dep:leptos_meta", ...]
```

### Phase 3: WASM Entry Point Check

Verify the WASM exports a `start` function:

```bash
grep "start" dist/*.js | head -5
```

Should show exports. If missing, check `lib.rs` has:

```rust
#[wasm_bindgen(start)]
pub fn start() {
    leptos::mount_to_body(|| view! { <App/> });
}
```

### Phase 4: Manual Browser Test

Create `/tmp/test.html`:

```html
<script>
fetch('http://localhost:8080/*.wasm')
  .then(r => {
    console.log('WASM status:', r.status);
    return r.text();
  })
  .then(t => console.log('WASM size:', t.length));

// Check if start() exists
setTimeout(() => {
  console.log('wasmBindings:', window.wasmBindings);
  console.log('has start:', window.wasmBindings && window.wasmBindings.start);
}, 2000);
</script>
```

Open in browser and check console.

---

## Common Issues & Fixes

### Blank Screen

| Symptom | Cause | Fix |
|---------|-------|-----|
| No console errors | `mount_to_body` not called | Add `leptos/csr` feature |
| SRI hash error | Stale HTML | `trunk build` |
| WASM 404 | Wrong target | Check `data-target-name` in index.html |
| `start` not called | Missing `#[wasm_bindgen(start)]` | Add attribute to entry fn |

### SRI Hash Mismatch

```bash
# Force rebuild
trunk clean
trunk build
```

### Feature Flag Issues

```toml
# Wrong (Leptos 0.6+)
web = ["dep:leptos", "dep:leptos_meta", ...]

# Correct
web = ["dep:leptos", "leptos/csr", "dep:leptos_meta", ...]
```

### Multiple Binaries Confusion

If you have both `main.rs` (CLI) and `main_web.rs` (web):

```toml
# In Cargo.toml - library approach is better for WASM
[lib]
name = "cowboyz"
crate-type = ["cdylib", "rlib"]

# In index.html - use library target
<link data-trunk rel="rust" data-target-name="cowboyz" />
```

Then put entry point in `lib.rs`:

```rust
#[wasm_bindgen(start)]
pub fn start() {
    leptos::mount_to_body(App);
}
```

---

## Debugging Checklist

- [ ] WASM files exist in `dist/`
- [ ] HTTP 200 for WASM and JS files
- [ ] No SRI hash errors in console
- [ ] `leptos/csr` feature enabled
- [ ] `#[wasm_bindgen(start)]` on entry function
- [ ] `mount_to_body` closure actually runs (add `console_log!`)
- [ ] No Rust panics (check console for red errors)

---

## Advanced: Playwright Debugging

For automated browser testing:

```javascript
const { chromium } = require('playwright');

(async () => {
  const browser = await chromium.launch();
  const page = await browser.newPage();

  page.on('console', msg => console.log(`[${msg.type()}] ${msg.text()}`));
  page.on('pageerror', err => console.log(`[ERROR] ${err.message}`));

  await page.goto('http://localhost:8080');
  await page.waitForTimeout(3000);

  const html = await page.evaluate(() => document.body.innerHTML);
  console.log('Body:', html.substring(0, 500));

  await browser.close();
})();
```

---

## Key Insights

1. **Always start with browser console** - Most WASM issues show errors there
2. **SRI hashes are fragile** - Any HTML change requires `trunk build`
3. **Leptos needs `csr` feature** - Not `web`, not `hydrate`, specifically `csr`
4. **`#[wasm_bindgen(start)]` is required** - `main()` doesn't auto-run in WASM
5. **Library > Binary for WASM** - Use `crate-type = ["cdylib"]` not `[[bin]]`
