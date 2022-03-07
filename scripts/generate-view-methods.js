#!/usr/bin/env node

const path = require('path');
const fs = require('fs');

const TAB = '    ';
const VIEW_SOURCE_PATH = path.join(__dirname, '..', 'lotus-compiler', 'prelude', 'engine', 'client', 'view.lt');
const FIELDS = {
    shape: 'Shape',
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
    text_margin: 'DisplaySize',
    text_max_width: 'DisplaySize',
    text_max_height: 'DisplaySize',
    text_background_color: 'Color',
    text_background_alpha: 'float',
    text_border_color: 'Color',
    text_border_alpha: 'float',
    text_horizontal_align: 'HorizontalAlign',
    text_vertical_align: 'VerticalAlign',
    text_bold: 'bool',
    text_italic: 'bool',
    text_cursor_index: 'int',
    detectable: 'bool',
    cursor: 'Cursor',
};

const GROUPS = [
    ['', '_graphics'],
    ['hover_', '_hovered_graphics()'],
    ['focus_', '_focused_graphics()'],
    ['disabled_', '_disabled_graphics()'],
];

const GENERATION_START_MARKER = '// GENERATION START';
const GENERATION_END_MARKER = '// GENERATION STOP';

function main() {
    let content = fs.readFileSync(VIEW_SOURCE_PATH, 'utf8');
    let generated = GROUPS.map(([prefix, graphics]) => {
        return Object.entries(FIELDS).map(([methodName, fields]) => {
            let start = `\n${TAB}${TAB}`;
            let end = `\n${TAB}`;
            let separator = `\n${TAB}${TAB}`
            if (typeof fields === 'string') {
                fields = { [methodName] : fields};
                start = ' ';
                end = ' ';
                separator = ' ';
            }
            let args = Object.entries(fields).map(([name, type]) => `${name}: ${type}`);
            let lines = Object.entries(fields).map(([name, type]) => `self.${graphics}.${name} = ${name};`);
            lines.push('self');

            return `${TAB}${prefix}${methodName}(${args.join(', ')}) -> Self {${start}${lines.join(separator)}${end}}`;
        }).join('\n');
    }).join('\n\n');

    let startIndex = content.indexOf(GENERATION_START_MARKER) + GENERATION_START_MARKER.length;
    let endIndex = content.indexOf(GENERATION_END_MARKER);
    let output = content.substring(0, startIndex) + '\n\n' + generated + '\n\n' + TAB + content.substring(endIndex);

    fs.writeFileSync(VIEW_SOURCE_PATH, output, 'utf8');
}

main();