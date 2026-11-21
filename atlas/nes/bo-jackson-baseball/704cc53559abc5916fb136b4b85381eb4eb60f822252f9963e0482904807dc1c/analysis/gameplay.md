# Bo Jackson Baseball (NES USA, local image)

Identified from rendered copyright/menu screens, 1991 Data East USA / Beam Software. SHA in workspace.json. No external ROM or game assets retrieved.

## Sources consulted (2026-11-20)
- https://gamefaqs.gamespot.com/nes/587149-bo-jackson-baseball/faqs/79529/basics — modes: Player vs NES, Player vs Player, NES vs NES, Play Offs, Game Settings; innings 5–9, icon/replay/view/difficulty settings. Menu modes confirmed in local frame; other settings are hypotheses until visited.
- https://www.freegameempire.com/games/Bo-Jackson-Baseball/manual-pdf — manual search excerpt corroborates selectable innings and view/icon/replay settings; not downloaded.
- https://etherealgames.com/nes/b/bo-jackson-baseball/preformatted-cht-file-action-replay-game-genie-us/ — hypotheses: scores internal RAM $041B/$041C; outs $6C18, balls $6C1A, strikes $6C1B, inning $6C1C in cartridge RAM. Current documented debugger writes internal RAM only, so these cartridge fields cannot be directly patched with supported API. Investigate internal mirrored state or understood transition handler contract before shortcutting.
- https://1upheart.com/nes/bo-jackson-baseball/ — directional move/selection; A pitch/swing/throw/jump; B pickoff/advance/run; Select time/lineup, Start begin. Controls remain partly untested.

## Confirmed local route
Zero-RAM cold reset; wait through copyright/Beam intro with empty buttons and a held Start during frames663–783. Menu reached by frame783. Release; A for 10 frames then empty for30 reaches team-selection screen at frame823. Full exact cycle/frame results in r001/rpc.jsonl (timed-out boot waits retain completed state, so use snapshots for exact replay). The RPC ledger is authoritative for button edges/budgets.

## Local progression confirmed
- Full main team list needs time to draw. Selecting user/computer teams and pitcher is supported by A; Start from pitcher screen also advances to pregame. Manual search excerpt says Select cycles pitcher/order/position configuration; Start worked to continue this ROM.
- Pregame anthem ran until approximately frame2968 along the recorded route (Start/A probes did not immediately skip it). Frame `r001/after-anthem.ppm` shows initial pitching matchup, Baltimore Mason versus Boston Marsh, inning1, no counts/outs. Durable snapshot `r001/after-anthem-snapshot.json`.
- External inning field $6C1C is locally confirmed as value1 at inning1. Outs/balls/strikes at $6C18/$6C1A/$6C1B are zero initially, consistent but awaiting event changes.
- Fixed PRG bank15 has one-instruction STX absolute stores at CPU$E592→$6C1C, $E5EC→$6C18, $E551→$6C1A, $E54E→$6C1B, $E5EF→$6C19. These can supply a bounded artificial SRAM-edit recipe: finish operation; save full registers; assert exact opcode+address bytes at target; set PC and X; execute exactly one operation; restore all original registers/PC. Reassert buttons. This is a state-edit workaround, not natural progression evidence.
