import { ImageLoader } from './image-loader';
import { TRIANGLE_POINTS, HORIZONTAL_HEXAGON_POINTS, VERTICAL_HEXAGON_POINTS, CURVE_POINTS, LINE_POINTS, SHAPES, HORIZONTAL_ALIGNS, VERTICAL_ALIGNS, CURSORS, FONTS } from './renderer-constants';
import { formatText } from './text-formatting';
import { colorToString, colorToU32, hashNumberList, hashString } from './utils';

export class Renderer {
    constructor(windowManager) {
        this._window = windowManager;
        this._ctx = null;
        this._imageLoader = new ImageLoader();
        this._stringHashes = new Map();
        this._cachedTexts = new Map();
        this._cachedImages = new Map();
    }

    registerImage(url, image) {
        this._imageLoader.register(url, image);
    }

    clearCache() {
        this._stringHashes.clear();
        this._cachedTexts.clear();
        this._cachedImages.clear();
    }

    drawFrameFromBuffer(buffer) {
        let cursor = buffer.readEnum(CURSORS);

        this._window.clear();
        this._window.setCursor(cursor);

        while (!buffer.isFinished()) {
            this._drawPrimitiveFromBuffer(buffer);
        }
    }

    _drawPrimitiveFromBuffer(buffer) {
        let x = buffer.readFloat();
        let y = buffer.readFloat();
        let z = buffer.readFloat();
        let width = buffer.readFloat();
        let height = buffer.readFloat();
        let angle = buffer.readFloat();
        let shape = buffer.readEnum(SHAPES);
        let borderColor = buffer.readColor();
        let borderWidth = buffer.readFloat();
        let borderRadius = buffer.readFloat();
        let borderDashLength = buffer.readFloat();
        let borderGapLength = buffer.readFloat();
        let backgroundColor = buffer.readColor();
        let overlayColor = buffer.readColor();
        let imageUrl = buffer.readString();
        let imageWidth = buffer.readFloat();
        let imageHeight = buffer.readFloat();
        let text = buffer.readString();
        let textFont = buffer.readEnum(FONTS);
        let textSize = buffer.readFloat();
        let textColor = buffer.readColor();
        let textMargin = buffer.readFloat();
        let textMaxWidth = buffer.readFloat();
        let textMaxHeight = buffer.readFloat();
        let textBackgroundColor = buffer.readColor();
        let textBorderColor = buffer.readColor();
        let textHorizontalAlign = buffer.readEnum(HORIZONTAL_ALIGNS);
        let textVerticalAlign = buffer.readEnum(VERTICAL_ALIGNS);
        let textBold = buffer.read();
        let textItalic = buffer.read();
        let textCursorIndex = buffer.read();

        // let primitive = { x, y, z, shape, width, height, angle, borderColor, borderWidth, borderRadius, borderDashLength, borderGapLength, backgroundColor, overlayColor, imageUrl, imageWidth, imageHeight, text, textFont, textSize, textColor, textMargin, textMaxWidth, textMaxHeight, textBackgroundColor, textBorderColor, textHorizontalAlign, textVerticalAlign, textBold, textItalic, textCursorIndex };
        // console.log(primitive);

        let x1 = x - width / 2;
        let y1 = y - height / 2;
        let x2 = x + width / 2;
        let y2 = y + height / 2;

        if (x2 < 0 || x1 > this._window.getWidth() || y2 < 0 || y1 > this._window.getHeight()) {
            return;
        }
        
        this._ctx = this._window.getCanvasContext(z);
        this._ctx.save();

        if (angle) {
            this._ctx.translate(x, y);
            this._ctx.rotate(angle);
            this._ctx.translate(-x, -y);
        }

        if (backgroundColor.a || borderColor.a || overlayColor.a) {
            this._drawShape(shape, x, y, width, height, borderRadius);
            if (shape !== 'line') {
                this._ctx.clip();
            }
        }

        if (backgroundColor.a) {
            this._ctx.fillStyle = colorToString(backgroundColor);
            this._ctx.fill();
        }

        if (imageUrl) {
            let image = this._getImageFromCache(imageUrl, Math.round(imageWidth), Math.round(imageHeight));

            if (image) {
                let imageX = Math.floor(x - image.width / 2);
                let imageY = Math.floor(y - image.height / 2);

                this._ctx.drawImage(image, imageX, imageY);
            }
        }

        if (text) {
            let textPadding = Math.max(borderRadius, textMargin);
            let textImage = this._getTextImageFromCache(text, textMaxWidth, textPadding, textSize, textColor, textFont, textBold, textItalic, textCursorIndex, textBackgroundColor, textBorderColor);
            let textX = x - textImage.width / 2;
            let textY = y - textImage.height / 2;
            let dx = (width - textImage.width) / 2;
            let dy = (height - textImage.height) / 2;

            if (textHorizontalAlign === 'left') {
                textX -= dx;
            } else if (textHorizontalAlign === 'right') {
                textX += dy;
            }

            if (textVerticalAlign === 'top') {
                textY -= dy;
            } else if (textHorizontalAlign === 'bottom') {
                textY += dy;
            }

            this._ctx.drawImage(textImage, Math.floor(textX), Math.floor(textY));
        }

        if (overlayColor.a) {
            this._ctx.fillStyle = colorToString(overlayColor);
            this._ctx.fill();
        }

        if (borderColor.a && borderWidth) {
            if (borderDashLength && borderGapLength) {
                this._ctx.setLineDash([borderDashLength, borderGapLength]);
            } else {
                this._ctx.setLineDash([]);
            }

            let m = shape === 'line' ? 1 : 2;

            this._ctx.lineWidth = borderWidth * m;
            this._ctx.strokeStyle = colorToString(borderColor);
            this._ctx.stroke();
        }

        this._ctx.restore();
    }

