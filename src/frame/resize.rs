use egui::*;

pub fn resize_ui(ui: &mut egui::Ui, rect: egui::Rect) {
    let thickness = 1.0;
    let left_top_corner_rect = egui::Rect::from_min_max(
        egui::Pos2::new(rect.min.x, rect.min.y),
        egui::Pos2::new(rect.min.x + thickness, rect.min.y + thickness),
    );
    let top_edge_rect = egui::Rect::from_min_max(
        egui::Pos2::new(rect.min.x + thickness, rect.min.y),
        egui::Pos2::new(rect.max.x - thickness, rect.min.y + thickness),
    );
    let right_top_corner_rect = egui::Rect::from_min_max(
        egui::Pos2::new(rect.max.x - thickness, rect.min.y),
        egui::Pos2::new(rect.max.x, rect.min.y + thickness),
    );
    let right_edge_rect = egui::Rect::from_min_max(
        egui::Pos2::new(rect.max.x - thickness, rect.min.y + thickness),
        egui::Pos2::new(rect.max.x, rect.max.y - thickness),
    );
    let right_bottom_corner_rect = egui::Rect::from_min_max(
        egui::Pos2::new(rect.max.x - thickness, rect.max.y - thickness),
        egui::Pos2::new(rect.max.x, rect.max.y),
    );
    let bottom_edge_rect = egui::Rect::from_min_max(
        egui::Pos2::new(rect.min.x + thickness, rect.max.y - thickness),
        egui::Pos2::new(rect.max.x - thickness, rect.max.y),
    );
    let left_bottom_corner_rect = egui::Rect::from_min_max(
        egui::Pos2::new(rect.min.x, rect.max.y - thickness),
        egui::Pos2::new(rect.min.x + thickness, rect.max.y),
    );
    let left_edge_rect = egui::Rect::from_min_max(
        egui::Pos2::new(rect.min.x, rect.min.y + thickness),
        egui::Pos2::new(rect.min.x + thickness, rect.max.y - thickness),
    );
    left_top_corner(ui, left_top_corner_rect);
    top_edge(ui, top_edge_rect);
    right_top_corner(ui, right_top_corner_rect);
    right_edge(ui, right_edge_rect);
    right_bottom_corner(ui, right_bottom_corner_rect);
    bottom_edge(ui, bottom_edge_rect);
    left_bottom_corner(ui, left_bottom_corner_rect);
    left_edge(ui, left_edge_rect);
}

pub fn top_edge(ui: &mut egui::Ui, top_edge_rect: eframe::epaint::Rect) {
    let top_edge_response = ui.interact(top_edge_rect, Id::new("top_edge"), Sense::click());

    let top_edge_hover = ui.interact(top_edge_rect, Id::new("top_edge"), Sense::hover());

    if top_edge_response.is_pointer_button_down_on() {
        ui.ctx().send_viewport_cmd(ViewportCommand::BeginResize(
            viewport::ResizeDirection::North,
        ));
    }

    top_edge_hover.on_hover_cursor(CursorIcon::ResizeVertical);
}

pub fn bottom_edge(ui: &mut egui::Ui, bottom_edge_rect: eframe::epaint::Rect) {
    let bottom_edge_response =
        ui.interact(bottom_edge_rect, Id::new("bottom_edge"), Sense::click());

    let bottom_edge_hover = ui.interact(bottom_edge_rect, Id::new("bottom_edge"), Sense::hover());

    if bottom_edge_response.is_pointer_button_down_on() {
        ui.ctx().send_viewport_cmd(ViewportCommand::BeginResize(
            viewport::ResizeDirection::South,
        ));
    }

    bottom_edge_hover.on_hover_cursor(CursorIcon::ResizeVertical);
}

