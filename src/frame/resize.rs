use egui::*;

pub fn resize_ui(ui: &mut egui::Ui, rect: egui::Rect) {
    let thickness = 2.0;
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

    if top_edge_response.is_pointer_button_down_on() {
        ui.ctx().set_cursor_icon(CursorIcon::ResizeVertical);
        ui.ctx().send_viewport_cmd(ViewportCommand::BeginResize(
            viewport::ResizeDirection::North,
        ));
    } else if top_edge_response.contains_pointer() {
        ui.ctx().set_cursor_icon(CursorIcon::ResizeVertical);
    }
}

pub fn bottom_edge(ui: &mut egui::Ui, bottom_edge_rect: eframe::epaint::Rect) {
    let bottom_edge_response =
        ui.interact(bottom_edge_rect, Id::new("bottom_edge"), Sense::click());

    if bottom_edge_response.is_pointer_button_down_on() {
        ui.ctx().set_cursor_icon(CursorIcon::ResizeVertical);
        ui.ctx().send_viewport_cmd(ViewportCommand::BeginResize(
            viewport::ResizeDirection::South,
        ));
    } else if bottom_edge_response.contains_pointer() {
        ui.ctx().set_cursor_icon(CursorIcon::ResizeVertical);
    }
}

pub fn left_edge(ui: &mut egui::Ui, left_edge_rect: eframe::epaint::Rect) {
    let left_edge_response = ui.interact(left_edge_rect, Id::new("left_edge"), Sense::click());

    if left_edge_response.is_pointer_button_down_on() {
        ui.ctx().set_cursor_icon(CursorIcon::ResizeHorizontal);
        ui.ctx().send_viewport_cmd(ViewportCommand::BeginResize(
            viewport::ResizeDirection::West,
        ));
    } else if left_edge_response.contains_pointer() {
        ui.ctx().set_cursor_icon(CursorIcon::ResizeHorizontal);
    }
}

pub fn right_edge(ui: &mut egui::Ui, right_edge_rect: eframe::epaint::Rect) {
    let right_edge_response = ui.interact(right_edge_rect, Id::new("right_edge"), Sense::click());

    if right_edge_response.is_pointer_button_down_on() {
        ui.ctx().set_cursor_icon(CursorIcon::ResizeHorizontal);
        ui.ctx().send_viewport_cmd(ViewportCommand::BeginResize(
            viewport::ResizeDirection::East,
        ));
    } else if right_edge_response.contains_pointer() {
        ui.ctx().set_cursor_icon(CursorIcon::ResizeHorizontal);
    }
}

pub fn left_top_corner(ui: &mut egui::Ui, left_top_corner_rect: eframe::epaint::Rect) {
    let left_top_corner_response = ui.interact(
        left_top_corner_rect,
        Id::new("left_top_corner"),
        Sense::click(),
    );

    if left_top_corner_response.is_pointer_button_down_on() {
        ui.ctx().set_cursor_icon(CursorIcon::ResizeNwSe);
        ui.ctx().send_viewport_cmd(ViewportCommand::BeginResize(
            viewport::ResizeDirection::NorthWest,
        ));
    } else if left_top_corner_response.contains_pointer() {
        ui.ctx().set_cursor_icon(CursorIcon::ResizeNwSe);
    }
}

pub fn right_top_corner(ui: &mut egui::Ui, right_top_corner_rect: eframe::epaint::Rect) {
    let right_top_corner_response = ui.interact(
        right_top_corner_rect,
        Id::new("right_top_corner"),
        Sense::click(),
    );

    if right_top_corner_response.is_pointer_button_down_on() {
        ui.ctx().set_cursor_icon(CursorIcon::ResizeNeSw);
        ui.ctx().send_viewport_cmd(ViewportCommand::BeginResize(
            viewport::ResizeDirection::NorthEast,
        ));
    } else if right_top_corner_response.contains_pointer() {
        ui.ctx().set_cursor_icon(CursorIcon::ResizeNeSw);
    }
}

pub fn left_bottom_corner(ui: &mut egui::Ui, left_bottom_corner_rect: eframe::epaint::Rect) {
    let left_bottom_corner_response = ui.interact(
        left_bottom_corner_rect,
        Id::new("left_bottom_corner"),
        Sense::click(),
    );

    if left_bottom_corner_response.is_pointer_button_down_on() {
        ui.ctx().set_cursor_icon(CursorIcon::ResizeNeSw);
        ui.ctx().send_viewport_cmd(ViewportCommand::BeginResize(
            viewport::ResizeDirection::SouthWest,
        ));
    } else if left_bottom_corner_response.contains_pointer() {
        ui.ctx().set_cursor_icon(CursorIcon::ResizeNeSw);
    }
}

pub fn right_bottom_corner(ui: &mut egui::Ui, right_bottom_corner_rect: eframe::epaint::Rect) {
    let right_bottom_corner_response = ui.interact(
        right_bottom_corner_rect,
        Id::new("right_bottom_corner"),
        Sense::click(),
    );

    if right_bottom_corner_response.is_pointer_button_down_on() {
        ui.ctx().set_cursor_icon(CursorIcon::ResizeNwSe);
        ui.ctx().send_viewport_cmd(ViewportCommand::BeginResize(
            viewport::ResizeDirection::SouthEast,
        ));
    } else if right_bottom_corner_response.contains_pointer() {
        ui.ctx().set_cursor_icon(CursorIcon::ResizeNwSe);
    }
}
