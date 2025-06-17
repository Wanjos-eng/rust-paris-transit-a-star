use egui::Vec2;
use super::app::MinhaAplicacaoGUI;

/// Centraliza a visualização do grafo na tela
pub fn centralizar_visualizacao(app: &mut MinhaAplicacaoGUI, tamanho_disponivel: Vec2) {
    let centro = tamanho_disponivel / 2.0;
    let centro_grafo = app.posicoes_estacoes_tela.iter()
        .fold(Vec2::ZERO, |acc, p| acc + p.to_vec2()) / (app.posicoes_estacoes_tela.len() as f32);
    app.offset_rolagem = centro - centro_grafo * app.zoom_nivel;
}

/// Processa eventos de navegação
pub fn processar_eventos_navegacao(
    app: &mut MinhaAplicacaoGUI, 
    ui: &mut egui::Ui, 
    response: &egui::Response, 
    rect_desenho: egui::Rect
) {
    processar_arrasto(app, ui, response);
    processar_zoom(app, ui, response, rect_desenho);
}

fn processar_arrasto(app: &mut MinhaAplicacaoGUI, ui: &mut egui::Ui, response: &egui::Response) {
    if response.dragged() {
        if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
            if let Some(last) = app.ultima_posicao_mouse {
                let delta = pos - last;
                app.offset_rolagem += delta;
            }
            app.ultima_posicao_mouse = Some(pos);
        }
    } else {
        app.ultima_posicao_mouse = None;
    }
}

fn processar_zoom(
    app: &mut MinhaAplicacaoGUI, 
    ui: &mut egui::Ui, 
    response: &egui::Response, 
    rect_desenho: egui::Rect
) {
    if response.hovered() {
        let scroll_delta = ui.input(|i| i.raw_scroll_delta.y);
        if scroll_delta != 0.0 {
            let fator_zoom = if scroll_delta > 0.0 { 1.1 } else { 0.9 };
            let novo_zoom = (app.zoom_nivel * fator_zoom).clamp(0.3, 2.5);
            
            if let Some(pos_mouse) = ui.input(|i| i.pointer.interact_pos()) {
                let pos_antes = (pos_mouse - rect_desenho.min.to_vec2() - app.offset_rolagem) / app.zoom_nivel;
                app.zoom_nivel = novo_zoom;
                let pos_depois = (pos_mouse - rect_desenho.min.to_vec2() - app.offset_rolagem) / app.zoom_nivel;
                
                app.offset_rolagem += (pos_depois - pos_antes) * app.zoom_nivel;
            } else {
                app.zoom_nivel = novo_zoom;
            }
        }
    }
}
