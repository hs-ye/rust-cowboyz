# Creative Lead Agent - Summary

## What Changed

The `creative-lead` agent has been updated from a **complex 3D-focused asset management system** to a **simple, indie-friendly approach** tailored for 2D sprites, audio, and minimal video.

---

## Key Differences

### Before (Complex - for 3D AAA studios):
- Tracked 3D models, textures, rigs, animations
- Complex polycount budgets and LODs
- Multi-stage review pipelines
- Heavy metadata schemas
- Required specialized tools (Perforce, ShotGrid)

### After (Simple - for indie 2D studios):
- Tracks **sprites** (PNG/SVG), **audio** (MP3/WAV), **video** (MP4)
- Simple technical specs (dimensions, format)
- Lightweight YAML manifest
- Git-friendly workflow
- No specialized tools needed

---

## Core Responsibilities

The creative-lead now handles:

1. **Narrative Creation** (unchanged)
   - Story, characters, dialogue, quests, lore

2. **Asset Identification** (enhanced)
   - Identify sprites needed for characters/locations/UI
   - Identify audio needed for music/SFX/voice
   - Identify video needed for cutscenes (minimal)

3. **Asset Tracking** (simplified)
   - Maintain `data/creative-assets/manifest.yaml`
   - Specify clear requirements for each asset
   - Track status: pending → assigned → wip → review → complete

4. **Asset Review** (streamlined)
   - Review completed assets against narrative requirements
   - Approve or request revisions
   - Update manifest status

---

## Asset Types Supported

### 1. Sprites & Graphics
- **Character sprites**: Player, NPCs, enemies (PNG with transparency)
- **Environment backgrounds**: Locations, buildings (PNG/SVG)
- **UI elements**: Buttons, health bars, dialogue boxes (PNG/SVG)
- **Item icons**: Weapons, potions, collectibles (PNG)

### 2. Audio
- **Music**: Main theme, battle, ambient (MP3/OGG)
- **Sound effects**: Attacks, UI clicks, environmental (WAV/MP3)
- **Voice lines**: Character dialogue (WAV/MP3, if any)
- **Ambient sounds**: Wind, water, crowds (WAV/MP3)

### 3. Video (Minimal)
- **Cutscenes**: Story sequences (MP4/WebM, optional)
- **Title screens**: Opening animations (MP4/WebM)

---

## Workflow Overview

```
1. Narrative Created
   ↓
2. Assets Identified
   ↓
3. Manifest Updated
   ↓
4. Asset Tickets Created
   ↓
5. Specialists Assigned (by project-manager)
   ↓
6. Assets Created
   ↓
7. Creative-lead Reviews
   ↓
8. Status Updated to "Complete"
   ↓
9. Technical Team Integrates
```

---

## File Structure

```
project-root/
├── agent-design/
│   └── creative-lead.md          # Updated agent definition
│
├── data/
│   ├── narrative/                 # Story, characters, dialogue
│   │   ├── characters/
│   │   ├── dialogue/
│   │   ├── quests/
│   │   └── lore/
│   │
│   └── creative-assets/           # Asset tracking
│       ├── manifest.yaml          # Main asset database
│       ├── README.md              # Documentation
│       └── WORKFLOW_EXAMPLE.md    # Example workflow
│
└── assets/                        # Final asset files
    ├── sprites/
    │   ├── characters/
    │   ├── environments/
    │   ├── ui/
    │   └── items/
    ├── audio/
    │   ├── music/
    │   ├── sfx/
    │   └── voice/
    └── video/
        └── cutscenes/
```

---

## Manifest Example

```yaml
assets:
  sprite_player_idle:
    type: "sprite"
    category: "character"
    name: "Player - Idle Sprite"
    status: "pending"
    priority: "critical"
    requirements:
      style: "2D pixel art, hero character"
      technical:
        format: "PNG with transparency"
        dimensions: "64x64 pixels"
      visual_reference: "Determined expression, adventurer gear"
    notes: "Main player character sprite"

  music_main_theme:
    type: "music"
    category: "theme"
    name: "Main Theme"
    status: "pending"
    priority: "high"
    requirements:
      mood: "Epic but approachable, sense of adventure"
      technical:
        format: "MP3 or OGG"
        length: "Loopable 1-2 minute track"
    notes: "Title screen and safe areas music"
```

---

## Why This Works for Indie Studios

### ✅ Advantages
- **Simple to understand** - No complex pipelines or tools
- **Git-friendly** - Manifest merges cleanly, assets in regular folders
- **Engine-agnostic** - Works with any game engine (custom, Unity, Godot, etc.)
- **Flexible** - Easy to adapt as project grows
- **Lightweight** - Doesn't slow down development
- **Clear tracking** - Know what assets are needed and their status

### ✅ Scales Well
- Works for 10 assets or 1,000 assets
- Can add more fields to manifest if needed
- Can upgrade to Trello/Airtable if manifest gets too large
- Can add Git LFS if assets get too big for regular Git

### ✅ Focus on Making Games
- Minimal overhead
- No tool learning curve
- No complex setup
- Just write narrative → identify assets → track progress

---

## Industry Context

### Professional Studios Use:
- **Perforce** - Version control for TB-scale assets
- **ShotGrid/ftrack** - Asset tracking and review pipelines
- **Custom databases** - Complex metadata schemas
- **DCC tools** - Maya, Blender, Photoshop with export pipelines

### Why We Don't Need That:
- **Small asset count** - Dozens or hundreds, not thousands
- **Simple formats** - PNG, MP3, not complex 3D scenes
- **Small team** - 1-3 people, not 100+
- **Rapid iteration** - Need flexibility, not rigid pipelines

### Our Approach Mirrors:
- **Early indie studios** - Simple, effective, focused on shipping
- **Game jams** - Lightweight tracking that doesn't get in the way
- **Solo developers** - Practical solutions over complex systems

---

## When to Revisit

Consider upgrading if:
- Asset count exceeds 500+ files
- Multiple artists working simultaneously
- Need advanced versioning for large binary files
- Team grows beyond 5-10 people
- Complex dependencies between assets

For now: **Keep it simple and ship the game.**

---

## Quick Start

1. **Read narrative** - Understand what needs to be created
2. **Identify assets** - What sprites/audio/video are needed?
3. **Update manifest** - Add asset entries with requirements
4. **Create tickets** - Generate GitHub issues for specialists
5. **Review assets** - Check completed work against narrative
6. **Update status** - Mark assets as "complete" when approved

That's it! Simple, effective, and gets out of your way.

---

## Files Created/Updated

✅ `/agent-design/creative-lead.md` - Simplified agent definition  
✅ `/data/creative-assets/manifest.yaml` - Example asset manifest  
✅ `/data/creative-assets/README.md` - Complete documentation  
✅ `/data/creative-assets/WORKFLOW_EXAMPLE.md` - Step-by-step example  
✅ `/assets/` directory structure - Organized asset folders  

---

## Summary

The creative-lead agent is now a **complete creative lead** that:
- Creates compelling narrative content
- Identifies all creative assets needed (sprites, audio, video)
- Tracks assets in a simple, maintainable manifest
- Reviews assets for narrative alignment
- Coordinates with specialists through project-manager

**All tailored for indie studio needs - no over-engineering, just effective asset management.**
