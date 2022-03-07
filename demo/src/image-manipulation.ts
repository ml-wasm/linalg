import './styles/image-manipulation.css'

import * as Comlink from 'comlink';
import * as jpeg from 'jpeg-js';
import * as buffer from 'buffer';
import { ImageManipulation } from './workers/image-manipulation';

(window as any).Buffer = buffer.Buffer;

const WrappedImageManipulation = Comlink.wrap<typeof ImageManipulation>(new Worker(
  new URL('./workers/image-manipulation.ts', import.meta.url),
  { type: 'module' }
));

const demo = await new WrappedImageManipulation();
await demo.init();

const fileInput: HTMLInputElement | null = document.querySelector(".controls > .input-image")

class ImageData {
  data: Uint8Array;
  height: number;
  width: number;
  constructor(buffer: ArrayBuffer) {
    const imageData = jpeg.decode(buffer, { formatAsRGBA: false });
    console.log(imageData);

    this.data = imageData.data;
    this.height = imageData.height;
    this.width = imageData.width;
  }
  writeToImage(element: HTMLImageElement) {

  }
}

let image: ImageData | null = null;

fileInput?.addEventListener('change', async function() {
  const files = fileInput.files;
  if (files === null || files.length === 0) {
    return;
  }

  const file = files[0];
  image = new ImageData(await file.arrayBuffer());
  return;
})
