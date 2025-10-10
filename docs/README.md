# 📚 Documentation Directory
**Scrabble Learning Game (TileMania)**

---

## 📋 Documentation Overview

This directory contains all architectural, design, and decision records for the TileMania project. Documents are organized chronologically and by purpose.

**Last Updated:** 2025-10-09
**Project Phase:** Pre-Sprint 1 (Architecture Approved)

---

## 🗂️ Document Categories

### 1. Core Architecture & Design (CURRENT - Use These)

These are the **authoritative** documents for development:

| Document | Purpose | Status | Audience |
|----------|---------|--------|----------|
| [ARCHITECTURE.md](ARCHITECTURE.md) | Technical architecture specification | ✅ Current | Developers |
| [GAME_DESIGN.md](GAME_DESIGN.md) | Gameplay mechanics & UX design | ✅ Current | Designers, Developers |
| [IMPLEMENTATION_ROADMAP.md](IMPLEMENTATION_ROADMAP.md) | Sprint-by-sprint implementation plan | ✅ Current | All team |
| [ARCHITECTURE_DECISIONS.md](ARCHITECTURE_DECISIONS.md) | Architecture Decision Record (ADR) | ✅ Approved | Tech leads, Architects |

**Usage:** Reference these documents during Sprint 1+ implementation.

---

### 2. Architecture Review Process (ARCHIVE - Historical Record)

These documents capture the review/approval process. Keep for **historical reference** but not needed for daily work:

| Document | Purpose | Date | Status |
|----------|---------|------|--------|
| [01_ARCHITECTURE_REVIEW.md](archive/01_ARCHITECTURE_REVIEW.md) | Tech Lead's initial critical review | 2025-10-08 | 📁 Archived |
| [02_TECH_LEAD_FEEDBACK.md](archive/02_TECH_LEAD_FEEDBACK.md) | Conditional approval with 5 conditions | 2025-10-08 | 📁 Archived |
| [03_ARCHITECT_RESPONSE.md](archive/03_ARCHITECT_RESPONSE.md) | Senior Architect addresses all concerns | 2025-10-08 | 📁 Archived |
| [04_TECH_LEAD_FINAL_APPROVAL.md](archive/04_TECH_LEAD_FINAL_APPROVAL.md) | Unconditional approval granted | 2025-10-09 | 📁 Archived |

**Usage:** Reference if questions arise about "why we chose X over Y" or for onboarding new team members.

---

## 📖 Quick Start Guide

### For New Team Members

**Start here:**
1. Read [Executive Summary](../Executive%20Summary.md) (5 min) - Project overview
2. Read [GAME_DESIGN.md](GAME_DESIGN.md) (30 min) - Understand gameplay
3. Skim [ARCHITECTURE.md](ARCHITECTURE.md) (20 min) - System architecture
4. Review [IMPLEMENTATION_ROADMAP.md](IMPLEMENTATION_ROADMAP.md) (15 min) - Sprint plan

**Total:** ~70 minutes to get up to speed

### For Developers Starting Sprint 1

**Reference documents:**
1. [IMPLEMENTATION_ROADMAP.md](IMPLEMENTATION_ROADMAP.md) - Your sprint tasks
2. [ARCHITECTURE.md](ARCHITECTURE.md) - Technical specifications
3. [ARCHITECTURE_DECISIONS.md](ARCHITECTURE_DECISIONS.md) - Why we chose wolges, etc.

### For Understanding "Why Did We Decide X?"

**Check the archive:**
1. [archive/01_ARCHITECTURE_REVIEW.md](archive/01_ARCHITECTURE_REVIEW.md) - Original concerns
2. [archive/03_ARCHITECT_RESPONSE.md](archive/03_ARCHITECT_RESPONSE.md) - Rationale for decisions

---

## 🔄 Document Lifecycle

### Living Documents (Updated Regularly)

- **IMPLEMENTATION_ROADMAP.md** - Updated each sprint
- **ARCHITECTURE.md** - Updated when architecture changes
- **GAME_DESIGN.md** - Updated when gameplay mechanics change

### Frozen Documents (Historical Record)

- **ARCHITECTURE_DECISIONS.md** - Frozen after approval (add new ADRs if needed)
- **Archive documents** - Never modified (historical record)

---

## 📂 Proposed Directory Structure

We recommend reorganizing as follows:

```
docs/
├── README.md                          # This file
├── ARCHITECTURE.md                    # Core technical spec
├── GAME_DESIGN.md                     # Core gameplay spec
├── IMPLEMENTATION_ROADMAP.md          # Core sprint plan
├── ARCHITECTURE_DECISIONS.md          # ADR (frozen)
│
├── archive/                           # Historical review process
│   ├── 01_ARCHITECTURE_REVIEW.md
│   ├── 02_TECH_LEAD_FEEDBACK.md
│   ├── 03_ARCHITECT_RESPONSE.md
│   └── 04_TECH_LEAD_FINAL_APPROVAL.md
│
├── guides/                            # To be created in Sprint 3+
│   ├── WOLGES_INTEGRATION_GUIDE.md
│   ├── AI_HEURISTICS.md
│   └── LEXICON_CONVERSION_GUIDE.md
│
└── adr/                               # Future ADRs (if needed)
    └── 001-use-wolges-engine.md       # Example future ADR
```

