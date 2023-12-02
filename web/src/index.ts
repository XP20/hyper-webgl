import('hyper-webgl').then(() => {
  const CANVAS_ID = 'canvas';
  let canvas: HTMLCanvasElement = document.getElementById(CANVAS_ID) as HTMLCanvasElement;
  if (canvas) {
    canvas.width = window.innerWidth;
    canvas.height = window.innerHeight;
    const onResize = (event: Event) => {
      canvas.width = window.innerWidth;
      canvas.height = window.innerHeight;
    }
    window.addEventListener("resize", onResize);
  }
}).catch(console.error);
