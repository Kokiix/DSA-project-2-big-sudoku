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

let count = 0;

 //https://www.w3schools.com/jsref/prop_html_innerhtml.asp
 //https://www.w3schools.com/html/html_scripts.asp 
 //https://stackoverflow.com/questions/63755989/how-to-link-html-css-and-js-together 

//Generate button
//Once values are selected in dropdown and percent textbox,
//create puzzles
generateBtn.addEventListener('click', () =>{
    const size = parseInt(sizeSelect.value);
    if (size === 0) {
        return;
    }

    const missingPercent = parseFloat(userPercentage.value);

    if (isNaN(missingPercent)) {
        return;
    }

   generateBoard(size,missingPercent);
   runControls.classList.remove('hidden');
})


 //DLX button
 //Runs DLX algorithm on puzzle in output box
runDlxBtn.addEventListener('click', () => {
    const size = parseInt(sizeSelect.value);
    count += 1;
    countLbl.textContent = 'Clicked ' + count + ' times.'
   
});

//Brute Force button
//Runs brute force algorithm on puzzle in output box
runBruteBtn.addEventListener('click', () => {
    const size = parseInt(sizeSelect.value);
    count += 1;
    countLbl.textContent = 'Clicked ' + count + ' times.'
});

//Reset Button
//Resets all values and buttons to default
resetBtn.addEventListener('click', () => {
    output.style.display = 'block';
    output.style.gridTemplateColumns = '';
    output.style.padding = '1.5rem';
    countLbl.textContent = '';
    output.innerHTML = '';
    count = 0;
    sizeSelect.value = '0';
    userPercentage.value = '';
    runControls.classList.add('hidden');
    output.classList.remove('expanded');
    expandBtn.textContent = 'Expand All';
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

function generateBoard(size, missingPercent) {
    output.style.display = 'grid';
    output.style.gridTemplateColumns = `repeat(${size}, 1fr)`;
    output.style.padding = '0';
    output.innerHTML = '';

    for (let row = 0; row < size; row++) {
        for (let col = 0; col < size; col++) {
            const isMissing = Math.random() * 100 < missingPercent;
            const cell = document.createElement('div');
            cell.classList.add('grid-cell');
            cell.textContent = isMissing ? '' : Math.floor(Math.random() * size) + 1;
            output.appendChild(cell);
        }
    }
}


// import init, { generate_sudoku } from './bigSodoku.js';

// await init();

// const board = generate_sodoku(4);
// console.log(board);
