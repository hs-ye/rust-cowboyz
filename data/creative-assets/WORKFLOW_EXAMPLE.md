# Example Workflow: Creating a New NPC

This demonstrates the complete workflow from narrative to asset creation.

## Scenario
We need to add "Elder Willow" - a wise old NPC who gives the player their first quest.

---

## Step 1: Narrative Creation

**File**: `data/narrative/characters/elder_willow.yaml`

```yaml
name: "Elder Willow"
role: "Village Elder"
location: "Starting Town - Elder's Hut"
personality:
  traits: ["wise", "gentle", "mysterious"]
  speech_style: "slow, deliberate, with pauses for wisdom"
background: |
  Elder Willow has guided the village for decades. She possesses ancient knowledge
  of the world and senses the player's destiny. Though frail in body, her mind is sharp.
dialogue:
  greeting: "Ah, I've been expecting you, child. The winds speak of your arrival."
  quest_offer: "A darkness stirs in the old forest. Will you investigate for our village?"
  farewell: "May the old gods watch over your path."
```

**File**: `data/narrative/quests/first_quest.yaml`

```yaml
title: "The Darkening Forest"
type: "main"
npc: "elder_willow"
objectives:
  - "Speak to Elder Willow"
  - "Investigate the forest edge"
  - "Return with findings"
reward:
  items: ["rusty_sword"]
  experience: 100
```

---

## Step 2: Asset Identification

Creative-lead reviews the narrative and identifies required assets:

### Assets Needed:
1. **Character sprite** - Elder Willow (idle, talk animations)
2. **Background** - Elder's Hut interior
3. **UI element** - Quest notification popup
4. **Audio** - Ambient hut sounds, quest accept SFX
5. **Item icon** - Rusty sword icon

---

## Step 3: Update Manifest

Add to `data/creative-assets/manifest.yaml`:

```yaml
  sprite_elder_willow_idle:
    type: "sprite"
    category: "character"
    name: "Elder Willow - Idle Sprite"
    status: "pending"
    priority: "high"
    narrative_source: "data/narrative/characters/elder_willow.yaml"
    requirements:
      style: "2D pixel art, elderly woman, wise expression, simple robes"
      technical:
        format: "PNG with transparency"
        dimensions: "64x64 pixels"
        states_needed: ["idle", "talk"]
      visual_reference: "White hair, kind face, leaning on staff, humble clothing"
    notes: "Should convey wisdom and gentleness"

  sprite_elders_hut_interior:
    type: "background"
    category: "environment"
    name: "Elder's Hut - Interior Background"
    status: "pending"
    priority: "medium"
    narrative_source: "data/narrative/characters/elder_willow.yaml"
    requirements:
      style: "Cozy hut interior, wooden walls, simple furniture, warm lighting"
      technical:
        format: "PNG"
        dimensions: "1920x1080"
      mood: "Warm, safe, slightly mysterious with herbs and old books"
    notes: "Should feel like a place of wisdom and comfort"

  ui_quest_notification:
    type: "ui"
    category: "interface"
    name: "Quest Notification Popup"
    status: "pending"
    priority: "medium"
    requirements:
      style: "Matches game UI aesthetic, scroll-like or parchment design"
      technical:
        format: "PNG with transparency"
        dimensions: "400x200 pixels"
      usage: "Appears when player receives new quest"
    notes: "Should be eye-catching but not obstructive"

  audio_ambient_hut:
    type: "audio"
    category: "ambient"
    name: "Elder's Hut - Ambient Sounds"
    status: "pending"
    priority: "low"
    requirements:
      style: "Crackling fireplace, gentle wind, occasional cricket sounds"
      technical:
        format: "MP3 or OGG"
        length: "Loopable 30-second track"
    notes: "Creates cozy, peaceful atmosphere"

  sfx_quest_accept:
    type: "sfx"
    category: "ui"
    name: "Quest Accept Sound"
    status: "pending"
    priority: "low"
    requirements:
      style: "Magical chime or soft whoosh sound"
      technical:
        format: "WAV or MP3"
        length: "Under 1 second"
    notes: "Pleasant sound when player accepts quest"

  item_rusty_sword_icon:
    type: "sprite"
    category: "item"
    name: "Rusty Sword - Icon"
    status: "pending"
    priority: "medium"
    narrative_source: "data/narrative/quests/first_quest.yaml"
    requirements:
      style: "Simple sword icon, slightly worn/dull appearance"
      technical:
        format: "PNG with transparency"
        dimensions: "32x32 pixels"
      usage: "Inventory and reward display"
    notes: "Should look like a starter weapon"
```

---

## Step 4: Create Asset Tickets

Creative-lead creates GitHub issues:

```bash
# Create tickets for sprite artist
gh issue create \
  --title "[ASSET] Create: Elder Willow Sprite" \
  --body "## Asset Required
**ID**: sprite_elder_willow_idle
**Type**: sprite (character)
**Priority**: high

## Narrative Context
- **Source**: data/narrative/characters/elder_willow.yaml
**Purpose**: NPC character for first quest

## Requirements
Style: 2D pixel art, elderly woman, wise expression, simple robes
Technical: PNG with transparency, 64x64 pixels, idle + talk states
Visual: White hair, kind face, leaning on staff, humble clothing

See manifest.yaml for full details.

**Assigned by**: creative-lead
**Status**: pending" \
  --label "asset" --label "pending" --label "priority-high"

# Repeat for other assets...
```

---

## Step 5: Assignment & Creation

**Project-manager assigns tickets:**
- sprite-artist gets: Elder Willow sprite, hut background, UI notification, sword icon
- sound-designer gets: ambient hut sounds, quest accept SFX

**Specialists create assets:**
- sprite-artist creates files and places in:
  - `assets/sprites/characters/elder_willow_idle.png`
  - `assets/sprites/environments/elders_hut_interior.png`
  - `assets/sprites/ui/quest_notification.png`
  - `assets/sprites/items/rusty_sword_icon.png`

- sound-designer creates files and places in:
  - `assets/audio/ambient/hut_ambient.mp3`
  - `assets/audio/sfx/quest_accept.wav`

---

## Step 6: Review

**Creative-lead reviews each asset:**

### Review Checklist:
- [ ] Visual/audio style matches narrative description
- [ ] Technical specs correct (format, dimensions, quality)
- [ ] Fits game's overall aesthetic
- [ ] Serves intended purpose clearly

**If approved:**
```bash
# Update manifest status
# Edit data/creative-assets/manifest.yaml:
#   sprite_elder_willow_idle:
#     status: "complete"
```

**If revisions needed:**
- Comment on asset ticket with specific feedback
- Specialist makes changes
- Re-review until approved

---

## Step 7: Integration

**Technical team integrates assets:**
- Import sprites into game engine
- Add audio files to sound system
- Wire up quest notification UI
- Test that Elder Willow appears correctly and gives quest

---

## Result

✅ **Narrative complete**: Elder Willow character and quest written  
✅ **Assets created**: Sprite, background, UI, audio all done  
✅ **Tracked**: All assets logged in manifest with status  
✅ **Integrated**: NPC functional in game  

---

## Key Takeaways

1. **Every narrative element needs assets** - Don't forget to identify them
2. **Manifest is single source of truth** - Keep it updated
3. **Clear requirements prevent rework** - Be specific about style and specs
4. **Review ensures quality** - Creative-lead validates against narrative
5. **Simple workflow scales** - Works for 1 asset or 100 assets

This example shows how the creative-lead bridges narrative and asset creation, ensuring the story comes to life visually and audibly.
