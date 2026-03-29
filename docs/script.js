import init, { generate_sudoku, solve_sudoku} from './wasm/big_sudoku.js';
import initBrute, {solve} from './wasm/brute_force_big_sudoku.js';
await init();
await initBrute();

let currentBoard = null;
let currentSize = 0;

let isSolved = false;

const runDlxBtn = document.getElementById('runDlxBtn');
const runBruteBtn = document.getElementById('runBruteBtn');
const resetBtn = document.getElementById('resetBtn');
const output = document.getElementById('output');
const countLbl = document.getElementById('countLbl');
const sizeSelect = document.getElementById('sizeSelect');
const userPercentage = document.getElementById('userPercentage');
const generateBtn = document.getElementById('generateBtn');
const runControls = document.getElementById('runControls');

 //https://www.w3schools.com/jsref/prop_html_innerhtml.asp
 //https://www.w3schools.com/html/html_scripts.asp 
 //https://stackoverflow.com/questions/63755989/how-to-link-html-css-and-js-together 

//Generate button
//Once values are selected in dropdown and percent textbox, create puzzles
generateBtn.addEventListener('click', () =>{
    const size = parseInt(sizeSelect.value);
    if (size === 0) {
        return;
    }

    const missingPercent = parseFloat(userPercentage.value);

    let num = Number(missingPercent);

    if (isNaN(missingPercent)|| !Number.isInteger(num)) {
        return;
    }

    const seed = Math.floor(Math.random() * 1000000);

   // https://developer.mozilla.org/en-US/docs/Web/API/Performance/now
    
    currentBoard = generate_sudoku(size, missingPercent/100, seed);
    currentSize = size;

    generateBoard(currentSize, currentBoard.init_grid);
    console.log('init_grid:', currentBoard.init_grid);  // add this
    console.log('solution:', currentBoard.solution); 
    runControls.classList.remove('hidden');
})


 //DLX button
 //Runs DLX algorithm on puzzle in output box
runDlxBtn.addEventListener('click', () => {
    const start = performance.now();
    const solution = solve_sudoku(currentSize, currentBoard.init_grid);
    const end = performance.now();
    
    generateBoard(currentSize, solution);
    isSolved = true;

    const elaspedTime = end - start;
    countLbl.textContent = 'Ran for ' + elaspedTime + ' milliseconds';
    resetBtn.textContent= "Go Back";
});

//Brute Force button
//Runs brute force algorithm on puzzle in output box
runBruteBtn.addEventListener('click', () => {
    //Brute force algo uses Uint8 while DLX uses Uint32so conversion is required
    const board = new Uint8Array(currentBoard.init_grid);

    const start = performance.now();
    const solution = solve(currentSize, board);
    const end = performance.now();

    generateBoard(currentSize, solution);
    isSolved = true;
    const elaspedTime = end - start;
    countLbl.textContent = 'Ran for ' + elaspedTime + ' milliseconds';
    resetBtn.textContent= "Go Back";
});

//Reset Button
//Resets all values and buttons to default
resetBtn.addEventListener('click', () => {
    if(isSolved){
        generateBoard(currentSize, currentBoard.init_grid);
        countLbl.textContent = '';
        isSolved = false;
        resetBtn.textContent = 'Reset';
    }else{
    output.style.display = 'block';
    output.style.gridTemplateColumns = '';
    output.style.padding = '1.5rem';
    countLbl.textContent = '';
    output.innerHTML = '';
    sizeSelect.value = '0';
    userPercentage.value = '';
    runControls.classList.add('hidden');
    currentBoard = null;
}
});

function generateBoard(size, cells) {
    //Dynamic font sizing
    //https://stackoverflow.com/questions/14431411/pure-css-to-make-font-size-responsive-based-on-dynamic-amount-of-characters

    output.style.display = 'grid';
    output.style.gridTemplateColumns = `repeat(${size}, 1fr)`;
    output.style.padding = '0';
    output.innerHTML = '';

    const subgridSize = Math.sqrt(size);

    cells.forEach((value,index) =>{
        const row = Math.floor(index / size);
        const col = index % size; 
        const cell = document.createElement('div');
        const cellSize = output.clientWidth / size;      
        const fontSize = Math.max(8, cellSize * 0.5); 
        cell.classList.add('grid-cell');
        cell.style.fontSize = `${fontSize}px`;  
        cell.textContent = value === 0 ? '' : value;

        // adds thick border on the right edge of each subgrid column
        if ((col + 1) % subgridSize === 0 && col + 1 !== size) {
            cell.classList.add('thick-right');
        }

        // adds thick border on the bottom edge of each subgrid row
        if ((row + 1) % subgridSize === 0 && row + 1 !== size) {
            cell.classList.add('thick-bottom');
        }

        output.appendChild(cell);
    });
}
