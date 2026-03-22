const runDlxBtn = document.getElementById('runDlxBtn');
const runBruteBtn = document.getElementById('runBruteBtn');
const resetBtn = document.getElementById('resetBtn');
const output = document.getElementById('output');
const countLbl = document.getElementById('countLbl');

let count = 0;

runDlxBtn.addEventListener('click', () => {
    count += 1;
    output.innerHTML = '<p>Running...</p>';
    countLbl.textContent = 'Clicked ' + count + ' times.'
});

runBruteBtn.addEventListener('click', () => {
    count += 1;
    output.innerHTML = '<p>Running...</p>';
    countLbl.textContent = 'Clicked ' + count + ' times.'
});

resetBtn.addEventListener('click', () => {
    output.innerHTML = '<p class="placeholder">Nothing here yet...</p>';
    countLbl.textContent = '';
    count = 0;
});

expandBtn.addEventListener('click'),()=> {
    

}

import init, { generate_sudoku } from './bigSodoku.js';

await init();

const board = generate_sodoku(4);
console.log(board);
