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
        let drawPrimitiveCount = buffer.read();

        this._window.clear();
        this._window.setCursor(cursor);

        for (let i = 0; i < drawPrimitiveCount; ++i) {
            this._drawPrimitiveFromBuffer(buffer);
        }
    }

    _drawPrimitiveFromBuffer(buffer) {
        let primitive = buffer.readObject();
        let x = primitive.readFloat();
        let y = primitive.readFloat();
        let z = primitive.readFloat();
        let width = primitive.readFloat();
        let height = primitive.readFloat();
        let angle = primitive.readFloat();
        let horizontal_anchor = primitive.readEnum(HORIZONTAL_ALIGNS);
        let vertical_anchor = primitive.readEnum(VERTICAL_ALIGNS);
        let shape = primitive.readEnum(SHAPES);
        let borderColor = primitive.readColor();
        let borderWidth = primitive.readFloat();
        let borderRadius = primitive.readFloat();
        let borderDashLength = primitive.readFloat();
        let borderGapLength = primitive.readFloat();
        let backgroundColor = primitive.readColor();
        let overlayColor = primitive.readColor();
        let imageUrl = primitive.readString();
        let imageWidth = primitive.readFloat();
        let imageHeight = primitive.readFloat();
        let imageSx = primitive.readFloat();
        let imageSy = primitive.readFloat();
        let imageSw = primitive.readFloat();
        let imageSh = primitive.readFloat();
        let text = primitive.readString();
        let textFont = primitive.readEnum(FONTS);
        let textSize = primitive.readFloat();
        let textColor = primitive.readColor();
        let textPadding = primitive.readFloat();
        let textHorizontalAlign = primitive.readEnum(HORIZONTAL_ALIGNS);
        let textVerticalAlign = primitive.readEnum(VERTICAL_ALIGNS);
        let textBold = primitive.read();
        let textItalic = primitive.read();
        let textCursorIndex = primitive.read();
        let fitText = primitive.read();
        let shrinkToFixText = primitive.read();

        // let primitive = { x, y, z, shape, width, height, angle, borderColor, borderWidth, borderRadius, borderDashLength, borderGapLength, backgroundColor, overlayColor, imageUrl, imageWidth, imageHeight, text, textFont, textSize, textColor, textMargin, textMaxWidth, textMaxHeight, textBackgroundColor, textBorderColor, textHorizontalAlign, textVerticalAlign, textBold, textItalic, textCursorIndex };
        // console.log(primitive);

        let textImage = null;

        if (text) {
            textPadding = Math.max(borderRadius, textPadding);
            let textMaxWidth = (shrinkToFixText || fitText) ? width : 0;
            textImage = this._getTextImageFromCache(text, textMaxWidth, textPadding, textSize, textColor, textFont, textBold, textItalic, textCursorIndex);

            if (shrinkToFixText) {
                width = Math.min(width, textImage.width);
                height = Math.min(height, textImage.height);
            }
        }

        let w = width / 2;
        let h = height / 2;

        if (horizontal_anchor === 'left') {
            x += w;
        } else if (horizontal_anchor === 'right') {
            x -= w;
        }

        if (vertical_anchor === 'top') {
            y += h;
        } else if (horizontal_anchor === 'bottom') {
            y -= h
        }

        let x1 = x - w;
        let y1 = y - h;
        let x2 = x + w;
        let y2 = y + h;

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
            let targetWidth = Math.round(imageWidth / imageSw);
            let targetHeight = Math.round(imageHeight / imageSh);
            let image = this._getImageFromCache(imageUrl, targetWidth, targetHeight);

            if (image && targetWidth && targetHeight) {
                let sx = Math.round(imageSx * targetWidth);
                let sy = Math.round(imageSy * targetHeight);
                let sw = Math.round(imageSw * targetWidth);
                let sh = Math.round(imageSh * targetHeight);
                let imageX = Math.floor(x - sw / 2);
                let imageY = Math.floor(y - sh / 2);

                this._ctx.drawImage(image, sx, sy, sw, sh, imageX, imageY, sw, sh);

                // if (!window.TOTAL) {
                //     window.TOTAL = 0;
                //     window.COUNT = 0;
                //     window.GET_AVERAGE = () => console.log(`${window.TOTAL / window.COUNT}ms`);
                // }
                // let now = performance.now();
                // for (let i = 0; i < 10000; ++i) {
                //     this._ctx.drawImage(image, sx, sy, sw, sh, imageX, imageY, sw, sh);
                //     // this._ctx.drawImage(image, imageX, imageY);
                // }
                // window.COUNT += 10000;
                // window.TOTAL += performance.now() - now;
            }
        }

        if (textImage && textImage.width && textImage.height) {
            let textX = x - textImage.width / 2;
            let textY = y - textImage.height / 2;
            let dx = (width - textImage.width) / 2;
            let dy = (height - textImage.height) / 2;

            if (textHorizontalAlign === 'left') {
                textX -= dx;
            } else if (textHorizontalAlign === 'right') {
                textX += dx;
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

            if (shape === 'rectangle') {
                borderWidth = Math.ceil(borderWidth);
            }

            let m = shape === 'line' ? 1 : 2;
            let lineWidth = borderWidth * m;

            this._ctx.lineWidth = lineWidth;
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

    _getTextImageFromCache(text, maxWidth, padding, textSize, textColor, textFont, textBold, textItalic, textCursorIndex) {
        let textHash = this._getStringHash(text);
        let textFontHash = this._getStringHash(textFont);
        let hash = hashNumberList([textHash, maxWidth, padding, textSize, colorToU32(textColor), textFontHash, textCursorIndex ]);
        let image = this._cachedTexts.get(hash);

        if (!image) {
            image = formatText({ text, maxWidth, padding, textSize, textColor, textFont, textBold, textItalic, textCursorIndex });
            this._cachedTexts.set(hash, image);
        }

        return image;
    }

    _getImageFromCache(url, targetWidth, targetHeight) {
        let urlHash = this._getStringHash(url);
        let id = hashNumberList([urlHash, targetWidth, targetHeight]);
        let image = this._cachedImages.get(id);

        if (!image) {
            image = this._imageLoader.get(url);
            image = resizeImage(image, targetWidth, targetHeight);

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
            let w = Math.max(x2 - x1, 1);
            let h = Math.max(y2 - y1, 1);
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