---

## 📝 Naming Conventions

### Current Documents
- Use SCREAMING_SNAKE_CASE (e.g., `ARCHITECTURE.md`)
- Descriptive names (what, not when)
- No dates in filename

### Archive Documents
- Prefix with chronological number (`01_`, `02_`, etc.)
- Original descriptive name preserved
- Moved to `archive/` subdirectory

### Future Documents
- ADRs: `adr/NNN-descriptive-name.md` (e.g., `001-use-wolges-engine.md`)
- Guides: `guides/DESCRIPTIVE_NAME.md`

---

## 🔍 Document Relationships

```
Executive Summary.md (Root)
    ↓
    ├─→ ARCHITECTURE.md ────────┐
    ├─→ GAME_DESIGN.md          ├─→ Inform ─→ IMPLEMENTATION_ROADMAP.md
    └─→ ARCHITECTURE_DECISIONS.md┘
                ↑
                │ Influenced by
                │
         [Archive: Review Process]
         ├─ 01_ARCHITECTURE_REVIEW.md
         ├─ 02_TECH_LEAD_FEEDBACK.md
         ├─ 03_ARCHITECT_RESPONSE.md
         └─ 04_TECH_LEAD_FINAL_APPROVAL.md
```

---

## ✅ Recommended Actions

### Immediate (Before Sprint 1 Start)

1. **Create `archive/` subdirectory**
   ```bash
   mkdir -p docs/archive
   ```

2. **Move review process documents to archive with chronological prefixes**
   ```bash
   mv docs/ARCHITECTURE_REVIEW.md docs/archive/01_ARCHITECTURE_REVIEW.md
   mv docs/TECH_LEAD_FEEDBACK.md docs/archive/02_TECH_LEAD_FEEDBACK.md
   mv docs/ARCHITECT_RESPONSE.md docs/archive/03_ARCHITECT_RESPONSE.md
   mv docs/TECH_LEAD_FINAL_APPROVAL.md docs/archive/04_TECH_LEAD_FINAL_APPROVAL.md
   ```

3. **Update internal links in current documents**
   - Update ARCHITECTURE_DECISIONS.md references
   - Update any cross-references

4. **Add this README.md to docs/**
   - Helps new team members navigate

### Near-Term (Sprint 1-3)

5. **Create `guides/` subdirectory when needed**
   ```bash
   mkdir -p docs/guides
   ```

6. **Add integration guides as we build**
   - WOLGES_INTEGRATION_GUIDE.md (Sprint 3)
   - AI_HEURISTICS.md (Sprint 6-7)
   - LEXICON_CONVERSION_GUIDE.md (Sprint 1)

### Long-Term (Sprint 4+)

7. **Create `adr/` for future ADRs**
   - Only if we need more major decisions
   - Use ADR template format

---

## 📊 Document Status Legend

| Status | Meaning |
|--------|---------|
| ✅ Current | Authoritative, use for development |
| 📁 Archived | Historical record, don't edit |
| 🚧 Draft | Work in progress |
| ❌ Deprecated | Superseded, ignore |

---

## 🔗 External References

- [Executive Summary](../Executive%20Summary.md) - Project overview (root directory)
- [CSW24.txt](../CSW24.txt) - Word list (root directory)
- [Bevy Engine Docs](https://bevyengine.org/)
- [wolges Crate](https://crates.io/crates/wolges)

---

## 📞 Document Ownership

| Document Type | Owner | Approver |
|--------------|-------|----------|
| ARCHITECTURE.md | Senior Architect | Tech Lead |
| GAME_DESIGN.md | Game Designer | Product Owner |
| IMPLEMENTATION_ROADMAP.md | Tech Lead | Senior Architect |
| ADRs | Senior Architect | Tech Lead + PO |
| Guides | Lead Developer | Tech Lead |

---

## 🔄 Update Process

### For Core Documents (ARCHITECTURE.md, etc.)

1. Create feature branch
2. Make changes
3. Submit PR with:
   - Rationale for change
   - Impact assessment
   - Updated "Last Updated" date
4. Require approval from document owner
5. Merge to main

### For Archive Documents

**DO NOT MODIFY** - These are historical records.

If you need to correct something, add an addendum in the current documents.

---

## 📝 Changelog

| Date | Change | By |
|------|--------|-----|
| 2025-10-09 | Created docs/README.md | Senior Architect |
| 2025-10-09 | Proposed archive/ structure | Senior Architect |

---

## 🆘 Need Help?

**Questions about:**
- Architecture → Senior Architect
- Game Design → Game Designer
- Sprint Tasks → Tech Lead
- Documentation Structure → Tech Lead

**Want to add a document?**
1. Check with document owner first
2. Use appropriate naming convention
3. Add entry to this README
4. Submit PR for review

---

**Document Status:** ✅ Current
**Maintained By:** Tech Lead + Senior Architect
**Review Frequency:** Monthly or when structure changes
