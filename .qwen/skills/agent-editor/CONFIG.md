# Agent Editor Configuration

This file documents the configuration options and patterns available in the agent-editor skill.

## Agent Configuration Template

```yaml
---
name: agent-name
description: Brief description of when and how to use this agent. MUST BE USED for [specific tasks].
tools:
  - read_file
  - write_file
  - read_many_files
  - run_shell_command
  - web_search
  - grep_search
  - glob
  - edit
  - task
---

# System Prompt

You are a [specialty] specialist with expertise in [domain].

## Core Responsibilities

1. **Responsibility 1**: Description
2. **Responsibility 2**: Description
3. **Responsibility 3**: Description

## Tools and Capabilities

- **Tool 1**: Purpose and usage
- **Tool 2**: Purpose and usage
- **Tool 3**: Purpose and usage

## Workflow

### Standard Process

1. Step 1 description
2. Step 2 description
3. Step 3 description

### Handling Blocking Issues

1. Create blocking issue with appropriate labels:
   - `blocking`: Indicates blocked progress
   - `user-input-required`: Needs user decision
   - `[specialty]-review`: Needs specialist review

2. Comment on blocked ticket:
```bash
gh issue comment [ticket-number] \
  --body "[Agent] blocked. Created blocking issue #[blocking-issue-number]. Waiting on [coordinator] to communicate with user."
```

3. Add escalation label:
```bash
gh issue edit [blocking-issue-number] --add-label "[escalation-label]"
```

4. Continue with other non-blocked work if available
5. Let coordinator handle user communication

## Best Practices

- Practice 1
- Practice 2
- Practice 3

## Output Standards

- Standard 1
- Standard 2
- Standard 3

## Examples

### Example 1: [Scenario]

```bash
# Command or code example
```

### Example 2: [Scenario]

```bash
# Command or code example
```
```

## Common Patterns

### Pattern 1: Coordinator-Based Workflow

```yaml
description: Works under [coordinator-name] direction and [specialty]. MUST BE USED for all [task-type] tasks.
```

**System Prompt Section:**
```
You work under the direction of the [coordinator-name] and should never wait silently for user input.
```

**Escalation:**
```
3. **Add [coordinator]-review label**:
```bash
gh issue edit [blocking-issue-number] --add-label "[coordinator]-review"
```
```

### Pattern 2: Direct User Access

```yaml
description: Works directly with users for [specialty] decisions. SECOND POINT OF CONTACT for [specific scenarios].
```

**System Prompt Section:**
```
You are the SECOND POINT OF CONTACT for user interactions in [domain]. 
All other agents pause work during your sessions.
```

### Pattern 3: File-Based Communication

**Escalation:**
```bash
# Write to status file instead of commenting
cat > .qwen/status/escalations.json << 'EOF'
{
  "timestamp": "TIMESTAMP",
  "agent": "[agent-name]",
  "ticket": "[ticket-number]",
  "type": "needs-review",
  "message": "[message]"
}
EOF
```

## Label Conventions

### Escalation Labels
- `needs-review`: Requires coordinator attention
- `user-input-required`: Needs user decision
- `blocking`: Cannot proceed without resolution
- `[specialty]-review`: Needs specialist review (e.g., `security-review`, `performance-review`)

### Status Labels
- `in-progress`: Work is actively being done
- `ready-for-review`: Work complete, awaiting review
- `blocked`: Waiting on external dependency
- `completed`: Work finished and verified

### Specialty Labels
- `backend`: Backend development work
- `frontend`: Frontend development work
- `ui`: UI/UX design work
- `api`: API development
- `security`: Security review
- `performance`: Performance optimization
- `documentation`: Documentation tasks

## Tool Selection Guide

### File Operations
- `read_file`: Read single file contents
- `write_file`: Write to single file
- `read_many_files`: Read multiple files at once
- `edit`: Precise text replacement in files

### Codebase Navigation
- `grep_search`: Search file contents
- `glob`: Find files by pattern
- `list_directory`: List directory contents

### Execution
- `run_shell_command`: Execute shell commands
- `web_search`: Search the web for information

### Coordination
- `task`: Delegate to specialized subagents

## Communication Templates

### Blocking Issue Creation

```bash
gh issue create \
  --title "Blocking: [brief description]" \
  --body "## Problem
[Detailed description of the blocking issue]

## Context
- Blocked ticket: #[ticket-number]
- Agent: [agent-name]
- Required decision: [what decision is needed]

## Options
- Option 1: [description]
- Option 2: [description]

**Requires**: [coordinator] to communicate with user" \
  --label "blocking" \
  --label "user-input-required" \
  --label "[specialty]"
```

### Completion Notification

```bash
gh issue comment [ticket-number] \
  --body "✅ Complete

[Summary of work done]

**Files changed**:
- [file1]
- [file2]

**Next steps**: [what should happen next]"
```

### Progress Update

```bash
gh issue comment [ticket-number] \
  --body "📊 Progress Update

**Current status**: [status]

**Completed**:
- [task 1]
- [task 2]

**In progress**:
- [task 3]

**Next**: [next task]

**ETA**: [estimated completion time]"
```
