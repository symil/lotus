import { EventTarget } from './event-target';

const BUTTON_TO_STRING = ['left', 'middle', 'right'];

export class WindowWrapper extends EventTarget {
    constructor() {
        super();

        this.registerEvents(['resize', 'keyDown', 'keyUp', 'mouseInput', 'wheel']);

        this._aspectRatio = 16 / 9;
        this._zIndexToCanvas = new Map();
        this._canvaxX = 0;
        this._canvasY = 0;
        this._canvasWidth = 0;
        this._canvasHeight = 0;

        this._cursorX = 0;
        this._cursorY = 0;
        this._buttons = {};
        this._keys = {};

        this._initialized = false;
    }

    _init() {
        if (this._initialized) {
            return;
        }

        window.addEventListener('resize', () => this._onResize());
        document.addEventListener('mousemove', evt => this._onMouseMove(evt));
        // document.addEventListener('mouseleave', evt => this._onMouseMove(evt));
        document.addEventListener('mousedown', evt => this._onMouseDown(evt));
        document.addEventListener('mouseup', evt => this._onMouseUp(evt));
        document.addEventListener('wheel', evt => this._onWheel(evt));
        document.addEventListener('keydown', evt => this._onKeyDown(evt));
        document.addEventListener('keyup', evt => this._onKeyUp(evt));
        document.addEventListener('visibilitychange', () => this._resetKeys());
        document.addEventListener('contextmenu', evt => evt.preventDefault());

        this._initialized = true;
        this._onResize();
    }

    init() {
        this._init();
    }

    getCanvasContext(zIndex) {
        let data = this._zIndexToCanvas.get(zIndex);

        if (!data) {
            let canvas = document.createElement('canvas');
            let context = canvas.getContext('2d');
            data = { canvas, context };

            this._updateCanvas(canvas);
            this._zIndexToCanvas.set(zIndex, data);
            this._updateDom();
        }
    
        return data.context;
    }

    setAspectRatio(aspectRatio) {
        this._aspectRatio = aspectRatio;
        this._onResize();
    }

    clear() {
        for (let { canvas, context } of this._zIndexToCanvas.values()) {
            context.clearRect(0, 0, canvas.width, canvas.height);
            canvas.style.cursor = 'default';
        }
    }

    setCursor(cursor) {
        for (let { canvas } of this._zIndexToCanvas.values()) {
            canvas.style.cursor = cursor;
        }
    }

    reset() {
        this._zIndexToCanvas.clear();
        this._updateDom();
    }

    _updateCanvasBox() {
        let aspectRatio = this._aspectRatio;
        let width = window.innerWidth;
        let height = window.innerHeight;

        if (height * aspectRatio > width) {
            height = width / aspectRatio;
        } else {
            width = height * aspectRatio;
        }

        let x = (window.innerWidth - width) / 2;
        let y = (window.innerHeight - height) / 2;

        this._canvasX = Math.round(x);
        this._canvasY = Math.round(y);
        this._canvasWidth = Math.round(width);
        this._canvasHeight = Math.round(height);
    }

    _updateCanvas(canvas) {
        // TODO: handle devicePixelRatio
        canvas.width = this._canvasWidth;
        canvas.height = this._canvasHeight;
        canvas.style.position = 'absolute';
        canvas.style.left = `${this._canvasX}px`;
        canvas.style.top = `${this._canvasY}px`;
    }

    _updateDom() {
        if (!document.body) {
            setTimeout(() => this._updateDom(), 0);
            return;
        }

        document.body.style.margin = 0;
        document.body.style.backgroundColor = 'black';

        while (document.body.firstChild) {
            document.body.removeChild(document.body.firstChild);
        }

        let canvasList = Array.from(this._zIndexToCanvas.entries())
            .sort(([zIndex1], [zIndex2]) => zIndex1 - zIndex2)
            .map(([zIndex, { canvas }]) => canvas);
        
        for (let canvas of canvasList) {
            document.body.appendChild(canvas);
        }
    }

    _onResize() {
        if (!this._initialized) {
            return;
        }

        this._updateCanvasBox();

        for (let { canvas } of this._zIndexToCanvas.values()) {
            this._updateCanvas(canvas);
        }

        this._updateDom();

        this.triggerEvent('resize');
    }

    _emit(eventName, payload) {
        this.triggerEvent(eventName, payload);
    }

    _getXY(evt) {
        let x1 = this._canvasBox.x1;
        let y1 = this._canvasBox.y1;
        let realToVirtualRatio = this._virtualWidth / this._canvasBox.width;
        let x = (evt.clientX - x1) * realToVirtualRatio;
        let y = (evt.clientY - y1) * realToVirtualRatio;

        this._realX = evt.clientX - x1;
        this._realY = evt.clientY - y1;

        return { x, y };
    }

    _triggerMouseEvent(payload) {
        this._x = payload.x;
        this._y = payload.y;

        if (payload.action === 'down') {
            this._buttons[payload.button] = true;
        } else if (payload.action === 'up') {
            this._buttons[payload.button] = false;
        }

        this._emit('mouseInput', payload);
    }

    _onMouseMove(evt) {
        this._triggerMouseEvent({
            action: 'move',
            ...this._getXY(evt),
            button: null,
            domEvent: evt
        });
    }

    _onMouseDown(evt) {
        this._triggerMouseEvent({
            action: 'down',
            ...this._getXY(evt),
            button: BUTTON_TO_STRING[evt.button],
            domEvent: evt
        });
    }

    _onMouseUp(evt) {
        this._triggerMouseEvent({
            action: 'up',
            ...this._getXY(evt),
            button: BUTTON_TO_STRING[evt.button],
            domEvent: evt
        });
    }

    _onKeyDown(evt) {
        this._keys[evt.code] = true;

        this.triggerEvent('keyDown', evt);
    }

    _onKeyUp(evt) {
        this._keys[evt.code] = false;

        this.triggerEvent('keyUp', evt);
    }

    _onWheel(evt) {
        this.triggerEvent('wheel', evt);
    }

    _resetKeys() {
        for (let [code, pressed] of Object.entries(this._keys)) {
            if (pressed) {
                let evt = {
                    code,
                    key: '',
                    repeat: false,
                    altKey: false,
                    ctrlKey: false,
                    metaKey: false,
                    shiftKey: false,
                    domEvent: null
                };

                this._keys[code] = false;

                this.triggerEvent('keyUp', evt);
            }
        }
    }

    emit(payload) {
        this._triggerMouseEvent(payload);
    }

    emitDummyMove() {
        this._triggerMouseEvent({
            action: 'move',
            x: this._x,
            y: this._y,
            button: null,
            domEvent: null
        });
    }

    getCursorPosition() {
        return { x: this._x, y: this._y };
    }

    getRealCursorPosition() {
        return { x: this._realX, y: this._realY };
    }

    isMouseButtonPressed(button) {
        return !!this._buttons[button];
    }

    isKeyPressed(code) {
        return this._keys[code];
    }
}