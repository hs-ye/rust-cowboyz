# Test Plan: Orbital Mechanics System

## Issue Reference
- **GitHub Issue**: #97 - Create Unit Tests for Orbital Mechanics
- **Parent Epic**: #92 - Orbital Mechanics Implementation
- **Related Issues**: #94, #95, #96
- **ADR Reference**: /design/adr/0002-movement-mechanics-system.md

## Objective
Verify the correctness and robustness of the orbital mechanics system, including planet position advancement, travel time calculations, fuel consumption, and travel state transitions.

## Test Environment
- **Framework**: Rust built-in test framework (`cargo test`)
- **Location**: `src/orbital_mechanics/` module tests
- **Dependencies**: Standard library only (no external test dependencies required)

---

## Test Suite 1: advance_planet_positions

### Test 1.1: Single Planet Advances Correctly
**Description**: Verify that a single planet's position advances by 1 each turn.

**Preconditions**:
- Planet with orbital_period = 10, position = 0

**Test Steps**:
1. Create a planet: `{ name: "TestPlanet", orbital_radius: 5, orbital_period: 10, position: 0 }`
2. Call `advance_planet_positions()` once
3. Verify position = 1
4. Call `advance_planet_positions()` 4 more times
5. Verify position = 5

**Expected Results**:
- After 1 turn: position = 1
- After 5 turns: position = 5

**Pass Criteria**: Position increments correctly without wrapping

---

### Test 1.2: Multiple Planets Advance Correctly
**Description**: Verify that multiple planets with different periods advance simultaneously.

**Preconditions**:
- Planet A: period = 10, position = 0
- Planet B: period = 15, position = 7
- Planet C: period = 20, position = 19

**Test Steps**:
1. Create vector with 3 planets at specified positions
2. Call `advance_planet_positions()` once
3. Verify all positions incremented by 1

**Expected Results**:
- Planet A: position = 1
- Planet B: position = 8
- Planet C: position = 0 (wrapped from 19)

**Pass Criteria**: All planets advance; Planet C wraps correctly

---

### Test 1.3: Position Wraps at orbital_period
**Description**: Verify that position wraps to 0 when reaching orbital_period.

**Preconditions**:
- Planet with orbital_period = 10, position = 9

**Test Steps**:
1. Create planet at position = 9
2. Call `advance_planet_positions()` once
3. Verify position = 0
4. Advance 10 more turns
5. Verify position returns to 0 again

**Expected Results**:
- After 1 turn from 9: position = 0
- After 10 turns from 0: position = 0

**Pass Criteria**: Wrap-around uses modulo arithmetic correctly

---

### Test 1.4: Different Orbital Periods Handled Correctly
**Description**: Verify planets with different periods wrap independently.

**Preconditions**:
- Planet A: period = 5, position = 4
- Planet B: period = 8, position = 7

**Test Steps**:
1. Create both planets at max positions
2. Call `advance_planet_positions()` once
3. Verify both wrapped to 0
4. Advance 4 more turns
5. Verify Planet A at position = 4 (about to wrap), Planet B at position = 4

**Expected Results**:
- After wrap: both at 0
- After 4 more turns: A=4, B=4
- After 1 more turn: A=0, B=5

**Pass Criteria**: Each planet wraps based on its own period

---

## Test Suite 2: calculate_travel_turns

### Test 2.1: Basic Calculation with Known Values
**Description**: Verify the travel time formula with the ADR example.

**Preconditions**:
- Planet A: radius = 5, period = 10, position = 0
- Planet B: radius = 12, period = 15, position = 7
- Ship acceleration = 1

**Test Steps**:
1. Calculate base_distance = |12 - 5| = 7
2. Calculate travel_turns = 2 * sqrt(7 / 1) ≈ 5.29
3. Verify result is rounded/truncated to 5

**Expected Results**:
- base_distance = 7
- travel_turns = 5 (or 6 depending on rounding strategy)

**Pass Criteria**: Formula produces expected result

---

### Test 2.2: Same Planet Returns 0 or Minimal Value
**Description**: Verify travel to the same planet requires no travel time.

**Preconditions**:
- Planet A: radius = 5
- Ship acceleration = 1

**Test Steps**:
1. Call calculate_travel_turns with same planet for departure and destination
2. Verify result is 0 or 1 (minimum travel time)

**Expected Results**:
- travel_turns = 0 (or 1 if minimum enforced)

