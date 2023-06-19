document.addEventListener('DOMContentLoaded', () => {
    let count = 0;
    let countSpan = document.querySelector('#count');
    let decreaseButton = document.querySelector('#decrease');
    decreaseButton.addEventListener('click', () => {
        if (count > 0) {
            count -= 1;
            countSpan.innerText = count;
        }
    });
    let increaseButton = document.querySelector('#increase');
    increaseButton.addEventListener('click', () => {
        count += 1;
        countSpan.innerText = count;
    });
});