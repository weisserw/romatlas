// Convert browser controls to the shared runtime's physical input names.
const buttons = ['a','b','x','y','leftshoulder','rightshoulder',null,null,'back','start','leftstick','rightstick','dpup','dpdown','dpleft','dpright','guide'];
const axes = ['leftx','lefty','rightx','righty'];
export function gamepads(navigator) {
  const counts = new Map();
  return Array.from(navigator.getGamepads?.() || []).filter(p => p && p.connected !== false).map(pad => {
    const ordinal = counts.get(pad.id) || 0;
    counts.set(pad.id, ordinal + 1);
    return {pad, id: JSON.stringify([pad.id, ordinal])};
  });
}
export function padState(pad) {
  const state = {keys: [], buttons: [], axes: {}};
  pad.buttons.forEach((button, i) => {
    if (button.pressed) state.buttons.push(buttons[i] || `button${i}`);
  });
  pad.axes.forEach((value, i) => { state.axes[axes[i] || `axis${i}`] = value; });
  // Only standard pads guarantee that buttons 6/7 are analog triggers.
  if (pad.mapping === 'standard') {
    state.axes.triggerleft = pad.buttons[6]?.value || 0;
    state.axes.triggerright = pad.buttons[7]?.value || 0;
  }
  return state;
}
// Wait for initially held controls to be released before capturing a new input.
// In particular, idle raw trigger axes can rest at -1 instead of zero.
export function capturePadBinding(readPads) {
  const blocked = new Set();
  function active() {
    return readPads().flatMap(({pad, id}) => {
      const state = padState(pad);
      return [
        ...state.buttons.map(name => ({token: `${id}:button:${name}`, binding: {kind:'button', name}})),
        ...Object.entries(state.axes).filter(([,value]) => Math.abs(value) > .75).map(([name,value]) => ({
          token: `${id}:axis:${name}`, binding: {kind:'axis', name, direction:value < 0 ? -1 : 1}
        }))
      ];
    });
  }
  for (const {token} of active()) blocked.add(token);
  return () => {
    const current = active();
    for (const token of blocked) if (!current.some(input => input.token === token)) blocked.delete(token);
    return current.find(input => !blocked.has(input.token))?.binding;
  };
}
