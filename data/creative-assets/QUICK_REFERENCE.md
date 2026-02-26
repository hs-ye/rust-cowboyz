# Creative Lead - Quick Reference

## 📋 Core Responsibilities
1. Write narrative (story, characters, dialogue, quests)
2. Identify assets needed (sprites, audio, video)
3. Update asset manifest with requirements
4. Review completed assets for narrative alignment

## 🎯 Asset Types

### Sprites (PNG/SVG)
- Characters: `assets/sprites/characters/`
- Environments: `assets/sprites/environments/`
- UI: `assets/sprites/ui/`
- Items: `assets/sprites/items/`

### Audio (MP3/WAV/OGG)
- Music: `assets/audio/music/`
- SFX: `assets/audio/sfx/`
- Voice: `assets/audio/voice/`

### Video (MP4/WebM) - Optional
- Cutscenes: `assets/video/cutscenes/`

## 🔄 Workflow

```
Write Narrative
    ↓
Identify Assets
    ↓
Update manifest.yaml
    ↓
Create Tickets
    ↓
Specialists Create Assets
    ↓
Review & Approve
    ↓
Mark Complete
    ↓
Technical Integration
```

## 📊 Asset Status
`pending` → `assigned` → `wip` → `review` → `complete`

## 📁 Key Files
- **Agent**: `agent-design/creative-lead.md`
- **Manifest**: `data/creative-assets/manifest.yaml`
- **Docs**: `data/creative-assets/README.md`
- **Example**: `data/creative-assets/WORKFLOW_EXAMPLE.md`

## 💡 Priority Levels
- **Critical**: Player character, core UI, main theme
- **High**: NPCs, main backgrounds, battle music
- **Medium**: Side content, ambient audio
- **Low**: Polish, optional content

## ✅ Review Checklist
- [ ] Style matches narrative description
- [ ] Technical specs correct (format, dimensions)
- [ ] Fits game aesthetic
- [ ] Serves intended purpose

## 🚀 Quick Commands
```bash
# Check pending assets
grep "status: pending" data/creative-assets/manifest.yaml

# Update asset status
# Edit manifest.yaml directly

# Count assets by type
grep "type:" data/creative-assets/manifest.yaml | sort | uniq -c
```

---

**Remember**: Keep it simple. Focus on the story and the assets that bring it to life.
