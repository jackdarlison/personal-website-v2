const game_canvas = document.getElementById('glcanvas');

function sendKeyDown(key) {
    const event = new KeyboardEvent('keydown', { code: key, bubbles: true });
    game_canvas.dispatchEvent(event);
}

function sendKeyUp(key) {
    const event = new KeyboardEvent('keyup', { code: key, bubbles: true });
    game_canvas.dispatchEvent(event);
}

let max_width = 0;

document.querySelectorAll('.key-button').forEach(button => {
    max_width = Math.max(max_width, button.offsetWidth);

    const key = button.dataset.key;

    button.addEventListener('mousedown', () => sendKeyDown(key));
    button.addEventListener('mouseup', () => sendKeyUp(key));
    button.addEventListener('mouseleave', () => sendKeyUp(key));
    button.addEventListener('touchstart', (e) => {
        e.preventDefault();
        sendKeyDown(key);
    });
    button.addEventListener('touchend', (e) => {
        e.preventDefault();
        sendKeyUp(key);
    });
    button.addEventListener('touchleave', (e) => {
        e.preventDefault();
        sendKeyUp(key);
    });
    button.addEventListener('touchcancel', (e) => {
        e.preventDefault();
        sendKeyUp(key);
    });
});

max_width++;

document.querySelectorAll('.key-button').forEach(button => {
    button.style.width = `${max_width}px`;
});

window.sendKeyDown = sendKeyDown;
window.sendKeyUp = sendKeyUp;
