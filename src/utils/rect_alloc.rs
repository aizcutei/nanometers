use crate::setting::ModuleList;
use egui::*;

pub fn rect_alloc(list: Vec<ModuleList>, present_rects: Vec<Rect>, rect: Rect) -> Vec<Rect> {
    if list.len() == 0 {
        return vec![];
    } else {
        if list.len() == present_rects.len() {
            let present_max_x = present_rects[present_rects.len() - 1].max.x;
            let new_max_y = rect.max.y;
            let diff_x = rect.max.x - present_max_x;
            let add_x = diff_x / list.len() as f32;
            let mod_x = diff_x - (list.len() - 1) as f32 * add_x;
            let mut new_rects = vec![];
            for i in 0..list.len() {
                let mut new_rect = present_rects[i];
                new_rect.min.x += add_x * i as f32;
                new_rect.max.x += add_x * (i + 1) as f32;
                new_rect.max.y = new_max_y;
                new_rects.push(new_rect);
            }
            let len = new_rects.len();
            new_rects[len - 1].max.x += mod_x;
            return new_rects;
        } else {
            let new_max_y = rect.max.y;
            let new_max_x = rect.max.x;
            let mod_x = new_max_x % list.len() as f32;
            let each_x = (new_max_x - mod_x) / list.len() as f32;
            let mut new_rects = vec![];
            for i in 0..list.len() {
                let mut new_rect = rect;
                new_rect.min.x = rect.min.x + each_x * i as f32;
                new_rect.max.x = rect.min.x + each_x * (i + 1) as f32;
                new_rect.max.y = new_max_y;
                new_rects.push(new_rect);
            }
            let len = new_rects.len();
            new_rects[len - 1].max.x += mod_x;
            return new_rects;
        }
    }
}
