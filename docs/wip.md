We are building a team of agents for an indie game dev studio, focusing on the main development loop. ignore the creative agents we will come back to those later. Please read the readme and also make sure any changes are also updated in there.

I'd like to change the interactions/work flow between the tech-lead, the software-engineer and the qa-tester.

 starting with the software engineer: once a ticket has been created by the tech lead and is ready for development, the workflow should be: implement the ticket, commit the code (including relevant unit tests), then done marks ticket as 'ready for QA',
  does not pass it directly to the QA engineer. For 'blocking issue protocol' software eningeerss should write their concerns into the code as comments - commit the change and pass the issue to the tech-lead by tagging it with a 'tech-lead-review' label. Do not directly escalate to user. This includes low level technical issues, such as merge conflicts, unclear acceptance
  criteria, instructions which lead to major design changes in the existing codebase etc. The engineer should also have instructions to check the ticket for
  updated details from the tech-lead and verify required changes against what is already committed on the branch.
  once a feature is complete



QA-tester:
- QA tester is not responsible for low level unit test, but rather high level scenario based play testing. QA tester should be invoked by the project-manager once an 'epic' level ticket has been completed by software engineers and all tickets are moved to 'ready for test' status. QA tester should evaluate the game scenario, as well as the ticket that is passed to it and evaluate whether it should create new scenarios, or just run existing scenarios, and or if any testing is required at all (may not be required for small technical changes, minor dependency patching etc)
- QA tester should consider game scenarios, narratives and come up with catalogue / framework for scenario tests. This should scale with the complexity and progress of game. I.e. consideration should be given to what is avaliable and what 
- if a scenario test fails, the QA tester should notify the tech-lead agent for analysis, using the 'tech-lead-review' label and writing
-  

  Tech-lead:
- The Tech lead is also responsible for reviewing tickets tagged with the 'tech-lead-review' label, and then decide whether to escalate to the user, or
  refining the ticket and passing it back to the software engineer. The TL should review the comments on the commit by the user and make updates to either the ticket or a commit in the code where appropriate.
- Tech lead


---

Ok next I want you to create two more ADRs before we are ready to move to development:
ADR 0003: using the Game-Designer subagent on creating the web UI view:
- the main screen will be a map of the solar system, showing all the planets in their actual position around the solar system for the given turn. This should be prominent given the main strategic choice of the game is around choosing which planet to travel to. There should also be info panels around the side (can be to the right of the screen) which gives status report on player/ship/inventory/market/planet details. Importantly, the details of other planets (including goods and market info) must also be visible to the user, as this is what the user will use to make their decisions on where to travel. Initially this can just be a single page app - we can add a configuration pop-up modal for configuring the game/starting a new game etc.

ADR 0004: Tech stack using the software-architect subagent: based on the current requirements given, use the most appropriate rust-based tech stack to create this game. The only decision I am making here is we use leptos for the UI framework as we want to experiment with this modern approach (do NOT use Yew).