**Pass Criteria**: No travel time for same planet

---

### Test 2.3: Different Accelerations Produce Correct Results
**Description**: Verify higher acceleration reduces travel time.

**Preconditions**:
- Planet A: radius = 5
- Planet B: radius = 20 (distance = 15)

**Test Steps**:
1. Calculate with acceleration = 1: turns = 2 * sqrt(15) ≈ 7.75 → 8
2. Calculate with acceleration = 2: turns = 2 * sqrt(7.5) ≈ 5.48 → 5
3. Calculate with acceleration = 4: turns = 2 * sqrt(3.75) ≈ 3.87 → 4

**Expected Results**:
- accel=1: ~8 turns
- accel=2: ~5-6 turns
- accel=4: ~4 turns

**Pass Criteria**: Higher acceleration = fewer turns

---

### Test 2.4: Verify Formula Implementation
**Description**: Verify exact formula: 2 * sqrt(distance / acceleration)

**Preconditions**:
- Various distance/acceleration combinations

**Test Steps**:
1. Test distance=4, accel=1: 2 * sqrt(4) = 4 turns
2. Test distance=9, accel=1: 2 * sqrt(9) = 6 turns
3. Test distance=16, accel=4: 2 * sqrt(4) = 4 turns
4. Test distance=25, accel=1: 2 * sqrt(25) = 10 turns

**Expected Results**:
- Perfect squares should yield exact integer results
- Formula matches mathematical expectation

**Pass Criteria**: Formula implemented correctly

---

## Test Suite 3: Fuel Consumption

### Test 3.1: Base Fuel Cost Proportional to travel_turns
**Description**: Verify fuel cost scales linearly with travel time.

**Preconditions**:
- Ship with base fuel consumption rate
- Various travel distances

**Test Steps**:
1. Calculate fuel for 5-turn journey
2. Calculate fuel for 10-turn journey
3. Verify 10-turn uses exactly 2x fuel of 5-turn

**Expected Results**:
- fuel(10 turns) = 2 * fuel(5 turns)
- Linear relationship maintained

**Pass Criteria**: Fuel consumption is linear with travel turns

---

### Test 3.2: Fuel Efficiency Factor Applied Correctly
**Description**: Verify fuel efficiency upgrades reduce consumption.

**Preconditions**:
- Base ship: efficiency = 1.0
- Upgraded ship: efficiency = 0.5 (50% reduction)
- Same travel distance for both

**Test Steps**:
1. Calculate fuel for base ship on 10-turn journey
2. Calculate fuel for upgraded ship on same journey
3. Verify upgraded ship uses 50% less fuel

**Expected Results**:
- fuel_upgraded = fuel_base * efficiency_factor

**Pass Criteria**: Efficiency multiplier applied correctly

---

### Test 3.3: Fuel Calculation with Zero Efficiency
**Description**: Verify edge case of zero efficiency (should not cause issues).

**Preconditions**:
- Ship with efficiency = 0 (theoretical edge case)

**Test Steps**:
1. Attempt fuel calculation with efficiency = 0
2. Verify graceful handling (minimum fuel or error)

**Expected Results**:
- Either minimum fuel cost applied or appropriate error

**Pass Criteria**: No panic or undefined behavior

---

## Test Suite 4: Travel State Transitions

### Test 4.1: Idle to InTransit Transition
**Description**: Verify ship can start travel from idle state.

**Preconditions**:
- Ship in Idle state at Planet A
- Sufficient fuel for journey to Planet B

**Test Steps**:
1. Verify initial state is Idle
2. Initiate travel to Planet B
3. Verify state changes to InTransit
4. Verify destination recorded
5. Verify travel_turns_remaining set correctly

**Expected Results**:
- State: Idle → InTransit
- Destination: Planet B
- travel_turns_remaining: calculated value

**Pass Criteria**: Transition successful with correct metadata

---

### Test 4.2: InTransit to Idle on Arrival
**Description**: Verify ship arrives and transitions back to Idle.

**Preconditions**:
- Ship in InTransit state
- travel_turns_remaining = 1

**Test Steps**:
1. Advance one turn
2. Verify state changes to Idle
3. Verify location updated to destination
4. Verify travel_turns_remaining = 0 or None

**Expected Results**:
- State: InTransit → Idle
- Location: destination planet
- No remaining travel turns

**Pass Criteria**: Arrival completes successfully

---

