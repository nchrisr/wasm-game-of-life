import { Universe, Cell, InitialState } from "rust-wasm-game-of-life";
import { memory } from "rust-wasm-game-of-life/rust_wasm_game_of_life_bg"

const CELL_SIZE = 10; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const universe = Universe.new(InitialState.SingleShip);
const height = universe.get_height();
const width = universe.get_width();

const canvas = document.getElementById("game-of-life-canvas");
const playPauseButton = document.getElementById("play-pause");
const resetButton = document.getElementById("reset");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const canvasContext = canvas.getContext('2d');

let animationId = null;

const play = () => {
    playPauseButton.textContent = "⏸";
    renderLoop();
};

const pause = () =>  {
    playPauseButton.textContent = "▶";
    cancelAnimationFrame(animationId);
    animationId = null;
}

const isPaused = () => {
    return animationId === null;
};

playPauseButton.addEventListener("click", event => {
    if (isPaused()) {
      play();
    } else {
      pause();
    }
});

resetButton.addEventListener("click", event => {
    pause();
    universe.reset(InitialState.SingleShip)
    play();
})

canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height-1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

    universe.toggle_cell(row, col);

    drawGrid();
    drawCells();
});

const renderLoop = () => {
    //debugger;
    universe.tick();

    drawGrid();
    drawCells();

    animationId = requestAnimationFrame(renderLoop);
};

const drawGrid = () => {
    canvasContext.beginPath();
    canvasContext.strokeStyle = GRID_COLOR;

    // Vertical lines
    for (let i=0; i<= width; i++){
        canvasContext.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        canvasContext.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        canvasContext.moveTo(0,j * (CELL_SIZE + 1) + 1);
        canvasContext.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    canvasContext.stroke();
}

const getIndex = (row, column) => {
    return row * width + column;
};

const drawCells = () => {
    const cellsPtr = universe.get_cells_ptr();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    canvasContext.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);

            canvasContext.fillStyle = cells[idx] === Cell.Dead
                ? DEAD_COLOR
                : ALIVE_COLOR;

            canvasContext.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    canvasContext.stroke();
};

drawGrid();
drawCells();
play();//requestAnimationFrame(renderLoop);