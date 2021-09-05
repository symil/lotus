export const HORIZONTAL_ALIGN_TO_OFFSET_X = { left: 0, center: 0.5, right: 1 };
export const VERTICAL_ALIGN_TO_OFFSET_Y = { top: 0, middle: 0.5, bottom: 1 };
export const VERTICAL_ALIGN_TO_TEXT_BASELINE = {
    top: 'top',
    middle: 'middle',
    bottom: 'alphabetic'
}

const HEXAGON_WIDTH_TO_HEIGHT_RATIO = 0.8660254037844386;

export const LINE_POINTS = [
    [-0.5, 0],
    [0.5, 0],
];
export const TRIANGLE_POINTS = [
    [-0.5, -0.5],
    [-0.5,  0.5],
    [ 0.5, 0]
];
export const HORIZONTAL_HEXAGON_POINTS = computePointsOnCircle([0, 1/6, 2/6, 3/6, 4/6, 5/6]).map(([x, y]) => [x, y / HEXAGON_WIDTH_TO_HEIGHT_RATIO]);
export const VERTICAL_HEXAGON_POINTS = computePointsOnCircle([0, 1/6, 2/6, 3/6, 4/6, 5/6], 0.25).map(([x, y]) => [x / HEXAGON_WIDTH_TO_HEIGHT_RATIO, y]);
export const STAR_POINTS = computePointsOnCircle([0, 2/5, 4/5, 1/5, 3/5], 0.25);
export const CURVE_POINTS = computePointsOnCurve(0.1, 0);

function computePointsOnCircle(percents, start = 0) {
    return percents.map(a => {
        let angle = (a + start) * Math.PI * 2;

        return [
            Math.cos(angle) / 2,
            Math.sin(angle) / 2
        ];
    });
}

function computePointsOnCurve(w1, w2) {
    let precision = 24;
    let dw = (w1 + w2) / 2;
    let pStart1 = { x: -0.5, y: -w1 / 2 };
    let pStart2 = { x: -0.5, y: +w1 / 2 };
    let pEnd1 = { x: 0.5, y: -w2 / 2 };
    let pEnd2 = { x: 0.5, y: +w2 / 2 };
    let pControl1 = { x: 0, y: -0.5 };
    let pControl2 = { x: 0, y: -0.5 + dw };

    let firstLine = [];
    let secondLine = [];

    for (let i = 0; i <= precision; ++i) {
        let t = i / precision;
        let p1 = getPointOnQuadraticCurve(pStart1, pEnd1, pControl1, t);
        let p2 = getPointOnQuadraticCurve(pStart2, pEnd2, pControl2, t);

        firstLine.push([p1.x, p1.y]);
        secondLine.push([p2.x, p2.y]);
    }

    return [...firstLine, ...secondLine.reverse()];
}

function getPointOnQuadraticCurve(p1, p2, cp, t) {
    let u = 1 - t;
    let t2 = t * t;
    let u2 = u * u;
    let tu = 2 * t * u;
    let x = u2 * p1.x + tu * cp.x + t2 * p2.x;
    let y = u2 * p1.y + tu * cp.y + t2 * p2.y;

    return { x, y };
}