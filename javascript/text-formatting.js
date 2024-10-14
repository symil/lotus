import { colorToString } from './utils.js';

const BOTTOM_CHARACTERS = new Set(['g', 'j', 'p', 'q', 'y']);

export function formatText(parameters) {
    let {
        text,
        maxWidth,
        maxHeight,
        padding,
        textSize,
        textFont,
        textColor,
        textBold,
        textItalic,
        textCursorIndex,
    } = parameters;

    textColor = colorToString(textColor);

    let canvas = document.createElement('canvas');
    let ctx = canvas.getContext('2d');

    let baseTextParams = {
        font: textFont,
        size: textSize,
        color: textColor,
        bold: textBold,
        italic: textItalic,
        horizontalAlign: 'left',
        verticalAlign: 'middle'
    };
    let lines = [];
    let maxLineWidth = maxWidth ? maxWidth - padding * 2 : Infinity;
    let startX = padding + 1;
    let startY = padding;
    let x = startX;
    let y = startY;
    let currentLine = [];
    let currentLineWidth = 0;
    let longestLineWidth = 0;
    let currentLineHeight = textSize;
    let lineHorizontalAlign = false;
    let previousLineHeight = 0;
    let lastToken = { content: '' };
    let suffix = textCursorIndex > -1 ? ' \n' : '\n';
    let tokens = tokenize(text + suffix, baseTextParams);

    for (let token of tokens) {
        setCanvasPropertiesFromToken(ctx, token);
        
        let tokenWidth = ctx.measureText(token.content).width;
        let tokenHeight = token.size || 0;

        if (token.content === '\n' || (currentLineWidth + tokenWidth > maxLineWidth && currentLine.length > 0)) {
            if (currentLine.length === 0) {
                currentLineHeight = Math.ceil(textSize * 2 / 3);
            }

            y += (currentLineHeight || previousLineHeight);

            for (let obj of currentLine) {
                obj.y = y;
            }

            lines.push({
                tokens: currentLine,
                width: currentLineWidth,
                height: currentLineHeight,
                align: lineHorizontalAlign
            });

            currentLineWidth = isBlank(token) ? 0 : tokenWidth;
            x = startX;
            previousLineHeight = currentLineHeight || previousLineHeight;
            currentLineHeight = 0;
            currentLine = [];
        } else {
            currentLineWidth += tokenWidth;
        }

        if (token.content !== '\n') {
            currentLineHeight = Math.max(currentLineHeight, tokenHeight + 2);
            lineHorizontalAlign = token.horizontalAlign;
        }

        longestLineWidth = Math.max(longestLineWidth, currentLineWidth);

        if (token.content !== '\n') {
            token.x = x;

            if (token.content !== ' ' || currentLine.length > 0 || token === tokens[0]) {
                currentLine.push(token);
            }

            if (token.content !== ' ' || x !== startX || lastToken.content === '\n') {
                x += tokenWidth;   
            }
        }

        lastToken = token;
    }

    for (let line of lines) {
        let lineHeight = line.height;
        let widthDif = longestLineWidth - line.width;
        let offsetX = 0;

        if (line.align === 'center') {
            offsetX = widthDif / 2;
        } else if (line.align === 'right') {
            offsetX = widthDif;
        }

        for (let token of line.tokens) {
            let dif = (lineHeight - token.size);
            let m = 0.3;

            if (token.verticalAlign === 'bottom') {
                m = -0.1;
            }

            token.x += offsetX;
            token.y -= dif * m;
        }
    }

    if (lines.length === 1) {
        let line = lines[0];
        let has_bottom_characters = false;

        for (let token of line.tokens) {
            for (let c of token.content) {
                if (BOTTOM_CHARACTERS.has(c)) {
                    has_bottom_characters = true;
                }
            }
        }

        if (!has_bottom_characters) {
            for (let token of line.tokens) {
                token.y += line.height * 0.09;
            }
        }
    }

    let totalHeight = Math.round(y - startY + padding * 2 + 1);
    let totalWidth = Math.round(longestLineWidth + padding * 2 + 2);

    canvas.width = totalWidth;
    canvas.height = totalHeight;

    // ctx.fillStyle = 'white'; ctx.fillRect(0, 0, totalWidth, totalHeight);

    // if (backgroundColor) {
    //     ctx.fillStyle = backgroundColor;
    //     ctx.fillRect(0, 0, totalWidth, totalHeight);
    // }

    // if (borderColor) {
    //     ctx.lineWidth = 2;
    //     ctx.strokeStyle = borderColor;
    //     ctx.strokeRect(0, 0, totalWidth, totalHeight);
    // }

    ctx.textBaseline = 'bottom';

    let cursorDone = textCursorIndex < 0;
    let index = 0;

    for (let line of lines) {
        for (let token of line.tokens) {
            setCanvasPropertiesFromToken(ctx, token);
            ctx.fillText(token.content, token.x, token.y);

            let nextIndex = index + token.content.length;

            if (!cursorDone && textCursorIndex <= nextIndex) {
                // TODO: properly display the cursor on multi-line texts
                let subText = text.substring(0, textCursorIndex - index);
                let subTextWidth = ctx.measureText(subText).width;
                let cursorWidth = 1;
                let cursorHeight = token.size * 0.85;
                let cursorX = token.x + subTextWidth;
                let cursorY = token.y - token.size * 0.95;

                ctx.rect(Math.round(cursorX), Math.round(cursorY), cursorWidth, Math.round(cursorHeight));
                ctx.fillStyle = token.color;
                ctx.fill();

                cursorDone = true;
            }

            index = nextIndex;
        }
    }

    return canvas;
}

