// Часть проекта MoonWalk с открытым исходным кодом.
// Лицензия EPL 2.0, подробнее в файле LICENSE. UpdateDeveloper, 2025

use glam::{Vec2, Vec4};

use crate::objects;
use crate::objects::ObjectId;

/// Хранилище для объектов
pub struct ObjectStore {
    // Основные данные (SoA)
    pub positions: Vec<Vec2>,
    pub sizes: Vec<Vec2>,
    pub colors: Vec<Vec4>,
    pub rotations: Vec<f32>,
    pub z_indices: Vec<f32>,
    
    // Айди объектов
    pub rect_ids: Vec<ObjectId>,
    
    // Данные специфичные для прямоугольника
    pub rect_radii: Vec<Vec4>,

    pub dirty: bool,
}

impl ObjectStore {
    pub fn new() -> Self {
        Self {
            // Оптимизация: Сразу же даём капасити
            positions: Vec::with_capacity(1024),
            sizes: Vec::with_capacity(1024),
            colors: Vec::with_capacity(1024),
            rotations: Vec::with_capacity(1024),
            z_indices: Vec::with_capacity(1024),
            rect_ids: Vec::with_capacity(1024),
            rect_radii: Vec::with_capacity(1024),

            // Объекты изначально не грязные потому-что их нет
            dirty: false,
        }
    }

    fn alloc_common(&mut self) -> usize {
        let index = self.positions.len();

        self.positions.push(Vec2::ZERO); // Нулевая позиция (Левый верхний угол)
        self.sizes.push(Vec2::new(100.0, 100.0)); // Позиция 100 на 100
        self.colors.push(Vec4::ONE); // Цвет белый (1, 1, 1, 1)
        self.rotations.push(0.0); // Вращение: 0.0 радиан
        self.z_indices.push(0.0); // Нулевой z индекс
        
        // После создания объекта нам нужно пересобрать всё, поэтому
        // делаем хранилище грязным
        self.dirty = true; 

        index
    }

    pub fn new_rect(&mut self) -> ObjectId {
        // Делаем аллокацию
        let id = self.alloc_common();

        // Добавляем прямоугольник
        self.rect_ids.push(objects::ObjectId(id));
        
        // Нулевое скруглением углов
        self.rect_radii.push(Vec4::ZERO);
        
        objects::ObjectId(id)
    }

    /// Каждая функция конфигурации должна делать хранилище объектов
    /// грязным чтобы пересобрать всё

    #[inline(always)]
    pub fn config_position(&mut self, id: ObjectId, pos: Vec2) {
        self.positions[id.index()] = pos;
        self.dirty = true;
    }

    #[inline(always)]
    pub fn config_size(&mut self, id: ObjectId, size: Vec2) {
        self.sizes[id.index()] = size;
        self.dirty = true;
    }

    #[inline(always)]
    pub fn config_color(&mut self, id: ObjectId, color: Vec4) {
        self.colors[id.index()] = color;
        self.dirty = true;
    }
    
    #[inline(always)]
    pub fn config_rotation(&mut self, id: ObjectId, rad: f32) {
        self.rotations[id.index()] = rad;
        self.dirty = true;
    }

    #[inline(always)]
    pub fn config_z_index(&mut self, id: ObjectId, z: f32) {
        self.z_indices[id.index()] = z;
        self.dirty = true;
    }

    pub fn set_rounded(&mut self, id: ObjectId, radii: Vec4) {
        // Тут нужен поиск, так как radii специфичный
        if let Some(idx) = self.rect_ids.iter().position(|&x| x == id) {
            self.rect_radii[idx] = radii;
            self.dirty = true;
        }
    }
}