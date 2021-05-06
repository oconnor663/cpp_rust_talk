use std::sync::Arc;

#[derive(Debug)]
struct Widget {
    x: i32,
}

impl Widget {
    fn new() -> Self {
        Self { x: 0 }
    }
}

fn use_(w: &Widget) {
    dbg!(w);
}

fn f(w: &Widget, g_p: &mut Arc<Widget>) {
    g(g_p);
    use_(w);
}

fn g(g_p: &mut Arc<Widget>) {
    *g_p = Arc::new(Widget::new());
}

fn main() {
    let mut g_p = Arc::new(Widget::new());
    f(&*g_p, &mut g_p);
}
