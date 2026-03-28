import init, { generate_sudoku } from '../backend/DLX/big_sudoku.js';
await init();

let currentBoard = null;
let currentSize = 0;

let isSolved = false;

let startDLX = 0;
let endDLX = 0;
let elaspedTimeDLX = 0;

let startBrute = 0;
let endBrute = 0;
let elaspedTimeBrute = 0;

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
    startDLX = performance.now();
    currentBoard = generate_sudoku(size, missingPercent/100, seed);
    endDLX = performance.now();

    elaspedTimeDLX = endDLX - startDLX;
    currentSize = size;

    generateBoard(currentSize, currentBoard.init_grid);
    runControls.classList.remove('hidden');
})


 //DLX button
 //Runs DLX algorithm on puzzle in output box
runDlxBtn.addEventListener('click', () => {
    const size = parseInt(sizeSelect.value);
    countLbl.textContent = 'Ran for ' + elaspedTimeDLX; 
    generateBoard(currentSize, currentBoard.solution);
    isSolved = true;
    resetBtn.textContent= "Go Back";
});

//Brute Force button
//Runs brute force algorithm on puzzle in output box
runBruteBtn.addEventListener('click', () => {
    const size = parseInt(sizeSelect.value);
    countLbl.textContent = 'Ran for ';

    isSolved = true;
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
