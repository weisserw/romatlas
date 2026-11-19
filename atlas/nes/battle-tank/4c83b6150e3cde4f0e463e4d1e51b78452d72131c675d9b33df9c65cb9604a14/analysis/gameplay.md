# Gameplay familiarization

Likely Garry Kitchen’s Battletank (NES), revision unconfirmed. Embedded FROM NATO COMMAND / M1 NATO FORCES mission strings match the guide and observed mission-download UI.

Source: https://gamefaqs.gamespot.com/nes/587122-garry-kitchens-battletank/faqs/46886 (search excerpt retrieved; full-page fetch restricted). Guide hypotheses: A+Up/Down changes speed, Left/Right steers, B fires, Select changes weapon and Start opens the map. First mission has three tanks. These control claims require local confirmation; unrelated review reverses A/B, so use runtime evidence.

Local confirmation: initial opening lasts hundreds of frames. Pressing Start for two frames after frame 1022, releasing, then 30 frames reached NAVSTAR UP LINK / DOWNLOAD DATA screen. Preserve snapshot after-start2-snapshot.json and scripts for exact route.

Redirect001 local control confirmation: six independent32-frame trials from gameplay-snapshot after explicit2-frame release contradict the early guide A/B hypothesis. A consumes ammunition50→49; B alone does not. B+Up speed0→2; A+Up remains0. A+B pause0→255. No RAM/register patches, fixedPRG[0,1,2,3]; successful trials use trace disabled, so these RAM observations do not add fetch-coverage claims. Replay/evidence: redirect/r001-20261119/probe.py and control-summary.json.