### Test 4.3: Invalid Travel Attempts Rejected
**Description**: Verify invalid travel requests are rejected.

**Preconditions**:
- Ship in various states

**Test Steps**:
1. Attempt travel while already InTransit (should fail)
2. Attempt travel to non-existent planet (should fail)
3. Attempt travel with insufficient fuel (should fail)
4. Attempt travel with zero/negative acceleration (should fail)

**Expected Results**:
- All invalid attempts return Err or false
- State remains unchanged on failure
- Appropriate error messages returned

**Pass Criteria**: Invalid operations rejected gracefully

---

### Test 4.4: Travel Progress During Transit
**Description**: Verify travel_turns_remaining decrements correctly.

**Preconditions**:
- Ship in InTransit state with travel_turns_remaining = 5

**Test Steps**:
1. Advance one turn
2. Verify travel_turns_remaining = 4
3. Advance 3 more turns
4. Verify travel_turns_remaining = 1
5. Advance final turn
6. Verify arrival (state = Idle)

**Expected Results**:
- Counter decrements by 1 each turn
- Arrival when counter reaches 0

**Pass Criteria**: Progress tracking accurate

---

## Test Suite 5: Edge Cases

### Test 5.1: Zero Acceleration Handling
**Description**: Verify behavior when acceleration is 0.

**Preconditions**:
- Ship with acceleration = 0
- Travel distance > 0

**Test Steps**:
1. Attempt to calculate travel turns with acceleration = 0
2. Verify graceful handling (division by zero prevention)

**Expected Results**:
- Returns error, maximum turns, or uses default acceleration
- No panic from division by zero

**Pass Criteria**: Safe handling of zero acceleration

---

### Test 5.2: Very Large Distances
**Description**: Verify system handles large orbital radii.

**Preconditions**:
- Planet A: radius = 1
- Planet B: radius = 1,000,000
- Ship acceleration = 1

**Test Steps**:
1. Calculate travel turns for distance = 999,999
2. Verify result = 2 * sqrt(999999) ≈ 1999 turns
3. Verify no integer overflow

**Expected Results**:
- Calculation completes without overflow
- Result is approximately 1999

**Pass Criteria**: Large distances handled safely

---

### Test 5.3: Planet at Maximum Position Wrapping
**Description**: Verify planets at position = period - 1 wrap correctly.

**Preconditions**:
- Planet with period = u32::MAX (or large value)
- Position = period - 1

**Test Steps**:
1. Advance position
2. Verify wrap to 0
3. Verify no overflow in modulo operation

**Expected Results**:
- Position wraps from max to 0
- No arithmetic overflow

**Pass Criteria**: Maximum values handled safely

---

### Test 5.4: Insufficient Fuel Scenarios
**Description**: Verify travel blocked when fuel is insufficient.

**Preconditions**:
- Ship with 10 fuel units
- Journey requires 50 fuel units

**Test Steps**:
1. Attempt to initiate travel
2. Verify request rejected
3. Verify fuel unchanged
4. Verify state remains Idle

**Expected Results**:
- Travel not initiated
- Error indicates insufficient fuel
- No state changes

**Pass Criteria**: Fuel check prevents travel

---

### Test 5.5: Exact Fuel Boundary
**Description**: Verify travel allowed with exactly enough fuel.

**Preconditions**:
- Ship with exactly 50 fuel units
- Journey requires exactly 50 fuel units

**Test Steps**:
1. Attempt to initiate travel
2. Verify request accepted
3. Verify fuel reduced to 0
4. Verify state changes to InTransit

**Expected Results**:
- Travel initiated successfully
- Fuel = 0 after departure

**Pass Criteria**: Exact fuel amount handled correctly

---

### Test 5.6: Empty Planet List
**Description**: Verify behavior with no planets.

**Preconditions**:
- Empty vector of planets

**Test Steps**:
1. Call advance_planet_positions on empty vector
2. Verify no panic
3. Verify vector remains empty

**Expected Results**:
- Function returns successfully
- No changes to empty vector

**Pass Criteria**: Empty input handled gracefully

---

### Test 5.7: Single Planet in System
**Description**: Verify system works with only one planet.

**Preconditions**:
- Single planet system

**Test Steps**:
1. Advance positions (should work)
2. Attempt travel from planet to itself
3. Verify appropriate handling

**Expected Results**:
- Position advancement works
- Self-travel handled (0 turns or rejected)

