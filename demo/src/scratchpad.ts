import * as Comlink from 'comlink';
import './style.css'
import { ScratchPad } from './workers/scratchpad';

const WrappedScratchPad = Comlink.wrap<typeof ScratchPad>(new Worker(
  new URL('./workers/scratchpad.ts', import.meta.url),
  { type: 'module' }
));

const demo = await new WrappedScratchPad();
await demo.init();
await demo.parallelComparison();
await demo.test();
