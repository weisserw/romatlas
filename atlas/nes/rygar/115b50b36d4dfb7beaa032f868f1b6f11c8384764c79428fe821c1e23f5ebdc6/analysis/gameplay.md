# Gameplay research

## NES Rygar, likely USA (ROM revision not independently identified)

Sources consulted 2026-10-20:
- https://www.world-of-nintendo.com/manuals/nes/rygar.shtml — transcription identified NES-RY-USA. D-pad moves/crouches/climbs; A jumps, B attacks; Start enters/exits status subscreen; Select pauses/resumes. Sueru→Gran→Garloz progression, with later region-specific equipment. Primary manual instructions; no passwords established.
- https://strategywiki.org/wiki/Rygar_(NES)/Starting_point — actionable route: proceed right across Sueru to Gran, jump onto enemies when trapped; Gran entrance has elevated branches/door and a long gap requiring branch crossings. Version not guaranteed identical to supplied ROM.

Local confirmation: title observed after 90 frames. Start held 2 frames then release 90 frames reaches mountain terrain (frame 182) with current banks [6,7,14,15]; screenshot `analysis/pass-001/replay-game.png`. Controls beyond Start are being tested. Snapshot export at `analysis/pass-001/replay-game.snapshot.json` persists machine state; trace history is separately saved and not part of snapshot.

Target experiments: right+B attacks/runs across starting terrain; pulse A at gaps/enemies; test Start status and Select pause as alternative state coverage. Seek main game update, enemy, collision, transition and UI code contexts. Source descriptions are hypotheses, not structural classification evidence.

## Round 1 commentary reconciliation

The completed pass-001 handoff and `analysis/scenarios.json` record natural
movement/combat/jump, death/late continue, Start status and Select pause routes.
They supersede the earlier provisional "controls beyond Start are being tested"
status above. No new runtime experiment or successful spell cast was performed
during commentary. Static source corroborates serial A/B/Select/Start input bits,
fresh presses as held AND changed, a three-row spell selection, a two-column item
selection and Mind affordability checks. These consumer interpretations guide
future L001 state experiments; exact spell effects and later-area routes remain
untested. See `commentary/uncertainties.md`; external source descriptions above
remain hypotheses where the supplied ROM has not confirmed them.

## Redirect r1-20261020 targeted startup experiment

S005 repeats emulator.reset with released buttons for30 frames (queue/PPU traces)
or90 frames in30-frame calls (ROM reads). No patches or new progression route.
Exact descriptor/source/PPU transfer evidence in `redirect/r1-20261020/`; this
confirms startup graphics mechanics, not later-area selection or game completion.
Prior sourced controls/routes remain authoritative; no additional web research
was needed for this static/consumer-directed experiment.

## Redirect r2 static route constraints

No new gameplay inputs or external gameplay claims. Static destination fields describe76destinations/15area pairs; new area/trigger/text/spawn consumers are structural evidence, not proof later areas or ending were naturally reached. Existing sourcedmanual/routes and testedscenarios remain unchanged. A future targeted later-area experiment should choose a specific normalizedgeometry/transition-event question from redirect/r2-20261020/review.md; unchanged Sueru/menu replay would not resolve the residualcontainerbounds.

## Round3 static disassembly review

No new gameplay inputs or external claims. Retained natural routes/research remain unchanged; new static sprite/area/trigger/text/spawn/render contracts do not prove later-area selection. Future runtime work should target a specific normalized geometry or behavior-dependent event-width question; unchanged Sueru/status replay has no discriminating prospect. See disassembly-r3/handoff.md.

## Round3 commentary static familiarity

No new gameplay experiment or external source. Reviewed source now explains player contact damage/knockback, area-line interaction, status health restoration and the3-row/2-column spell/item layout with bounded icon/cost/position tables. These remain static consumer interpretations; saved routes still do not demonstrate successful spell casting or later-area line interactions. Geometry-normalization and action/animation-domain questions remain in commentary/uncertainties.md for targeted future routes only when they discriminate a concrete hypothesis.


## Redirect r3 static constraints

No new gameplay inputs or external sources. Existing sourced/manual routes remain unchanged. Static state closure supports six attack subtypes and seven auxiliary interaction subtypes; bounded geometry node/bitmap representation claims do not prove normalized per-area dimensions, natural all-selector use or later-area reachability. Respawn fallback five available destination fields are separate from natural page eligibility. See redirect/r3-20261020/findings.md. No unchanged startup/Sueru replay.


## Round4 static role review

L008/L009 classified bounded backing representations without new natural-route claims. Existing normal routes/scenarios retained; no unchanged replay or new artificial input used. Geometry normalized coordinates, later-area transitions and fallback page eligibility remain gameplay hypotheses; these are separate from the supported data roles. See analysis/disassembly-r4/review.md.


## Redirect r4 static backing review

