# Game Development Studio Agent Roster

This directory contains the subagent profiles for our autonomous indie game studio. Each agent specializes in a specific aspect of game development and works under the coordination of the project-manager.

## Agent Hierarchy

### Primary Coordinators
- **project-manager**: The PRIMARY POINT OF CONTACT for all user interactions. Orchestrates workflow, manages GitHub tickets, maintains the master backlog, and coordinates all other agents.

- **architect-designer**: The SECOND POINT OF CONTACT for user interactions. Works directly with users for major design decisions. Creates detailed game design specifications following ADR patterns and saves decisions to `/design/adr/` folder.

### Design and Planning
- **technical-lead**: Translates ADRs into actionable GitHub tickets. Breaks down epics into sub-tickets, assigns labels for specialist routing, and manages technical implementation planning.

### Implementation
- **software-engineer**: Implements game features, fixes bugs, and writes production-ready code based on GitHub tickets. Works with Rust, WebAssembly, and full-stack development.

- **design-specialist**: Designs and implements user interfaces using Yew framework. Creates wireframes, ensures accessibility, and optimizes user experience for web-based interfaces.

### Quality Assurance
- **qa-tester**: Tests game functionality, identifies bugs, creates test plans, and verifies implementations against acceptance criteria. Ensures quality before deployment.

### Infrastructure and Performance
- **build-deployer**: Manages builds, deployment pipelines, CI/CD automation, and release management. Handles infrastructure configuration and deployment to production.

- **systems-analyst**: Analyzes performance, identifies optimization opportunities, monitors system health, and suggests architectural improvements. Focuses on scalability and efficiency.

### Content Creation
- **creative-lead**: Creates storylines, character backgrounds, dialogue, lore, and quest content. Develops the game's narrative world and characters.

## Workflow Overview

### Standard Development Flow

1. **User Request** → User communicates with **project-manager** (PRIMARY POINT OF CONTACT)
2. **Backlog Management** → **project-manager** adds to master backlog epic
3. **Assess if Architect Needed** → **project-manager** determines if major design decisions are required
4. **If Architect Needed** → **project-manager** pauses all work, invokes **architect-designer**, waits for ADR completion
5. **Design Phase** → **architect-designer** works directly with user to create/update ADRs (SECOND POINT OF CONTACT)
6. **Ticket Creation** → **project-manager** invokes **technical-lead** to break down ADRs into GitHub tickets
7. **Implementation** → **project-manager** assigns tickets to appropriate specialists:
   - Tickets in 'Ready' status → **software-engineer** 
   - UI/UX work → **design-specialist**
8. **Ready for QA** → **software-engineer** moves completed tickets to 'In review' status on project board (does not pass directly to QA)
9. **Testing** → **qa-tester** picks up tickets in 'In review' status on project board for testing
10. **Deployment** → **project-manager** coordinates with **build-deployer** for publishing
11. **Optimization** → **project-manager** assigns performance tickets to **systems-analyst**
12. **Content** → **project-manager** assigns narrative tickets to **creative-lead**

### When Architect-Designer is Required

The **project-manager** recognizes when **architect-designer** consultation is needed:

**Scenarios:**
1. User explicitly requests design consultation
2. Technical-lead escalates a major design decision
3. New feature requires fundamental game mechanics design
4. Existing design needs significant revision
5. Conflicting requirements need architectural resolution

**Process:**
1. **project-manager** pauses all other agent work
2. **project-manager** invokes **architect-designer** with specific design requirements
3. **architect-designer** works directly with user to make design decisions
4. **architect-designer** creates/updates ADRs in `/design/adr/`
5. **architect-designer** notifies **project-manager** when complete
6. **project-manager** resumes other agent work based on new ADRs

### Blocking Issue Protocol

When any agent encounters a blocking issue:

#### For User Input Required:
1. Agent creates a GitHub issue with labels: `blocking`, `user-input-required`, and relevant specialty label
2. Agent comments on the blocked ticket referencing the blocking issue
3. Agent notifies **project-manager** via comment on master backlog
4. **project-manager** communicates with user and collects decision
5. **project-manager** updates blocking issue with user decision
6. Original agent resumes work

#### For Technical Issues (Software Engineer):
When the **software-engineer** encounters low-level technical issues such as merge conflicts, unclear acceptance criteria, or instructions leading to major design changes:
1. Write concerns in code as comments
2. Commit the changes with detailed explanation
3. Tag the ticket with `tech-lead-review` label
4. Continue with other non-blocked tickets
5. Wait for **technical-lead** to address concerns before resuming work on this ticket

**Critical Rule**: No agent should ever wait silently for user input. All user communication flows through **project-manager** (except major design decisions which go through **architect-designer**).

