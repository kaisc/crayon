use imgui;
use crayon::{application, graphics, input};
use errors::*;

pub struct Canvas {
    canvas: imgui::ImGui,
}

impl Canvas {
    pub fn new(mut imgui: imgui::ImGui) -> Result<Self> {
        Self::bind_keycode(&mut imgui);
        Ok(Canvas { canvas: imgui })
    }

    pub fn paint<'a>(&'a mut self, ctx: &application::Context) -> imgui::Ui<'a> {
        // Update input device states.
        let input = ctx.shared::<input::InputSystem>();
        Self::update_mouse_state(&mut self.canvas, &input);
        Self::update_keycode_state(&mut self.canvas, &input);

        // Generates frame builder.
        let v = ctx.shared::<graphics::GraphicsSystem>();
        let duration = ctx.frame_delta();
        let ts = duration.as_secs() as f32 + duration.subsec_nanos() as f32 / 1_000_000_000.0;

        //
        let (dp, d) = (v.dimensions_in_pixels(), v.dimensions());
        self.canvas.frame(d, dp, ts)
    }

    fn bind_keycode(imgui: &mut imgui::ImGui) {
        use imgui::ImGuiKey;

        imgui.set_imgui_key(ImGuiKey::Tab, 0);
        imgui.set_imgui_key(ImGuiKey::LeftArrow, 1);
        imgui.set_imgui_key(ImGuiKey::RightArrow, 2);
        imgui.set_imgui_key(ImGuiKey::UpArrow, 3);
        imgui.set_imgui_key(ImGuiKey::DownArrow, 4);
        imgui.set_imgui_key(ImGuiKey::PageUp, 5);
        imgui.set_imgui_key(ImGuiKey::PageDown, 6);
        imgui.set_imgui_key(ImGuiKey::Home, 7);
        imgui.set_imgui_key(ImGuiKey::End, 8);
        imgui.set_imgui_key(ImGuiKey::Delete, 9);
        imgui.set_imgui_key(ImGuiKey::Backspace, 10);
        imgui.set_imgui_key(ImGuiKey::Enter, 11);
        imgui.set_imgui_key(ImGuiKey::Escape, 12);
        imgui.set_imgui_key(ImGuiKey::A, 13);
        imgui.set_imgui_key(ImGuiKey::C, 14);
        imgui.set_imgui_key(ImGuiKey::V, 15);
        imgui.set_imgui_key(ImGuiKey::X, 16);
        imgui.set_imgui_key(ImGuiKey::Y, 17);
        imgui.set_imgui_key(ImGuiKey::Z, 18);
    }

    fn update_keycode_state(imgui: &mut imgui::ImGui, input: &input::InputSystemShared) {
        use self::input::KeyboardButton;

        imgui.set_key(0, input.is_key_press(KeyboardButton::Tab));
        imgui.set_key(1, input.is_key_press(KeyboardButton::Left));
        imgui.set_key(2, input.is_key_press(KeyboardButton::Right));
        imgui.set_key(3, input.is_key_press(KeyboardButton::Up));
        imgui.set_key(4, input.is_key_press(KeyboardButton::Down));
        imgui.set_key(5, input.is_key_press(KeyboardButton::PageUp));
        imgui.set_key(6, input.is_key_press(KeyboardButton::PageDown));
        imgui.set_key(7, input.is_key_press(KeyboardButton::Home));
        imgui.set_key(8, input.is_key_press(KeyboardButton::End));
        imgui.set_key(9, input.is_key_press(KeyboardButton::Delete));
        imgui.set_key(10, input.is_key_press(KeyboardButton::Back));
        imgui.set_key(11, input.is_key_press(KeyboardButton::Return));
        imgui.set_key(12, input.is_key_press(KeyboardButton::Escape));
        imgui.set_key(13, input.is_key_press(KeyboardButton::A));
        imgui.set_key(14, input.is_key_press(KeyboardButton::C));
        imgui.set_key(15, input.is_key_press(KeyboardButton::V));
        imgui.set_key(16, input.is_key_press(KeyboardButton::X));
        imgui.set_key(17, input.is_key_press(KeyboardButton::Y));
        imgui.set_key(18, input.is_key_press(KeyboardButton::Z));
    }

    fn update_mouse_state(imgui: &mut imgui::ImGui, input: &input::InputSystemShared) {
        let scale = imgui.display_framebuffer_scale();

        let pos = input.mouse_position();
        let pos = (pos.0 as f32 / scale.0, pos.1 as f32 / scale.1);
        imgui.set_mouse_pos(pos.0, pos.1);

        let l = input.is_mouse_down(input::MouseButton::Left);
        let r = input.is_mouse_down(input::MouseButton::Right);
        let m = input.is_mouse_down(input::MouseButton::Middle);
        imgui.set_mouse_down(&[l, r, m, false, false]);
    }
}