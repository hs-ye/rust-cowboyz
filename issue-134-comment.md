## 📋 Assignment: design-specialist

**Priority**: High - Part of Epic #131 (Cargo Hold Management)

### 📖 Implementation Plan Reference
- **ADR #0008**: Cargo Hold Management - Cargo UI Display section
- **ADR #0003**: Web UI View - Information Panel layout

### ✅ Acceptance Criteria
- [ ] CargoPanel component created
- [ ] Progress bar displays correct fill percentage
- [ ] Color zones change based on fill level
- [ ] Numeric overlay shows used/total
- [ ] State labels appear at 0% and 100%
- [ ] Component is reactive to state changes
- [ ] Fits in ship status section layout
- [ ] All existing UI tests pass

### 🔗 Dependencies
- **Blocks**: #137 (UI to Backend Connection)
- **Blocked by**: None (can work in parallel with #132)

### ⚙️ Technical Considerations
- Create component: src/ui/cargo_panel.rs
- Use Leptos signals for reactive updates
- Cargo capacity default: 20 units (MVP)
- Component should be reusable for future ship upgrades
- Consider accessibility (aria-labels for progress)

### 🎨 Design Specifications
- **Progress bar color zones**:
  - Green (bg-green-500): 0-50% full
  - Yellow (bg-yellow-500): 51-80% full
  - Red (bg-red-500): 81-100% full
- **Numeric overlay**: "35/50 units" format
- **State labels**:
  - "Cargo Empty" (grey) when 0%
  - "CARGO FULL" warning (red, bold) when 100%

### 📁 Files to Create/Modify
- **Create**: src/ui/cargo_panel.rs
- **Integrate with**: Ship status panel component
- **Connect to**: Player.cargo from game state

---
**Status**: Ready to start. Please acknowledge and begin implementation.