    _getStringHash(string) {
        let hash = this._stringHashes.get(string);

        if (!hash) {
            hash = hashString(string);
            this._stringHashes.set(string, hash);
        }

        return hash;
    }

    _getTextImageFromCache(text, maxWidth, padding, textSize, textColor, textFont, textBold, textItalic, textCursorIndex, backgroundColor, borderColor) {
        let textHash = this._getStringHash(text);
        let textFontHash = this._getStringHash(textFont);
        let hash = hashNumberList([textHash, maxWidth, padding, textSize, colorToU32(textColor), textFontHash, textCursorIndex, colorToU32(backgroundColor), colorToU32(borderColor)]);
        let image = this._cachedTexts.get(hash);

        if (!image) {
            image = formatText({ text, maxWidth, padding, textSize, textColor, textFont, textBold, textItalic, textCursorIndex, backgroundColor, borderColor });
            this._cachedTexts.set(hash, image);
        }

        return image;
    }

    _getImageFromCache(url, width, height) {
        let urlHash = this._getStringHash(url);
        let id = hashNumberList([urlHash, width, height]);
        let image = this._cachedImages.get(id);

        if (!image) {
            image = this._imageLoader.get(url);
            image = resizeImage(image, width, height);

            this._cachedImages.set(id, image);
        }

        return image;
    }

    _fill(box, color) {
        if (color) {
            this._ctx.fillStyle = color;
            this._ctx.fillRect(box.x1, box.y1, box.width, box.height);
        }
    }

    _drawShape(shape, x, y, width, height, borderRadius) {
        this._ctx.beginPath();

        if (shape === 'rectangle') {
            let x1 = Math.round(x - width / 2);
            let y1 = Math.round(y - height / 2);
            let x2 = Math.round(x + width / 2);
            let y2 = Math.round(y + height / 2);
            let w = x2 - x1;
            let h = y2 - y1;
            let r = Math.round(borderRadius);

            if (r === 0) {
                this._ctx.rect(x1, y1, w, h);
            } else {
                this._ctx.moveTo(x1 + r, y1);
                this._ctx.lineTo(x2 - r, y1);
                this._ctx.quadraticCurveTo(x2, y1, x2, y1 + r);
                this._ctx.lineTo(x2, y2 - r);
                this._ctx.quadraticCurveTo(x2, y2, x2 - r, y2);
                this._ctx.lineTo(x1 + r, y2);
                this._ctx.quadraticCurveTo(x1, y2, x1, y2 - r);
                this._ctx.lineTo(x1, y1 + r);
                this._ctx.quadraticCurveTo(x1, y1, x1 + r, y1);
                this._ctx.closePath();
            }
        } else if (shape === 'line') {
            this._polygon(LINE_POINTS, x, y, width, height);
        } else if (shape === 'circle') {
            this._ctx.ellipse(x, y, width / 2, height / 2, 0, 0, Math.PI * 2);
        } else if (shape === 'triangle') {
            this._polygon(TRIANGLE_POINTS, x, y, width, height);
        } else if (shape === 'vertical-hexagon') {
            this._polygon(VERTICAL_HEXAGON_POINTS, x, y, width, height);
        } else if (shape === 'horizontal-hexagon') {
            this._polygon(HORIZONTAL_HEXAGON_POINTS, x, y, width, height);
        } else if (shape === 'curve') {
            this._polygon(CURVE_POINTS, x, y, width, height);
        }
    }

    _polygon(points, x, y, width, height) {
        this._ctx.beginPath();
        this._ctx.moveTo(cx + points[0][0] * width, cy + points[0][1] * height);

        for (let i = 1; i < points.length; ++i) {
            let [px, py] = points[i];

            this._ctx.lineTo(x + px * width, y + py * height);
        }

        if (points.length > 2) {
            this._ctx.closePath();
        }
    }
}

function resizeImage(image, targetWidth, targetHeight) {
    if (!image || image.width === targetWidth || image.height === targetHeight) {
        return image;
    }

    let widthRatio = image.width / targetWidth;
    let heightRatio = image.height / targetHeight;
    let ratio = Math.max(widthRatio, heightRatio);
    let width = image.width;
    let height = image.height;

    if (ratio >= 2) {
        width /= 2;
        height /= 2;
    } else {
        width = targetWidth;
        height = targetHeight;
    }

    let canvas = document.createElement('canvas');
    let ctx = canvas.getContext('2d');

    canvas.width = width;
    canvas.height = height;

    ctx.drawImage(image, 0, 0, width, height);

    return resizeImage(canvas, targetWidth, targetHeight);
}