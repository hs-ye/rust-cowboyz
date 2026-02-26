# Agent Editor Quick Start Guide

## Creating Your First Agent

### Step 1: Plan Your Agent

Ask yourself:
- What specific task should this agent handle?
- What tools does it need?
- Should it interact with users directly?
- How will it communicate with other agents?

### Step 2: Create the Configuration File

Create a new file in `.qwen/agents/your-agent-name.md`:

```yaml
---
name: your-agent-name
description: Brief description of what this agent does. MUST BE USED for [specific tasks].
tools:
  - read_file
  - write_file
  - run_shell_command
---

You are a [specialty] specialist.

Your responsibilities:
1. Do task A
2. Do task B
3. Handle blocking issues properly

When you encounter blocking issues:
1. Create a blocking issue
2. Add appropriate labels
3. Continue with other work
```

### Step 3: Define Communication

Decide how your agent will:
- **Escalate issues**: Use labels like `needs-review` or `user-input-required`
- **Report completion**: Comment on tickets or update status files
- **Coordinate with others**: Use shared artifacts (files, issues, etc.)

### Step 4: Test Your Agent

1. Save the configuration
2. Ask the main AI to use your agent
3. Verify it works as expected
4. Adjust the prompt if needed

## Common Patterns

### Pattern A: Coordinator + Specialists

```
User → Coordinator → Specialist 1
                  → Specialist 2
                  → Specialist 3
```

**Coordinator agent:**
- Handles all user communication
- Delegates work to specialists
- Monitors progress and escalations

**Specialist agents:**
- Focus on specific tasks
- Escalate blocking issues to coordinator
- Never communicate directly with user

### Pattern B: Direct Access

```
User → Agent (direct)
```

**Use when:**
- Specialized domain knowledge required
- Immediate user feedback needed
- Decisions can't be delegated

### Pattern C: File-Based Coordination

```
Agent 1 → writes to file → Agent 2 reads file
```

**Use when:**
- Asynchronous workflow preferred
- Multiple agents need same information
- Audit trail important

## Next Steps

1. Read **SKILL.md** for detailed documentation
2. Check **USAGE_EXAMPLES.md** for real-world examples
3. Review **CONFIG.md** for configuration templates
4. Use helper scripts to validate your agents

## Helper Scripts

```bash
# Navigate to skill directory
cd .qwen/skills/agent-editor/scripts

# List all agents
./helper.sh list

# Validate your new agent
./helper.sh validate your-agent-name.md

# Check consistency across all agents
./helper.sh check-consistency
```

