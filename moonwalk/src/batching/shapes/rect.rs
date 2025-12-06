// Часть проекта MoonWalk с открытым исходным кодом.
// Лицензия EPL 2.0, подробнее в файле LICENSE. UpdateDeveloper, 2025

use easy_gpu::{Buffer, Context, RenderPass};

use crate::rendering::vertex::{QuadVertex, RectInstance};
use crate::objects::store::ObjectStore;

pub struct RectBatch {
    static_vbo: Buffer<QuadVertex>,
    static_ibo: Buffer<u32>,
    instance_buffer: Option<Buffer<RectInstance>>,
    cpu_instances: Vec<RectInstance>,
}

impl RectBatch {
    pub fn new(ctx: &Context) -> Self {
        let static_vbo = Buffer::vertex(ctx, &QuadVertex::QUAD);
        let static_ibo = Buffer::<u32>::index(ctx, &QuadVertex::INDICES);

        Self {
            static_vbo,
            static_ibo,
            instance_buffer: None,
            cpu_instances: Vec::with_capacity(1024),
        }
    }

    pub fn prepare(&mut self, ctx: &Context, store: &ObjectStore) {
        if !store.dirty {
            return;
        }

        self.cpu_instances.clear();
        
        for (i, &global_id) in store.rect_ids.iter().enumerate() {
            let idx = global_id.index();

            self.cpu_instances.push(RectInstance {
                // Упаковываем позицию и размер в один вектор
                // для оптимизации
                pos_size: [
                    store.positions[idx].x,
                    store.positions[idx].y,
                    store.sizes[idx].x,
                    store.sizes[idx].y,
                ],

                color: store.colors[idx].to_array(),
                
                radii: store.rect_radii[i].to_array(),
                
                // Упаковываем z индекс, вращение и отсупы
                extra: [
                    store.z_indices[idx],
                    store.rotations[idx],
                    0.0, // Padding
                    0.0  // Padding
                ],
            });
        }
        
        // Сортировка объектов по Z идексу 
        self.cpu_instances.sort_unstable_by(|a, b| {
            a.extra[0].total_cmp(&b.extra[0])
        });

        if self.cpu_instances.is_empty() {
            return;
        }

        if let Some(buf) = &mut self.instance_buffer {
            buf.update(ctx, &self.cpu_instances);
        } else {
            self.instance_buffer = Some(Buffer::vertex(ctx, &self.cpu_instances));
        }
    }

    pub fn render<'a>(&'a self, pass: &mut RenderPass<'a>) {
        if let Some(inst_buf) = &self.instance_buffer {
            let count = self.cpu_instances.len() as u32;
            
            if count > 0 {
                pass.set_vertex_buffer(0, &self.static_vbo);
                pass.set_vertex_buffer(1, inst_buf);
                pass.set_index_buffer(&self.static_ibo);
                pass.draw_indexed_instanced(6, count);
            }
        }
    }
}