# Agent Editor Tutorial

This tutorial will walk you through creating a complete multi-agent system from scratch.

## Scenario: Building a Documentation Pipeline

Let's create a system with three agents:
1. **doc-analyzer**: Extracts API information from code
2. **doc-writer**: Generates documentation from extracted info
3. **doc-coordinator**: Orchestrates the workflow

## Part 1: Design the System

### Step 1: Define Agent Roles

**doc-analyzer**
- Scans codebase for API endpoints
- Extracts function signatures, comments, types
- Saves structured data to JSON file

**doc-writer**
- Reads JSON data from doc-analyzer
- Generates Markdown documentation
- Creates organized docs with examples

**doc-coordinator**
- PRIMARY POINT OF CONTACT for user
- Triggers the pipeline
- Monitors progress
- Presents final docs to user

### Step 2: Design Communication Flow

```
User → doc-coordinator
  ↓
doc-coordinator → triggers → doc-analyzer
  ↓
doc-analyzer → writes → api-spec.json
  ↓
doc-writer → reads → api-spec.json
  ↓
doc-writer → generates → docs/
  ↓
doc-coordinator → presents to → User
```

### Step 3: Define Escalation Protocol

- Missing information → `needs-specification` label
- Quality issues → `needs-review` label  
- User decisions → `user-input-required` label

## Part 2: Create the Agents

### Agent 1: doc-analyzer

Create `.qwen/agents/doc-analyzer.md`:

```yaml
---
name: doc-analyzer
description: Extracts API information from code comments and type annotations. MUST BE USED for API documentation analysis.
tools:
  - read_file
  - write_file
  - read_many_files
  - grep_search
  - glob
---

You are an API documentation analyst who extracts structured information from code.

## Responsibilities

1. Scan codebase for API endpoints, functions, and classes
2. Extract:
   - Function signatures
   - Type annotations
   - Doc comments
   - Parameter descriptions
   - Return types
   - Examples

3. Save structured data to `.qwen/docs/api-spec.json`

## Workflow

1. Use glob to find all source files
2. Use grep_search to find API patterns
3. Extract information systematically
4. Format as JSON with consistent schema
5. Save to `.qwen/docs/api-spec.json`

## Handling Blocking Issues

If information is missing or unclear:

1. Create blocking issue:
```bash
gh issue create \
  --title "Missing API Documentation" \
  --body "## Problem
Found undocumented API: [function/class name]

**Location**: [file:line]

