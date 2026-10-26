import {StateSnapshot, digest} from './save-state.mjs';
import {gamepads, padState, capturePadBinding} from './browser-gamepads.mjs';
let pollCapture, stateSnapshot, stateKey, restoringState=false, savingState=false;
import init, { BrowserAssets, BrowserGame } from './game.js';
const $ = id => document.getElementById(id);
const report = error => { $('error').textContent = String(error); console.error(error); };
const empty = () => ({keys:[],buttons:[],axes:{}});
const keyNames = {ArrowUp:'Up',ArrowDown:'Down',ArrowLeft:'Left',ArrowRight:'Right',Enter:'Return',Escape:'Escape',' ':'Space',Backspace:'Backspace',Tab:'Tab',Shift:'Left Shift',Control:'Left Ctrl',Alt:'Left Alt'};
const keyName = event => ({ShiftRight:'Right Shift',ControlRight:'Right Ctrl',AltRight:'Right Alt',MetaLeft:'Left GUI',MetaRight:'Right GUI',CapsLock:'CapsLock',NumLock:'Numlock',ScrollLock:'ScrollLock',NumpadEnter:'Keypad Enter'})[event.code] || keyNames[event.key] || (event.key.length === 1 ? event.key.toUpperCase() : event.key);
let game, audio, settings, defaults, draft, config, settingsKey, saveKey;
let started=false, paused=true, capture=null, last=null, nextAudio=0, ticks=0;
let assignments = ['auto','auto','auto','auto'];
const keys = new Set(), keyCodes = new Map(), sources = new Set();
let signature='';
function pads() { return gamepads(navigator); }
function physical() {
  const all=pads(), used=new Set(), reserved=new Set(assignments.filter(id=>id!=='auto' && id!=='none'));
  return assignments.map((id,index)=>{
    const state=empty(); if(index===0) state.keys=[...keys];
    const item=id==='none'?null:id==='auto'?all.find(p=>!used.has(p.id) && !reserved.has(p.id)):all.find(p=>p.id===id && !used.has(p.id));
    if(item) {
      used.add(item.id); const p=item.pad;
      Object.assign(state, padState(p)); if(index===0)state.keys=[...keys];
    }
    return state;
  });
}
function stopAudio() {
  for(const source of sources) {try{source.stop();}catch{ /* already ended */ }}
  sources.clear(); if(audio) nextAudio=audio.currentTime;
}
function flush() {
  if(!game || !saveKey)return;
  try { const save=game.save_json(); if(save){localStorage.setItem(saveKey,save);game.acknowledge_save();} }
  catch(error){report(`Battery save failed: ${error}`);}
}
function boundary() {
  if(game)game.boundary(JSON.stringify(physical()));
  keys.clear();keyCodes.clear();last=null;stopAudio();flush();
}
function gameMessage(message='') {
  $('game-message').textContent=message;
  $('game-overlay').hidden=!message;
  $('status').hidden=Boolean(message);
}
async function resumeGame() {
  await audio.resume();
  if(audio.state!=='running')throw Error('Audio could not start');
  boundary();started=true;paused=false;gameMessage();$('state-status').textContent='';
  $('pause').disabled=false;$('status').textContent='Running';
}
function pause() {
  paused=true;boundary();$('status').textContent='Paused — press Start to resume';gameMessage('Paused — press Start to resume');
  if(audio)audio.suspend().catch(report);
}
function draw() {
  const canvas=$('screen');canvas.width=game.width();canvas.height=game.height();
  canvas.style.aspectRatio=String(game.display_aspect());
  canvas.getContext('2d').putImageData(new ImageData(new Uint8ClampedArray(game.pixels()),canvas.width,canvas.height),0,0);
  // Height controls the presentation of the native gameplay viewport.
  const height=settings.height;
  const displayHeight=document.fullscreenElement=== $('game-panel')?Math.min(height,innerHeight):height;
  canvas.style.width=`${displayHeight*game.display_aspect()}px`;canvas.style.height='auto';
}
function play(samples) {
  if(!samples.length || audio.state!=='running')return;
  if(nextAudio>audio.currentTime+.25)stopAudio();
  const buffer=audio.createBuffer(2,samples.length/2,audio.sampleRate);
  for(let c=0;c<2;c++){const data=buffer.getChannelData(c);for(let i=0;i<data.length;i++)data[i]=samples[i*2+c];}
  const source=audio.createBufferSource();source.buffer=buffer;source.connect(audio.destination);
  sources.add(source);source.onended=()=>sources.delete(source);
  nextAudio=Math.max(nextAudio,audio.currentTime+.015);source.start(nextAudio);nextAudio+=buffer.duration;
}
function loop(now) {
  try {
    const connected=JSON.stringify(pads().map(p=>[p.id,p.pad.index]));
    if(connected!==signature){signature=connected;boundary();if($('preferences').open)controllerOptions();}
    if(capture && pollCapture){const binding=pollCapture();if(binding)bind(binding);}
    if(game && started && !restoringState && !paused && !$('preferences').open) {
      ticks+=game.advance(last===null?0:Math.min(now-last,1000),JSON.stringify(physical()));last=now;
      draw();play(game.audio());flush();$('screen').dataset.ticks=String(ticks);$('status').textContent='Running';
    } else last=null;
  } catch(error){pause();report(error);}
  requestAnimationFrame(loop);
}
function controllerOptions() {
  const field=$('players');field.replaceChildren();
  const legend=document.createElement('legend');legend.textContent='Controller assignment (keyboard: player 1)';field.append(legend);
  for(let player=0;player<4;player++) {
    const label=document.createElement('label');label.textContent=`Player ${player+1} `;
    const select=document.createElement('select');select.dataset.player=player;
    const choices=[['auto','Automatic'],['none','None'],...pads().map(p=>[p.id,`${p.pad.index+1}: ${p.pad.id}`])];
    if(!choices.some(([id])=>id===draft.assignments[player]))choices.push([draft.assignments[player],'Disconnected controller']);
    for(const [id,name]of choices){const option=new Option(name,id);select.add(option);}
    select.value=draft.assignments[player];select.onchange=()=>{draft.assignments[player]=select.value;boundary();};label.append(select);field.append(label);
  }
}
function bindings() {
  const field=$('bindings');field.replaceChildren();
  const legend=document.createElement('legend');legend.textContent='Bindings';field.append(legend);
  for(const [action,list]of Object.entries(draft.settings.bindings)) {
    const label=document.createElement('label');label.textContent=`${action}: `;
    const button=document.createElement('button');button.type='button';button.textContent=list.map(b=>`${b.kind} ${b.name}${b.direction||''}`).join(', ')||'Unbound';
    button.onclick=()=>{capture=action;pollCapture=capturePadBinding(pads);button.textContent='Press a key / pad / axis…';boundary();};
    const clear=document.createElement('button');clear.type='button';clear.textContent='Clear';clear.onclick=()=>{draft.settings.bindings[action]=[];bindings();};
    const reset=document.createElement('button');reset.type='button';reset.textContent='Default';reset.onclick=()=>{draft.settings.bindings[action]=structuredClone(defaults.bindings[action]||[]);bindings();};
    label.append(button,clear,reset);field.append(label);
  }
}
function bind(binding) {
  if(!capture)return;
  const list=draft.settings.bindings[capture];
  // Replace only this physical kind, preserving keyboard + controller alternatives.
  draft.settings.bindings[capture]=[...list.filter(b=>b.kind!==binding.kind),binding];
  capture=null;bindings();boundary();
}
function preferences() {
  if(!game)return;pause();draft={settings:structuredClone(settings),assignments:[...assignments]};
  for(const key of ['volume','dead_zone','height'])$(key).value=settings[key];
  $('fullscreen').checked=settings.fullscreen;
  controllerOptions();bindings();$('preferences').showModal();
}
$('options').onsubmit=async event=>{
  event.preventDefault();
  try{
    for(const key of ['volume','dead_zone','height'])draft.settings[key]=Number($(key).value);
    draft.settings.fullscreen=$('fullscreen').checked;
    draft.settings.width=Math.round(draft.settings.height*game.display_aspect());
    const explicit=draft.assignments.filter(id=>id!=='auto' && id!=='none');
    if(new Set(explicit).size!==explicit.length)throw new Error('Assign each controller to only one player');
    game.configure(JSON.stringify(draft.settings));
    try { localStorage.setItem(settingsKey,JSON.stringify(draft)); }
    catch(error) { game.configure(JSON.stringify(settings)); throw error; }
    settings=draft.settings;assignments=draft.assignments;
    // Resume audio inside the submit gesture, before awaiting fullscreen.
    if(started)audio.resume().catch(report);
    if(settings.fullscreen && !document.fullscreenElement)await $('game-panel').requestFullscreen();
    if(!settings.fullscreen && document.fullscreenElement)await document.exitFullscreen();
    capture=null;$('preferences').close();boundary();draw();
  }catch(error){report(`Settings failed: ${error}`);}
};
$('cancel').onclick=()=>{capture=null;$('preferences').close();boundary();};
$('preferences').addEventListener('cancel',()=>{capture=null;boundary();});
$('preferences').addEventListener('close',()=>{if(started && !restoringState)resumeGame().catch(report);});
$('settings').onclick=preferences;$('pause').onclick=pause;
$('save-state').onclick=async()=>{
  if(savingState || restoringState)return;
  savingState=true;$('save-state').disabled=true;
  try {localStorage.setItem(stateKey,await stateSnapshot.capture());$('error').textContent='';$('state-status').textContent='State saved.';$('restore-state').disabled=false;}
  catch(error){report(`Save state failed: ${error}`);}
  finally{savingState=false;$('save-state').disabled=restoringState;}
};
$('restore-state').onclick=async()=>{
  if(restoringState)return;
  let candidate;
  try {
    const saved=localStorage.getItem(stateKey);if(!saved)throw Error('No saved state');
    pause();restoringState=true;
    for(const id of ['start','pause','reset','settings','save-state','restore-state'])$(id).disabled=true;
    $('state-status').textContent='Restoring state…';
    candidate=await stateSnapshot.restore(saved);
    // Restore the simulation independently of current browser preferences.
    candidate.game.configure(JSON.stringify(settings));
    game.free();stateSnapshot=candidate;game=candidate.game;candidate=null;
    ticks=stateSnapshot.ticks;boundary();draw();$('screen').dataset.ticks=String(ticks);
    $('error').textContent='';$('state-status').textContent='';gameMessage('State restored. Press Start to resume');
  } catch(error){if(candidate)candidate.runtime.free();$('state-status').textContent='Restore failed; current game retained.';report(`Restore state failed: ${error}`);}
  finally {restoringState=false;for(const id of ['start','pause','reset','settings','save-state','restore-state'])$(id).disabled=false;}
};
$('reset').onclick=()=>{flush();stopAudio();window.location.reload();};
window.addEventListener('keydown',event=>{
  if(capture){event.preventDefault();if(event.key==='Escape'){capture=null;bindings();}else bind({kind:'key',name:keyName(event)});return;}
  const shortcuts={F1:'start',F2:'pause',F3:'reset',F4:'save-state',F5:'restore-state',F6:'settings'};
  const target=shortcuts[event.key];
  if(target && !event.ctrlKey && !event.altKey && !event.metaKey && !event.shiftKey){
    event.preventDefault();
    if(event.repeat || restoringState)return;
    if($('preferences').open){
      if(event.key==='F6')$('cancel').click();
      return;
    }
    if(!$(target).disabled)$(target).click();
    return;
  }
  if($('preferences').open)return;
  if(['ArrowUp','ArrowDown','ArrowLeft','ArrowRight',' ','Backspace','Tab'].includes(event.key))event.preventDefault();
  if(!event.repeat){const name=keyName(event);keys.add(name);keyCodes.set(event.code,name);}
});
window.addEventListener('keyup',event=>{keys.delete(keyCodes.get(event.code)||keyName(event));keyCodes.delete(event.code);});
// Release held inputs and reset timing without pausing on browser focus changes.
window.addEventListener('blur',boundary);document.addEventListener('visibilitychange',boundary);
window.addEventListener('pagehide',()=>{flush();stopAudio();});
$('start').onclick=async()=>{
  try {
    const resume=resumeGame();
    if(settings.fullscreen && !document.fullscreenElement){
      try{await $('game-panel').requestFullscreen();}catch(error){report(`Fullscreen unavailable: ${error}`);}
    }
    await resume;
  }catch(error){report(error);}
};

