import init, { World } from "snake_game";

init().then((_) => {
  const CELL_SIZE = 40;
  const WORLD_WIDTH = 8;
  const SNAKE_SPAWN_IDX = 20;

  const world = World.new(WORLD_WIDTH, SNAKE_SPAWN_IDX);

  const worldWidth = world.width();

  const canvas = <HTMLCanvasElement> document.getElementById("snake-canvas");
  const ctx = canvas.getContext("2d");
  canvas.height = worldWidth * CELL_SIZE;
  canvas.width = worldWidth * CELL_SIZE;

  document.addEventListener("keydown", e => {
    switch (e.code) {
      case "ArrowUp":
        world.change_state(1); 
        break;
      case "ArrowRight":
        world.change_state(2);
        break;
      case "ArrowDown":
        world.change_state(3);
        break;
      case "ArrowLeft":
        world.change_state(0);
        break;
    }
  })

  function drawWorld() {
    ctx?.beginPath();

    for (let x = 0; x < worldWidth + 1; x++) {
      ctx?.moveTo(CELL_SIZE * x, 0);
      ctx?.lineTo(CELL_SIZE * x, worldWidth * CELL_SIZE);
    }

    for (let y = 0; y < worldWidth + 1; y++) {
      ctx?.moveTo(0, CELL_SIZE * y);
      ctx?.lineTo(worldWidth * CELL_SIZE, CELL_SIZE * y);
    }

    ctx?.stroke();
  }

  function drawSnake() {
    const snakeIdx = world.snake_head_idx();
    const col = snakeIdx % worldWidth;
    const row = Math.floor(snakeIdx / worldWidth);

    // @ts-ignore
    ctx?.fillStyle = "#000";

    ctx?.beginPath();
    ctx?.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);

    ctx?.stroke();
  }

  function paint() {
    drawApple();
    drawSnake();
    drawWorld();
  }

  function drawApple(){
    const apple_idx = world.get_apple();

    const col = apple_idx % worldWidth;
    const row = Math.floor(apple_idx / worldWidth)

    ctx?.beginPath();
    // @ts-ignore
    ctx?.fillStyle = "#FF0000";
    ctx?.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);

    ctx?.stroke();
  }

  function update() {
    setTimeout(() => {
      ctx?.clearRect(0, 0, canvas.width, canvas.height);
      world.update();
      paint();
      // This animation frame takes a callback to invoke before the next repaint
      requestAnimationFrame(update);
    }, 100);
  }

  paint();
  update();
});
