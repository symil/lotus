#!/usr/bin/env node

const path = require('path');
const fs = require('fs');

const TAB = '    ';
const GENERATION_START_MARKER = '// GENERATION START';
const GENERATION_END_MARKER = '// GENERATION STOP';

const SRC_DIR_PATH = path.join(__dirname, '..', 'lotus-compiler', 'prelude', 'src');
const CLIENT_SRC_SIR_PATH = path.join(SRC_DIR_PATH, 'engine', 'client');
const VIEW_SOURCE_PATH = path.join(CLIENT_SRC_SIR_PATH, 'view.lt');
const LAYOUT_SOURCE_PATH = path.join(CLIENT_SRC_SIR_PATH, 'layout', 'view_layout.lt');

const VIEW_METHODS = {
    shape: 'Shape',
    anchor: 'Anchor',
    border_radius: 'DisplaySize',
    border_width: 'DisplaySize',
    border_dash_length: 'DisplaySize',
    border_gap_length: 'DisplaySize',
    border_color: 'Color',
    border_alpha: 'float',
    background_color: 'Color',
    background_alpha: 'float',
    overlay_color: 'Color',
    overlay_alpha: 'float',
    image_url: 'string',
    image_scale: 'float',
    image_layout: {
        image_sprite_count_per_row: 'int',
        image_sprite_count_per_column: 'int',
    },
    image_sprite_index: 'int',
    animation_start_time: 'float',
    animation_current_time: 'float',
    animation_duration: 'float',
    text: 'string',
    text_font: 'Font',
    text_size: 'DisplaySize',
    text_color: 'Color',
    text_alpha: 'float',
    text_padding: 'DisplaySize',
    text_horizontal_align: 'HorizontalAlign',
    text_vertical_align: 'VerticalAlign',
    text_bold: 'bool',
    text_italic: 'bool',
    text_cursor_index: 'int',
    shrink_to_fit_text: 'bool',
    cursor: 'Cursor',
};

function main() {
    generateCode(VIEW_SOURCE_PATH, {
        '': args => args.map(name => `self._graphics.${name} = ${name};`),
        'hover_': args => args.map(name => `self._hovered_graphics().${name} = ${name};`),
        'focus_': args => args.map(name => `self._focused_graphics().${name} = ${name};`),
        'disabled_': args => args.map(name => `self._disabled_graphics().${name} = ${name};`),
    });

    generateCode(LAYOUT_SOURCE_PATH, (args, method) => `self._get_last_view().${method}(${args.join(', ')});`);
}

function generateCode(sourceFilePath, generateLinesFunctions) {
    if (typeof generateLinesFunctions === 'function') {
        generateLinesFunctions = { '': generateLinesFunctions };
    }

    let content = fs.readFileSync(sourceFilePath, 'utf8');
    let blocks = [];

    for (let [prefix, generateLines] of Object.entries(generateLinesFunctions)) {
        let block = [];

        for (let [methodName, fields] of Object.entries(VIEW_METHODS)) {
            let start = `\n${TAB}${TAB}`;
            let end = `\n${TAB}`;
            let separator = `\n${TAB}${TAB}`
            if (typeof fields === 'string') {
                fields = { [methodName] : fields};
            }
            let entries = Object.entries(fields);
            let args = entries.map(([name, type]) => `${name}: ${type}`);
            let argNames = entries.map(([name]) => name);
            let lines = generateLines(argNames, methodName);

            if (!Array.isArray(lines)) {
                lines = [lines];
            }

            if (lines.length <= 1) {
                start = ' ';
                end = ' ';
                separator = ' ';
            }

            lines.push('self');

            block.push(`${TAB}${prefix}${methodName}(${args.join(', ')}) -> Self {${start}${lines.join(separator)}${end}}`);
        }

        blocks.push(block.join('\n'));
    }

    let generated = blocks.join('\n\n');
    let startIndex = content.indexOf(GENERATION_START_MARKER) + GENERATION_START_MARKER.length;
    let endIndex = content.indexOf(GENERATION_END_MARKER);
    let output = content.substring(0, startIndex) + '\n\n' + generated + '\n\n' + TAB + content.substring(endIndex);

    fs.writeFileSync(sourceFilePath, output, 'utf8');
}

main();