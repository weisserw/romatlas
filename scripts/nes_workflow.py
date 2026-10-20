"""NES-specific generated instructions. Game Boy profile text stays unchanged."""
import json
from pathlib import Path

GUIDANCE = '''\n## NES instruction and address context

Use physical file offsets including the iNES/NES 2.0 header and optional trainer.
PRG ROM, CHR ROM, CPU RAM, cartridge RAM, PPU VRAM, OAM and palette are separate
spaces. CPU aliases are not physical identities. For banked code supply effective
physical 8 KiB PRG bank indices in context.prg_banks for CPU slots $8000/$A000/
$C000/$E000; mapper register values are not bank indices. Static supported entry
contracts may establish context. Unknown banks remain unresolved locally.
6502 instruction lengths do not depend on status flags. All official and
unofficial opcode lengths decode; ambiguous unofficial encodings use exact byte
directives with mnemonic comments for rebuilding. Pointers are hypotheses, not
proof of code. Mapper writes/calls may invalidate static bank context. Discover
from validated reset/NMI/IRQ vectors and supported entry contracts, supplementing
with observed context/edges where useful. Structural classification, reachability,
whole-ROM classification, full current-source readability and editorial gates
remain separate and unchanged. Unknown bytes stay unknown.
Portable maps use romatlas-nes-map JSON and explicit CPU execution views. Read
docs/nes.md. Never use Game Boy .sym or SNES Asar syntax. Use bounded headless
emulator.step and immutable inspection. RAM execution does not prove a ROM upload
origin. Native helpers in romatlas-port::nes are independent MIT code; do not
copy GPL core code into the native runtime. Baseline debugger targets NTSC,
standard controllers and mappers 0/1/2/3/4/7. Other mapper/peripheral support is
explicitly deferred. No full hardware/game compatibility claim follows.
'''



def profile(text):
    text = text.replace('docs/sym.md', 'docs/nes.md')
    text = text.replace('portable stock .sym map', 'portable NES JSON map')
    text = text.replace('stock .sym', 'portable NES JSON mapping')
    text = text.replace('.sym', '.json').replace('input/game.rom', 'input/game.nes')
    text = text.replace('RGBDS', 'ca65/ld65').replace('rgbfix header changes', 'automatic checksum/header changes')
    text = text.replace("make's default game.gb target", "an assembler command with automatic checksum repair")
    text = text.replace('Discover from entry offset 256 and valid interrupt/restart entry points after',
                        'Discover from validated reset/interrupt vectors and supported entry contracts after')
    text = text.replace('Restart/reset disables tracing; re-enable it.', 'Reset preserves trace configuration; inspect it explicitly.')
    text = text.replace('Merge observed roots/edges with static discovery.', 'Merge compatible observed bank contexts/edges with static discovery (use_observed=false is static-only). RAM traces retain execution storage identity.')
    text = text.replace(' (include_sym=false)', '')
    return text


def files(original, metadata):
    result = {name: profile(value) if name.endswith(('.md','.txt','.py')) else value
              for name,value in original.items()}
    meta=json.loads(result['workspace.json'])
    meta.update(platform='nes',rom='input/game.nes')
    result['workspace.json']=json.dumps(meta,indent=2)+'\n'
    # This extension is applied only to newly generated NES profiles.
    for name in ['AGENTS.md','workflow.md','prompts/disassembly.txt','prompts/commentary.txt',
                 'prompts/redirect.txt']:
        result[name]+=GUIDANCE
    result['run.py']=result['run.py'].replace('call("serve", "--rom", rom)',
        'call("serve", "--rom", rom, "--store", ROOT / "analysis/state.json")')
    mapping=dict(format='romatlas-nes-map',version=1,image_sha256=meta['sha256'],
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
