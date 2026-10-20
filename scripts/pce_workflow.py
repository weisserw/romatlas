"""PC Engine-specific generated instructions. Game Boy profile text stays unchanged."""
import json
from pathlib import Path

GUIDANCE = '''
## TurboGrafx-16 / PC Engine HuCard context

Use original physical file offsets including the optional 512-byte copier prefix.
HuCards have no universal cartridge title/header/mapper ID. Size/CRC-based mapper
identification is explicitly heuristic. Preserve every original byte; do not repair
headers/vectors. ROM, 8 KiB mirrored RAM, VDC VRAM, VCE CRAM and I/O are distinct.
CPU addresses are 16-bit aliases through eight 8 KiB context.mpr bus pages; MPRs
are not immutable file-bank indices. Supply effective MPR values or leave unknown
slots null. Optional context.sf2_bank supplies the cartridge window selector.
384/512 KiB HuCards have special mirroring layouts. Unmapped/CD/I/O pages must
never become ROM claims. HuC6280 zero-page and stack are CPU $2000/$2100.
HuC6280 adds TAM/TMA, T-bit operations, block transfers, bit branches and CPU
speed switching. TAM invalidates affected static MPRs unless independently proven.
Calls/mapper writes may invalidate return context. Observe accepted fetches rather
than interrupt entry/block-transfer continuation as execution. Use bounded steps,
snapshots, immutable peeks and contextual edges. Pointers are explicit little-endian
16-bit CPU hypotheses under an MPR context, never proof of code or execution.
Validate vectors at bank zero $1FF6-$1FFF and their execution contracts first.
Portable maps use romatlas-pce-map JSON, physical regions and execution views.
Rebuild with pinned cc65 ca65 --cpu huc6280 and ld65 using game.asm/layout.cfg;
no Game Boy .sym, RGBDS or Asar. Read docs/pce.md and docs/pce-native.md.
Default pad: I, II, Run, Select, Up/Down/Left/Right. Display defaults 256x240,
programmable width/dot clock and 262/263-line cadence. VDC scroll/raster interrupts,
sprite priority/collision/overflow, VRAM/SATB DMA, VCE palette/greyscale and six-channel
wavetable/DDA/noise/LFO stereo PSG need source-informed native implementations.
Native helpers are independent MIT code; never copy GPL emulator implementation.
HuCard only; CD, SuperGrafx, multitaps and uncommon boards are outside baseline.
Structural classification and reachability remain separate. Whole-ROM classification,
full current-source readability and editorial gates are unchanged. Unknown stays
unknown; raw byte-identical reconstruction does not complete classification.
'''



def profile(text):
    text = text.replace('docs/sym.md', 'docs/pce.md')
    text = text.replace('portable stock .sym map', 'portable PC Engine JSON map')
    text = text.replace('stock .sym', 'portable PC Engine JSON mapping')
    text = text.replace('.sym', '.json').replace('input/game.rom', 'input/game.pce')
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
    meta.update(platform='pce',rom='input/game.pce')
    result['workspace.json']=json.dumps(meta,indent=2)+'\n'
    # This extension is applied only to newly generated PC Engine profiles.
    for name in ['AGENTS.md','workflow.md','prompts/disassembly.txt','prompts/commentary.txt',
                 'prompts/redirect.txt']:
        result[name]+=GUIDANCE
    result['run.py']=result['run.py'].replace('call("serve", "--rom", rom)',
        'call("serve", "--rom", rom, "--store", ROOT / "analysis/state.json")')
    mapping=dict(format='romatlas-pce-map',version=1,image_sha256=meta['sha256'],
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
