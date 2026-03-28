const runDlxBtn = document.getElementById('runDlxBtn');
const runBruteBtn = document.getElementById('runBruteBtn');
const resetBtn = document.getElementById('resetBtn');
const output = document.getElementById('output');
const countLbl = document.getElementById('countLbl');
const expandBtn = document.getElementById('expandBtn');
const sizeSelect = document.getElementById('sizeSelect');
const userPercentage = document.getElementById('userPercentage');
const generateBtn = document.getElementById('generateBtn');


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

    const board = getMockBoard(size, missingPercent);

    let html = '<table class="sudoku-table">';
    for (let row of board) {
        html += '<tr>';
        for (let cell of row) {
            html += `<td>${cell}</td>`;
        }
        html += '</tr>';
    }
    html += '</table>';

    output.innerHTML = html;
})


 //DLX button
 //Runs DLX algorithm on puzzle in output box
runDlxBtn.addEventListener('click', () => {
    const size = parseInt(sizeSelect.value);
    count += 1;
    output.innerHTML = 'Running ' + size;
    countLbl.textContent = 'Clicked ' + count + ' times.'
   
});

//Brute Force button
//Runs brute force algorithm on puzzle in output box
runBruteBtn.addEventListener('click', () => {
    const size = parseInt(sizeSelect.value);
    count += 1;
    output.innerHTML = 'Running ' + size;
    countLbl.textContent = 'Clicked ' + count + ' times.'
});

//Reset Button
//Resets all values to default
resetBtn.addEventListener('click', () => {
    countLbl.textContent = '';
    count = 0;
    sizeSelect.value = '0';
    userPercentage.value = 'Enter percentage';
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


// Fake data that mimics what generate_sudoku() will return
function getMockBoard(size, missingPercent) {
    const board = [];
    for (let row = 0; row < size; row++) {
        const currentRow = [];
        for (let col = 0; col < size; col++) {
            const isMissing = Math.random() * 100 < missingPercent;  // randomly blank cell
            currentRow.push(isMissing ? '' : Math.floor(Math.random() * size) + 1);
        }
        board.push(currentRow);
    }
    return board;
}



// import init, { generate_sudoku } from './bigSodoku.js';

// await init();

// const board = generate_sodoku(4);
// console.log(board);