## GitHub Issue Structure

### Master Backlog
- **Label**: `master-backlog`
- **Purpose**: Single source of truth for project priorities
- **Content**: Ordered list of epics with status

### Epics
- **Label**: `epic`
- **Purpose**: High-level tracking tickets for major features
- **Content**: Scope description, sub-tickets list, acceptance criteria

### Sub-Tickets
- **Labels**: `backend-swe`, `frontend-swe`, `ui-design`, `optimization`, `qa-testing`, `architect` etc.
- **Purpose**: Atomic, actionable tasks for specialists
- **Content**: Detailed description, acceptance criteria, dependencies, technical notes

### Blocking Issues
- **Labels**: `blocking`, `user-input-required`, `[specialty]`
- **Purpose**: Flag issues requiring user decisions
- **Content**: Problem description, options, impact, required decision

## Agent Communication Protocol

### With Project-Manager
- Report progress regularly
- Flag blocking issues immediately
- Request clarification on requirements
- Provide status updates and estimates
- **project-manager is PRIMARY POINT OF CONTACT for all agents**

### With Architect-Designer
- **architect-designer is SECOND POINT OF CONTACT for major design decisions only**
- Technical-lead escalates design clarifications to architect-designer
- All other communication flows through project-manager

### With Other Agents
- Collaborate on shared work
- Provide feedback and reviews
- Coordinate on integration points
- Share knowledge and best practices

### With User
- **project-manager**: PRIMARY POINT OF CONTACT for all user interactions
- **architect-designer**: SECOND POINT OF CONTACT for major design decisions only
- **All other agents**: NEVER communicate directly with user
- All user communication flows through project-manager (except major design decisions)

## File Organization

```
agent-design/
├── README.md                          # This file
├── project-manager.md                 # Primary coordinator agent
├── architect-designer.md              # Design specification agent (direct user access)
├── technical-lead.md                  # Ticket creation agent
├── software-engineer.md               # Code implementation agent
├── design-specialist.md               # UI/UX design agent
├── qa-tester.md                       # Quality assurance agent
├── build-deployer.md                  # Build/deployment agent
├── systems-analyst.md                 # Performance analysis agent
└── creative-lead.md                   # Narrative content agent

/design/adr/                           # ADRs created by architect-designer
├── 0001-initial-game-concept.md
├── 0002-economy-system.md
└── ...

data/narrative/                        # Narrative content created by creative-lead
├── characters/
├── dialogue/
├── quests/
├── lore/
└── stories/
```

## Getting Started

1. **Install Agents**: Copy agent profiles from `agent-design/` to `~/.qwen/agents/` or `.qwen/agents/` in your project
2. **Initialize Master Backlog**: Create the master backlog issue on GitHub
3. **Start Development**: Communicate with **project-manager** to begin work

## Best Practices

1. **project-manager is PRIMARY POINT OF CONTACT**: All user communication flows through project-manager
2. **architect-designer is SECOND POINT OF CONTACT**: Only for major design decisions
3. **Create blocking tickets promptly**: Don't wait silently when decisions are needed
4. **Maintain clear ticket hierarchy**: Epics → Sub-tickets → Implementation
5. **Follow ADR patterns**: All design decisions should be documented in `/design/adr/`
6. **Keep agents specialized**: Each agent should focus on their area of expertise
7. **Communicate proactively**: Report status and flag issues early
8. **Document everything**: Maintain clear records of decisions and changes

## Critical Communication Rules

1. **project-manager**: PRIMARY POINT OF CONTACT for ALL user interactions
2. **architect-designer**: SECOND POINT OF CONTACT for MAJOR DESIGN DECISIONS ONLY
3. **All other agents**: NEVER communicate directly with user
4. **When architect-designer is invoked**: project-manager pauses all other work
5. **After architect-designer completes**: project-manager resumes work based on ADRs
6. **Blocking issues**: Create tickets, notify project-manager, never wait silently

## System Interaction Flowchart

