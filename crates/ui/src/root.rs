use gpui::{div, AnyView, ParentElement as _, Render, Styled, ViewContext, WindowContext};
use std::{
    ops::{Deref, DerefMut},
    rc::Rc,
};

use crate::{drawer::Drawer, modal::Modal, theme::ActiveTheme};

/// Extension trait for [`WindowContext`] and [`ViewContext`] to add drawer functionality.
pub trait ContextModal: Sized {
    /// Opens a Drawer.
    fn open_drawer<F>(&mut self, build: F)
    where
        F: Fn(Drawer, &mut WindowContext) -> Drawer + 'static;

    /// Return the active Drawer builder, you must add the Drawer to the view.
    fn active_drawer(&self) -> Option<Rc<dyn Fn(Drawer, &mut WindowContext) -> Drawer + 'static>>;

    /// Closes the active Drawer.
    fn close_drawer(&mut self);

    /// Opens a Modal.
    fn open_modal<F>(&mut self, build: F)
    where
        F: Fn(Modal, &mut WindowContext) -> Modal + 'static;

    /// Return the active Modal builder, you must add the Modal to the view.
    fn active_modal(&self) -> Option<Rc<dyn Fn(Modal, &mut WindowContext) -> Modal + 'static>>;

    /// Closes the active Modal.
    fn close_modal(&mut self);
}

impl<'a> ContextModal for WindowContext<'a> {
    fn open_drawer<F>(&mut self, build: F)
    where
        F: Fn(Drawer, &mut WindowContext) -> Drawer + 'static,
    {
        Root::update_root(self, move |root, cx| {
            root.active_drawer = Some(Rc::new(build));
            cx.notify();
        })
    }

    fn active_drawer(&self) -> Option<Rc<dyn Fn(Drawer, &mut WindowContext) -> Drawer + 'static>> {
        Root::read_root(&self).active_drawer.clone()
    }

    fn close_drawer(&mut self) {
        Root::update_root(self, |root, cx| {
            root.active_drawer = None;
            cx.notify();
        })
    }

    fn open_modal<F>(&mut self, build: F)
    where
        F: Fn(Modal, &mut WindowContext) -> Modal + 'static,
    {
        Root::update_root(self, move |root, cx| {
            root.active_modal = Some(Rc::new(build));
            cx.notify();
        })
    }

    fn active_modal(&self) -> Option<Rc<dyn Fn(Modal, &mut WindowContext) -> Modal + 'static>> {
        Root::read_root(&self).active_modal.clone()
    }

    fn close_modal(&mut self) {
        Root::update_root(self, |root, cx| {
            root.active_modal = None;
            cx.notify();
        })
    }
}
impl<'a, V> ContextModal for ViewContext<'a, V> {
    fn open_drawer<F>(&mut self, build: F)
    where
        F: Fn(Drawer, &mut WindowContext) -> Drawer + 'static,
    {
        self.deref_mut().open_drawer(build)
    }

    fn active_modal(&self) -> Option<Rc<dyn Fn(Modal, &mut WindowContext) -> Modal + 'static>> {
        self.deref().active_modal()
    }

    fn close_drawer(&mut self) {
        self.deref_mut().close_drawer()
    }

    fn open_modal<F>(&mut self, build: F)
    where
        F: Fn(Modal, &mut WindowContext) -> Modal + 'static,
    {
        self.deref_mut().open_modal(build)
    }

    fn active_drawer(&self) -> Option<Rc<dyn Fn(Drawer, &mut WindowContext) -> Drawer + 'static>> {
        self.deref().active_drawer()
    }

    fn close_modal(&mut self) {
        self.deref_mut().close_modal()
    }
}

pub struct Root {
    active_drawer: Option<Rc<dyn Fn(Drawer, &mut WindowContext) -> Drawer + 'static>>,
    active_modal: Option<Rc<dyn Fn(Modal, &mut WindowContext) -> Modal + 'static>>,
    root_view: AnyView,
}

impl Root {
    pub fn new(root_view: AnyView, _: &mut ViewContext<Self>) -> Self {
        Self {
            active_drawer: None,
            active_modal: None,
            root_view,
        }
    }

    fn update_root<F>(cx: &mut WindowContext, f: F)
    where
        F: FnOnce(&mut Self, &mut ViewContext<Self>) + 'static,
    {
        let root = cx
            .window_handle()
            .downcast::<Root>()
            .and_then(|w| w.root_view(cx).ok())
            .expect("The window root view should be of type `ui::Root`.");

        root.update(cx, |root, cx| f(root, cx))
    }

    fn read_root<'a>(cx: &'a WindowContext) -> &'a Self {
        let root = cx
            .window_handle()
            .downcast::<Root>()
            .and_then(|w| w.root_view(cx).ok())
            .expect("The window root view should be of type `ui::Root`.");

        root.read(cx)
    }
}

impl Render for Root {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl gpui::IntoElement {
        div()
            .size_full()
            .text_color(cx.theme().foreground)
            .child(self.root_view.clone())
    }
}