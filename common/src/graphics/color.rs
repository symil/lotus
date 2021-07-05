use wasm_bindgen::prelude::*;

macro_rules! impl_color {
    ($name:ident : [$r:literal, $g:literal, $b:literal, $a:literal]) => {
        pub fn $name() -> Self {
            Self::rgba($r, $g, $b, $a)
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl Default for Color {
    fn default() -> Self {
        Self::rgba(0, 0, 0, 0)
    }
}

impl Color {
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub fn as_u32(&self) -> u32 {
        ((self.r as u32) << 24) + ((self.g as u32) << 16) + ((self.b as u32) << 8) + (self.a as u32)
    }

    pub fn apply_alpha(&self, alpha: f64) -> Self {
        let mut result = self.clone();

        result.a = ((self.a as f64) * alpha).round() as u8;

        result
    }
}

impl Color {
    impl_color! { transparent : [0, 0, 0, 0] }
    impl_color! { aliceblue : [240, 248, 255, 255] }
    impl_color! { antiquewhite : [250, 235, 215, 255] }
    impl_color! { aqua : [0, 255, 255, 255] }
    impl_color! { aquamarine : [127, 255, 212, 255] }
    impl_color! { azure : [240, 255, 255, 255] }
    impl_color! { beige : [245, 245, 220, 255] }
    impl_color! { bisque : [255, 228, 196, 255] }
    impl_color! { black : [0, 0, 0, 255] }
    impl_color! { blanchedalmond : [255, 235, 205, 255] }
    impl_color! { blue : [0, 0, 255, 255] }
    impl_color! { blueviolet : [138, 43, 226, 255] }
    impl_color! { brown : [165, 42, 42, 255] }
    impl_color! { burlywood : [222, 184, 135, 255] }
    impl_color! { cadetblue : [95, 158, 160, 255] }
    impl_color! { chartreuse : [127, 255, 0, 255] }
    impl_color! { chocolate : [210, 105, 30, 255] }
    impl_color! { coral : [255, 127, 80, 255] }
    impl_color! { cornflowerblue : [100, 149, 237, 255] }
    impl_color! { cornsilk : [255, 248, 220, 255] }
    impl_color! { crimson : [220, 20, 60, 255] }
    impl_color! { cyan : [0, 255, 255, 255] }
    impl_color! { darkblue : [0, 0, 139, 255] }
    impl_color! { darkcyan : [0, 139, 139, 255] }
    impl_color! { darkgoldenrod : [184, 134, 11, 255] }
    impl_color! { darkgray : [169, 169, 169, 255] }
    impl_color! { darkgrey : [169, 169, 169, 255] }
    impl_color! { darkgreen : [0, 100, 0, 255] }
    impl_color! { darkkhaki : [189, 183, 107, 255] }
    impl_color! { darkmagenta : [139, 0, 139, 255] }
    impl_color! { darkolivegreen : [85, 107, 47, 255] }
    impl_color! { darkorange : [255, 140, 0, 255] }
    impl_color! { darkorchid : [153, 50, 204, 255] }
    impl_color! { darkred : [139, 0, 0, 255] }
    impl_color! { darksalmon : [233, 150, 122, 255] }
    impl_color! { darkseagreen : [143, 188, 143, 255] }
    impl_color! { darkslateblue : [72, 61, 139, 255] }
    impl_color! { darkslategray : [47, 79, 79, 255] }
    impl_color! { darkslategrey : [47, 79, 79, 255] }
    impl_color! { darkturquoise : [0, 206, 209, 255] }
    impl_color! { darkviolet : [148, 0, 211, 255] }
    impl_color! { deeppink : [255, 20, 147, 255] }
    impl_color! { deepskyblue : [0, 191, 255, 255] }
    impl_color! { dimgray : [105, 105, 105, 255] }
    impl_color! { dimgrey : [105, 105, 105, 255] }
    impl_color! { dodgerblue : [30, 144, 255, 255] }
    impl_color! { firebrick : [178, 34, 34, 255] }
    impl_color! { floralwhite : [255, 250, 240, 255] }
    impl_color! { forestgreen : [34, 139, 34, 255] }
    impl_color! { fuchsia : [255, 0, 255, 255] }
    impl_color! { gainsboro : [220, 220, 220, 255] }
    impl_color! { ghostwhite : [248, 248, 255, 255] }
    impl_color! { gold : [255, 215, 0, 255] }
    impl_color! { goldenrod : [218, 165, 32, 255] }
    impl_color! { gray : [128, 128, 128, 255] }
    impl_color! { grey : [128, 128, 128, 255] }
    impl_color! { green : [0, 128, 0, 255] }
    impl_color! { greenyellow : [173, 255, 47, 255] }
    impl_color! { honeydew : [240, 255, 240, 255] }
    impl_color! { hotpink : [255, 105, 180, 255] }
    impl_color! { indianred : [205, 92, 92, 255] }
    impl_color! { indigo : [75, 0, 130, 255] }
    impl_color! { ivory : [255, 255, 240, 255] }
    impl_color! { khaki : [240, 230, 140, 255] }
    impl_color! { lavender : [230, 230, 250, 255] }
    impl_color! { lavenderblush : [255, 240, 245, 255] }
    impl_color! { lawngreen : [124, 252, 0, 255] }
    impl_color! { lemonchiffon : [255, 250, 205, 255] }
    impl_color! { lightblue : [173, 216, 230, 255] }
    impl_color! { lightcoral : [240, 128, 128, 255] }
    impl_color! { lightcyan : [224, 255, 255, 255] }
    impl_color! { lightgoldenrodyellow : [250, 250, 210, 255] }
    impl_color! { lightgray : [211, 211, 211, 255] }
    impl_color! { lightgrey : [211, 211, 211, 255] }
    impl_color! { lightgreen : [144, 238, 144, 255] }
    impl_color! { lightpink : [255, 182, 193, 255] }
    impl_color! { lightsalmon : [255, 160, 122, 255] }
    impl_color! { lightseagreen : [32, 178, 170, 255] }
    impl_color! { lightskyblue : [135, 206, 250, 255] }
    impl_color! { lightslategray : [119, 136, 153, 255] }
    impl_color! { lightslategrey : [119, 136, 153, 255] }
    impl_color! { lightsteelblue : [176, 196, 222, 255] }
    impl_color! { lightyellow : [255, 255, 224, 255] }
    impl_color! { lime : [0, 255, 0, 255] }
    impl_color! { limegreen : [50, 205, 50, 255] }
    impl_color! { linen : [250, 240, 230, 255] }
    impl_color! { magenta : [255, 0, 255, 255] }
    impl_color! { maroon : [128, 0, 0, 255] }
    impl_color! { mediumaquamarine : [102, 205, 170, 255] }
    impl_color! { mediumblue : [0, 0, 205, 255] }
    impl_color! { mediumorchid : [186, 85, 211, 255] }
    impl_color! { mediumpurple : [147, 112, 219, 255] }
    impl_color! { mediumseagreen : [60, 179, 113, 255] }
    impl_color! { mediumslateblue : [123, 104, 238, 255] }
    impl_color! { mediumspringgreen : [0, 250, 154, 255] }
    impl_color! { mediumturquoise : [72, 209, 204, 255] }
    impl_color! { mediumvioletred : [199, 21, 133, 255] }
    impl_color! { midnightblue : [25, 25, 112, 255] }
    impl_color! { mintcream : [245, 255, 250, 255] }
    impl_color! { mistyrose : [255, 228, 225, 255] }
    impl_color! { moccasin : [255, 228, 181, 255] }
    impl_color! { navajowhite : [255, 222, 173, 255] }
    impl_color! { navy : [0, 0, 128, 255] }
    impl_color! { oldlace : [253, 245, 230, 255] }
    impl_color! { olive : [128, 128, 0, 255] }
    impl_color! { olivedrab : [107, 142, 35, 255] }
    impl_color! { orange : [255, 165, 0, 255] }
    impl_color! { orangered : [255, 69, 0, 255] }
    impl_color! { orchid : [218, 112, 214, 255] }
    impl_color! { palegoldenrod : [238, 232, 170, 255] }
    impl_color! { palegreen : [152, 251, 152, 255] }
    impl_color! { paleturquoise : [175, 238, 238, 255] }
    impl_color! { palevioletred : [219, 112, 147, 255] }
    impl_color! { papayawhip : [255, 239, 213, 255] }
    impl_color! { peachpuff : [255, 218, 185, 255] }
    impl_color! { peru : [205, 133, 63, 255] }
    impl_color! { pink : [255, 192, 203, 255] }
    impl_color! { plum : [221, 160, 221, 255] }
    impl_color! { powderblue : [176, 224, 230, 255] }
    impl_color! { purple : [128, 0, 128, 255] }
    impl_color! { rebeccapurple : [102, 51, 153, 255] }
    impl_color! { red : [255, 0, 0, 255] }
    impl_color! { rosybrown : [188, 143, 143, 255] }
    impl_color! { royalblue : [65, 105, 225, 255] }
    impl_color! { saddlebrown : [139, 69, 19, 255] }
    impl_color! { salmon : [250, 128, 114, 255] }
    impl_color! { sandybrown : [244, 164, 96, 255] }
    impl_color! { seagreen : [46, 139, 87, 255] }
    impl_color! { seashell : [255, 245, 238, 255] }
    impl_color! { sienna : [160, 82, 45, 255] }
    impl_color! { silver : [192, 192, 192, 255] }
    impl_color! { skyblue : [135, 206, 235, 255] }
    impl_color! { slateblue : [106, 90, 205, 255] }
    impl_color! { slategray : [112, 128, 144, 255] }
    impl_color! { slategrey : [112, 128, 144, 255] }
    impl_color! { snow : [255, 250, 250, 255] }
    impl_color! { springgreen : [0, 255, 127, 255] }
    impl_color! { steelblue : [70, 130, 180, 255] }
    impl_color! { tan : [210, 180, 140, 255] }
    impl_color! { teal : [0, 128, 128, 255] }
    impl_color! { thistle : [216, 191, 216, 255] }
    impl_color! { tomato : [255, 99, 71, 255] }
    impl_color! { turquoise : [64, 224, 208, 255] }
    impl_color! { violet : [238, 130, 238, 255] }
    impl_color! { wheat : [245, 222, 179, 255] }
    impl_color! { white : [255, 255, 255, 255] }
    impl_color! { whitesmoke : [245, 245, 245, 255] }
    impl_color! { yellow : [255, 255, 0, 255] }
    impl_color! { yellowgreen : [154, 205, 50, 255] }
}