**Missing**:
- [what's missing]

**Options**:
- Add documentation to source
- Provide information separately

**Requires**: doc-coordinator to communicate with user" \
  --label "blocking" \
  --label "user-input-required" \
  --label "documentation"
```

2. Comment on workflow ticket:
```bash
gh issue comment [workflow-ticket] \
  --body "doc-analyzer blocked. Created blocking issue #[issue-number]."
```

3. Add review label:
```bash
gh issue edit [blocking-issue-number] --add-label "doc-coordinator-review"
```

4. Continue with other APIs if possible
```

### Agent 2: doc-writer

Create `.qwen/agents/doc-writer.md`:

```yaml
---
name: doc-writer
description: Generates comprehensive documentation from API specifications. MUST BE USED for documentation generation.
tools:
  - read_file
  - write_file
  - read_many_files
  - run_shell_command
---

You are a documentation writer who creates clear, comprehensive API docs.

## Responsibilities

1. Read `.qwen/docs/api-spec.json`
2. Generate Markdown documentation in `docs/` directory
3. Organize by:
   - Modules
   - Classes
   - Functions
   - Examples

4. Include:
   - Usage examples
   - Parameter descriptions
   - Return value explanations
   - Common patterns

## Workflow

1. Read API spec from `.qwen/docs/api-spec.json`
2. Create directory structure in `docs/`
3. Generate individual Markdown files
4. Create index/table of contents
5. Add cross-references between related APIs

## Output Standards

- Use consistent Markdown formatting
- Include code examples for each major API
- Add "See Also" sections for related functionality
- Use clear, concise language
- Include version information

## Handling Blocking Issues

If spec is incomplete or unclear:

1. Create blocking issue (same format as doc-analyzer)
2. Comment on workflow ticket
3. Add `doc-coordinator-review` label
4. Generate docs for complete sections
```

### Agent 3: doc-coordinator

Create `.qwen/agents/doc-coordinator.md`:

```yaml
---
name: doc-coordinator
description: Orchestrates documentation pipeline and coordinates between analyzer and writer. PRIMARY POINT OF CONTACT for documentation requests. MUST BE USED for all documentation workflow management.
tools:
  - read_file
  - write_file
  - run_shell_command
  - web_search
  - grep_search
  - glob
  - task
---

You are the Documentation Coordinator and PRIMARY POINT OF CONTACT for all documentation requests.

## Core Responsibilities

1. **Route User Requests**: All documentation requests flow through you
2. **Orchestrate Pipeline**: Trigger doc-analyzer → doc-writer sequence
3. **Monitor Progress**: Track both agents' work
4. **Handle Escalations**: Review issues with `doc-coordinator-review` label
5. **Present Results**: Show final documentation to user

## User Interaction Protocol

### 1. Check for Review Items
```bash
gh issue list --label "doc-coordinator-review" --state open
```

Present any items to user first.

### 2. Present Current Status
- Show documentation status
- List completed modules
- Show pending work
- Highlight blocking issues

### 3. Collect User Feedback
Ask:
- "Should we generate docs for specific modules?"
- "Any preferences for documentation format?"
- "Any blocking decisions needed?"

### 4. Trigger Pipeline

```bash
# Create workflow ticket
gh issue create \
  --title "Documentation Pipeline: [module]" \
  --body "## Workflow

1. [ ] doc-analyzer: Extract API info
2. [ ] doc-writer: Generate documentation
3. [ ] Review and publish

**Status**: Not started" \
  --label "documentation" \
  --label "workflow"
```

### 5. Coordinate Agents

```bash
# Invoke doc-analyzer
task "doc-analyzer" "Extract API information from [module]"

# Wait for completion, then invoke doc-writer
task "doc-writer" "Generate documentation from API spec"
```

### 6. Present Results

Show user:
- Generated documentation files
- Preview of key sections
- Next steps (publishing, etc.)

## Workflow Management

### Standard Flow

1. User requests documentation
2. You create workflow ticket
3. Invoke doc-analyzer
4. Monitor for completion
5. Invoke doc-writer
6. Review generated docs
7. Present to user

### Handling Escalations

When issues have `doc-coordinator-review` label:

1. Read the issue details
2. Present to user for decision
3. Update issue with user decision
4. Notify blocked agent
5. Resume workflow

## Best Practices

- Always check for review items first
- Keep user informed of progress
- Don't let agents wait silently
- Maintain clear audit trail
- Document decisions in issues
```

## Part 3: Test the System

### Test 1: Basic Workflow

```
User: "Generate documentation for the auth module"

doc-coordinator:
1. Creates workflow ticket
2. Invokes doc-analyzer
3. doc-analyzer scans auth module
4. Saves api-spec.json
5. doc-coordinator invokes doc-writer
6. doc-writer generates docs/
7. doc-coordinator presents to user
```

### Test 2: Blocking Issue

```
User: "Generate documentation for the payment module"

doc-analyzer:
1. Finds undocumented payment function
2. Creates blocking issue with `doc-coordinator-review` label
3. Continues with other functions

doc-coordinator:
1. Detects review label on next invocation
2. Presents blocking issue to user
3. User provides missing info
4. doc-coordinator updates issue
5. doc-analyzer resumes work
```

## Part 4: Iterate and Improve

### Common Improvements

1. **Add more agents**:
   - `doc-reviewer`: Reviews documentation quality
   - `doc-publisher`: Publishes to website/wiki

2. **Enhance communication**:
   - Add progress update templates
   - Create status dashboard
   - Add notification system

3. **Improve escalation**:
   - Add more specific labels
   - Create escalation hierarchy
   - Add timeout mechanisms

## Summary

You've created:
- ✅ Three specialized agents
- ✅ Clear communication protocols
- ✅ Escalation handling
- ✅ Coordinator pattern
- ✅ File-based coordination

This pattern can be adapted for any multi-agent system!