```mermaid
flowchart TD
    User[👤 User] -->|1. Requests feature/feedback| PM[project-manager]
    PM -->|2. Adds to| MB[📝 Master Backlog]
    MB -->|3. Contains| EPICS[📋 List of Epics]
    
    PM -->|4. Assesses if architect needed| DECISION{Major Design\nDecision?}
    
    DECISION -->|Yes| PAUSE[⏸️ Pause All Work]
    PAUSE -->|5. Invokes| AD[architect-designer]
    AD -->|6. Works directly with| User
    AD -->|7. Creates/Updates| ADR[📄 ADRs\n/design/adr/]
    ADR -->|8. Notifies completion| PM
    PM -->|9. Resumes work| TECH[technical-lead]
    
    DECISION -->|No| TECH
    
    TECH -->|10. Reads| ADR
    TECH -->|11. Breaks down into| TICKETS[🎫 GitHub Tickets\nSub-tickets]
    TICKETS -->|12. Assigns| SE[software-engineer]
    TICKETS -->|12. Assigns| DS[design-specialist]
    TICKETS -->|12. Assigns| CL[creative-lead]
    
    SE -->|13. Implements| CODE[💾 Codebase\nsrc/]
    DS -->|13. Implements UI| CODE
    CL -->|13. Adds content| CODE
    
    SE -->|14. Creates blocking ticket| BLOCK[⚠️ Blocking Issue]
    DS -->|14. Creates blocking ticket| BLOCK
    CL -->|14. Creates blocking ticket| BLOCK
    TECH -->|14. Escalates design issue| BLOCK
    
    BLOCK -->|15. Notifies| PM
    PM -->|16. Communicates with| User
    User -->|17. Provides decision| PM
    PM -->|18. Updates| BLOCK
    BLOCK -->|19. Resumes| SE
    BLOCK -->|19. Resumes| DS
    BLOCK -->|19. Resumes| CL
    BLOCK -->|19. Resumes| TECH
    
    TECH -->|20. Moves to 'Ready' status| READY[📋 'Ready' status on project board]
    
    READY -->|21. Works on tickets in 'Ready' status| SE[software-engineer]
    
    SE -->|22. Moves complete ticket to 'In review'| REVIEW[🔍 'In review' status (ready for QA)]
    REVIEW -->|23. Picks up tickets in 'In review' status| QA[qa-tester]
    
    SE -->|24. Creates tech-lead review ticket| TECH_REVIEW[🏷️ 'tech-lead-review' label]
    TECH_REVIEW -->|25. Reviews and addresses concerns| TECH
    TECH -->|26. Moves ticket back to 'In progress'| IN_PROGRESS
    
    QA -->|27. Tests| CODE
    QA -->|28. Creates bug report| SE
    QA -->|29. Moves approved ticket to 'Done'| DONE[✅ 'Done' status on project board]
    DONE -->|30. Approves| PM

    PM -->|31. Assigns optimization| SA[systems-analyst]
    SA -->|32. Analyzes| CODE
    SA -->|33. Creates optimization ticket| TECH

    PM -->|34. Coordinates deployment| BD[build-deployer]
    BD -->|35. Deploys| CODE
    BD -->|36. Updates| TICKETS

    PM -->|37. Updates status| MB
    PM -->|38. Reports to| User
    
    classDef user fill:#ff6b6b,stroke:#333,stroke-width:2px
    classDef manager fill:#4ecdc4,stroke:#333,stroke-width:2px
    classDef architect fill:#45b7d1,stroke:#333,stroke-width:2px
    classDef specialist fill:#96ceb4,stroke:#333,stroke-width:2px
    classDef artifact fill:#ffeaa7,stroke:#333,stroke-width:2px
    classDef blocking fill:#ff7675,stroke:#333,stroke-width:2px
    
    class User user
    class PM,TECH manager
    class AD architect
    class SE,DS,QA,BD,SA,CL specialist
    class MB,EPICS,ADR,TICKETS,CODE artifact
    class BLOCK,DECISION blocking
```

### Flowchart Legend

**Agents:**
- 🔴 **User**: Primary stakeholder and decision maker
- 🟢 **project-manager**: Orchestrates all workflow and communication
- 🔵 **architect-designer**: Handles major design decisions (direct user access)
- 🟣 **technical-lead**: Translates designs into tickets
- 🟢 **Specialists**: Implementation and support agents

**Artifacts:**
- 📝 **Master Backlog**: Single source of truth for priorities
- 📋 **Epics**: High-level feature tracking
- 📄 **ADRs**: Architectural Decision Records
- 🎫 **Tickets**: Actionable implementation tasks
- 💾 **Codebase**: Source code and assets

**Key Interactions:**
1. All user requests flow through **project-manager**
2. **architect-designer** is only agent with direct user access (for major decisions)
3. Blocking issues are escalated to **project-manager** who communicates with user
4. No agent waits silently - all blocking issues create tickets
5. Work pauses during major design sessions with **architect-designer**

## Future Enhancements

Potential additional agents to consider:
- **art-asset-generator**: Creates visual assets, sprites, textures
- **audio-composer**: Creates sound effects and music
- **animation-sequencer**: Creates character and environmental animations
- **localization-specialist**: Handles multi-language support
- **community-manager**: Manages player feedback and community engagement
