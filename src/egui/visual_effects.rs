use egui::{Color32, Pos2, Stroke, Vec2};
use crate::grafo_metro::{CorLinha, GrafoMetro, IdEstacao};
use super::app::MinhaAplicacaoGUI;

/// Desenha marcadores visuais acima das estações
pub fn desenhar_marcadores_estacoes(
    app: &MinhaAplicacaoGUI, 
    painter: &egui::Painter, 
    rect_desenho: egui::Rect, 
    grafo: &GrafoMetro, 
    _ui: &egui::Ui
) {
    if !app.mostrar_marcadores_estacoes {
        return;
    }
    
    for (i, _estacao) in grafo.estacoes.iter().enumerate() {
        let pos = app.posicoes_estacoes_tela[i] * app.zoom_nivel + app.offset_rolagem + rect_desenho.min.to_vec2();
        let pos_marcador = pos + Vec2::new(0.0, -35.0 * app.zoom_nivel);
        
        let marcador_info = determinar_marcador(app, i);
        
        if let Some((texto, cor_fundo, cor_borda)) = marcador_info {
            desenhar_marcador(app, painter, pos_marcador, pos, texto, cor_fundo, cor_borda);
        }
    }
}

fn determinar_marcador(app: &MinhaAplicacaoGUI, id_estacao: IdEstacao) -> Option<(&'static str, Color32, Color32)> {
    if id_estacao == app.id_estacao_inicio_selecionada {
        Some(("INÍCIO", Color32::from_rgb(0, 140, 0), Color32::from_rgb(20, 80, 20)))
    } else if id_estacao == app.id_estacao_objetivo_selecionada {
        Some(("FIM", Color32::from_rgb(220, 50, 50), Color32::from_rgb(80, 20, 20)))
    } else if app.estacoes_exploradas_ui.contains(&id_estacao) && app.resultado_caminho_ui.is_some() {
        Some(("CAMINHO", Color32::from_rgb(0, 120, 60), Color32::from_rgb(20, 60, 40)))
    } else if app.estacoes_exploradas_ui.contains(&id_estacao) && app.solucionador_a_estrela.is_some() {
        Some(("EXPLORANDO", Color32::from_rgb(60, 100, 200), Color32::from_rgb(30, 50, 100)))
    } else if app.vizinhos_sendo_analisados_ui.contains(&id_estacao) {
        Some(("ANALISANDO", Color32::from_rgb(255, 140, 0), Color32::from_rgb(120, 60, 0)))
    } else {
        None
    }
}

fn desenhar_marcador(
    app: &MinhaAplicacaoGUI, 
    painter: &egui::Painter, 
    pos_marcador: Pos2, 
    pos_estacao: Pos2,
    texto: &str, 
    cor_fundo: Color32, 
    cor_borda: Color32
) {
    let fonte = egui::FontId::proportional((9.0 * app.zoom_nivel).max(8.0));
    let tamanho_texto = painter.ctx().fonts(|f| f.layout_no_wrap(texto.to_string(), fonte.clone(), Color32::WHITE));
    
    let padding = Vec2::new(6.0, 3.0) * app.zoom_nivel;
    let rect_fundo = egui::Rect::from_center_size(
        pos_marcador,
        tamanho_texto.rect.size() + padding * 2.0
    );
    
    painter.rect_filled(
        rect_fundo.translate(Vec2::new(1.0, 1.0) * app.zoom_nivel),
        3.0 * app.zoom_nivel,
        Color32::from_rgba_premultiplied(0, 0, 0, 120)
    );
    
    painter.rect_filled(rect_fundo, 3.0 * app.zoom_nivel, cor_fundo);
    
    painter.rect_stroke(
        rect_fundo,
        3.0 * app.zoom_nivel,
        Stroke::new(1.0 * app.zoom_nivel, cor_borda),
        egui::StrokeKind::Middle
    );
    
    painter.text(pos_marcador, egui::Align2::CENTER_CENTER, texto, fonte, Color32::WHITE);
    
    desenhar_seta_marcador(app, painter, pos_marcador, pos_estacao, rect_fundo, cor_borda);
}

