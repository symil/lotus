import { ImageLoader } from './image-loader';
import { HORIZONTAL_ALIGN_TO_OFFSET_X, VERTICAL_ALIGN_TO_OFFSET_Y, VERTICAL_ALIGN_TO_TEXT_BASELINE, TRIANGLE_POINTS, HORIZONTAL_HEXAGON_POINTS, VERTICAL_HEXAGON_POINTS, CURVE_POINTS, LINE_POINTS } from './renderer-constants';
import { formatText } from './text-formatting';
import { colorToString, colorToU32, hashNumberList, hashString, stringToArray } from './utils';

export class Renderer {
    constructor(windowManager) {
        this._window = windowManager;
        this._ctx = null;
        this._imageLoader = new ImageLoader();
        this._strings = new Map();
        this._cachedTexts = new Map();
        this._cachedImages = new Map();
    }

    registerImage(url, image) {
        this._imageLoader.register(url, image);
    }

    getStringId(string) {
        let hash = hashString(string);

        this._strings.set(hash, string);

        return hash;
    }

    clearCache() {
        this._strings.clear();
        this._cachedTexts.clear();
        this._cachedImages.clear();
    }

    clear() {
        this._window.clear();
        this.setCursor(null);

        // TODO: implement a mechanism to clear cache if needed
    }

    setCursor(cursor) {
        this._window.setCursor(cursor || 'default');
    }

    draw(primitive) {
        let {
            x,
            y,
            z,
            shape,
            width,
            height,
            angle,
            border_color,
            border_width,
            border_radius,
            border_dash_length,
            border_gap_length,
            background_color,
            overlay_color,
            image_url,
            image_width,
            image_height,
            text,
            text_font,
            text_size,
            text_color,
            text_margin,
            text_max_width,
            text_max_height,
            text_background_color,
            text_border_color,
            text_horizontal_align,
            text_vertical_align,
            text_bold,
            text_italic,
            text_cursor_index,
        } = primitive;

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

        if (background_color.a || borderColor.a || overlayColor.a) {
            this._drawShape(shape, x, y, width, height, border_radius);
            if (shape !== 'line') {
                this._ctx.clip();
            }
        }

        if (background_color.a) {
            this._ctx.fillStyle = colorToString(background_color);
            this._ctx.fill();
        }

        if (image_url) {
            let image = this._getImageFromCache(image_url, Math.round(image_width), Math.round(image_height));

            if (image) {
                let imageX = Math.floor(x - image.width / 2);
                let imageY = Math.floor(y - image.height / 2);

                this._ctx.drawImage(image, imageX, imageY);
            }
        }

        if (text) {
            let textPadding = Math.max(border_radius, text_margin);
            let textImage = this._getTextImageFromCache(text, text_max_width, textPadding, text_size, text_color, text_font, text_bold, text_italic, text_cursor_index, text_background_color, text_border_color);
            let textX = x - textImage.width / 2;
            let textY = y - textImage.height / 2;
            let dx = (width - textImage.width) / 2;
            let dy = (height - textImage.height) / 2;

            if (text_horizontal_align === 'left') {
                textX -= dx;
            } else if (text_horizontal_align === 'right') {
                textX += dy;
            }

            if (text_vertical_align === 'top') {
                textY -= dy;
            } else if (text_horizontal_align === 'bottom') {
                textY += dy;
            }

            this._ctx.drawImage(textImage, Math.floor(textX), Math.floor(textY));
        }

        if (overlay_color.a) {
            this._ctx.fillStyle = colorToString(overlay_color);
            this._ctx.fill();
        }

        if (border_color.a && border_width) {
            if (border_dash_length && border_gap_length) {
                this._ctx.setLineDash([border_dash_length, border_gap_length]);
            } else {
                this._ctx.setLineDash([]);
            }

            let m = shape === 'line' ? 1 : 2;

            this._ctx.lineWidth = border_width * m;
            this._ctx.strokeStyle = colorToString(border_color);
            this._ctx.stroke();
        }

        this._ctx.restore();
    }

    _getTextImageFromCache(textId, maxWidth, padding, textSize, textColor, textFont, textBold, textItalic, textCursorIndex, backgroundColor, borderColor) {
        let text = this._strings.get(textId);
        let hash = hashNumberList([textId, maxWidth, padding, textSize, colorToU32(textColor), ...stringToArray(textFont), textCursorIndex, colorToU32(backgroundColor), colorToU32(borderColor)]);
        let image = this._cachedTexts.get(hash);

        if (!image) {
            image = formatText({ text, maxWidth, padding, textSize, textColor, textFont, textBold, textItalic, textCursorIndex, backgroundColor, borderColor });
            this._cachedTexts.set(hash, image);
        }

        return image;
    }

    _getImageFromCache(urlId, width, height) {
        let id = hashNumberList([urlId, width, height]);
        let image = this._cachedImages.get(id);

        if (!image) {
            let url = this._strings.get(urlId);

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