# Agent Editor Skill - Complete Documentation Index

## Core Documentation

1. **README.md** (1.9K)
   - Quick overview of the skill
   - Basic usage instructions
   - File structure

2. **SKILL.md** (13K) ⭐ **START HERE**
   - Comprehensive skill documentation
   - Agent configuration format
   - Key principles and patterns
   - Best practices
   - References

## Learning Path

### For Beginners

3. **QUICKSTART.md** (2.5K)
   - Step-by-step guide to creating your first agent
   - Common patterns explained
   - Helper scripts usage

4. **TUTORIAL.md** (8.3K)
   - Complete hands-on tutorial
   - Build a documentation pipeline from scratch
   - Three-agent system example
   - Testing and iteration guide

### For Intermediate Users

5. **USAGE_EXAMPLES.md** (6.7K)
   - Real-world examples
   - Creating testing agents
   - Updating communication protocols
   - Multi-agent workflow design
   - Changing escalation protocols
   - Adding specialty agents

6. **CONFIG.md** (5.1K)
   - Configuration templates
   - Common patterns reference
   - Label conventions
   - Tool selection guide
   - Communication templates

### For Advanced Users

7. **CHANGELOG.md** (1.1K)
   - Version history
   - Feature tracking

## Helper Scripts

Located in `scripts/` directory:

- **helper.sh** - Agent management utilities
  - `list` - List all agents
  - `validate <file>` - Validate specific agent
  - `validate-all` - Validate all agents
  - `check-consistency` - Check consistency across agents
  - `update-pattern` - Replace patterns in all agents
  - `stats` - Show agent statistics

## Recommended Reading Order

1. Start with **SKILL.md** for comprehensive understanding
2. Follow **QUICKSTART.md** to create your first agent
3. Work through **TUTORIAL.md** for hands-on experience
4. Reference **USAGE_EXAMPLES.md** for real-world scenarios
5. Use **CONFIG.md** as a quick reference guide
6. Check **CHANGELOG.md** for updates

## Quick Reference

### Creating an Agent
See: QUICKSTART.md → Step 2

### Common Patterns
See: SKILL.md → Agent Interaction Patterns

### Escalation Protocols
See: CONFIG.md → Label Conventions

### Communication Templates
See: CONFIG.md → Communication Templates

### Multi-Agent Workflows
See: TUTORIAL.md → Part 1 & 2

### Updating Existing Agents
See: USAGE_EXAMPLES.md → Example 2 & 4

## External References

- Qwen Subagent Specification: `/home/yehan/GitRepos/qwen-code/docs/users/features/sub-agents.md`
- Example Agent Roster: `/home/yehan/GitRepos/rust-cowboyz/agent-design/README.md`