fn desenhar_seta_marcador(
    app: &MinhaAplicacaoGUI,
    painter: &egui::Painter,
    pos_marcador: Pos2,
    pos_estacao: Pos2,
    rect_fundo: egui::Rect,
    cor_borda: Color32
) {
    let pos_seta_inicio = pos_marcador + Vec2::new(0.0, rect_fundo.height() / 2.0);
    let pos_seta_fim = pos_estacao + Vec2::new(0.0, -22.0 * app.zoom_nivel);
    
    painter.line_segment(
        [pos_seta_inicio, pos_seta_fim],
        Stroke::new(1.5 * app.zoom_nivel, cor_borda)
    );
    
    let tamanho_ponta = 3.0 * app.zoom_nivel;
    let ponta1 = pos_seta_fim + Vec2::new(-tamanho_ponta, -tamanho_ponta);
    let ponta2 = pos_seta_fim + Vec2::new(tamanho_ponta, -tamanho_ponta);
    
    painter.line_segment([pos_seta_fim, ponta1], Stroke::new(1.5 * app.zoom_nivel, cor_borda));
    painter.line_segment([pos_seta_fim, ponta2], Stroke::new(1.5 * app.zoom_nivel, cor_borda));
}

/// Desenha ícone de baldeação entre linhas
pub fn desenhar_icone_baldeacao(
    app: &MinhaAplicacaoGUI,
    painter: &egui::Painter,
    posicao: Pos2,
    tamanho: f32,
    linhas: Option<(CorLinha, CorLinha)>
) {
    if let Some((de_linha, para_linha)) = linhas {
        let cor1 = obter_cor_linha_baldeacao(de_linha);
        let cor2 = obter_cor_linha_baldeacao(para_linha);
        
        // Desenhar base da baldeação
        desenhar_base_baldeacao(app, painter, posicao, tamanho);
        
        // Desenhar semi-círculos de cores diferentes
        desenhar_semicirculos_baldeacao(app, painter, posicao, tamanho, cor1, cor2);
        
        // Desenhar símbolo de transferência
        desenhar_simbolo_transferencia(app, painter, posicao, tamanho);
        
        // Desenhar balão de tempo
        desenhar_balao_tempo_baldeacao(app, painter, posicao, tamanho);
    }
}

fn obter_cor_linha_baldeacao(linha: CorLinha) -> Color32 {
    match linha {
        CorLinha::Azul => Color32::from_rgb(0, 120, 255),
        CorLinha::Amarela => Color32::from_rgb(255, 215, 0),
        CorLinha::Vermelha => Color32::RED,
        CorLinha::Verde => Color32::from_rgb(0, 180, 0),
        _ => Color32::GRAY,
    }
}

fn desenhar_base_baldeacao(app: &MinhaAplicacaoGUI, painter: &egui::Painter, posicao: Pos2, tamanho: f32) {
    // Sombra do círculo
    painter.circle_filled(
        posicao + Vec2::new(1.0, 1.0) * app.zoom_nivel,
        tamanho + 4.0,
        Color32::from_rgba_premultiplied(0, 0, 0, 180),
    );
    
    // Círculo de fundo branco
    painter.circle_filled(
        posicao,
        tamanho + 4.0,
        Color32::from_rgba_premultiplied(255, 255, 255, 250),
    );
    
    // Círculo de fundo preto
    painter.circle_filled(
        posicao,
        tamanho + 2.0,
        Color32::from_rgba_premultiplied(0, 0, 0, 220),
    );
}

fn desenhar_semicirculos_baldeacao(
    app: &MinhaAplicacaoGUI,
    painter: &egui::Painter,
    posicao: Pos2,
    tamanho: f32,
    cor1: Color32,
    cor2: Color32
) {
    let raio = tamanho * 1.5;
    
    // Semi-círculo da primeira linha
    painter.add(egui::epaint::PathShape::convex_polygon(
        vec![
            posicao,
            posicao + Vec2::new(-raio, 0.0),
            posicao + Vec2::new(-raio * 0.7071, -raio * 0.7071),
            posicao + Vec2::new(0.0, -raio),
            posicao + Vec2::new(raio * 0.7071, -raio * 0.7071),
            posicao + Vec2::new(raio, 0.0),
        ],
        cor1,
        Stroke::new(0.0, Color32::BLACK),
    ));
    
    // Semi-círculo da segunda linha
    painter.add(egui::epaint::PathShape::convex_polygon(
        vec![
            posicao,
            posicao + Vec2::new(raio, 0.0),
            posicao + Vec2::new(raio * 0.7071, raio * 0.7071),
            posicao + Vec2::new(0.0, raio),
            posicao + Vec2::new(-raio * 0.7071, raio * 0.7071),
            posicao + Vec2::new(-raio, 0.0),
        ],
        cor2,
        Stroke::new(0.0, Color32::BLACK),
    ));
}

fn desenhar_simbolo_transferencia(app: &MinhaAplicacaoGUI, painter: &egui::Painter, posicao: Pos2, tamanho: f32) {
    let seta_tam = tamanho * 0.8;
    
    // Círculo central
    painter.circle_stroke(
        posicao, 
        seta_tam,
        Stroke::new(1.5 * app.zoom_nivel, Color32::WHITE)
    );
    
    // Desenhar setas circulares
    desenhar_setas_circulares(app, painter, posicao, seta_tam, tamanho);
}

