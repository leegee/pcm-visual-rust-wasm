// A dependency graph that contains any wasm must all be imported
// asynchronously. This `bootstrap.js` file does the single async import, so
// that no one else needs to worry about it again.

console.log('Enter bootstrap.ts');

import './PcmRealtime';
import './PcmOnload';

// import("../pkg/pcm_visual").then(module => {
//   console.log('Enter bootstrap.ts imported pkg/pcm_visual');
//   module.run('Oh dear');
// }).catch(e => console.error("Error importing `index`:", e));
