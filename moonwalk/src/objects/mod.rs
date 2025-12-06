// Часть проекта MoonWalk с открытым исходным кодом.
// Лицензия EPL 2.0, подробнее в файле LICENSE. UpdateDeveloper, 2025

pub mod store;

/// Айди объекта
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectId(pub usize);

/// Айди шейдера
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ShaderId(pub u32);

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectType {
    Rect = 1,
}

impl ObjectId {
    // Хардкод
    const INDEX_MASK: usize = 0x00FF_FFFF;
    const TYPE_SHIFT: usize = 24;

    #[inline(always)]
    pub fn new(ty: ObjectType, index: usize) -> Self {
        let ty_val = (ty as usize) << Self::TYPE_SHIFT;
        let idx_val = index & Self::INDEX_MASK;
        Self(ty_val | idx_val)
    }

    #[inline(always)]
    pub fn get_type(&self) -> ObjectType {
        // Кастим обратно в u8 для std::mem::transmute
        let ty_val = ((self.0 >> Self::TYPE_SHIFT) & 0xFF) as u8;
        unsafe {
            std::mem::transmute(ty_val)
        }
    }

    #[inline(always)]
    pub fn index(&self) -> usize {
        self.0 & Self::INDEX_MASK
    }
}