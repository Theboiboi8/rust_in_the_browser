import { Universe, Cell } from "wasm_test";
import { memory } from "wasm_test/wasm_test_bg";

const urlQuery = window.location.search;
const searchParams = new URLSearchParams(urlQuery);

const canvas = document.getElementById('game-of-life-canvas');
const universe = searchParams.get('random') == null ? Universe.new() : Universe.random();
const width = universe.width();
const height = universe.height();

const CELL_SIZE = 8; // px
const GRID_COLOR = "#191f26";
const DEAD_COLOR = "#0f1318";
const ALIVE_COLOR = "#36a3d9";

canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const context = canvas.getContext('2d');

if (searchParams.get('random') != null) {
    let button = document.getElementById('rand_button');
    button.setAttribute('href', 'game-of-life.html');
    button.setAttribute('class', "rand_link rand_active");
}

let reload_button = document.getElementById('reload_button');
reload_button.addEventListener('click', (_ => {
    window.location.reload();
}))

const drawGrid = () => {
    context.beginPath();
    context.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        context.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        context.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let j = 0; j <= height; j++) {
        context.moveTo(0,                           j * (CELL_SIZE + 1) + 1);
        context.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    context.stroke();
};

const getIndex = (row, column) => {
    return row * width + column;
};

const drawCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    context.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);

            context.fillStyle = cells[idx] === Cell.Dead
                ? DEAD_COLOR
                : ALIVE_COLOR;

            context.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    context.stroke();
};

const renderLoop = () => {
    universe.tick();

    drawGrid();
    drawCells();

    requestAnimationFrame(renderLoop);
}

drawGrid();
drawCells();
requestAnimationFrame(renderLoop);