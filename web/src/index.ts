import { draw_triangle } from 'hyper-webgl';

const CANVAS_ID = 'triangle';
let canvas: HTMLCanvasElement = document.getElementById(CANVAS_ID) as HTMLCanvasElement;
if (canvas) {
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;
}

async function run() {
  // await init();
  const color = new Float32Array([1.0, 0.0, 0.0, 1.0]);
  draw_triangle(CANVAS_ID, color);
}

run();

setInterval(() => {
  const color = new Float32Array([
    1.0,
    0.0,
    0.0,
    1.0
  ]);
  
  draw_triangle(CANVAS_ID, color);
}, 16);
