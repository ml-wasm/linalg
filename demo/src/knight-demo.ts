import * as Comlink from 'comlink';
import './style.css'
import { KnightDemo } from './workers/knight-demo-worker';

const WrappedKnight = Comlink.wrap<typeof KnightDemo>(new Worker(
  new URL('./workers/knight-demo-worker.ts', import.meta.url),
  { type: 'module' }
));

const demo = await new WrappedKnight();
await demo.init();

await demo.test();