function isBlank(token) {
    return token.content === ' ' || token.content === '\n';
}

function setCanvasPropertiesFromToken(ctx, token) {
    let { font, size, bold, italic, color } = token;

    ctx.font = `${bold ? 'bold ' : ''}${italic ? 'italic ' : ''}${Math.round(size)}px "${font}"`;
    ctx.fillStyle = color;
}

function makeToken(content, textParams, rules) {
    let token = { content, ...textParams, x: 0, y: 0 };

    for (let rule of rules) {
        if (!isNaN(+rule)) {
            token.size *= (+rule);
        } else if (rule === 'right') {
            token.horizontalAlign = 'right';
        } else if (rule === 'center') {
            token.horizontalAlign = 'center';
        } else if (rule === 'bold') {
            token.bold = true;
        } else if (rule === 'italic') {
            token.italic = true;
        } else if (rule === 'small') {
            token.size *= 0.75;
        } else if (rule === 'big') {
            token.size *= 1.6;
        } else if (rule === 'sub') {
            token.size *= 0.5;
            token.verticalAlign = 'bottom';
        } else {
            token.color = rule;
        }
    }

    return token;
}

const SHORTCUTS = {
    '*': 'bold',
    '_': 'italic',
    '|': 'center',
    '#': 'big',
    '~': 'small'
};

function tokenize(text, textParams) {
    let tokens = [];
    let activeShortcuts = new Set();
    let ruleStack = [];
    let content = '';

    for (let i = 0; i <= text.length; ++i) {
        let c = text[i];

        if (c in SHORTCUTS) {
            let rule = SHORTCUTS[c];

            if (activeShortcuts.has(c)) {
                tokens.push(makeToken(content, textParams, ruleStack));
                activeShortcuts.delete(c);
                ruleStack.splice(ruleStack.lastIndexOf(rule), 1);
                content = '';
            } else {
                tokens.push(makeToken(content, textParams, ruleStack));
                activeShortcuts.add(c);
                ruleStack.push(rule);
                content = '';
            }
        } else if (c === '@') {
            let startBracketIndex = text.indexOf('{', i + 1);

            if (startBracketIndex === -1) {
                content += c;
            } else {
                let rule = text.substring(i + 1, startBracketIndex);

                tokens.push(makeToken(content, textParams, ruleStack));
                ruleStack.push(rule);
                content = '';
            }

            i = startBracketIndex;
        } else if (c === '}' && ruleStack.length) {
            tokens.push(makeToken(content, textParams, ruleStack));
            ruleStack.pop();
            content = '';
        } else if (c === '\n') {
            tokens.push(makeToken(content, textParams, ruleStack));
            tokens.push(makeToken('\n', textParams, ruleStack));
            ruleStack = [];
            activeShortcuts.clear();
            content = '';
        } else if (c === ' ') {
            let token = makeToken(content, textParams, ruleStack);
            let spaceToken = { ...token, content: ' ' };

            tokens.push(token, spaceToken);
            content = '';
        } else if (c === undefined) {
            tokens.push(makeToken(content, textParams, ruleStack));
        } else {
            content += c;
        }
    }

    return tokens.filter(token => token.content);
}