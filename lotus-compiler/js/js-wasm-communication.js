const EVENT_TYPES = ['keyboard', 'mouse', 'wheel'];
const KEYBOARD_ACTIONS = ['down', 'up'];
const KEYBOARD_CODES = ['Escape', 'F1', 'F2', 'F3', 'F4', 'F5', 'F6', 'F7', 'F8', 'F9', 'F10', 'F11', 'F12', 'Backquote', 'Digit1', 'Digit2', 'Digit3', 'Digit4', 'Digit5', 'Digit6', 'Digit7', 'Digit8', 'Digit9', 'Digit0', 'Minus', 'Equal', 'Backspace', 'Tab', 'KeyQ', 'KeyW', 'KeyE', 'KeyR', 'KeyT', 'KeyY', 'KeyU', 'KeyI', 'KeyO', 'KeyP', 'BracketLeft', 'BracketRight', 'Enter', 'CapsLock', 'KeyA', 'KeyS', 'KeyD', 'KeyF', 'KeyG', 'KeyH', 'KeyJ', 'KeyK', 'KeyL', 'Semicolon', 'Quote', 'Backslash', 'ShiftLeft', 'IntlBackslash', 'KeyZ', 'KeyX', 'KeyC', 'KeyV', 'KeyB', 'KeyN', 'KeyM', 'Comma', 'Period', 'Slash', 'ShiftRight', 'ControlLeft', 'MetaLeft', 'AltLeft', 'Space', 'AltRight', 'ContextMenu', 'ControlRight', 'ArrowUp', 'ArrowLeft', 'ArrowDown', 'ArrowRight', 'Insert', 'Home', 'PageUp', 'Delete', 'End', 'PageDown', 'NumLock', 'NumpadDivide', 'NumpadMultiply', 'NumpadSubstract', 'Numpad7', 'Numpad8', 'Numpad9', 'NumpadAdd', 'Numpad4', 'Numpad5', 'Numpad6', 'Numpad1', 'Numpad2', 'Numpad3', 'NumpadEnter', 'Numpad0', 'NumpadDecimal'];
const MOUSE_ACTIONS = ['move', 'down', 'click', 'up'];
const MOUSE_BUTTONS = ['left', 'middle', 'right'];
const WHEEL_DELTA_MODES = ['pixel', 'line', 'page'];

export function writeWindowEventToBuffer(event, buffer) {
    let { type, payload } = event;
    let eventTypeId = EVENT_TYPES.indexOf(type)

    if (type == 'keyboard') {
        buffer.write(eventTypeId);
        buffer.write(KEYBOARD_ACTIONS.indexOf(payload.action));
        buffer.write(KEYBOARD_CODES.indexOf(payload.code));
        buffer.write(payload.text ? payload.text.charCodeAt(0) : -2147483648);
        buffer.write(payload.ctrlKey);
        buffer.write(payload.shiftKey);
        buffer.write(payload.altKey);
    } else if (type == 'mouse') {
        buffer.write(eventTypeId);
        buffer.write(MOUSE_ACTIONS.indexOf(payload.action));
        buffer.write(MOUSE_BUTTONS.indexOf(payload.button));
        buffer.writeFloat(payload.x);
        buffer.writeFloat(payload.y);
    } else if (type == 'wheel') {
        buffer.write(eventTypeId);
        buffer.writeFloat(payload.deltaX);
        buffer.writeFloat(payload.deltaY);
        buffer.writeFloat(payload.deltaZ);
        buffer.write(WHEEL_DELTA_MODES.indexOf(payload.deltaMode));
    }
}

export function readStringFromBuffer(buffer) {
    let length = buffer.read();
    let hash = buffer.read(); // not needed here
    let codes = new Array(length);

    for (let i = 0; i < length; ++i) {
        codes[i] = buffer.read();
    }

    return String.fromCodePoint(...codes);
}

export function readGraphicsFromBuffer() {

}