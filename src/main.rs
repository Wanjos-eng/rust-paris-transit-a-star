mod grafo_metro;
mod dados_metro;
mod algoritmo_a_estrela;
mod aplicacao_gui; // Nosso módulo da GUI

use aplicacao_gui::MinhaAplicacaoGUI; // Importa nossa struct da aplicação

fn main() -> Result<(), eframe::Error> {
    println!("Iniciando aplicação GUI do Metrô de Paris A*...");

    let opcoes_nativas = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0]) // Tamanho inicial da janela
            .with_min_inner_size([800.0, 600.0]),  // Tamanho mínimo
        ..Default::default()
    };

    // Roda a aplicação egui/eframe
    eframe::run_native(
        "Metrô de Paris - Planejador de Rotas A*", // Título da Janela
        opcoes_nativas,
        Box::new(|cc| Ok(Box::new(MinhaAplicacaoGUI::new(cc)))), // Cria e passa nossa app
    )
}