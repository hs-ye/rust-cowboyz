## Technical Architecture Review - Epic #92

### Review Status: ✅ APPROVED with Recommendations

I've reviewed the movement mechanics system design (ADR #0002) and the associated subtasks. Overall, this is a well-architected approach that appropriately balances gameplay needs with implementation simplicity.

---

### 1. Data Structure Design Review ✅

**Planet, Ship, TravelState structures** (#94) are appropriately designed:

| Aspect | Assessment | Notes |
|--------|------------|-------|
| Integer-only approach | ✅ Good | Aligns with discrete turn-based design |
| PlanetId usage | ⚠️ Consider | Recommend `PlanetId = usize` instead of String references for serialization |
| TravelState enum | ✅ Good | Clean state representation |
| Derive macros | ✅ Good | Serialize/Deserialize essential for browser storage |

**Recommendations for #94:**
```rust
// Consider adding for richer gameplay
pub struct Ship {
    pub current_location: PlanetId,
    pub acceleration: u32,        // default: 1
    pub fuel_capacity: u32,
    pub current_fuel: u32,
    pub fuel_efficiency: u32,     // NEW: fuel per turn factor
}

pub struct TravelState {
    pub departure_turn: u32,      // NEW: for progress tracking
    pub arrival_turn: u32,
    pub destination: PlanetId,
}
```

---

### 2. Orbital Mechanics Algorithm Review ✅

**Algorithm implementation** (#96) follows ADR specifications correctly:

| Algorithm | Formula | Assessment |
|-----------|---------|------------|
| Position advancement | `(pos + 1) % period` | ✅ Efficient O(n) |
| Travel time | `2 * sqrt(distance / accel)` | ✅ Correct Brachistochrone |

**Technical Concerns:**
1. **Floating-point precision**: The formula uses `f64` sqrt then casts to `u32`. Suggest using `ceil()` to ensure travel isn't underestimated:
   ```rust
   let travel_turns = (2.0 * (base_distance as f64 / acceleration as f64).sqrt()).ceil() as u32;
   ```

2. **Same-planet travel**: Edge case not explicitly defined. Recommend returning `0` turns or rejecting at validation layer.

3. **Zero acceleration**: Should panic or return error - document this precondition.

---

### 3. Game State Integration Review ✅

**State management approach** (#95) is sound. Recommendations:
- `next_turn()` should return `Option<ArrivalEvent>` to notify UI of arrivals
- `initiate_travel()` should validate fuel BEFORE setting state (atomic operation)
- Consider adding `can_travel_to(destination)` query method for UI pre-validation

---

### 4. Testing Strategy Review ✅

**Test coverage** (#97) is comprehensive. Additional test cases to consider:

| Test Case | Priority | Description |
|-----------|----------|-------------|
| Overflow protection | Medium | `position + 1` won't overflow u32 |
| Large distance calc | Medium | Verify f64->u32 conversion at max values |
| Fuel exhaustion | High | Travel fails if insufficient fuel |
| Concurrent turns | Medium | Multiple `next_turn()` calls while in transit |

**Suggested test data:**
- distance: 4, accel: 1 → expected: 4 (2*sqrt(4) = 4)
- distance: 7, accel: 1 → expected: 6 (2*sqrt(7) ≈ 5.29 → ceil = 6)
- distance: 16, accel: 4 → expected: 4 (2*sqrt(4) = 4)

---

### 5. Architectural Decisions to Document

The following should be added to ADR #0002 or documented as implementation notes:

1. **Rounding behavior**: Travel time uses ceiling to avoid underestimation
2. **Fuel formula**: Define `fuel_cost = travel_turns * fuel_efficiency` (or similar)
3. **Error handling**: Panic vs Result for invalid inputs (zero acceleration, same planet)
4. **Serialization format**: JSON vs binary for browser localStorage

---

### 6. Implementation Order Validation ✅

The proposed development order is correct:
1. #94 (Data structures) - Foundation ✅
2. #96 + #95 (Algorithms + State) - Can parallelize ✅
3. #97 (Tests) - Final validation ✅

---

### Summary

| Component | Status | Notes |
|-----------|--------|-------|
| Overall Design | ✅ Approved | Clean, appropriate for scope |
| Data Structures | ✅ Approved | Minor suggestions above |
| Algorithms | ✅ Approved | Add ceiling for rounding |
| State Integration | ✅ Approved | Consider event notifications |
| Testing | ✅ Approved | Comprehensive coverage |

**No blocking issues identified.** The design is ready for implementation.

---

*Reviewed by: software-architect*  
*Date: 2026-03-08*  
*Reference: ADR #0002*