**Pass Criteria**: Single planet system functional

---

## Test Suite 6: Integration Tests

### Test 6.1: Full Travel Cycle
**Description**: Verify complete travel workflow.

**Preconditions**:
- Ship at Planet A, Idle state
- Sufficient fuel for Planet B

**Test Steps**:
1. Initiate travel to Planet B
2. Advance turns until arrival
3. Verify final state at Planet B
4. Verify fuel consumed correctly
5. Verify turn counter advanced correctly

**Expected Results**:
- Complete journey successful
- All state updates correct
- Resources consumed correctly

**Pass Criteria**: End-to-end travel works

---

### Test 6.2: Multiple Concurrent Travels (if applicable)
**Description**: Verify system handles multiple ships traveling.

**Preconditions**:
- Multiple ships in different states

**Test Steps**:
1. Ship A starts travel (5 turns)
2. Ship B starts travel (3 turns)
3. Advance 3 turns
4. Verify Ship B arrived, Ship A still traveling
5. Advance 2 more turns
6. Verify Ship A arrived

**Expected Results**:
- Each ship tracks independent travel time
- Arrivals happen at correct turns

**Pass Criteria**: Multiple ships handled correctly

---

### Test 6.3: Planet Movement During Travel
**Description**: Verify planets continue orbiting during ship travel.

**Preconditions**:
- Ship traveling for 10 turns
- Planets with various periods

**Test Steps**:
1. Record initial planet positions
2. Start ship travel
3. Advance 10 turns
4. Verify planets advanced 10 positions each
5. Verify ship arrived

**Expected Results**:
- Planets moved correctly during travel
- Ship arrival independent of planet positions

**Pass Criteria**: Time advances consistently for all entities

---

## Test Data Reference

### Standard Test Planets
```rust
// From ADR example
let planet_a = Planet {
    name: "Planet A",
    orbital_radius: 5,
    orbital_period: 10,
    position: 0,
};

let planet_b = Planet {
    name: "Planet B",
    orbital_radius: 12,
    orbital_period: 15,
    position: 7,
};
```

### Edge Case Test Values
- **Zero values**: 0 acceleration, 0 distance, 0 fuel
- **Maximum values**: u32::MAX for periods, large radii
- **Boundary values**: position = period - 1, exact fuel amounts
- **Typical values**: acceleration 1-10, distances 5-100

---

## Success Criteria Summary

### Must Pass (Critical)
- [ ] All position advancement tests
- [ ] All travel time calculation tests
- [ ] All state transition tests
- [ ] All edge case safety tests (no panics)

### Should Pass (High Priority)
- [ ] Fuel consumption tests
- [ ] Integration tests
- [ ] Error handling tests

### Nice to Have (Medium Priority)
- [ ] Performance tests (large number of planets)
- [ ] Stress tests (rapid state changes)

---

## Testing Concerns & Recommendations

### 1. Floating Point Precision
**Concern**: The formula `2 * sqrt(distance / acceleration)` uses floating-point math.
**Recommendation**: 
- Document rounding strategy (floor, ceil, or round)
- Ensure consistent behavior across platforms
- Consider using integer math if precision issues arise

### 2. Integer Overflow
**Concern**: Large orbital periods or radii could cause overflow.
**Recommendation**:
- Use `checked_add` and `checked_mul` where applicable
- Document maximum supported values
- Consider using u64 for internal calculations if needed

### 3. Division by Zero
**Concern**: Zero acceleration causes division by zero in formula.
**Recommendation**:
- Add validation to reject zero acceleration
- Or use default minimum acceleration (e.g., 1)
- Document behavior in API

### 4. Test Coverage
**Concern**: Edge cases may be missed.
**Recommendation**:
- Aim for >90% code coverage
- Use `cargo tarpaulin` to measure coverage
- Add property-based tests with `proptest` for fuzzing

### 5. State Consistency
**Concern**: Travel state could become inconsistent.
**Recommendation**:
- Add invariant checks in debug builds
- Verify state machine completeness
- Consider using typestate pattern for compile-time safety

---

## Related Documentation
- ADR: /design/adr/0002-movement-mechanics-system.md
- Implementation: src/orbital_mechanics/
- Epic: #92

---

## Revision History
| Date | Version | Author | Changes |
|------|---------|--------|---------|
| 2026-03-08 | 1.0 | QA Engineer | Initial test plan creation |
