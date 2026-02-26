i want to create a team of qwen sub-agents that model a software engineering team, that primarily communicates and tracks work through github. the goal of the team is to interface with a human user and translate request from the user into code/documentation in the git repo, but also document their work via the tools avaliable in Github so that the human can easily inspect the work being done. This work shall be organised by issues/tickets (the terms are used interchangeably in github) and interactions through a kanban board (the github "project" feature).

Roles:
PM: Project manager
TL: Tech Lead
QA: QA tester
AD: Architect Designer
SWE: Software Engineer


# Use of github and board:
- agents should be aware of board (i.e. "project" in Github language)
- The PM is the main point of contact with the human, and should invoke sub-agents following this framework complete tasks
    - to communicate with human user, use a 'master backlog' issue, a prioritised list of work agreed upon with user. 

    - PM has main responsibiltiy of communication with human user
    , other agents should escalate to PM if decision / communication to human is needed
    - PM invokes relevant sub-agents based on issue task and work needed to complete task, makes best judgement on which agent to call, assigns them specific ticket/issue to work on.
    - Completing Some tickets may require creating more tickets (e.g. TL -> detailed implementation tasks, QA -> test scenarios)
    - There may be non-development tasks, e.g. narrative writing, creating user test scenarios, which should be assigned to appropriate agent
    - When Sub Agent is done with work, they write an update on their assigned github issue and alert PM
    - PM responsible for checking on sub-agents. If they crash or are stuck, PM is responsible for restarting and making sure tickets have the appropriate sub-agent working on it
    - User may request for the PM to work on a specific ticket/task (i.e. hold it so no sub-agent is working on it). use an appropriate label

## Agent to Agent communication
- agents should be aware of Status field on tickets and use this to communicate work appropriately
     - "Backlog" (ID: f75ad846)
     - "Ready" (ID: 61e4505c)
     - "In progress" (ID: 47fc9ee4)
     - "In review" (ID: df73e18b)
     - "Done" (ID: 98236657)

### Example flow:
    - when PMs first create epic tickets they go into the backlog. 
    - A TL picks up an epic and creates sub-tickets based on those and puts them into the 'ready' status. 
    - PM asks SWE to pick up 'ready' tickets, which gets moved into "in progress"
    - When SWE completes the task, they raise a PR and it goes into "In review"
    - PM asks TL to check "in review" tickets and either approve or send back to "ready" for an SWE agent to make changes
    - If TL approves then ticket can be moved to "Done"
    - QA and AD have their own specific issues, which have appropriate issue labels on them and not part of the TL/SWE dev flow.

### Labels
agents should use labels to communicate with each other on github. This also helps user understand what's going on in the team.
- `master-backlog` should be managed by the PM, and `epics` created from the backlog tasks
- issues with the `epic` labels should be looked at by the TL
- all other sub-ticket labels are for the appropriate sub-agent
- `user-review` label for ser to make a decision due to blocking issue, or when user wants to consider a ticket and pause agent work on it.

### Comments
Agents should comment on the issue with key updates when they are doing work. A summary Must be provided before the ticket is finished and handed back to the PM for next steps.
- when writing comments on issues, agents must always comment prefixed with their own role. This helps both the user and other agents understand where the feedback is coming from.
- E.g. if a TL makes a comment, it is given higher precedence over an SWE comment as the TL has context of broader project standards and goals, compared to an SWE working on a single task. 
- PM should read comments on ticket when deciding who / which agent to call next to resolve a ticket (or to escalate to user). 
- If User is required to resolve an issue, that should be considered final.


# General Flow of work

## PM and User
- PM asks user to describe what they want, summarises requirements and confirm with user, writes a prioritised list of 'epic' tickets 

## PM and architect and User
- PM invokes AD architect-designer, to check if major technical decisions need to be made that should guide the development of the entire project.
- AD is the only other agent that should interact with user directly to obtain decisions
- Key output: document user decisions in ADR (Architecture Decision Record) format in the repo. These can be anything big or small, functional or non-functional, tech or key user requirement/jourrney, which must always be considered by the other agents implementing or creating detailed work issues
- PM may also invoke AD at the request of other agents, e.g. if a important decision is required from the user. If AD is invoked it should be considered blocking as decisions may have major impacts to existing work
- PM should judge if after each new ADR, existing work needs to be re-organised or previous work re-done based on new decision.

## PM and TL
PM then invokes TL responsible for translating high level requirements + ADR decisions into detailed implementation tasks. Should generally only be a single TL at once to avoid creating duplicate / contradicting work. 
- TL can also be invoked to look at bugs / issues identified by QA or other agents

The TL should:
- examine top priority item(s), examine codebase, and creating work tickets in the backlog based on what is required, track dependencies between work.
- Also check existing tickets to make sure no duplicate work exists.
- TL must always consult appropriate ADRs. If ADR doesn't exist, and it's a major new pattern, may need to stop work and ask PM to invoke AD for decision.
- Label work  with appropriate role. Try and figure out which role is needed

Reviews:
- TL also needs to be invoked to check on work that is submitted by SWEs, to review tickets and Pull-requests. 
- Once ticket is judged completed TL should set ticket status to 'done'
- If a ticket was a `bug` report from QA, PM should be notified and sent back to QA for re-test or review

Key outputs:
- new tasks for SWE to implement, review PRs and approve / merge to main. 
- Raise blocking issues to PM / user as required. 


## PM and SWE
PM invokes SWE once there is a 'ready' ticket in backlog. There may or may not be specialised SWE agents configured (e.g. backend, frontend, api, game engine etc.), PM should pick best one or generic one to invoke.

There can be multiple SWE agents
- If there are multiple SWE agents, PM should keep track of which agent is working on which task. PM should not ask for work with dependencies if the dependent work is not yet done.
- SWE implements task in language appropriate best-practices 
- SWE is responsible for adding unit tests and checking basic functionality, regression testing
- Once SWE is done with ticket, write a summary on the ticket and set ticket status to 'in review'
    - If task was completed successfully, raise a PR and label with `tech-lead-review`
    - If SWE was blocked, follow 'blocking protocol'
- Either way dev loop is such that TL agent needs to review all tickets produced by SWE before it can be considered done

## PM and QA
PM should create QA tickets when scenario testing is required, with the `epic` label as they sit adjacent to dev epic work
- QA is not responsible for low level unit tests, but rather high level scenario based testing. 
- QA should be invoked by the PM the linked `epic` level ticket has been marked completed by a TL, and all sub tickets are moved to 'done' status.
- QA is responsible for creating a functional test scenario based on the feature, including thinking about edge cases and testing those
- QA is responsible for maintaining library of existing test scenarios, making updates to old scenarios if functionality is updated or changed
- QA may decide if any testing is required at all, may not be required for small technical changes, minor dependency patching etc
- if a scenario test fails, the QA should create a new linked issue using the `tech-lead-review` and `bug` labels and notify the PM to pass to the TL
- QA should use the ticket 'status' field appropriately. Once a test is verified and complete, the ticket can be set to 'done' status and PM notified.
