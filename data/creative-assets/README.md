# Creative Asset Management - Indie Studio Approach

## Overview

Simple, lightweight asset tracking system for indie studios working with:
- **Sprites/Graphics**: PNG, SVG files for characters, backgrounds, UI
- **Audio**: MP3, WAV, OGG files for music, SFX, voice
- **Video**: MP4, WebM files for cutscenes (minimal)

## Why This Approach?

### For Indie Studios:
✅ **No complex 3D pipelines** - Assets are simpler and faster to create  
✅ **Standard file formats** - Works with any game engine (custom, Unity, Godot, etc.)  
✅ **Lightweight tracking** - Single YAML manifest instead of complex databases  
✅ **Git-friendly** - Manifest files merge cleanly in version control  
✅ **Flexible workflow** - Easy to adapt as project grows  

### What We Avoid:
❌ Over-engineering with enterprise tools (Perforce, ShotGrid)  
❌ Complex metadata schemas for simple assets  
❌ Engine-specific asset pipelines  
❌ Heavyweight tracking systems  

## Asset Categories

### 1. Sprites & Graphics
**File formats**: PNG (with transparency), SVG (scalable)

| Type | Examples | Priority |
|------|----------|----------|
| Character sprites | Player, NPCs, enemies | Critical/High |
| Environment backgrounds | Towns, dungeons, landscapes | High |
| UI elements | Buttons, health bars, dialogue boxes | High |
| Item icons | Weapons, potions, collectibles | Medium |

**Technical specs**:
- PNG: Typically 64x64 to 1920x1080 depending on use
- SVG: Scalable for UI elements and logos
- Transparency: Alpha channel for sprites

### 2. Audio
**File formats**: MP3 (compressed), WAV (uncompressed), OGG (game-friendly)

| Type | Examples | Priority |
|------|----------|----------|
| Music | Main theme, battle, ambient | High |
| Sound effects | Sword swings, UI clicks, footsteps | Medium |
| Voice lines | Character dialogue (if any) | Medium/Low |
| Ambient sounds | Wind, water, crowd noise | Low |

**Technical specs**:
- Music: 128-320 kbps MP3 or OGG, loopable
- SFX: 44.1kHz-48kHz WAV or MP3, short duration
- Voice: Clear recording, minimal background noise

### 3. Video (Minimal)
**File formats**: MP4 (H.264), WebM (VP9)

| Type | Examples | Priority |
|------|----------|----------|
| Cutscenes | Story sequences, intro | Low (optional) |
| Title screens | Opening animations | Low |

**Technical specs**:
- Resolution: 1080p minimum
- Codec: H.264 (MP4) or VP9 (WebM)
- Keep short: 30-60 seconds max

## Workflow

### Step 1: Narrative Creation
Creative-lead writes story, characters, dialogue, quests.

### Step 2: Asset Identification
For each narrative element, identify required assets:
- Character description → character sprite
- Location description → background art
- UI needs → UI elements
- Mood/atmosphere → music and SFX

### Step 3: Update Manifest
Add asset entries to `data/creative-assets/manifest.yaml` with:
- Clear requirements (style, technical specs)
- Priority level
- Narrative source reference
- Status: "pending"

### Step 4: Create Asset Ticket
Generate GitHub issue for each asset (or batch similar assets):
```
[ASSET] Create: [Asset Name]
Type: [sprite/audio/video/ui]
Priority: [high/medium/low]
Requirements: [from manifest]
```

### Step 5: Assignment
Project-manager assigns tickets to specialists:
- **sprite-artist**: All visual assets
- **sound-designer**: All audio assets
- **video-editor**: Video cutscenes (if any)

### Step 6: Creation & Review
1. Specialist creates asset and places in `assets/` folder
2. Creative-lead reviews against narrative requirements
3. Approve or request revisions
4. Update manifest status to "complete"

### Step 7: Integration
Technical team imports assets into game engine.

## File Organization

