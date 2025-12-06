// Часть проекта MoonWalk с открытым исходным кодом.
// Лицензия EPL 2.0, подробнее в файле LICENSE. UpdateDeveloper, 2025

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct QuadVertex {
    // Позиция вершины в 2D пространстве (Мировая система координат)
    pub position: [f32; 2],
}

impl QuadVertex {
    // Описание констант для прямоугольника. Всегда 4 вершины
    pub const QUAD: [Self; 4] = [
        Self { position: [0.0, 0.0] },
        Self { position: [0.0, 1.0] },
        Self { position: [1.0, 1.0] },
        Self { position: [1.0, 0.0] },
    ];

    // и 6 индексов.
    pub const INDICES: [u32; 6] = [0, 1, 2, 0, 2, 3];
}

/// Структура для экземпляра прямоугольника. Лайаут:
/// 1: pos_size (x, y, w, h) (координаты x/y и ширина/высота w/h)
/// 2: color (r, g, b, a) (красный, зелёный, синий и альфв канал)
/// 3: radii (tl, tr, br, bl) (Верх-лево, верх-право, низ-право, низ-лево)
/// 4: extra (z, rotation, padding, padding)
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct RectInstance {
    pub pos_size: [f32; 4],
    pub color:    [f32; 4],
    pub radii:    [f32; 4],
    pub extra:    [f32; 4],
}