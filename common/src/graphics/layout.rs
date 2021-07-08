use std::{mem::take, rc::Rc};

use crate::traits::{view::{View}};

use super::rect::Rect;

#[derive(Debug)]
pub enum LayoutType {
    Cell,
    Line,
    Column
}

#[derive(Debug)]
pub struct Layout<U, R, E, D> {
    pub view: Option<Rc<dyn View<U, R, E, D>>>,
    pub layout_type: LayoutType,
    pub force: f64,
    pub inner_margin: f64,
    pub outer_margin: f64,
    pub scale: f64,
    pub aspect_ratio: Option<f64>,
    pub children: Vec<Layout<U, R, E, D>>
}

impl<U, R, E, D> Default for Layout<U, R, E, D> {
    fn default() -> Self {
        Self {
            view: None,
            layout_type: LayoutType::Cell,
            force: 1.,
            inner_margin: 0.,
            outer_margin: 0.,
            scale: 1.,
            aspect_ratio: None,
            children: vec![]
        }
    }
}

impl<U, R, E, D> Layout<U, R, E, D> {
    pub fn apply(&mut self, input: Rect) -> Vec<(Rc<dyn View<U, R, E, D>>, Rect)> {
        let mut result = vec![];

        let rect = input
            .strip_margin(self.outer_margin)
            .strip_to_match_aspect_ratio(self.aspect_ratio)
            .scale(self.scale);
        
        if let Some(view) = take(&mut self.view) {
            result.push((view, rect));
        }

        if self.children.len() == 0 {
            return result;
        }

        let is_horizontal = match self.layout_type {
            LayoutType::Cell => return result,
            LayoutType::Line => true,
            LayoutType::Column => false,
        };
        let flex_space = match is_horizontal {
            true => rect.width,
            false => rect.height
        };
        let available_space = flex_space - (self.inner_margin * (self.children.len() - 1) as f64);
        let total_force = self.children.iter().fold(0., |acc, child| acc + child.force);

        let mut x = rect.x1();
        let mut y = rect.y1();

        for child in &mut self.children {
            let variable_dimension = child.force / total_force * available_space;
            let rect_x = x;
            let rect_y = y;
            let mut rect_width = rect.width;
            let mut rect_height = rect.height;

            match is_horizontal {
                true => {
                    rect_width = variable_dimension;
                    x += variable_dimension + self.outer_margin;
                },
                false => {
                    rect_height = variable_dimension;
                    y += variable_dimension + self.outer_margin;
                }
            }

            let child_rect = Rect::from_top_left(rect_x, rect_y, rect_width, rect_height);

            result.append(&mut child.apply(child_rect));
        }

        result
    }

    pub fn v<V : View<U, R, E, D> + 'static>(&mut self, value: V) {
        self.view = Some(Rc::new(value));
    }

    pub fn f<T : Into<f64>>(&mut self, value: T) {
        self.force = value.into();
    }

    pub fn m<T : Into<f64>>(&mut self, value: T) {
        let margin = value.into();

        self.inner_margin = margin;
        self.outer_margin = margin;
    }

    pub fn s<T : Into<f64>>(&mut self, value: T) {
        self.scale = value.into();
    }

    pub fn a<T : Into<f64>>(&mut self, value: T) {
        self.aspect_ratio = Some(value.into());
    }

    pub fn cell(&mut self) {
        self.layout_type = LayoutType::Cell;
    }

    pub fn line(&mut self) {
        self.layout_type = LayoutType::Line;
    }

    pub fn column(&mut self) {
        self.layout_type = LayoutType::Column;
    }
}

#[macro_export]
macro_rules! compute_layout {
    ( { $( $method:ident $(: $value:expr)? ),* $( ,[ $( $child:tt ),* ])? } ) => {
        {
            let mut layout = lotus::Layout::default();

            $(
                layout.$method(
                    $($value)?
                );
            )*

            $(
                $(
                {
                    let child = compute_layout!($child);

                    layout.children.push(child);
                }
                )*
            )?

            layout
        }
    };
}

#[macro_export]
macro_rules! layout {
    ($output:expr, $value:tt) => {
        {
            let output = $output;
            let mut layout = compute_layout!($value);
            let mut views = layout.apply(output.parent_rect.clone());

            output.children.append(&mut views);
        }
    };
}

pub use layout;
pub use compute_layout;