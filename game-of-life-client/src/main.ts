import init, { Universe, InitOutput } from "../pkg/game_of_life.js";

const CELL_SIZE = 6; // px per cell
const GRID_COLOR = "#333";
const DEAD_COLOR = "#111";
const ALIVE_COLOR = "#00ff88";
const UNIVERSE_WIDTH = 128;
const UNIVERSE_HEIGHT = 128;

// Preset patterns defined as [row, col] offsets relative to center
type Pattern = [number, number][];

const PRESETS: Record<string, Pattern | "random"> = {
  random: "random",
  clear: [],
  glider: [
    [0, 1],
    [1, 2],
    [2, 0],
    [2, 1],
    [2, 2],
  ],
  "glider-gun": [
    // Gosper Glider Gun
    [0, 24],
    [1, 22],
    [1, 24],
    [2, 12],
    [2, 13],
    [2, 20],
    [2, 21],
    [2, 34],
    [2, 35],
    [3, 11],
    [3, 15],
    [3, 20],
    [3, 21],
    [3, 34],
    [3, 35],
    [4, 0],
    [4, 1],
    [4, 10],
    [4, 16],
    [4, 20],
    [4, 21],
    [5, 0],
    [5, 1],
    [5, 10],
    [5, 14],
    [5, 16],
    [5, 17],
    [5, 22],
    [5, 24],
    [6, 10],
    [6, 16],
    [6, 24],
    [7, 11],
    [7, 15],
    [8, 12],
    [8, 13],
  ],
  pulsar: (() => {
    const offsets: Pattern = [];
    const rows = [
      [-6, [-4, -3, -2, 2, 3, 4]],
      [-4, [-6, -1, 1, 6]],
      [-3, [-6, -1, 1, 6]],
      [-2, [-6, -1, 1, 6]],
      [-1, [-4, -3, -2, 2, 3, 4]],
      [1, [-4, -3, -2, 2, 3, 4]],
      [2, [-6, -1, 1, 6]],
      [3, [-6, -1, 1, 6]],
      [4, [-6, -1, 1, 6]],
      [6, [-4, -3, -2, 2, 3, 4]],
    ] as [number, number[]][];
    for (const [r, cols] of rows) {
      for (const c of cols) offsets.push([r, c]);
    }
    return offsets;
  })(),
  lwss: [
    // Lightweight Spaceship
    [0, 1],
    [0, 4],
    [1, 0],
    [2, 0],
    [2, 4],
    [3, 0],
    [3, 1],
    [3, 2],
    [3, 3],
  ],
  acorn: [
    [0, 1],
    [1, 3],
    [2, 0],
    [2, 1],
    [2, 4],
    [2, 5],
    [2, 6],
  ],
  "r-pentomino": [
    [0, 1],
    [0, 2],
    [1, 0],
    [1, 1],
    [2, 1],
  ],
  diehard: [
    [0, 6],
    [1, 0],
    [1, 1],
    [2, 1],
    [2, 5],
    [2, 6],
    [2, 7],
  ],
};

let currentPreset = "random";

function buildCellArray(presetName: string): Uint8Array | null {
  const preset = PRESETS[presetName];
  if (preset === "random") return null; // use Universe.new() default
  const cells = new Uint8Array(UNIVERSE_WIDTH * UNIVERSE_HEIGHT);
  const centerRow = Math.floor(UNIVERSE_HEIGHT / 2);
  const centerCol = Math.floor(UNIVERSE_WIDTH / 2);
  for (const [dr, dc] of preset) {
    const r = centerRow + dr;
    const c = centerCol + dc;
    if (r >= 0 && r < UNIVERSE_HEIGHT && c >= 0 && c < UNIVERSE_WIDTH) {
      cells[r * UNIVERSE_WIDTH + c] = 1;
    }
  }
  return cells;
}

function createUniverse(presetName: string): Universe {
  const cells = buildCellArray(presetName);
  if (cells) {
    return Universe.new_with_cells(UNIVERSE_WIDTH, UNIVERSE_HEIGHT, cells);
  }
  return Universe.new(UNIVERSE_WIDTH, UNIVERSE_HEIGHT);
}

let universe: Universe;
let wasmMemory: WebAssembly.Memory;
let animationId: number | null = null;
let playing = false;

const canvas = document.getElementById("game-canvas") as HTMLCanvasElement;
const ctx = canvas.getContext("2d")!;

canvas.width = CELL_SIZE * UNIVERSE_WIDTH + 1;
canvas.height = CELL_SIZE * UNIVERSE_HEIGHT + 1;

function drawGrid(): void {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;
  ctx.lineWidth = 0.5;

  for (let i = 0; i <= UNIVERSE_WIDTH; i++) {
    const x = i * CELL_SIZE + 0.5;
    ctx.moveTo(x, 0);
    ctx.lineTo(x, canvas.height);
  }
  for (let j = 0; j <= UNIVERSE_HEIGHT; j++) {
    const y = j * CELL_SIZE + 0.5;
    ctx.moveTo(0, y);
    ctx.lineTo(canvas.width, y);
  }
  ctx.stroke();
}

