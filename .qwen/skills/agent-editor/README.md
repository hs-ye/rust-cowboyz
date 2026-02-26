# Agent Editor Skill

A skill for creating and managing Qwen subagent configurations.

## Overview

This skill helps you design, create, and maintain Qwen subagents for any domain or use case. It provides guidance on:
- Agent configuration format
- Multi-agent interaction patterns
- Best practices for agent design
- Escalation and communication protocols

## Quick Start

1. **Understand the pattern**: Read SKILL.md to understand agent design principles
2. **Design your agents**: Plan agent roles, responsibilities, and interactions
3. **Create configurations**: Use the format specified in SKILL.md
4. **Test and iterate**: Validate agents work as expected
5. **Maintain consistency**: Use helper scripts to check consistency

## Usage

When you need help with:
- Creating new specialized agents
- Updating existing agent behavior
- Designing multi-agent workflows
- Ensuring agent consistency
- Following Qwen subagent best practices

Invoke this skill and describe what you want to achieve.

## Examples

See SKILL.md for detailed examples of:
- Creating a new agent from requirements
- Updating agent interaction protocols
- Designing multi-agent workflows
- Common agent types and patterns

## Helper Scripts

```bash
# List all agents
./scripts/helper.sh list

# Validate agent files
./scripts/helper.sh validate <file>
./scripts/helper/helper.sh validate-all

# Check consistency
./scripts/helper.sh check-consistency

# Update patterns across all agents
./scripts/helper.sh update-pattern "old" "new"

# Show statistics
./scripts/helper.sh stats
```

## File Structure

```
.qwen/skills/agent-editor/
├── SKILL.md           # Main documentation
├── README.md          # This file
└── scripts/
    └── helper.sh      # Helper utilities
```

## References

- Qwen Subagent Specification: See SKILL.md
- Example implementations: See SKILL.md

