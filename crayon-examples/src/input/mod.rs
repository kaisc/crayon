use crayon::prelude::*;

use crayon_imgui;
use crayon_imgui::prelude::*;

use utils;

struct Window {
    canvas: Canvas,
    renderer: Renderer,
    info: FrameInfo,
    text: String,
}

impl Window {
    fn new(engine: &mut Engine) -> errors::Result<Self> {
        let ctx = engine.context().read().unwrap();
        let (canvas, renderer) = crayon_imgui::new(&ctx).unwrap();

        Ok(Window {
               canvas: canvas,
               renderer: renderer,
               info: Default::default(),
               text: String::new(),
           })
    }
}

impl Application for Window {
    fn on_update(&mut self, ctx: &Context) -> errors::Result<()> {
        let input = ctx.shared::<InputSystem>().clone();
        self.text += &input.text();

        let ui = self.canvas.paint(&ctx);
        let info = self.info;
        let text = &self.text;
        ui.window(im_str!("ImGui & Crayon"))
            .movable(false)
            .resizable(false)
            .title_bar(false)
            .position((50.0, 50.0), ImGuiCond::FirstUseEver)
            .size((400.0, 400.0), ImGuiCond::FirstUseEver)
            .build(|| {
                ui.text(im_str!("FPS: {:?}", info.fps));
                ui.text(im_str!("DrawCalls: {:?}, Vertices: {:?}",
                                info.video.drawcall,
                                info.video.vertices));

                ui.text(im_str!("CPU: {:.2?}ms, GPU: {:.2?}ms",
                                utils::to_ms(info.duration),
                                utils::to_ms(info.video.duration)));

                ui.separator();

                if ui.collapsing_header(im_str!("Mouse")).build() {
                    let pos = input.mouse_position();
                    let movement = input.mouse_movement();
                    let scroll = input.mouse_scroll();
                    ui.text(im_str!("Position: ({:.1},{:.1})", pos.0, pos.1));
                    ui.text(im_str!("Movement: ({:.1}, {:.1})", movement.0, movement.1));
                    ui.text(im_str!("Scroll: ({:.1}, {:.1})", scroll.0, scroll.1));

                    let is_down = input.is_mouse_down(event::MouseButton::Left);
                    let is_press = input.is_mouse_press(event::MouseButton::Left);
                    let is_release = input.is_mouse_release(event::MouseButton::Left);
                    ui.text(im_str!("Down({:?}) Pressed({:?}) Released({:?})",
                                    is_down,
                                    is_press,
                                    is_release));
                };

                if ui.collapsing_header(im_str!("Keyboard")).build() {
                    let is_down = input.is_key_down(event::KeyboardButton::A);
                    let is_press = input.is_key_press(event::KeyboardButton::A);
                    let is_release = input.is_key_release(event::KeyboardButton::A);
                    ui.text(im_str!("[A] Down({:?}) Pressed({:?}) Released({:?})",
                                    is_down,
                                    is_press,
                                    is_release));

                    let is_down = input.is_key_down(event::KeyboardButton::Z);
                    let is_press = input.is_key_press(event::KeyboardButton::Z);
                    let is_release = input.is_key_release(event::KeyboardButton::Z);
                    ui.text(im_str!("[Z] Down({:?}) Pressed({:?}) Released({:?})",
                                    is_down,
                                    is_press,
                                    is_release));

                    ui.text_wrapped(im_str!("Text: {:?}.", text));
                };
            });

        self.renderer.render(ui).unwrap();
        Ok(())
    }

    fn on_post_update(&mut self, _: &Context, info: &FrameInfo) -> errors::Result<()> {
        self.info = *info;
        Ok(())
    }
}

pub fn main(title: String, _: &[String]) {
    let mut settings = Settings::default();
    settings.window.width = 500;
    settings.window.height = 500;
    settings.window.title = title;

    let mut engine = Engine::new_with(settings).unwrap();
    let window = Window::new(&mut engine).unwrap();
    engine.run(window).unwrap();
}