```
project-root/
├── data/
│   ├── narrative/              # Story, characters, dialogue, quests
│   │   ├── characters/
│   │   ├── dialogue/
│   │   ├── quests/
│   │   └── lore/
│   │
│   └── creative-assets/
│       └── manifest.yaml       # Asset tracking database
│
├── assets/                     # Final asset files
│   ├── sprites/
│   │   ├── characters/
│   │   ├── environments/
│   │   ├── ui/
│   │   └── items/
│   │
│   ├── audio/
│   │   ├── music/
│   │   ├── sfx/
│   │   └── voice/
│   │
│   └── video/
│       └── cutscenes/
│
└── src/                        # Game code
```

## Asset Naming Convention

Use descriptive, consistent names:

**Sprites**:
- `sprite_character_player_idle.png`
- `sprite_environment_town_background.png`
- `ui_health_bar.png`
- `item_sword_icon.png`

**Audio**:
- `music_main_theme.mp3`
- `music_battle.mp3`
- `sfx_sword_swing.wav`
- `sfx_ui_click.wav`

**Video**:
- `video_intro_cutscene.mp4`

## Priority Guidelines

### Critical
- Player character sprites
- Core gameplay UI
- Main theme music

### High
- NPC characters
- Main environment backgrounds
- Battle music
- Essential SFX (attacks, UI)

### Medium
- Secondary characters
- Side location backgrounds
- Ambient music
- Voice lines (if any)

### Low
- Polish/extra SFX
- Optional cutscenes
- Decorative elements

## Status Workflow

```
pending → assigned → wip → review → complete
```

- **pending**: Identified but not assigned
- **assigned**: Ticket created, assigned to specialist
- **wip**: Specialist working on it
- **review**: Ready for creative-lead review
- **complete**: Approved and ready for integration

## Tools We Use

### Version Control
- **Git**: For code and small assets
- **Git LFS**: If assets get too large for regular Git

### Tracking
- **GitHub Issues**: Asset tickets and status
- **YAML Manifest**: Single source of truth for requirements
- **Simple commands**: `grep` to check status, edit YAML to update

### Optional (if needed later)
- **Trello/Notion**: Visual board for asset status
- **Airtable**: Spreadsheet-like database if manifest gets too large

## Comparison to Professional Studios

| Aspect | Professional Studio | Our Indie Approach |
|--------|-------------------|-------------------|
| **Asset Types** | 3D models, textures, animations, VFX, audio, video | Sprites, audio, minimal video |
| **Tracking Tool** | ShotGrid, ftrack, custom databases | Single YAML file |
| **Version Control** | Perforce (TB-scale) | Git + Git LFS |
| **Pipeline** | Complex DCC → Engine export | Direct file creation |
| **Team Size** | 100+ specialists | 1-3 generalists |
| **Review Process** | Multi-stage with stakeholders | Simple creative-lead review |

## When to Upgrade

Consider more complex tools when:
- Asset count exceeds 500+ files
- Multiple artists working simultaneously
- Need advanced versioning for binary assets
- Require detailed audit trails
- Team grows beyond 5-10 people

For now, **keep it simple**. The YAML manifest scales surprisingly well for indie projects.

## Quick Commands

```bash
# Check all pending assets
grep -A 20 "status: pending" data/creative-assets/manifest.yaml

# Count assets by type
grep "type:" data/creative-assets/manifest.yaml | sort | uniq -c

# Update asset status (edit manifest.yaml directly)
# Change: status: "pending" → status: "complete"

# List high priority assets
grep -B 5 "priority: high" data/creative-assets/manifest.yaml
```

## Summary

This approach provides:
✅ **Clear tracking** of all creative assets  
✅ **Simple workflow** that doesn't get in the way  
✅ **Flexibility** to adapt as project evolves  
✅ **Engine-agnostic** - works with any game engine  
✅ **Scalable** from prototype to full game  

**Focus on making the game, not managing complexity.**
