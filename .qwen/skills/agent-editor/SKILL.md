---
name: agent-editor
description: Creates and manages Qwen subagent configurations. Use when designing new agents, updating agent behavior, or managing multi-agent workflows.
---

# Agent Editor Skill

This skill helps translate user requirements into Qwen subagent configurations, managing both individual agent prompts and multi-agent interaction workflows.

## Purpose

The agent-editor skill is designed to:
- Create new subagent configurations from user requirements
- Update existing subagent prompts and behaviors
- Design and manage interactions between multiple agents
- Ensure consistency across agent specializations
- Follow Qwen subagent specification format

## Usage

When you need to:
- Create a new specialized agent for any domain
- Modify existing agent behavior or capabilities
- Design multi-agent coordination workflows
- Translate user requirements into agent specifications
- Ensure agents follow best practices and patterns

## Agent Configuration Format

Subagents follow the Qwen specification format:

```
---
name: agent-name
description: Brief description of when and how to use this agent. MUST BE USED for [specific tasks].
tools:
  - tool1
  - tool2
---

System prompt content goes here.
Multiple paragraphs are supported.
You can use ${variable} templating for dynamic content.
```

## Key Principles

### 1. Single Responsibility Principle
Each agent should have a clear, focused purpose.

**✅ Good:**
```
---
name: testing-expert
description: Writes comprehensive unit tests and integration tests
---
```

**❌ Avoid:**
```
---
name: general-helper
description: Helps with testing, documentation, code review, and deployment
---
```

**Why:** Focused agents produce better results and are easier to maintain.

### 2. Clear Specialization
Define specific expertise areas rather than broad capabilities.

**✅ Good:**
```
---
name: react-performance-optimizer
description: Optimizes React applications for performance using profiling and best practices
---
```

**❌ Avoid:**
```
---
name: frontend-developer
description: Works on frontend development tasks
---
```

**Why:** Specific expertise leads to more targeted and effective assistance.

### 3. Actionable Descriptions
Write descriptions that clearly indicate when to use the agent.

**✅ Good:**
```
description: Reviews code for security vulnerabilities, performance issues, and maintainability concerns
```

**❌ Avoid:**
```
description: A helpful code reviewer
```

**Why:** Clear descriptions help the main AI choose the right agent for each task.

### 4. Multi-Agent Coordination Design
Consider how agents interact and communicate with each other.

**Key Questions to Ask:**
- Should this agent interact directly with the user?
- Should it communicate through another agent (coordinator pattern)?
- Should it use file-based communication (e.g., GitHub issues, shared files)?
- What escalation protocol should be used for blocking issues?
- Are there specific labels, files, or channels for communication?

### 5. Consistency Patterns
Maintain consistent patterns across all agents in the system.

**Common Patterns:**
- **Coordinator Pattern**: One agent manages workflow and delegates to specialists
- **Direct Access Pattern**: Some agents may have direct user access for specific scenarios
- **File-Based Communication**: Agents communicate through shared files or issue trackers
- **Label-Based Escalation**: Use labels (e.g., `needs-review`, `blocking`) for escalation instead of @mentions

## Agent Interaction Patterns

### Pattern 1: Coordinator Pattern
One agent serves as the primary coordinator:
- Coordinator manages workflow and delegates tasks
- Specialists focus on their specific domains
- Communication flows through the coordinator
- Coordinator handles user interactions

**Use when:**
- Complex workflows require orchestration
- Multiple specialists need coordination
- Centralized decision-making is needed

### Pattern 2: Direct User Access
Some agents may interact directly with users:
- Typically for specialized domains (e.g., design decisions, content creation)
- May pause other work during direct sessions
- Clear boundaries on when direct access is appropriate

**Use when:**
- Domain expertise requires direct user consultation
- Decisions need immediate user feedback
- Specialized knowledge can't be effectively delegated

### Pattern 3: File-Based Communication
Agents communicate through shared artifacts:
- GitHub issues, labels, and comments
- Shared configuration files
- Status files or manifests
- Database or API endpoints

**Use when:**
- Asynchronous communication is preferred
- Audit trail is important
- Multiple agents need to monitor the same information

### Pattern 4: Escalation Protocol
Define how agents handle blocking issues:
- Create blocking tickets/issues with appropriate labels
- Notify coordinator or relevant agent through designated channel
- Never wait silently for user input
- Provide clear context and required decisions

**Common Escalation Labels:**
- `needs-review` - Requires coordinator attention
- `user-input-required` - Needs user decision
- `blocking` - Cannot proceed without resolution
- `[specialty]-review` - Needs specialist review

## Common Agent Types (Examples)

### Coordinator Agents
- **project-manager**: Primary workflow coordinator
- **technical-lead**: Technical planning and task breakdown

### Implementation Agents  
- **software-engineer**: Code implementation
- **design-specialist**: UI/UX design
- **content-creator**: Narrative or documentation content

### Support Agents
- **qa-tester**: Quality assurance and testing
- **build-deployer**: Deployment and infrastructure management
- **performance-analyst**: Performance optimization and monitoring

### Design/Planning Agents
- **architect-designer**: Major design decisions and specifications

**Note:** These are examples from a game development context. The agent-editor skill works for ANY domain.

## Workflow Management

### Communication Channels

**GitHub Integration (Example):**
- Master backlog (`master-backlog` label)
- Epics (`epic` label)
- Sub-tickets (specialty labels)
- Blocking issues (`blocking`, `user-input-required`, `needs-review`)

**File-Based Communication:**
- Status files in `.qwen/status/`
- Configuration files in `.qwen/config/`
- Shared manifests or task lists

