# Gameplay research — round 1

Identified Barbie (NES, 1991) from local copyright/title frame. Region/revision not independently established; ROM hash is authoritative in workspace.json.

[GameGenie guide](https://gamegenie.com/cheats/gamegenie/nes/barbie.html) lists infinite dream-meter/reentry codes; only background research, no ROM patch or Game Genie code used. [GameFAQs cheats](https://gamefaqs.gamespot.com/nes/587107-barbie/cheats) describes holding Select on reset and A,B,Up,Down,Left,Right,A,B secret message. Static consumer and eight button masks at physical $2ba9 agree; secret route not runtime tested. [Gameplay description](https://nesninja.com/game/nes/barbie/about) says Select chooses animal commands and B throws them; local jump/move/charm inputs exercised, specific animal effects unverified. Manual fetch via Gaming Alexandria failed (provider size limit); Neoseeker guide returned403. No ROM/assets downloaded.

Locally tested NES input schema is `buttons:["start"]`, releases `buttons:[]`; `button/pressed` is ignored. Natural Start route reached sports-store scene16/stage4. Supported RAM scene-request protocol $2f plus bit0 $24 initialized bakery13, galleria14, underwater15, sports16, music17, and story/ending-like scenes5–11. These are artificial reachability, not proof of normal progression. Full reproducible recipes and milestone checklist: [scenarios.json](scenarios.json). Scene11 ballroom continued after300 frames; no definitive final completion flag established. Boss mechanics and natural ending remain deferred to the specific stage/goal-contract lead R1-002.

Round2: reused all five final-stage checkpoint environments and artificial goal->result->exterior->ballroom presentation evidence in redirect/r1-resource-contract. Bounded90-frame music final encounter movement/attack inputs confirmed representative ongoing mechanics; no natural boss defeat or ordinary completion/credits claim. Exact route and frame/coverage evidence in scenarios.json round2_disassembly.

R3: prior five-stage/final and artificial result/exterior/ballroom progression reused. Targeted artificial audio entry mechanics from boss17 snapshot add no progression/reachability claim. scenarios.json round3_disassembly records exact edits/budgets; no repetitive route.

R4 disassembly reuses saved five-stage/final encounter and artificial result/exterior/ballroom evidence. No new discriminating gameplay question for13residual bytes; no route replay, state patch or natural-ending claim. See scenarios.json round4_disassembly.
