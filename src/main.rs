mod world;

use anyhow::Result;
use offbrand::{color, Context, MouseButton};
use world::{Cell, CellKind};

fn main() -> Result<()> {
    let mut world = world::World::new(100, 100);
    let mut ctx = Context::new(1000, 1000, "cellular".to_owned())?;

    while ctx.is_open() {
        ctx.clear();
        world.render(&mut ctx, 10);

        if ctx.get_mouse_down(MouseButton::Left) {
            if let Some(mouse_pos) = ctx.get_mouse_pos() {
                let (x, y) = (mouse_pos.0 as usize / 10, mouse_pos.1 as usize / 10);
                world.insert(x, y, Cell::new(CellKind::Solid { color: color::WHITE }))
            }
        }

        world.update();

        ctx.present()?;
    }

    Ok(())
}
