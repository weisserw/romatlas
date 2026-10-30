# Gameplay familiarization
ROM title screen identifies 8 Eye's, Taxan/SETA/Thinking Rabbit 1988/1989,
US-style English release (revision unverified). Source:
https://www.world-of-nintendo.com/manuals/nes/8-eyes.shtml (NES-8E-USA manual transcription,
consulted 2026-10-30). Controls and menu behavior confirmed locally:
A jumps, B stabs; up+B releases/recalls Cutrus, down+B attacks with flying Cutrus.
Start opens main menu, starts Initiate, and accepts the stage selector after A/B
cycles a destination. Start in gameplay pauses. Manual describes seven selectable
countries, Ruth unlocked after them, and an eight-jewel altar puzzle afterward.

Local tested route and per-milestone results live in scenarios.json. Correct NES
RPC input is buttons array, replacing held inputs; button/pressed is silently
ignored (NES-001). Early apparent gameplay was attract demo, not user play.
A1/A4/A7 from restored stage selector produced distinct castle exteriors, and
movement/sword/jump/falcon sequences were exercised. RAM $50=7 before Start
artificially initializes Ruth. At initializer $EE00, RAM $51=7 loads later room
layouts; this has not yet confirmed visible bosses. Artificial $D4E6 entry loads
jewel puzzle; success validator and ending screen experiment recorded separately.
No manual route completing a stage, boss defeat or natural ending is claimed.
