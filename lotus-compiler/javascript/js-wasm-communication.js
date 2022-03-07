export const WINDOW_EVENT_TYPES = ['keyboard', 'mouse', 'wheel'];
export const KEYBOARD_ACTIONS = ['down', 'up'];
export const KEYBOARD_CODES = ['Escape', 'F1', 'F2', 'F3', 'F4', 'F5', 'F6', 'F7', 'F8', 'F9', 'F10', 'F11', 'F12', 'Backquote', 'Digit1', 'Digit2', 'Digit3', 'Digit4', 'Digit5', 'Digit6', 'Digit7', 'Digit8', 'Digit9', 'Digit0', 'Minus', 'Equal', 'Backspace', 'Tab', 'KeyQ', 'KeyW', 'KeyE', 'KeyR', 'KeyT', 'KeyY', 'KeyU', 'KeyI', 'KeyO', 'KeyP', 'BracketLeft', 'BracketRight', 'Enter', 'CapsLock', 'KeyA', 'KeyS', 'KeyD', 'KeyF', 'KeyG', 'KeyH', 'KeyJ', 'KeyK', 'KeyL', 'Semicolon', 'Quote', 'Backslash', 'ShiftLeft', 'IntlBackslash', 'KeyZ', 'KeyX', 'KeyC', 'KeyV', 'KeyB', 'KeyN', 'KeyM', 'Comma', 'Period', 'Slash', 'ShiftRight', 'ControlLeft', 'MetaLeft', 'AltLeft', 'Space', 'AltRight', 'ContextMenu', 'ControlRight', 'ArrowUp', 'ArrowLeft', 'ArrowDown', 'ArrowRight', 'Insert', 'Home', 'PageUp', 'Delete', 'End', 'PageDown', 'NumLock', 'NumpadDivide', 'NumpadMultiply', 'NumpadSubstract', 'Numpad7', 'Numpad8', 'Numpad9', 'NumpadAdd', 'Numpad4', 'Numpad5', 'Numpad6', 'Numpad1', 'Numpad2', 'Numpad3', 'NumpadEnter', 'Numpad0', 'NumpadDecimal'];
export const MOUSE_ACTIONS = ['move', 'down', 'click', 'up'];
export const MOUSE_BUTTONS = ['left', 'middle', 'right'];
export const WHEEL_DELTA_MODES = ['pixel', 'line', 'page'];

const NETWORK_EVENT_TYPES = ['open', 'close', 'message'];

export function writeWindowEventToBuffer(event, buffer) {
    let { type, payload } = event;
    let eventTypeId = WINDOW_EVENT_TYPES.indexOf(type)

    if (type == 'keyboard') {
        buffer.write(eventTypeId);
        buffer.writeEnum(payload.action, KEYBOARD_ACTIONS);
        buffer.writeEnum(payload.code, KEYBOARD_CODES);
        buffer.write(payload.text ? payload.text.charCodeAt(0) : -2147483648);
        buffer.write(payload.ctrlKey);
        buffer.write(payload.shiftKey);
        buffer.write(payload.altKey);
    } else if (type == 'mouse') {
        buffer.write(eventTypeId);
        buffer.writeEnum(payload.action, MOUSE_ACTIONS);
        buffer.writeEnum(payload.button, MOUSE_BUTTONS);
        buffer.writeFloat(payload.x);
        buffer.writeFloat(payload.y);
    } else if (type == 'wheel') {
        buffer.write(eventTypeId);
        buffer.writeFloat(payload.deltaX);
        buffer.writeFloat(payload.deltaY);
        buffer.writeFloat(payload.deltaZ);
        buffer.writeEnum(payload.deltaMode, WHEEL_DELTA_MODES);
    }
}

export function writeNetworkEventToBuffer(event, buffer) {
    let { webSocketId, messageType, messagePayload } = event;

    buffer.write(webSocketId);
    buffer.writeEnum(messageType, NETWORK_EVENT_TYPES);
    buffer.writeBuffer(messagePayload);
}

export function readStringFromMemory(memoryAsInt32Array, stringAddr) {
    if (!stringAddr) {
        return null;
    }

    let length = memoryAsInt32Array[stringAddr];
    let codes = new Array(length);

    for (let i = 0; i < length; ++i) {
        codes[i] = memoryAsInt32Array[stringAddr + 2 + i];
    }

    return String.fromCodePoint(...codes);
}