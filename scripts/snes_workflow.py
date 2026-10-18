"""SNES-specific generated instructions. Game Boy profile text stays unchanged."""
import json
from pathlib import Path

GUIDANCE = '''\n## SNES instruction and address context

Decode each candidate using only the processor context affecting that instruction.
Static entry contracts and supported flag/control-flow reasoning may establish it;
runtime observations can supplement them. A runtime trace is not required for
statically justified classification. Many 65816 instructions have fixed lengths
and need no M/X assumption. If relevant state remains unknown, retain alternatives
or leave the affected boundary unresolved, record the specific missing fact and
continue unaffected ranges. Keep structural evidence and reachability separate.

65816 accumulator immediates depend on E/M, index immediates on E/X. SPC700 formats
are fixed. DB/direct page affect references independently. Preserve encoded CPU
aliases even when they map to one ROM byte. Physical file offsets include any
copier header. For uploaded code, record ROM storage bounds, execution CPU/address
and upload/overwrite provenance; equal bytes or pointers alone do not prove upload.
SNES map.annotate takes a region with explicit execution views; alternate views
belong to one physical role rather than overlapping duplicate regions. Read
platform API documentation for exact fields; do not invent GB .sym directives.
Use emulator.step cpu=main or spc700 with bounded operations/cycles/frames.
Code candidates require inspection/confirmation; static discoveries are not coverage.
'''


def profile(text):
    text = text.replace('docs/sym.md', 'docs/snes.md')
    text = text.replace('portable stock .sym map', 'portable SNES JSON map')
    text = text.replace('stock .sym', 'portable SNES JSON mapping')
    text = text.replace('.sym', '.json').replace('input/game.rom', 'input/game.sfc')
    text = text.replace('RGBDS', 'Asar').replace('rgbfix header changes', 'automatic checksum/header changes')
    text = text.replace("make's default game.gb target", "an assembler command with automatic checksum repair")
    text = text.replace('Discover from entry offset 256 and valid interrupt/restart entry points after',
                        'Discover from validated reset/interrupt vectors and confirmed uploaded-code entry points after')
    text = text.replace('Restart/reset disables tracing; re-enable it.', 'Reset preserves trace configuration; inspect it explicitly.')
    text = text.replace('Merge observed roots/edges with static discovery.', 'Merge compatible observed processor contexts/edges with static discovery (use_observed=false is static-only). RAM traces retain execution storage identity; confirm ROM upload provenance separately.')
    text = text.replace(' (include_sym=false)', '')
    return text


def files(original, metadata):
    result = {name: profile(value) if name.endswith(('.md','.txt','.py')) else value
              for name,value in original.items()}
    meta=json.loads(result['workspace.json'])
    meta.update(platform='snes',rom='input/game.sfc')
    result['workspace.json']=json.dumps(meta,indent=2)+'\n'
    # This extension is applied only to newly generated SNES profiles.
    for name in ['AGENTS.md','workflow.md','prompts/disassembly.txt','prompts/commentary.txt',
                 'prompts/redirect.txt','prompts/final-investigation.txt']:
        result[name]+=GUIDANCE
    result['run.py']=result['run.py'].replace('call("serve", "--rom", rom)',
        'call("serve", "--rom", rom, "--store", ROOT / "analysis/state.json")')
    mapping=dict(format='romatlas-snes-map',version=1,image_sha256=meta['sha256'],
                 image_size=metadata['size'],regions=[],next_id=1)
    result['analysis/state.json']=json.dumps(mapping,indent=2)+'\n'
    result['maps/current.json']=json.dumps(mapping,indent=2)+'\n'
    return result


def scaffold(state):
    """Adapt physical region records to the unchanged full-ROM editorial scaffold."""
    return dict(image_size=state['image_size'],regions={str(r['id']):dict(
        start=dict(space='cartridge-rom',offset=r['offset']),length=r['length'],kind=r['kind'])
        for r in state['regions']},region_metadata={str(r['id']):dict(
            views=r.get('views',[]),evidence=r.get('evidence',''),label=r.get('label'))
            for r in state['regions']})
