use std::rc::Rc;

use crate::traits::{local_data::LocalData, player::Player, request::Request, view::{View}};

use super::rect::Rect;

#[derive(Debug)]
pub enum LayoutType {
    Cell,
    Line,
    Column
}

#[derive(Debug)]
pub struct Layout<P : Player, R : Request, D : LocalData> {
    pub create_view: Option<fn(Rect) -> Rc<dyn View<P, R, D>>>,
    pub layout_type: LayoutType,
    pub force: f32,
    pub inner_margin: f32,
    pub outer_margin: f32,
    pub scale: f32,
    pub aspect_ratio: Option<f32>,
    pub children: Vec<Layout<P, R, D>>
}

impl<P : Player, R : Request, D : LocalData> Default for Layout<P, R, D> {
    fn default() -> Self {
        Self {
            create_view: None,
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

impl<P : Player, R : Request, D : LocalData> Layout<P, R, D> {
    pub fn apply(&self, input: Rect) -> Vec<Rc<dyn View<P, R, D>>> {
        let mut result = vec![];

        let rect = input
            .strip_margin(self.outer_margin)
            .strip_to_match_aspect_ratio(self.aspect_ratio)
            .scale(self.scale);
        
        if let Some(create) = self.create_view {
            result.push(create(rect.clone()));
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
        let available_space = flex_space - (self.inner_margin * (self.children.len() - 1) as f32);
        let total_force = self.children.iter().fold(0., |acc, child| acc + child.force);

        let mut x = rect.x1();
        let mut y = rect.y1();

        for child in &self.children {
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

    pub fn f(&mut self, value: f32) {
        self.force = value;
    }

    pub fn m(&mut self, value: f32) {
        self.inner_margin = value;
        self.outer_margin = value;
    }

    pub fn s(&mut self, value: f32) {
        self.scale = value;
    }

    pub fn a(&mut self, value: f32) {
        self.aspect_ratio = Some(value);
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
macro_rules! layout {
    (
        $layout:ident,
        $($view:ty)?
        $( $(,)? { $( $method:ident : $value:expr $(,)? )* } )?
        $( $(,)? [ $( $child:expr ),* ])?
    ) => {
        {
            let mut layout = lotus::Layout::default();

            layout.$layout();

            $( layout.create_view = Some(|rect| std::rc::Rc::new(<$view>::new(rect))); )?

            $( $( layout.$method(($value) as f32); )* )?

            $( $( layout.children.push($child); )* )?

            layout
        }
    };
}

#[macro_export]
macro_rules! row {
    (
        $($view:ty)?
        $( $(,)? { $( $method:ident : $value:expr $(,)? )* } )?
        $( $(,)? [ $( $child:expr ),* ])?
    ) => {
        layout!(line, $($view)? $({ $($method:$value),* })? $([ $($child),* ])?  )
    }
}

#[macro_export]
macro_rules! col {
    (
        $($view:ty)?
        $( $(,)? { $( $method:ident : $value:expr $(,)? )* } )?
        $( $(,)? [ $( $child:expr ),* ])?
    ) => {
        layout!(column, $($view)? $({ $($method:$value),* })? $([ $($child),* ])?  )
    }
}

#[macro_export]
macro_rules! l {
    (
        $($view:ty)?
        $( $(,)? { $( $method:ident : $value:expr $(,)? )* } )?
        $( $(,)? [ $( $child:expr ),* ])?
    ) => {
        layout!(cell, $($view)? $({ $($method:$value),* })? $([ $($child),* ])?  )
    }
}

pub use layout;
pub use row;
pub use col;
pub use l;