No new gameplay input or external source. Static destination relationships preserve orientation2 for23records in areaPairs0..2; finite hierarchy/backing roles do not establish normalized coordinate domains, natural every-selector use or later-area routes. Existing tested scenarios remain unchanged. See redirect/r4-20261020/findings.md.


## Round5 static backing acceptance

No new gameplay input or external source. Independently reviewed23orientation2 destination records and15/16-stride hierarchy consumers support structural backing fields; original per-area coordinate domains, natural every-selector use and later-area routes remain unconfirmed. Existing tested scenarios retained without unchanged replay; see disassembly-r5/review.md.

## Round5 commentary static familiarity

No new gameplay input or external source. Reviewed query16/render15 overlap and action orientation tile/OAM-attribute mechanics preserve natural coordinate/direction eligibility uncertainty. Existing tested routes remain unchanged; no full-domain or later-area reachability claim follows. See commentary-r5/scope-review.md.

## Redirect r5 static index contract

No new gameplay inputs or external source. Coarse trigger common key/result index starts1 after primary search hint; twelve result origins identify reserved index-zero alignment locations, without natural selection or stream-ID claims. Prior sourced routes remain unchanged; no unchanged gameplay replay. See redirect/r5-20261020/findings.md.


## Round6 coarse-result static slots

No new external source or gameplay sequence. Independent static search model confirms all256encoded byte coordinates select1..N under the established bank5 consumer; reserved result index0 is not selected. This establishes mechanics, not which coordinates occur naturally. Prior routes unchanged; [review](disassembly-r6/review.md).


## Round7 graphics representation review

No new external source or gameplay sequence. L012 acceptance concerns static tile representation only; no natural upload/use inferred. Prior routes unchanged. [Review](disassembly-r7/review.md).


## Round7 redirect representation review
No new gameplay input or external source. Corrected graphics representation does not establish natural use, and residual OAM/coordinate/audio questions retain prior reachability uncertainty. Existing tested routes/scenarios unchanged; no unchanged replay. [Review](redirect/r7-20261020/rejection-review.md).


## Round8 broad artificial progression — 2026-10-20

Revised user policy supersedes earlier lead-gated runtime advice. Maintained source gives supported transition MainState0301=6: clears area object/task state, builds destination graphics, then initializes camera/area/fade and gameplay tasks. Each branch restores the clean exported182-frame snapshot before patching; exact1Ddestination,0300..031Ftask slots,41step andCA/CBhealth48 writes are in disassembly-r8/destination-XX-patches.json. Source confirms later destination ranges; numeric areaIDs are not externally matched names. Five branches12/17/24/3A/47 show distinct rendered active areas03/07/0D/16/1B and actual fetches under effective bank contexts, with bounded movement/attack/jump/release and short complete-paged confirmation. Rope/water in0D is scenery confirmation only.

Boss47shows shield-bearing enemy. Two bounded combat attempts emit attacks (90480actual fetch, AttackActive0->1, visible weapon) and move/jump; separate close-position0512=B0/0502=00 branch uses a clean restore. Boss health05E2 remains15: no damage/victory claim. Spell shortcut is successful: status snapshot, MindC3=20/selector26=1/freshA yields Mind15 andCD10.

Ending main0301=10/step41=0 enters existing full22-step initializer. After ordinary timer/graphics/camera progression to step8, a clean snapshot precedes supported0533=FF timer acceleration,2B=00 at signed-delay states and heldStart at15. Full sequence reaches main0 and visually confirmed terminal white END on purple; terminal snapshot/frame/full fetched roots persist locally. This is artificial ending evidence, not natural boss victory. See [scenario checklist](scenarios.json), [gameplay inputs](disassembly-r8/gameplay-log.json), [ending timer log](disassembly-r8/ending-timer-log.json), [terminal screen](disassembly-r8/ending-terminal.png), [spell](disassembly-r8/spell-recipe.json), [boss attempts](disassembly-r8/boss-actions.json). No new external source. All snapshots/source/assets remain local.

## Round8 commentary reconciliation

No new gameplay experiment was run. Source now explains the alternate terrain updater at9354 using the existing strip producer's Work8={0,2,4,6} contract; this is static routine evidence, not proof of a natural caller. Full r8artificial later-area/spell/boss/ending recipes and observed limits remain in scenarios.json and disassembly-r8. Generic-data semantics remain uncertain without blocking structural completion or source readability.

## Round8 ordered redirect progression review
Reviewed current checklist and S006..S013 without unchanged replay. Five distinct RAM-assisted areas with active frame/fetch/mechanics, successful spell, two bounded boss combat attempts and full22-step ending to visible END provide representative broad best effort. Boss damage/victory, rope traversal and natural completion remain unclaimed; no necessary unvisited terminal milestone or new discriminating shortcut identified. See [redirect handoff](redirect/r8-20261020-editorial/handoff.md). No new external sources or runtime inputs.