async function checkedFetch(path) {const response=await fetch(path);if(!response.ok)throw new Error(`${path}: HTTP ${response.status}`);return response;}
async function boot(){
  config=await(await checkedFetch('/config.json')).json();$('title').textContent=config.title;document.title=config.title;
  await init();audio=new AudioContext({sampleRate:48000});
  const rom=new Uint8Array(await(await checkedFetch('/rom')).arrayBuffer());
  const metadata=JSON.stringify({enabled:config.assets.enabled,manifests:config.assets.manifests,files:{}});
  const files=[];
  for(const [name,url]of Object.entries(config.assets.files))files.push([name,new Uint8Array(await(await checkedFetch(url)).arrayBuffer())]);
  const wasmHash=await digest(await(await checkedFetch('./game_bg.wasm')).arrayBuffer());
  const fileHashes=await Promise.all(files.map(async([name,bytes])=>[name,await digest(bytes)]));
  const identity=JSON.stringify([config.rom_sha256,wasmHash,audio.sampleRate,metadata,fileHashes]);
  stateKey=`romatlas:${config.rom_sha256}:save-state:v1`;
  stateSnapshot=new StateSnapshot(()=>{
    const assets=new BrowserAssets(metadata);
    for(const [name,bytes]of files)assets.add_file(name,bytes);
    return new BrowserGame(rom,assets,audio.sampleRate);
  },identity);
  game=stateSnapshot.game;
  defaults=JSON.parse(game.settings());settings=structuredClone(defaults);
  settingsKey=`romatlas:${config.rom_sha256}:settings:v1`;
  try{const saved=localStorage.getItem(settingsKey);if(saved){const parsed=JSON.parse(saved);if(!Array.isArray(parsed.assignments)||parsed.assignments.length!==4||!parsed.assignments.every(v=>typeof v==='string'))throw new Error('Invalid controller assignments');delete parsed.settings.integer_scale;game.configure(JSON.stringify(parsed.settings));settings=parsed.settings;assignments=parsed.assignments;}}
  catch(error){report(`Settings load failed: ${error}`);}
  const version=game.save_version();saveKey=version===undefined?null:`romatlas:${config.rom_sha256}:battery:v${version}`;
  if(saveKey){try{const saved=localStorage.getItem(saveKey);if(saved)game.restore(saved);}catch(error){throw new Error(`Battery save load failed: ${error}`);}}
  game.advance(0,JSON.stringify([empty(),empty(),empty(),empty()]));draw();
  $('display-aspect').textContent=`${game.display_aspect()===4/3?'4:3':game.display_aspect()===16/9?'16:9':game.display_aspect().toFixed(3)} display`;
  $('save-state').disabled=!game.snapshots_supported();
  try{$('restore-state').disabled=!game.snapshots_supported() || !localStorage.getItem(stateKey);}catch(error){report(`Save state storage unavailable: ${error}`);}
  $('start').disabled=false;$('reset').disabled=false;$('settings').disabled=false;$('status').textContent='Ready — press Start to enable audio and play';
  requestAnimationFrame(loop);
}
boot().catch(report);
