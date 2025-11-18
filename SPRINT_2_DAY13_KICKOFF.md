# 🚀 Sprint 2 - Day 13 Kickoff

**Project:** TileMania - Scrabble Learning Game
**Sprint:** 2 of 13
**Day:** 13 (Wednesday, Week 1)
**Date:** 2025-10-14
**Focus:** Layout System & Spacing Utilities
**Status:** 🟢 **IN PROGRESS**

---

## 📋 Context

### Day 12 Recap
- ✅ Text component system complete
- ✅ 5 style variants, 7 color variants
- ✅ MainMenu and Splash refactored
- ✅ Clean compilation (3.82s, 0 errors/warnings)
- ✅ Committed and pushed

### Sprint 2 Progress
- **Week 1:** 2/5 days complete (40%)
- **Sprint 2:** 2/14 days complete (14%)
- **Components:** Button ✅, Text ✅

---

## 🎯 Day 13 Objectives

### Primary Goal
**Create layout helper system for consistent spacing and container patterns**

### Specific Deliverables
1. **Layout Component Module**
   - Create `src/ui/components/layout.rs`
   - Implement container components
   - Add spacing utilities
   - Create alignment helpers

2. **Container Components**
   - `Container` - Basic container with padding
   - `Stack` - Vertical/horizontal stacking with gap
   - `Center` - Centered content container
   - `Spacer` - Flexible spacing element

3. **Spacing System**
   - Define spacing scale (XS, SM, MD, LG, XL)
   - Implement padding utilities
   - Create margin utilities
   - Add gap helpers

4. **Documentation**
   - Document layout API
   - Create usage examples
   - Update DAY13_COMPLETION_SUMMARY.md

---

## 🏗️ Technical Design

### Layout Components API (Draft)

```rust
// Container with padding
#[derive(Component)]
pub struct Container {
    pub padding: Spacing,
}

// Stack for vertical/horizontal layouts
#[derive(Component)]
pub struct Stack {
    pub direction: StackDirection,
    pub gap: Spacing,
    pub align: Alignment,
}

#[derive(Clone, Copy)]
pub enum StackDirection {
    Vertical,
    Horizontal,
}

#[derive(Clone, Copy)]
pub enum Spacing {
    None,     // 0px
    XS,       // 8px
    SM,       // 16px
    MD,       // 24px
    LG,       // 32px
    XL,       // 48px
    Custom(f32),
}

#[derive(Clone, Copy)]
pub enum Alignment {
    Start,
    Center,
    End,
    Stretch,
}
```

### Integration Pattern

**Before (manual layout):**
```rust
parent.spawn((
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        row_gap: Val::Px(20.0),
        padding: UiRect::all(Val::Px(24.0)),
        ..default()
    },
));
```

**After (layout components):**
```rust
Container::spawn(
    commands,
    Spacing::MD,
    Stack {
        direction: StackDirection::Vertical,
        gap: Spacing::MD,
        align: Alignment::Center,
    },
);
```

---

## 📐 Design Specifications

### Spacing Scale

**Spacing Values:**
- `None` - 0px (no spacing)
- `XS` - 8px (tight spacing)
- `SM` - 16px (compact spacing)
- `MD` - 24px (standard spacing)
- `LG` - 32px (relaxed spacing)
- `XL` - 48px (loose spacing)
- `Custom(f32)` - Any custom value

### Layout Patterns

**Common Patterns:**
1. **Vertical Stack** - Column layout with gap
2. **Horizontal Stack** - Row layout with gap
3. **Centered Container** - Center content both axes
4. **Padded Container** - Container with padding
5. **Spacer** - Flexible space between elements

---

## 🔄 Implementation Plan

### Phase 1: Module Setup (15 min)
1. Create `src/ui/components/layout.rs`
2. Update `src/ui/components/mod.rs` to export layout
3. Define core enums (Spacing, StackDirection, Alignment)
4. Verify compilation

### Phase 2: Spacing System (30 min)
1. Implement `Spacing` enum with `to_val()` method
2. Create helper functions for common spacings
3. Add padding/margin utilities
4. Test spacing conversions

### Phase 3: Container Components (45 min)
1. Implement `Container` component
2. Implement `Stack` component
3. Implement `Center` component
4. Implement `Spacer` component
5. Create spawn methods for each

### Phase 4: Refactor Existing UI (30 min)
1. Refactor MainMenu to use layout components
2. Refactor Splash to use layout components
3. Test visual consistency
4. Verify spacing is correct

### Phase 5: Documentation (20 min)
1. Add inline documentation
2. Create usage examples
3. Update DAY13_COMPLETION_SUMMARY.md
4. Update SPRINT_2_TRACKER.md

**Total Estimated Time:** 2.5 hours

---

## ✅ Success Criteria

### Functional
- [ ] Layout components compile without errors
- [ ] Spacing scale is consistent
- [ ] Containers render correctly
- [ ] Stack direction works (vertical/horizontal)
- [ ] Components are reusable

### Technical
- [ ] Clean compilation (0 errors, 0 warnings)
- [ ] Code is well-documented
- [ ] Follows Bevy 0.15 best practices
- [ ] Consistent with Button/Text patterns

### Quality
- [ ] Layout looks polished
- [ ] Spacing is visually consistent
- [ ] Easy to use and understand
- [ ] Reduces layout boilerplate

---

## 🚨 Potential Challenges

### Challenge 1: Flexbox Complexity
**Risk:** Bevy's flexbox system can be verbose
**Mitigation:** Abstract common patterns into components

### Challenge 2: Spacing Consistency
**Risk:** Different spacing values across screens
**Mitigation:** Enforce spacing scale through enums

### Challenge 3: Component Composition
**Risk:** Nesting containers might be complex
**Mitigation:** Keep API simple, test thoroughly

---

## 📊 Day 13 Metrics

### Code Metrics (Target)
- New files: 1 (components/layout.rs)
- New lines: ~200-250
- Modified files: 3 (components/mod.rs, main_menu.rs, splash.rs)

### Compilation Metrics
- Expected compile time: <5 seconds (cargo check)
- Target: 0 errors, 0 warnings

### Time Allocation
- Setup: 15 min
- Implementation: 1 hour 45 min
- Testing: 30 min
- Documentation: 20 min
- **Total:** 2.5 hours

---

## 🔗 Dependencies

### Technical
- ✅ Bevy 0.15.3 (Node/FlexDirection/UiRect API)
- ✅ Button component system (Day 11)
- ✅ Text component system (Day 12)
- ✅ Rust 1.90.0

### Knowledge
- ✅ Bevy UI layout (flexbox)
- ✅ Component-based architecture
- ✅ Spacing systems (design tokens)

### Resources
- ✅ 3.8GB disk space
- ✅ Clean compilation baseline

---

## 📝 Documentation Plan

### Today's Documents
1. **DAY13_COMPLETION_SUMMARY.md**
   - What was built
   - Code samples
   - Refactoring examples
   - Spacing system details
   - Next steps

2. **Update SPRINT_2_TRACKER.md**
   - Mark Day 13 progress
   - Update metrics

---

## 🎯 Next Steps (Day 14 Preview)

**Day 14 Focus:** Animation system (bevy_tweening)
- Integrate bevy_tweening plugin
- Create button press animations
- Add fade in/out transitions
- Implement state transition animations
- Document animation API

---

## 🚀 Let's Build!

**Current Status:** Ready to code
**Confidence Level:** 95%
**Blockers:** None

**First Action:** Create `src/ui/components/layout.rs` and begin layout system implementation.

---

**Document Status:** ✅ Ready
**Last Updated:** 2025-10-14
**Next Update:** End of Day 13
