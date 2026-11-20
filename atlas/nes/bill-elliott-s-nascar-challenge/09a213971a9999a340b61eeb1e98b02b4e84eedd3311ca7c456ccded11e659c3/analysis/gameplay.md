# Gameplay evidence

ROM visually identified at startup as Bill Elliott's NASCAR Challenge (Konami, 1991): analysis/round-01/title-30.png. NTSC NES mapper 4. Identification comes from this ROM's rendered legal notice; external guide research and controls/progression remain pending. Racing game milestones should include menus/configuration, qualification/race, distinct tracks, pit mechanics, results and terminal/championship outcomes. No outside description currently supports a classification claim.

## Targeted manual research (round 01)

Sources: Konami NES manual mirrored at https://www.digitpress.com/library/manuals/nes/Bill%20Elliot%27s%20NASCAR%20Challenge.pdf and OCR https://manuals.plus/m/ea1c8c16e13473fdbe61fdc0c2e706330f00ab14a4b081fd8794c8114f4642c4 (retrieved 2026-11-20). USA NES applicability likely; externally sourced hypotheses awaiting runtime tests.

Start opens selection; Select cycles categories, A enters. Track choices include Daytona, Talladega, Sears Point and Watkins Glen plus eight-race championship. Distance adjustable 10–500 miles, offering a practical short-race route. Novice disables crashes. Auto Qualify Yes skips qualification. Start bypasses car setup and later starts racing. A accelerates, B brakes; arrows steer/shift, Start pauses. Pit entry uses inner marked lane. Championship produces long registration passwords after races, usable on License via Select; no ready password found. Road versus oval tracks, practice instruction screens, qualifying, pits and results offer distinct milestones. These are experiment recipes, not evidence of ROM byte roles.

## Redirect confirmation and shortcuts

Source/runtime confirms Start advances the long prerace speech/wait:3pressed frames,60released, then180frames initializes live racing. A540frames reaches144mph. Input route and natural snapshots are in redirect/r01-20261120-a. Artificial supported8112return throughBF7E/E485 yields Race Results; pit scene4selector1 after real cleanup renders top-down pit, A requests service, Start returns race. Final-season negative track selector88 reaches trophy/champion ending through bank0 gate8ADB->93FD; Start exits into championship standings. These are local artificial shortcuts, not passwords or natural championship victories. Exact entry/cleanup and patches in scenarios.json and round-01-redirect-handoff.md. Prior directE0E2 shortcuts withoutE485 lost state and are not reusable routes.
