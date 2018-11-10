extern crate time;

#[derive(FromForm)]
pub mod models {
    pub struct gasto {
        valor: f32,
        tipo: u8,
        data: time::Tm,
        cadastroData: time::Tm,
        descricao: String,
        idUsuario: u32,
    }
}