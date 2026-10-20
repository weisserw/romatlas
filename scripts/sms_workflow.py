"""SMS-specific generated instructions. Game Boy profile text stays unchanged."""
import json

GUIDANCE = '''
## Sega Master System context

Use physical file offsets including any 512-byte copier prefix. ROM, CPU, I/O,
RAM, cartridge RAM, VRAM and CRAM are separate spaces. For banked execution supply
effective physical 16 KiB indices in context.rom_banks for $0000/$4000/$8000,
plus ram_enabled and cartridge_enabled when known. Sega fixes the first 1 KiB;
Codemasters does not. Mapper identification is heuristic: the header has no mapper
ID. Static entry contracts may establish context; unknown windows stay unresolved.
Decode Z80 base/CB/ED/DD/FD/indexed-CB encodings. Ignored prefixes and ambiguous
aliases use exact .db bytes with mnemonic comments, never silent normalization.
Pointers are hypotheses, not proof of code. Calls and possible mapper or memory
control writes stop static bank propagation. Reset $0000, IM1 $0038 and pause NMI
$0066 are entry hypotheses requiring inspection. RAM execution does not prove a
ROM upload origin. Use portable romatlas-sms-map JSON with explicit CPU execution
views and WLA-DX syntax; do not use Game Boy .sym, ca65 or Asar directives.
Read docs/sms.md. Use bounded headless execution and immutable inspection. Baseline
is NTSC SMS2, standard two-button controllers, Sega/Codemasters mappers and PSG/FM.
Do not claim complete hardware/game compatibility. Native helpers are independent
MIT; never copy GPL emulator code into romatlas-port. Structural classification,
reachability, whole-ROM classification, current-source readability and editorial
gates remain separate and unchanged. Unknown bytes remain unknown.
'''



def profile(text):
    text = text.replace('docs/sym.md', 'docs/sms.md')
    text = text.replace('portable stock .sym map', 'portable SMS JSON map')
    text = text.replace('stock .sym', 'portable SMS JSON mapping')
    text = text.replace('.sym', '.json').replace('input/game.rom', 'input/game.sms')
    text = text.replace('RGBDS', 'WLA-DX').replace('rgbfix header changes', 'automatic checksum/header changes')
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
    meta.update(platform='sms',rom='input/game.sms')
    result['workspace.json']=json.dumps(meta,indent=2)+'\n'
    # This extension is applied only to newly generated SMS profiles.
    for name in ['AGENTS.md','workflow.md','prompts/disassembly.txt','prompts/commentary.txt',
                 'prompts/redirect.txt']:
        result[name]+=GUIDANCE
    result['run.py']=result['run.py'].replace('call("serve", "--rom", rom)',
        'call("serve", "--rom", rom, "--store", ROOT / "analysis/state.json")')
    mapping=dict(format='romatlas-sms-map',version=1,image_sha256=meta['sha256'],
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
