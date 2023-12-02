import { draw_triangle } from 'hyper-webgl';

const CANVAS_ID = 'triangle';
let canvas: HTMLCanvasElement = document.getElementById(CANVAS_ID) as HTMLCanvasElement;
if (canvas) {
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;
}

const def_color = new Float32Array([1.0, 0.0, 0.0, 1.0]);
async function run() {
  // await init();
  draw_triangle(CANVAS_ID, def_color);
}

run();

const onResize = (event: Event) => {
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;
  draw_triangle(CANVAS_ID, def_color);
}
window.addEventListener("resize", onResize);

setInterval(() => {
  const color = new Float32Array([
    1.0,
    0.0,
    0.0,
    1.0
  ]);
  
  draw_triangle(CANVAS_ID, color);
}, 16);
