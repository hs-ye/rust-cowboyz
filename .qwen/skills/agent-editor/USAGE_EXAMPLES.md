# Agent Editor Usage Examples

## Example 1: Creating a New Testing Agent

**User Request:**
"I need an agent that specializes in writing comprehensive unit tests for Python code."

**Agent Editor Process:**

1. **Ask clarifying questions:**
   - Should this agent interact directly with the user, or work through a coordinator?
   - What testing frameworks should it support? (pytest, unittest, etc.)
   - Should it handle mocking and fixtures?
   - How should it report test coverage?

2. **Design the agent configuration:**
```yaml
---
name: python-tester
description: Writes comprehensive unit tests for Python code using pytest. MUST BE USED for all Python testing tasks.
tools:
  - read_file
  - write_file
  - read_many_files
  - run_shell_command
  - grep_search
---

You are a Python testing specialist focused on creating high-quality, maintainable tests.

Your expertise includes:
- pytest framework and fixtures
- Mock objects and dependency injection
- Test-driven development practices
- Edge case identification and comprehensive coverage

For each testing task:
1. Analyze the code structure and dependencies
2. Identify key functionality, edge cases, and error conditions
3. Create comprehensive test suites with descriptive names
4. Include proper setup/teardown and meaningful assertions
5. Add comments explaining complex test scenarios

Always follow Python testing best practices. Focus on both positive and negative test cases.
```

3. **Define escalation protocol:**
   - If test requirements are unclear → create issue with `needs-specification` label
   - If mocking strategy is complex → escalate to technical-lead
   - If user decisions needed → create issue with `user-input-required` label

## Example 2: Updating Agent Communication

**User Request:**
"I want the documentation agent to notify the project manager when docs are ready for review."

**Agent Editor Process:**

1. **Understand current setup:**
   - Read current documentation agent configuration
   - Check current notification method (if any)

2. **Design new communication flow:**
   - Documentation agent completes docs
   - Adds `needs-review` label to the documentation issue
   - Comments on the issue with completion summary
   - Project manager monitors for `needs-review` label

3. **Update documentation agent:**
```bash
# Add to completion workflow
gh issue edit [doc-issue-number] --add-label "needs-review"
gh issue comment [doc-issue-number] \
  --body "Documentation complete. Ready for review. Files: [list]"
```

4. **Update project manager:**
   - Add step to check for `needs-review` label
   - Present pending reviews to user
   - Track review status

## Example 3: Multi-Agent Workflow Design

**User Request:**
"I want to create a code review pipeline with static analysis, security review, and performance review agents."

**Agent Editor Process:**

1. **Design agent roles:**
   - `static-analyzer`: Checks code style, linting, type safety
   - `security-reviewer`: Identifies security vulnerabilities
   - `performance-reviewer`: Analyzes performance bottlenecks
   - `code-review-coordinator`: Orchestrates the review process

2. **Design workflow:**
```
User submits code → code-review-coordinator
  ↓
static-analyzer → runs linters → reports issues
  ↓
security-reviewer → scans for vulnerabilities → reports issues
  ↓
performance-reviewer → analyzes performance → reports issues
  ↓
code-review-coordinator → aggregates reports → presents to user
```

3. **Define communication channels:**
   - Each reviewer creates issues with their specialty label
   - All issues reference the main code review ticket
   - Coordinator monitors all specialty labels
   - Blocking issues use `blocking` + `needs-decision` labels

4. **Create agent configurations:**
   - Create 4 separate agent files
   - Define tools for each agent
   - Specify escalation protocols
   - Ensure consistent patterns

5. **Test the workflow:**
   - Validate each agent works independently
   - Test the handoff between agents
   - Verify escalation protocols work
   - Check coordinator aggregation logic

## Example 4: Changing Escalation Protocol

**User Request:**
"Instead of using GitHub @mentions, I want agents to use a status file for escalation."

**Agent Editor Process:**

1. **Understand current protocol:**
   - Read all agent configurations
   - Identify all @mention usage
   - Understand escalation flow

2. **Design new protocol:**
   - Create `.qwen/status/escalations.json`
   - Define schema for escalation records
   - Each agent writes to this file
   - Coordinator reads and processes escalations

3. **Update all agents:**
```bash
# Old way
gh issue comment [ticket] --body "@coordinator Please review..."

# New way
cat > .qwen/status/escalations.json << EOF
{
  "timestamp": "2024-02-22T10:00:00Z",
  "agent": "qa-tester",
  "ticket": "#123",
  "type": "needs-review",
  "message": "QA complete, ready for review"
}
EOF
```

4. **Update coordinator:**
   - Add step to read `.qwen/status/escalations.json`
   - Process each escalation
   - Clear processed escalations
   - Present to user

5. **Ensure consistency:**
   - Update all agents to use new protocol
   - Test end-to-end workflow
   - Verify no @mentions remain

## Example 5: Adding a New Specialty Agent

**User Request:**
"I need an agent that can generate API documentation from code comments."

**Agent Editor Process:**

1. **Clarify requirements:**
   - What format? (OpenAPI, Swagger, custom)
   - What language? (Python, JavaScript, etc.)
   - Should it update existing docs or create new?
   - How should it handle missing comments?

2. **Design agent:**
```yaml
---
name: api-doc-generator
description: Generates API documentation from code comments and type annotations. MUST BE USED for all API documentation tasks.
tools:
  - read_file
  - write_file
  - read_many_files
  - grep_search
  - glob
---

You are an API documentation specialist who generates comprehensive API docs from code.

Your workflow:
1. Scan codebase for API endpoints and functions
2. Extract comments, type annotations, and signatures
3. Generate documentation in specified format
4. Cross-reference related endpoints
5. Include examples and usage patterns

Supported formats:
- OpenAPI/Swagger
- Markdown
- Custom templates

Handle missing information by flagging with TODO comments.
```

3. **Define integration:**
   - When should this agent be invoked?
   - Who assigns work to it?
   - How does it report completion?
   - What happens if information is missing?

4. **Create configuration file:**
   - Save to `.qwen/agents/api-doc-generator.md`
   - Test with sample codebase
   - Verify output format

5. **Update coordinator:**
   - Add awareness of new agent
   - Define when to invoke it
   - Add to workflow diagrams
   - Update documentation