pub fn left_edge(ui: &mut egui::Ui, left_edge_rect: eframe::epaint::Rect) {
    let left_edge_response = ui.interact(left_edge_rect, Id::new("left_edge"), Sense::click());

    let left_edge_hover = ui.interact(left_edge_rect, Id::new("left_edge"), Sense::hover());

    if left_edge_response.is_pointer_button_down_on() {
        ui.ctx().send_viewport_cmd(ViewportCommand::BeginResize(
            viewport::ResizeDirection::West,
        ));
    }

    left_edge_hover.on_hover_cursor(CursorIcon::ResizeHorizontal);
}

pub fn right_edge(ui: &mut egui::Ui, right_edge_rect: eframe::epaint::Rect) {
    let right_edge_response = ui.interact(right_edge_rect, Id::new("right_edge"), Sense::click());

    let right_edge_hover = ui.interact(right_edge_rect, Id::new("right_edge"), Sense::hover());

    if right_edge_response.is_pointer_button_down_on() {
        ui.ctx().send_viewport_cmd(ViewportCommand::BeginResize(
            viewport::ResizeDirection::East,
        ));
    }

    right_edge_hover.on_hover_cursor(CursorIcon::ResizeHorizontal);
}

pub fn left_top_corner(ui: &mut egui::Ui, left_top_corner_rect: eframe::epaint::Rect) {
    let left_top_corner_response = ui.interact(
        left_top_corner_rect,
        Id::new("left_top_corner"),
        Sense::click(),
    );

    let left_top_corner_hover = ui.interact(
        left_top_corner_rect,
        Id::new("left_top_corner"),
        Sense::hover(),
    );

    if left_top_corner_response.is_pointer_button_down_on() {
        ui.ctx().send_viewport_cmd(ViewportCommand::BeginResize(
            viewport::ResizeDirection::NorthWest,
        ));
    }

    left_top_corner_hover.on_hover_cursor(CursorIcon::ResizeNwSe);
}

pub fn right_top_corner(ui: &mut egui::Ui, right_top_corner_rect: eframe::epaint::Rect) {
    let right_top_corner_response = ui.interact(
        right_top_corner_rect,
        Id::new("right_top_corner"),
        Sense::click(),
    );

    let right_top_corner_hover = ui.interact(
        right_top_corner_rect,
        Id::new("right_top_corner"),
        Sense::hover(),
    );

    if right_top_corner_response.is_pointer_button_down_on() {
        ui.ctx().send_viewport_cmd(ViewportCommand::BeginResize(
            viewport::ResizeDirection::NorthEast,
        ));
    }

    right_top_corner_hover.on_hover_cursor(CursorIcon::ResizeNeSw);
}

pub fn left_bottom_corner(ui: &mut egui::Ui, left_bottom_corner_rect: eframe::epaint::Rect) {
    let left_bottom_corner_response = ui.interact(
        left_bottom_corner_rect,
        Id::new("left_bottom_corner"),
        Sense::click(),
    );

    let left_bottom_corner_hover = ui.interact(
        left_bottom_corner_rect,
        Id::new("left_bottom_corner"),
        Sense::hover(),
    );

    if left_bottom_corner_response.is_pointer_button_down_on() {
        ui.ctx().send_viewport_cmd(ViewportCommand::BeginResize(
            viewport::ResizeDirection::SouthWest,
        ));
    }

    left_bottom_corner_hover.on_hover_cursor(CursorIcon::ResizeNeSw);
}

pub fn right_bottom_corner(ui: &mut egui::Ui, right_bottom_corner_rect: eframe::epaint::Rect) {
    let right_bottom_corner_response = ui.interact(
        right_bottom_corner_rect,
        Id::new("right_bottom_corner"),
        Sense::click(),
    );

    let right_bottom_corner_hover = ui.interact(
        right_bottom_corner_rect,
        Id::new("right_bottom_corner"),
        Sense::hover(),
    );

    if right_bottom_corner_response.is_pointer_button_down_on() {
        ui.ctx().send_viewport_cmd(ViewportCommand::BeginResize(
            viewport::ResizeDirection::SouthEast,
        ));
    }

    right_bottom_corner_hover.on_hover_cursor(CursorIcon::ResizeNwSe);
}
