import init, { generate_sudoku, solve_sudoku} from 'wasm/big_sudoku.js';
import initBrute, {solve} from 'wasm/brute_force_big_sudoku.js';
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
const expandBtn = document.getElementById('expandBtn');
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

    if (isNaN(missingPercent)) {
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
    output.classList.remove('expanded');
    expandBtn.textContent = 'Expand All';
    currentBoard = null;
}
});

//Expand button
//Expands/Minimizes the output box
expandBtn.addEventListener('click', ()=> {
    output.classList.toggle('expanded');
    if(output.classList.contains('expanded')){
        expandBtn.textContent = 'Collapse';
    } else{
        expandBtn.textContent = 'Expand All';
    }

});

function generateBoard(size, cells) {
    output.style.display = 'grid';
    output.style.gridTemplateColumns = `repeat(${size}, 1fr)`;
    output.style.padding = '0';
    output.innerHTML = '';

    cells.forEach(value => {
        const cell = document.createElement('div');
        cell.classList.add('grid-cell');
        cell.textContent = value === 0 ? '' : value;
        output.appendChild(cell);
    });
}
