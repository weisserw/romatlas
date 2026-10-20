"""Genesis-specific generated instructions. Game Boy profile text stays unchanged."""
import json

GUIDANCE = '''
## Sega Genesis / Mega Drive context

Use canonical physical backing offsets and romatlas-genesis-map JSON: copier prefix
followed by normalized big-endian ROM. SHA-256 identity is of the original file.
Identify big-endian, word-swapped and SMD layouts; original file positions are
a separate encoding, restored exactly at reconstruction. Main CPU is m68k (main); sound CPU is z80. 68000
pointers are big-endian 32-bit bus addresses or explicitly based signed 16-bit
offsets; Z80 pointers are little-endian 16-bit. Pointer targets are hypotheses.
Main bus is 24-bit; work RAM mirrors across $E00000-$FFFFFF. Z80 $0000-$3FFF
mirrors 8 KiB audio RAM, $8000-$FFFF uses context.z80_bank (32 KiB index).
SSF context.rom_banks contains eight effective 512 KiB indices; ram_enabled
controls SRAM overlays and byte lanes. Unknown contexts stay unresolved.
Validate reset SP/PC and interrupt vectors before treating them as entrypoints.
RAM execution never proves an upload origin/lifetime. GNU binutils 68000 syntax
and DeZ80 decoding apply; preserve exact encodings, source comments and all file
bytes. No automatic checksum/header repair. Source game.asm and layout.ld rebuild
with GNU as/ld/objcopy; no RGBDS, Asar or Game Boy .sym directives.
Use bounded headless debugging, immutable peeks, two standard six-button pads,
NTSC 320x224 defaults (H32 256px and interlace/raster changes belong to the game).
VDP planes A/B/window, scroll, sprites, priority, shadow/highlight and DMA, YM2612
six-channel four-operator FM/DAC/stereo and PSG require source-informed porting.
Read docs/genesis.md and docs/genesis-native.md. Native helpers are independent MIT:
never copy GPL implementation into the port runtime. Rare peripherals, SVP,
Sega CD/32X and uncommon mappers are deferred. No full hardware/game compatibility
claim. Structural role and reachability remain separate; whole-ROM classification,
current-source readability and editorial gates are unchanged. Unknown stays unknown.
'''



def profile(text):
    text = text.replace('docs/sym.md', 'docs/genesis.md')
    text = text.replace('portable stock .sym map', 'portable Genesis JSON map')
    text = text.replace('stock .sym', 'portable Genesis JSON mapping')
    text = text.replace('.sym', '.json').replace('input/game.rom', 'input/game.md')
    text = text.replace('RGBDS', 'GNU binutils').replace('rgbfix header changes', 'automatic checksum/header changes')
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
    meta.update(platform='genesis',rom='input/game.md')
    result['workspace.json']=json.dumps(meta,indent=2)+'\n'
    # This extension is applied only to newly generated Genesis profiles.
    for name in ['AGENTS.md','workflow.md','prompts/disassembly.txt','prompts/commentary.txt',
                 'prompts/redirect.txt']:
        result[name]+=GUIDANCE
    result['run.py']=result['run.py'].replace('call("serve", "--rom", rom)',
        'call("serve", "--rom", rom, "--store", ROOT / "analysis/state.json")')
    mapping=dict(format='romatlas-genesis-map',version=1,image_sha256=meta['sha256'],
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