**Direct Agent Communication:**
- Comments on shared artifacts
- Status updates in designated locations
- Clear escalation paths

### Agent Communication Best Practices

1. **Use labels or files for escalation**, not @mentions (unless your system supports them)
2. **Comment on relevant artifacts** (master backlog, tickets, status files)
3. **Maintain clear audit trail** of decisions and communications
4. **Define escalation paths** for each type of blocking issue
5. **Never wait silently** - always create a blocking artifact

## Best Practices

### System Prompt Guidelines

**Be Specific About Expertise:**
```
You are a Python testing specialist with expertise in:

- pytest framework and fixtures
- Mock objects and dependency injection
- Test-driven development practices
- Performance testing with pytest-benchmark
```

**Include Step-by-Step Approaches:**
```
For each testing task:

1. Analyze the code structure and dependencies
2. Identify key functionality and edge cases
3. Create comprehensive test suites with clear naming
4. Include setup/teardown and proper assertions
5. Add comments explaining complex test scenarios
```

**Specify Output Standards:**
```
Always follow these standards:

- Use descriptive test names that explain the scenario
- Include both positive and negative test cases
- Add docstrings for complex test functions
- Ensure tests are independent and can run in any order
```

### Tool Selection

**Choose tools based on agent responsibilities:**
- `read_file`, `write_file`: For file manipulation
- `read_many_files`: For analyzing codebases
- `run_shell_command`: For executing build/test/deploy commands
- `web_search`: For research and documentation
- `grep_search`, `glob`: For codebase navigation
- `edit`: For precise code modifications

**Security Considerations:**
- Only grant necessary tools
- Avoid sensitive operations unless required
- Consider sandboxing for production environments

### Escalation Design Questions

When designing agent interactions, ask:

1. **User Interaction:**
   - Should this agent interact directly with the user?
   - If not, what is the escalation path?
   - How does the user provide input/decisions?

2. **Agent Coordination:**
   - Is there a coordinator agent?
   - How do agents communicate with each other?
   - What happens when multiple agents need to collaborate?

3. **Blocking Issues:**
   - How are blocking issues identified?
   - What labels or markers are used?
   - Who gets notified and how?

4. **Workflow State:**
   - How is workflow state tracked?
   - Where are task assignments recorded?
   - How are completions reported?

## File Locations

- Project agents: `.qwen/agents/`
- User agents: `~/.qwen/agents/`
- This skill: `.qwen/skills/agent-editor/`

## Examples

### Example 1: Creating a New Agent

**User Request:**
"I need an agent that can optimize database queries"

**Agent Editor Actions:**
1. Ask clarifying questions:
   - What database type? (PostgreSQL, MySQL, etc.)
   - Should it interact with user directly or through coordinator?
   - What tools does it need? (SQL analysis, performance testing, etc.)
   - How should blocking issues be escalated?

2. Create agent configuration:
```yaml
---
name: database-optimizer
description: Analyzes and optimizes database queries for performance. MUST BE USED for all database performance optimization tasks.
tools:
  - read_file
  - write_file
  - run_shell_command
  - web_search
---

You are a database performance optimization specialist...
```

3. Define escalation protocol:
- Blocking issues → create issue with `needs-review` label
- Performance decisions → escalate to technical-lead
- Schema changes → require user approval via coordinator

### Example 2: Updating Agent Interactions

**User Request:**
"Change how the QA agent escalates issues"

**Agent Editor Actions:**
1. Read current QA agent configuration
2. Understand current escalation method
3. Ask user about desired changes:
   - Should it use labels, files, or direct communication?
   - Who should be notified?
   - What information should be included?

4. Update escalation protocol:
```bash
# Old way (using @mentions)
gh issue comment [ticket] --body "@coordinator QA blocked..."

# New way (using labels)
gh issue edit [blocking-issue] --add-label "needs-review"
```

5. Update coordinator agent to check for new labels
6. Ensure consistency across all agents
7. Save updated configurations

### Example 3: Designing Multi-Agent Workflow

**User Request:**
"I want to create a documentation pipeline with multiple agents"

**Agent Editor Actions:**
1. Design agent roles:
   - `doc-analyzer`: Analyzes code and extracts API information
   - `doc-writer`: Creates documentation from extracted information
   - `doc-reviewer`: Reviews and validates documentation quality

2. Design communication flow:
   ```
   doc-analyzer → extracts info → saves to .qwen/docs/api-spec.json
   doc-writer → reads spec → generates docs → saves to docs/
   doc-reviewer → reviews docs → creates issues for improvements
   ```

3. Define escalation paths:
   - Missing information → `needs-specification` label
   - Quality issues → `needs-review` label
   - User decisions → `user-input-required` label

4. Create all three agent configurations
5. Ensure consistent patterns across agents
6. Document the workflow in a README

## References

- Qwen Subagent Specification: `/home/yehan/GitRepos/qwen-code/docs/users/features/sub-agents.md`
- Example Agent Roster: `/home/yehan/GitRepos/rust-cowboyz/agent-design/README.md`

## Helper Scripts

This skill includes helper scripts for agent management:

```bash
# List all agents
./.qwen/skills/agent-editor/scripts/helper.sh list

# Validate agent files
./.qwen/skills/agent-editor/scripts/helper.sh validate <file>
./.qwen/skills/agent-editor/scripts/helper.sh validate-all

# Check consistency
./.qwen/skills/agent-editor/scripts/helper.sh check-consistency

# Update patterns across all agents
./.qwen/skills/agent-editor/scripts/helper.sh update-pattern "old" "new"

# Show statistics
./.qwen/skills/agent-editor/scripts/helper.sh stats
```
