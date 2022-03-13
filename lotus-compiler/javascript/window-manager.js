const BUTTON_TO_STRING = ['left', 'middle', 'right'];
const DELTA_MODE_TO_STRING = ['pixel', 'line', 'page'];

export class WindowManager {
    constructor({ getWindow }) {
        this._getWindow = getWindow;
        this._window = null;
        this._aspectRatio = null;
        this._zIndexToCanvas = new Map();
        this._canvaxX = 0;
        this._canvasY = 0;
        this._canvasWidth = 0;
        this._canvasHeight = 0;

        this._pendingEvents = [];
        this._initialized = false;

        this._pressLocation = {
            left: null,
            middle: null,
            right: null
        };
    }

    init(aspectRatio) {
        if (this._initialized) {
            return;
        }

        let window = this._getWindow();
        let document = window.document;

        window.addEventListener('resize', () => this._onResize());
        document.addEventListener('mousemove', evt => this._onMouseMove(evt));
        // document.addEventListener('mouseleave', evt => this._onMouseMove(evt));
        document.addEventListener('mousedown', evt => this._onMouseDown(evt));
        document.addEventListener('mouseup', evt => this._onMouseUp(evt));
        document.addEventListener('wheel', evt => this._onWheel(evt));
        document.addEventListener('keydown', evt => this._onKeyDown(evt));
        document.addEventListener('keyup', evt => this._onKeyUp(evt));
        // document.addEventListener('visibilitychange', () => this._resetKeys());
        document.addEventListener('contextmenu', evt => evt.preventDefault());

        this._window = window;
        this._aspectRatio = aspectRatio;
        this._initialized = true;
        this._onResize();
    }

    pollEvents() {
        let result = this._pendingEvents;

        this._pendingEvents = [];

        return result;
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

    getWidth() {
        return this._canvasWidth;
    }

    getHeight() {
        return this._canvasHeight;
    }

    setTitle(title) {
        this._window.document.title = title;
    }
    
    _updateCanvasRect() {
        let aspectRatio = this._aspectRatio;
        let width = this._window.innerWidth;
        let height = this._window.innerHeight;

        if (height * aspectRatio > width) {
            height = width / aspectRatio;
        } else {
            width = height * aspectRatio;
        }

        let x = (this._window.innerWidth - width) / 2;
        let y = (this._window.innerHeight - height) / 2;

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

        this._updateCanvasRect();

        for (let { canvas } of this._zIndexToCanvas.values()) {
            this._updateCanvas(canvas);
        }

        this._updateDom();
        this._emit('window', 'resize');
    }

    _emit(type, payload) {
        this._pendingEvents.push({ type, payload });
    }

    _parseMouseEvent(action, evt) {
        let button = BUTTON_TO_STRING[evt.button] || 'left';
        let x = evt.clientX - this._canvasX;
        let y = evt.clientY - this._canvasY;

        return { action, button, x, y };
    }

    _onMouseMove(evt) {
        let { action, button, x, y } = this._parseMouseEvent('move', evt);

        this._emit('mouse', { action, button, x, y });
    }

    _onMouseDown(evt) {
        let { action, button, x, y } = this._parseMouseEvent('down', evt);

        this._pressLocation[button] = { x, y };
        this._emit('mouse', { action, button, x, y });
    }

    _onMouseUp(evt) {
        let { action, button, x, y } = this._parseMouseEvent('up', evt);

        this._pressLocation[button] = null;
        this._emit('mouse', { action, button, x, y });
    }

    _onKeyDown(evt) {
        if (evt.key === 'Tab') {
            evt.preventDefault();
        }

        this._emit('keyboard', {
            action: 'down',
            code: evt.code,
            text: getText(evt),
            ctrlKey: evt.ctrlKey,
            shiftKey: evt.shiftKey,
            altKey: evt.altKey,
            repeat: evt.repeat
        });
    }

    _onKeyUp(evt) {
        this._emit('keyboard', {
            action: 'up',
            code: evt.code,
            text: getText(evt),
            ctrlKey: evt.ctrlKey,
            shiftKey: evt.shiftKey,
            altKey: evt.altKey,
            repeat: evt.repeat
        });
    }

    _onWheel(evt) {
        let { x, y } = this._parseMouseEvent('wheel', evt);

        this._emit('wheel', {
            x,
            y,
            deltaX: evt.deltaX,
            deltaY: evt.deltaY,
            deltaZ: evt.deltaZ,
            deltaMode: DELTA_MODE_TO_STRING[evt.deltaMode]
        });
    }
}

function getText(evt) {
    if (evt.key.length === 1) {
        return evt.key;
    } else {
        return null;
    }
}

function distance(p1, p2) {
    return Math.hypot(p2.x - p1.x, p2.y - p1.y);
}