fn desenhar_setas_circulares(
    app: &MinhaAplicacaoGUI,
    painter: &egui::Painter,
    posicao: Pos2,
    seta_tam: f32,
    tamanho: f32
) {
    let angulo_inicio = std::f32::consts::PI * 0.8;
    let comp_seta = std::f32::consts::PI * 0.6;
    
    // Primeira seta
    desenhar_seta_circular(app, painter, posicao, seta_tam, tamanho, angulo_inicio, comp_seta);
    
    // Segunda seta (espelhada)
    let angulo_inicio2 = angulo_inicio + std::f32::consts::PI;
    desenhar_seta_circular(app, painter, posicao, seta_tam, tamanho, angulo_inicio2, comp_seta);
}

fn desenhar_seta_circular(
    app: &MinhaAplicacaoGUI,
    painter: &egui::Painter,
    posicao: Pos2,
    seta_tam: f32,
    tamanho: f32,
    angulo_inicio: f32,
    comp_seta: f32
) {
    let ponto1 = posicao + Vec2::new(seta_tam * f32::cos(angulo_inicio), seta_tam * f32::sin(angulo_inicio));
    let ponto2 = posicao + Vec2::new(seta_tam * f32::cos(angulo_inicio + comp_seta), seta_tam * f32::sin(angulo_inicio + comp_seta));
    
    painter.line_segment(
        [ponto1, ponto2],
        Stroke::new(2.0 * app.zoom_nivel, Color32::WHITE),
    );
    
    // Ponta da seta
    let angulo_ponta = angulo_inicio + comp_seta;
    let ponta_seta = posicao + Vec2::new(seta_tam * f32::cos(angulo_ponta), seta_tam * f32::sin(angulo_ponta));
    let angulo_ponta_1 = angulo_ponta + std::f32::consts::PI * 0.8;
    let angulo_ponta_2 = angulo_ponta + std::f32::consts::PI * 1.2;
    let tam_ponta = tamanho * 0.3;
    
    painter.line_segment(
        [
            ponta_seta,
            ponta_seta + Vec2::new(tam_ponta * f32::cos(angulo_ponta_1), tam_ponta * f32::sin(angulo_ponta_1)),
        ],
        Stroke::new(2.0 * app.zoom_nivel, Color32::WHITE),
    );
    
    painter.line_segment(
        [
            ponta_seta,
            ponta_seta + Vec2::new(tam_ponta * f32::cos(angulo_ponta_2), tam_ponta * f32::sin(angulo_ponta_2)),
        ],
        Stroke::new(2.0 * app.zoom_nivel, Color32::WHITE),
    );
}

fn desenhar_balao_tempo_baldeacao(app: &MinhaAplicacaoGUI, painter: &egui::Painter, posicao: Pos2, tamanho: f32) {
    let texto_tempo = "+4.0min"; // Tempo fixo de baldeação
    
    let texto_galley = painter.layout_no_wrap(
        texto_tempo.to_string(),
        egui::FontId::proportional(11.0 * app.zoom_nivel),
        Color32::WHITE,
    );
    
    let padding = 4.0 * app.zoom_nivel;
    let tamanho_balao = texto_galley.size() + Vec2::new(padding * 2.0, padding * 2.0);
    let pos_balao = posicao + Vec2::new(0.0, -tamanho - 8.0 * app.zoom_nivel);
    
    let bg_rect = egui::Rect::from_center_size(pos_balao, tamanho_balao);
    
    // Sombra do balão
    painter.rect_filled(
        bg_rect.translate(Vec2::new(1.0, 1.0) * app.zoom_nivel),
        4.0 * app.zoom_nivel,
        Color32::from_rgba_premultiplied(0, 0, 0, 100),
    );
    
    // Fundo do balão
    painter.rect_filled(
        bg_rect,
        4.0 * app.zoom_nivel,
        Color32::from_rgba_premultiplied(60, 0, 60, 220),
    );
    
    // Borda do balão
    painter.rect_stroke(
        bg_rect,
        4.0 * app.zoom_nivel,
        Stroke::new(1.0 * app.zoom_nivel, Color32::from_rgba_premultiplied(255, 200, 255, 180)),
        egui::StrokeKind::Middle,
    );
    
    // Texto do tempo
    painter.text(
        pos_balao,
        egui::Align2::CENTER_CENTER,
        texto_tempo,
        egui::FontId::proportional(10.0 * app.zoom_nivel),
        Color32::WHITE,
    );
}