function drawCells(): void {
  const cellsPtr = universe.cells_ptr();
  const cells = new Uint8Array(
    wasmMemory.buffer,
    cellsPtr,
    UNIVERSE_WIDTH * UNIVERSE_HEIGHT,
  );

  // Batch alive and dead cells into two fills for performance
  ctx.beginPath();
  ctx.fillStyle = ALIVE_COLOR;
  for (let row = 0; row < UNIVERSE_HEIGHT; row++) {
    for (let col = 0; col < UNIVERSE_WIDTH; col++) {
      if (cells[row * UNIVERSE_WIDTH + col] === 1) {
        ctx.fillRect(
          col * CELL_SIZE + 1,
          row * CELL_SIZE + 1,
          CELL_SIZE - 1,
          CELL_SIZE - 1,
        );
      }
    }
  }

  ctx.beginPath();
  ctx.fillStyle = DEAD_COLOR;
  for (let row = 0; row < UNIVERSE_HEIGHT; row++) {
    for (let col = 0; col < UNIVERSE_WIDTH; col++) {
      if (cells[row * UNIVERSE_WIDTH + col] === 0) {
        ctx.fillRect(
          col * CELL_SIZE + 1,
          row * CELL_SIZE + 1,
          CELL_SIZE - 1,
          CELL_SIZE - 1,
        );
      }
    }
  }
}

function renderLoop(): void {
  universe.tick();
  drawGrid();
  drawCells();
  animationId = requestAnimationFrame(renderLoop);
}

function play(): void {
  playing = true;
  playPauseBtn.textContent = "⏸ Pause";
  renderLoop();
}

function pause(): void {
  playing = false;
  playPauseBtn.textContent = "▶ Play";
  if (animationId !== null) {
    cancelAnimationFrame(animationId);
    animationId = null;
  }
}

// Click-and-drag to paint/erase cells
let isDragging = false;
let paintValue: 0 | 1 = 1; // 1 = alive, 0 = dead (set on mousedown based on toggled cell)

function getCellCoords(event: MouseEvent): [number, number] {
  const rect = canvas.getBoundingClientRect();
  const scaleX = canvas.width / rect.width;
  const scaleY = canvas.height / rect.height;
  const canvasX = (event.clientX - rect.left) * scaleX;
  const canvasY = (event.clientY - rect.top) * scaleY;
  return [Math.floor(canvasY / CELL_SIZE), Math.floor(canvasX / CELL_SIZE)];
}

function setCellValue(row: number, col: number, value: 0 | 1): void {
  if (row < 0 || row >= UNIVERSE_HEIGHT || col < 0 || col >= UNIVERSE_WIDTH)
    return;
  const cellsPtr = universe.cells_ptr();
  const cells = new Uint8Array(
    wasmMemory.buffer,
    cellsPtr,
    UNIVERSE_WIDTH * UNIVERSE_HEIGHT,
  );
  cells[row * UNIVERSE_WIDTH + col] = value;
}

canvas.addEventListener("mousedown", (event) => {
  isDragging = true;
  const [row, col] = getCellCoords(event);
  if (row < 0 || row >= UNIVERSE_HEIGHT || col < 0 || col >= UNIVERSE_WIDTH)
    return;

  // Toggle the first cell and use its new value as the paint value for the drag
  const cellsPtr = universe.cells_ptr();
  const cells = new Uint8Array(
    wasmMemory.buffer,
    cellsPtr,
    UNIVERSE_WIDTH * UNIVERSE_HEIGHT,
  );
  const idx = row * UNIVERSE_WIDTH + col;
  paintValue = cells[idx] === 1 ? 0 : 1;
  cells[idx] = paintValue;

  drawGrid();
  drawCells();
});

canvas.addEventListener("mousemove", (event) => {
  if (!isDragging) return;
  const [row, col] = getCellCoords(event);
  setCellValue(row, col, paintValue);
  drawGrid();
  drawCells();
});

canvas.addEventListener("mouseup", () => {
  isDragging = false;
});

canvas.addEventListener("mouseleave", () => {
  isDragging = false;
});

// Controls
const playPauseBtn = document.getElementById("play-pause") as HTMLButtonElement;
const stepBtn = document.getElementById("step") as HTMLButtonElement;
const resetBtn = document.getElementById("reset") as HTMLButtonElement;

playPauseBtn.addEventListener("click", () => {
  if (playing) {
    pause();
  } else {
    play();
  }
});

stepBtn.addEventListener("click", () => {
  universe.tick();
  drawGrid();
  drawCells();
});

resetBtn.addEventListener("click", () => {
  universe.free();
  universe = createUniverse(currentPreset);
  drawGrid();
  drawCells();
});

// Preset selection
const presetButtons =
  document.querySelectorAll<HTMLButtonElement>("[data-preset]");

presetButtons.forEach((btn) => {
  btn.addEventListener("click", () => {
    const presetName = btn.dataset.preset!;
    currentPreset = presetName;

    // Update active button styling
    presetButtons.forEach((b) => b.classList.remove("active"));
    btn.classList.add("active");

    // Reset universe with new preset
    universe.free();
    universe = createUniverse(presetName);
    drawGrid();
    drawCells();
  });
});

// Initialize WASM and start
async function start(): Promise<void> {
  const wasm: InitOutput = await init();
  wasmMemory = wasm.memory;
  universe = createUniverse(currentPreset);

  drawGrid();
  drawCells();
}

start();
