const BUTTON_TO_STRING = ['left', 'middle', 'right'];

export class WindowManager {
    constructor() {
        this._aspectRatio = 16 / 9;
        this._zIndexToCanvas = new Map();
        this._canvaxX = 0;
        this._canvasY = 0;
        this._canvasWidth = 0;
        this._canvasHeight = 0;

        this._pendingEvents = [];
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
        // document.addEventListener('visibilitychange', () => this._resetKeys());
        document.addEventListener('contextmenu', evt => evt.preventDefault());

        this._initialized = true;
        this._onResize();
    }

    start() {
        this._init();
    }

    pollEvent() {
        return this._pendingEvents.shift();
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
    
    _updateCanvasRect() {
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

    _onMouseMove(evt) {
        this._emit('mouse', {
            action: 'move',
            button: 'none',
            x: evt.clientX - this._canvasX,
            y: evt.clientY - this._canvasY,
        });
    }

    _onMouseDown(evt) {
        this._emit('mouse', {
            action: 'down',
            button: BUTTON_TO_STRING[evt.button],
            x: evt.clientX - this._canvasX,
            y: evt.clientY - this._canvasY,
        });
    }

    _onMouseUp(evt) {
        this._emit('mouse', {
            action: 'up',
            button: BUTTON_TO_STRING[evt.button],
            x: evt.clientX - this._canvasX,
            y: evt.clientY - this._canvasY,
        });
    }

    _onKeyDown(evt) {
        this._emit('keyboard', {
            action: 'down',
            code: evt.code,
            text: getText(evt),
            ctrl: evt.ctrlKey,
            shift: evt.shiftKey,
            alt: evt.altKey
        });
    }

    _onKeyUp(evt) {
        this._emit('keyboard', {
            action: 'up',
            code: evt.code,
            text: getText(evt),
            ctrl: evt.ctrlKey,
            shift: evt.shiftKey,
            alt: evt.altKey
        });
    }

    _onWheel(evt) {
        // TODO
    }
}

function getText(evt) {
    if (evt.key.length === 1) {
        return evt.key;
    } else {
        return null;
    }
}