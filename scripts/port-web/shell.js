import init, { BrowserAssets, BrowserGame } from './game.js';
const $ = id => document.getElementById(id);
const report = error => { $('error').textContent = String(error); console.error(error); };
const empty = () => ({keys:[],buttons:[],axes:{}});
const buttons = ['a','b','x','y','leftshoulder','rightshoulder',null,null,'back','start','leftstick','rightstick','dpup','dpdown','dpleft','dpright','guide'];
const axes = ['leftx','lefty','rightx','righty'];
const keyNames = {ArrowUp:'Up',ArrowDown:'Down',ArrowLeft:'Left',ArrowRight:'Right',Enter:'Return',Escape:'Escape',' ':'Space',Backspace:'Backspace',Tab:'Tab',Shift:'Left Shift',Control:'Left Ctrl',Alt:'Left Alt'};
const keyName = event => ({ShiftRight:'Right Shift',ControlRight:'Right Ctrl',AltRight:'Right Alt',MetaLeft:'Left GUI',MetaRight:'Right GUI',CapsLock:'CapsLock',NumLock:'Numlock',ScrollLock:'ScrollLock',NumpadEnter:'Keypad Enter'})[event.code] || keyNames[event.key] || (event.key.length === 1 ? event.key.toUpperCase() : event.key);
let game, audio, settings, defaults, draft, config, settingsKey, saveKey;
let started=false, paused=true, capture=null, last=null, nextAudio=0, ticks=0;
let assignments = ['auto','auto','auto','auto'];
const keys = new Set(), keyCodes = new Map(), sources = new Set();
let signature='';
function pads() {
  const counts=new Map();
  return [...navigator.getGamepads()].filter(p=>p && p.mapping==='standard').map(p=>{
    const ordinal=counts.get(p.id)||0; counts.set(p.id,ordinal+1);
    return {pad:p,id:JSON.stringify([p.id,ordinal])};
  });
}
function physical() {
  const all=pads(), used=new Set(), reserved=new Set(assignments.filter(id=>id!=='auto' && id!=='none'));
  return assignments.map((id,index)=>{
    const state=empty(); if(index===0) state.keys=[...keys];
    const item=id==='none'?null:id==='auto'?all.find(p=>!used.has(p.id) && !reserved.has(p.id)):all.find(p=>p.id===id && !used.has(p.id));
    if(item) {
      used.add(item.id); const p=item.pad;
      p.buttons.forEach((b,i)=>{if(b.pressed && buttons[i])state.buttons.push(buttons[i]);});
      axes.forEach((name,i)=>state.axes[name]=p.axes[i]||0);
      state.axes.triggerleft=p.buttons[6]?.value||0; state.axes.triggerright=p.buttons[7]?.value||0;
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
function pause() {
  paused=true;boundary();$('status').textContent='Paused — press Start to resume';
  if(audio)audio.suspend().catch(report);
}
function draw() {
  const canvas=$('screen');canvas.width=game.width();canvas.height=game.height();
  canvas.getContext('2d').putImageData(new ImageData(new Uint8ClampedArray(game.pixels()),canvas.width,canvas.height),0,0);
  const scale=Math.min(settings.width/canvas.width,settings.height/canvas.height);
  const factor=settings.integer_scale && scale>=1?Math.floor(scale):scale;
  canvas.style.width=`${canvas.width*factor}px`;canvas.style.height='auto';
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
    if(capture){const state=physical().find(s=>s.buttons.length||Object.values(s.axes).some(v=>Math.abs(v)>.75));
      if(state){if(state.buttons.length)bind({kind:'button',name:state.buttons[0]});else{const [name,value]=Object.entries(state.axes).find(([,v])=>Math.abs(v)>.75);bind({kind:'axis',name,direction:value<0?-1:1});}}}
    if(game && started && !paused && !$('preferences').open && !document.hidden && document.hasFocus()) {
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
    button.onclick=()=>{capture=action;button.textContent='Press a key / pad / axis…';boundary();};
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
  for(const key of ['volume','dead_zone','width','height'])$(key).value=settings[key];
  for(const key of ['integer_scale','fullscreen'])$(key).checked=settings[key];
  controllerOptions();bindings();$('preferences').showModal();
}
$('options').onsubmit=async event=>{
  event.preventDefault();
  try{
    for(const key of ['volume','dead_zone','width','height'])draft.settings[key]=Number($(key).value);
    for(const key of ['integer_scale','fullscreen'])draft.settings[key]=$(key).checked;
    const explicit=draft.assignments.filter(id=>id!=='auto' && id!=='none');
    if(new Set(explicit).size!==explicit.length)throw new Error('Assign each controller to only one player');
    game.configure(JSON.stringify(draft.settings));
    try { localStorage.setItem(settingsKey,JSON.stringify(draft)); }
    catch(error) { game.configure(JSON.stringify(settings)); throw error; }
    settings=draft.settings;assignments=draft.assignments;
    if(settings.fullscreen && !document.fullscreenElement)await $('screen').requestFullscreen();
    if(!settings.fullscreen && document.fullscreenElement)await document.exitFullscreen();
    capture=null;$('preferences').close();boundary();draw();
  }catch(error){report(`Settings failed: ${error}`);}
};
$('cancel').onclick=()=>{capture=null;$('preferences').close();boundary();};
$('preferences').addEventListener('cancel',()=>{capture=null;boundary();});
$('settings').onclick=preferences;$('pause').onclick=pause;
window.addEventListener('keydown',event=>{
  if(capture){event.preventDefault();if(event.key==='Escape'){capture=null;bindings();}else bind({kind:'key',name:keyName(event)});return;}
  if(event.key==='F1'){event.preventDefault();if(!$('preferences').open)preferences();return;}
  if($('preferences').open)return;
  if(['ArrowUp','ArrowDown','ArrowLeft','ArrowRight',' ','Backspace','Tab'].includes(event.key))event.preventDefault();
  if(!event.repeat){const name=keyName(event);keys.add(name);keyCodes.set(event.code,name);}
});
window.addEventListener('keyup',event=>{keys.delete(keyCodes.get(event.code)||keyName(event));keyCodes.delete(event.code);});
window.addEventListener('blur',pause);document.addEventListener('visibilitychange',()=>{if(document.hidden)pause();});
window.addEventListener('pagehide',()=>{flush();stopAudio();});
$('start').onclick=async()=>{
  try{const resume=audio.resume();if(settings.fullscreen && !document.fullscreenElement){try{await $('screen').requestFullscreen();}catch(error){report(`Fullscreen unavailable: ${error}`);}}await resume;if(audio.state!=='running')throw new Error('Audio could not start');boundary();started=true;paused=false;$('pause').disabled=false;$('status').textContent='Running';}
  catch(error){report(error);}
};
async function checkedFetch(path) {const response=await fetch(path);if(!response.ok)throw new Error(`${path}: HTTP ${response.status}`);return response;}
async function boot(){
  config=await(await checkedFetch('/config.json')).json();$('title').textContent=config.title;document.title=config.title;
  await init();audio=new AudioContext({sampleRate:48000});
  const rom=new Uint8Array(await(await checkedFetch('/rom')).arrayBuffer());
  const assets=new BrowserAssets(JSON.stringify({enabled:config.assets.enabled,manifests:config.assets.manifests,files:{}}));
  for(const [name,url]of Object.entries(config.assets.files))assets.add_file(name,new Uint8Array(await(await checkedFetch(url)).arrayBuffer()));
  game=new BrowserGame(rom,assets,audio.sampleRate);
  defaults=JSON.parse(game.settings());settings=structuredClone(defaults);
  settingsKey=`romatlas:${config.rom_sha256}:settings:v1`;
  try{const saved=localStorage.getItem(settingsKey);if(saved){const parsed=JSON.parse(saved);if(!Array.isArray(parsed.assignments)||parsed.assignments.length!==4||!parsed.assignments.every(v=>typeof v==='string'))throw new Error('Invalid controller assignments');game.configure(JSON.stringify(parsed.settings));settings=parsed.settings;assignments=parsed.assignments;}}
  catch(error){report(`Settings load failed: ${error}`);}
  const version=game.save_version();saveKey=version===undefined?null:`romatlas:${config.rom_sha256}:battery:v${version}`;
  if(saveKey){try{const saved=localStorage.getItem(saveKey);if(saved)game.restore(saved);}catch(error){throw new Error(`Battery save load failed: ${error}`);}}
  game.advance(0,JSON.stringify([empty(),empty(),empty(),empty()]));draw();
  $('start').disabled=false;$('settings').disabled=false;$('status').textContent='Ready — press Start to enable audio and play';
  requestAnimationFrame(loop);
}
boot().catch(report);
