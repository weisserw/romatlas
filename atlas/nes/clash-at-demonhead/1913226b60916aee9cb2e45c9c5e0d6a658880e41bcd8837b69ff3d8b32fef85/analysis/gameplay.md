# Game familiarization — round 001

Opening/title capture identifies **Clash at Demonhead**, English US Vic Tokai release; exact revision unconfirmed. Local ROM metadata and hash are in workspace.json. The title and route/action captures confirm the identification; no external ROM/assets downloaded.

Sources consulted 2026-11-26:
- https://thegamersjournal.com/action/nes/clashatdemonhead/instructions.php — transcription of NES-B6-USA manual. Directional movement, A jump, B shoot, Start route map, Select inventory. The manual describes a purchased recorder password system, shops and two endings. Tested: Start map toggle, Select inventory, right/A/B action; equipment mechanics remain untested.
- https://thegamersjournal.com/action/nes/clashatdemonhead/walkthrough.php — route progression hypothesis: early 1→3→9, first boss on 11; late 40/33/39 and final 43. These route/boss claims are external hypotheses, not visits or classification evidence. Routes3 water,9 climbing,11 boss and43 finale are useful representative milestones.

Natural play reached route1 terrain, jumping/shooting, inventory, route map and Tom Guycot dialogue. See analysis/scenarios.json. $0342 is the zero-based **displayed route** operand: map code loads it, increments before decimal conversion. Merely patching it to10/42 and closing the map retains route1 terrain; a loader/transition contract remains needed. This negative result does not reject using $0342 during a genuine initialized junction transition.
