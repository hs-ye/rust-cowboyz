## GitHub Configuration for Qwen Agents

This file documents the GitHub-specific configurations that need to be customized for your project. This project is designed to be used by a set of LLM powered agents that will act as a software engineering team on GitHub, following the conventions listed in this file.

### Fixed Issue Labels (Hardcoded)
These labels are fixed across all projects and should not be changed:
- `master-backlog` - Used by project-manager to identify the master backlog issue
- `project-manager-review` - Issues requiring project-manager attention
- `user-input-required` - Issues that require user decisions
- `blocking` - Used for blocking issues that prevent progress
- `backend` - For backend tasks, assigned to software-engineer
- `frontend` - For frontend tasks, assigned to software-engineer
- `ui` - For UI tasks, assigned to design-specialist
- `api` - For API tasks, assigned to software-engineer
- `dependencies` - For dependency tasks, assigned to software-engineer
- `epic` - For epic-level issues
- `tech-lead-review` - For technical lead review
- `qa` - For tickets needing QA-tester attention. Do not use for unit/local tests
- `bug` - For bug reports
- `done` - For completed issues

### Project Board Configuration (Variable per project)
These configurations may vary between different projects:

- `[PROJECT_BOARD_FIELD_ID]` - Project board field ID for status tracking (example default: `PVTSSF_lAHOAHpRbM4BPxw-zg-FswM`)
- `[PROJECT_ID]` - Project ID (example default: `1`)
- `[READY_STATUS_OPTION_ID]` - 'Ready' status option ID (example default: `61e4505c`)
- `[IN_PROGRESS_STATUS_OPTION_ID]` - 'In progress' status option ID (example default: `47fc9ee4`)
- `[DONE_STATUS_OPTION_ID]` - 'Done' status option ID (example default: `98236657`)

### Directory Structure
- ADRs (Architectural Decision Records) are stored in: `/design/adr/`

### Git Workflow
- Feature branch naming convention: `feat/[ticket number] - [ticket name]`
- Branch for development work should be based on master branch

### Configuration Instructions

#### 1. Find Your GitHub Project Board Configuration Values

Run these commands to find the necessary IDs for your project:

1. **Get your project ID**:
   ```bash
   gh project list --format json
   ```
   Or visit your project board in the browser and note the number in the URL.

2. **Get your project board field ID for status tracking**:
   ```bash
   gh project field-list --format json --project-id [YOUR_PROJECT_ID]
   ```
   Look for the field named "Status" or similar.

3. **Get the status option IDs**:
   ```bash
   gh project field-value-list --format json --project-id [YOUR_PROJECT_ID] --field-status
   ```
   This will show you the IDs for "Ready", "In progress", "Done", etc. status options.

#### 2. Add Configuration to Your qwen.md File

Add this section to your project's qwen.md file:

```markdown
## GitHub Project Board Configuration

### Project Board Configuration
- `PROJECT_BOARD_FIELD_ID`: [Your status field ID from step 2]
- `PROJECT_ID`: [Your project ID from step 1]
- `READY_STATUS_OPTION_ID`: [ID for 'Ready' status from step 3]
- `IN_PROGRESS_STATUS_OPTION_ID`: [ID for 'In progress' status from step 3]
- `DONE_STATUS_OPTION_ID`: [ID for 'Done' status from step 3]
```

#### 3. Example Configuration

```markdown
## GitHub Project Board Configuration

### Project Board Configuration
- `PROJECT_BOARD_FIELD_ID`: PVTSSF_lAHOAHpRbM4BPxw-zg-FswM
- `PROJECT_ID`: 1
- `READY_STATUS_OPTION_ID`: 61e4505c
- `IN_PROGRESS_STATUS_OPTION_ID`: 47fc9ee4
- `DONE_STATUS_OPTION_ID`: 98236657
```

The Qwen agents will reference these values when interacting with your GitHub project board. The issue labels are standardized across all projects and do not need to be configured.