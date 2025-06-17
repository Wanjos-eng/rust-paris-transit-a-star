use egui::{Color32, Pos2, Stroke, Vec2};
use crate::grafo_metro::{CorLinha, GrafoMetro, IdEstacao};
use super::app::MinhaAplicacaoGUI;

/// Desenha todas as conexões entre estações
pub fn desenhar_conexoes(app: &MinhaAplicacaoGUI, painter: &egui::Painter, rect_desenho: egui::Rect, grafo: &GrafoMetro) {
    // Desenhar conexões normais (não na solução)
    desenhar_conexoes_normais(app, painter, rect_desenho, grafo);
    
    // Desenhar o caminho da solução destacado
    desenhar_caminho_solucao(app, painter, rect_desenho, grafo);
}

fn desenhar_conexoes_normais(app: &MinhaAplicacaoGUI, painter: &egui::Painter, rect_desenho: egui::Rect, grafo: &GrafoMetro) {
    for (id_origem, conexoes) in grafo.lista_adjacencia.iter().enumerate() {
        for conexao in conexoes {
            let id_destino = conexao.para_estacao;
            
            // Verificar se esta conexão faz parte da solução
            let na_solucao = esta_na_solucao(app, id_origem, id_destino, conexao.cor_linha);
            if na_solucao {
                continue; // Pular conexões da solução
            }
            
            // Posições das estações
            let pos_origem = app.posicoes_estacoes_tela[id_origem] * app.zoom_nivel + app.offset_rolagem + rect_desenho.min.to_vec2();
            let pos_destino = app.posicoes_estacoes_tela[id_destino] * app.zoom_nivel + app.offset_rolagem + rect_desenho.min.to_vec2();
            
            // Cor e espessura da linha
            let (cor_linha, espessura) = obter_cor_linha(conexao.cor_linha);
            
            // Desenhar linha com opacidade reduzida
            painter.line_segment(
                [pos_origem, pos_destino], 
                Stroke::new(espessura * app.zoom_nivel, cor_linha.gamma_multiply(0.3))
            );
            
            // Desenhar tempo da conexão se ativado
            if app.mostrar_tempos_conexao {
                let meio = (pos_origem + pos_destino.to_vec2()) / 2.0;
                let texto_tempo = format!("{:.1}", conexao.tempo_minutos);
                let tamanho_texto = egui::FontId::proportional(11.0 * app.zoom_nivel);
                
                desenhar_balao_tempo(
                    app,
                    painter,
                    meio,
                    texto_tempo,
                    tamanho_texto,
                    false
                );
            }
        }
    }
}

fn desenhar_caminho_solucao(app: &MinhaAplicacaoGUI, painter: &egui::Painter, rect_desenho: egui::Rect, grafo: &GrafoMetro) {
    if let Some(ref caminho_info) = app.resultado_caminho_ui {
        let cor_solucao = Color32::from_rgb(0, 150, 136); // Verde-azul escuro
        let espessura_solucao = 6.5 * app.zoom_nivel;
        
        // Desenhar cada segmento do caminho
        for i in 0..caminho_info.estacoes_do_caminho.len().saturating_sub(1) {
            let (id_origem, _) = caminho_info.estacoes_do_caminho[i];
            let (id_destino, _) = caminho_info.estacoes_do_caminho[i+1];
            
            let pos_origem = app.posicoes_estacoes_tela[id_origem] * app.zoom_nivel + app.offset_rolagem + rect_desenho.min.to_vec2();
            let pos_destino = app.posicoes_estacoes_tela[id_destino] * app.zoom_nivel + app.offset_rolagem + rect_desenho.min.to_vec2();
            
            // Linha principal com brilho
            painter.line_segment(
                [pos_origem, pos_destino], 
                Stroke::new(espessura_solucao, cor_solucao)
            );
            
            // Efeito de brilho externo
            painter.line_segment(
                [pos_origem, pos_destino], 
                Stroke::new(espessura_solucao + 2.0, Color32::from_rgba_premultiplied(0, 100, 80, 40))
            );
            
            // Desenhar tempo da conexão se ativado
            if app.mostrar_tempos_conexao {
                let tempo = obter_tempo_conexao(grafo, id_origem, id_destino);
                let meio = (pos_origem + pos_destino.to_vec2()) / 2.0;
                let texto_tempo = format!("{:.1}", tempo);
                let tamanho_texto = egui::FontId::proportional(12.0 * app.zoom_nivel);
                
                desenhar_balao_tempo(
                    app,
                    painter,
                    meio,
                    texto_tempo,
                    tamanho_texto,
                    true
                );
            }
            
            // Desenhar ícones de baldeação
            desenhar_icones_baldeacao(app, painter, rect_desenho, caminho_info, i);
        }
    }
}

/// Desenha as estações com seus status visuais
pub fn desenhar_estacoes(app: &mut MinhaAplicacaoGUI, painter: &egui::Painter, rect_desenho: egui::Rect, grafo: &GrafoMetro, ui: &mut egui::Ui) {
    for (i, estacao) in grafo.estacoes.iter().enumerate() {
        let pos = app.posicoes_estacoes_tela[i] * app.zoom_nivel + app.offset_rolagem + rect_desenho.min.to_vec2();
        
        // Determinar status da estação
        let esta_na_solucao = if let Some(ref info_caminho) = app.resultado_caminho_ui {
            info_caminho.estacoes_do_caminho.iter().any(|(id, _)| *id == i)
        } else {
            false
        };
        
        let e_vizinho_sendo_analisado = app.vizinhos_sendo_analisados_ui.contains(&i);
        let e_sendo_explorada_agora = if let Some(ref solucionador) = app.solucionador_a_estrela {
            solucionador.estacao_sendo_explorada_no_momento == Some(i)
        } else {
            false
        };
        
        // Desenhar efeitos visuais
        desenhar_efeitos_estacao(app, painter, pos, e_vizinho_sendo_analisado, e_sendo_explorada_agora);
        
        // Desenhar círculo da estação
        desenhar_circulo_estacao(app, painter, pos, i, grafo);
        
        // Processar interação com a estação
        processar_interacao_estacao(app, ui, pos, i, grafo);
    }
}

fn desenhar_efeitos_estacao(app: &MinhaAplicacaoGUI, painter: &egui::Painter, pos: Pos2, e_vizinho: bool, e_explorando: bool) {
    // Efeito para vizinhos sendo analisados
    if e_vizinho {
        painter.circle_stroke(
            pos,
            20.0 * app.zoom_nivel,
            Stroke::new(2.0 * app.zoom_nivel, Color32::from_rgba_premultiplied(255, 165, 0, 160))
        );
        
        painter.circle_stroke(
            pos,
            16.0 * app.zoom_nivel,
            Stroke::new(1.0 * app.zoom_nivel, Color32::from_rgba_premultiplied(255, 140, 0, 100))
        );
    }
    
    // Efeito para estação sendo explorada
    if e_explorando {
        painter.circle_stroke(
            pos,
            24.0 * app.zoom_nivel,
            Stroke::new(3.0 * app.zoom_nivel, Color32::from_rgba_premultiplied(255, 220, 0, 200))
        );
        
        painter.circle_stroke(
            pos,
            21.0 * app.zoom_nivel,
            Stroke::new(2.0 * app.zoom_nivel, Color32::from_rgba_premultiplied(255, 200, 0, 150))
        );
    }
}

fn desenhar_circulo_estacao(app: &MinhaAplicacaoGUI, painter: &egui::Painter, pos: Pos2, id_estacao: IdEstacao, grafo: &GrafoMetro) {
    // Cor de preenchimento baseada no status
    let cor_preenchimento = obter_cor_preenchimento_estacao(app, id_estacao);
    
    // Sombra
    painter.circle_filled(
        pos + Vec2::new(1.5, 1.5) * app.zoom_nivel,
        18.0 * app.zoom_nivel,
        Color32::from_rgba_premultiplied(0, 0, 0, 160)
    );
    
    // Círculo principal
    painter.circle_filled(
        pos,
        18.0 * app.zoom_nivel,
        cor_preenchimento
    );
    
    // Borda
    let (cor_borda, espessura_borda) = obter_cor_borda_estacao(app, id_estacao);
    painter.circle_stroke(
        pos, 
        18.0 * app.zoom_nivel, 
        Stroke::new(espessura_borda * app.zoom_nivel, cor_borda)
    );
    
    // ID da estação
    if app.mostrar_ids_estacoes {
        painter.text(
            pos,
            egui::Align2::CENTER_CENTER,
            &format!("E{}", id_estacao + 1),
            egui::FontId::proportional(12.5 * app.zoom_nivel),
            Color32::WHITE,
        );
    }
    
    // Nome da estação para estações relevantes
    let mostrar_nome = app.resultado_caminho_ui.as_ref()
        .map(|info| info.estacoes_do_caminho.iter().any(|(id, _)| *id == id_estacao))
        .unwrap_or(false) ||
        id_estacao == app.id_estacao_inicio_selecionada ||
        id_estacao == app.id_estacao_objetivo_selecionada;
    
    if mostrar_nome {
        painter.text(
            pos + Vec2::new(0.0, 20.0 * app.zoom_nivel),
            egui::Align2::CENTER_TOP,
            &grafo.estacoes[id_estacao].nome,
            egui::FontId::proportional(12.0 * app.zoom_nivel),
            Color32::WHITE
        );
    }
}

fn processar_interacao_estacao(app: &mut MinhaAplicacaoGUI, ui: &mut egui::Ui, pos: Pos2, id_estacao: IdEstacao, grafo: &GrafoMetro) {
    let area_interacao = egui::Rect::from_center_size(pos, Vec2::splat(32.0 * app.zoom_nivel));
    let response = ui.interact(
        area_interacao,
        egui::Id::new(format!("estacao_{}", id_estacao)),
        egui::Sense::click_and_drag(),
    );
    
    // Hover effect
    if response.hovered() {
        ui.painter().circle_stroke(
            pos, 
            17.0 * app.zoom_nivel, 
            Stroke::new(1.0 * app.zoom_nivel, Color32::WHITE)
        );
        
        // Mostrar popup informativo
        if app.vizinhos_sendo_analisados_ui.contains(&id_estacao) {
            super::popups::mostrar_popup_vizinho_hover(app, ui, pos, id_estacao, grafo);
        } else {
            super::popups::mostrar_popup_estacao_hover(app, ui, pos, id_estacao, grafo);
        }
    }
    
    // Click handling
    if response.clicked() && !ui.input(|i| i.pointer.is_decidedly_dragging()) {
        super::popups::abrir_popup_estacao(app, id_estacao, grafo);
    }
}

// Funções auxiliares

fn esta_na_solucao(app: &MinhaAplicacaoGUI, id_origem: IdEstacao, id_destino: IdEstacao, cor_linha: CorLinha) -> bool {
    if let Some(ref info_caminho) = app.resultado_caminho_ui {
        for i in 0..info_caminho.estacoes_do_caminho.len().saturating_sub(1) {
            let (id1, _linha1) = info_caminho.estacoes_do_caminho[i];
            let (id2, linha2) = info_caminho.estacoes_do_caminho[i+1];
            
            if id1 == id_origem && id2 == id_destino {
                return linha2.map_or(false, |l| l == cor_linha);
            }
        }
    }
    false
}

fn obter_cor_linha(cor_linha: CorLinha) -> (Color32, f32) {
    match cor_linha {
        CorLinha::Azul => (Color32::from_rgb(0, 120, 255), 3.5),
        CorLinha::Amarela => (Color32::from_rgb(255, 215, 0), 3.5),
        CorLinha::Vermelha => (Color32::RED, 3.5),
        CorLinha::Verde => (Color32::from_rgb(0, 180, 0), 3.5),
        _ => (Color32::GRAY, 3.0)
    }
}

fn obter_tempo_conexao(grafo: &GrafoMetro, id_origem: IdEstacao, id_destino: IdEstacao) -> f32 {
    if let Some(conexoes) = grafo.lista_adjacencia.get(id_origem) {
        conexoes.iter()
               .find(|con| con.para_estacao == id_destino)
               .map_or(0.0, |con| con.tempo_minutos)
    } else {
        0.0
    }
}

fn obter_cor_preenchimento_estacao(app: &MinhaAplicacaoGUI, id_estacao: IdEstacao) -> Color32 {
    if id_estacao == app.id_estacao_inicio_selecionada {
        Color32::from_rgb(0, 60, 0)
    } else if id_estacao == app.id_estacao_objetivo_selecionada {
        Color32::from_rgb(60, 0, 0)
    } else if let Some(ref solucionador) = app.solucionador_a_estrela {
        match solucionador.obter_status_estacao(id_estacao) {
            crate::algoritmo_a_estrela::StatusEstacao::Disponivel => Color32::from_rgb(40, 42, 54),
            crate::algoritmo_a_estrela::StatusEstacao::SelecionadaParaExpansao => Color32::from_rgb(60, 60, 20),
            crate::algoritmo_a_estrela::StatusEstacao::ExpandindoVizinhos => Color32::from_rgb(20, 40, 60),
            crate::algoritmo_a_estrela::StatusEstacao::Explorada => {
                if app.resultado_caminho_ui.is_some() {
                    Color32::from_rgb(0, 40, 20)
                } else {
                    Color32::from_rgb(20, 30, 50)
                }
            },
        }
    } else {
        Color32::from_rgb(40, 42, 54)
    }
}

fn obter_cor_borda_estacao(app: &MinhaAplicacaoGUI, id_estacao: IdEstacao) -> (Color32, f32) {
    if id_estacao == app.id_estacao_inicio_selecionada {
        (Color32::from_rgb(0, 220, 0), 4.0)
    } else if id_estacao == app.id_estacao_objetivo_selecionada {
        (Color32::from_rgb(220, 50, 50), 4.0)
    } else if let Some(ref solucionador) = app.solucionador_a_estrela {
        match solucionador.obter_status_estacao(id_estacao) {
            crate::algoritmo_a_estrela::StatusEstacao::Disponivel => (Color32::from_rgb(150, 150, 200), 2.0),
            crate::algoritmo_a_estrela::StatusEstacao::SelecionadaParaExpansao => (Color32::from_rgb(255, 255, 0), 4.0),
            crate::algoritmo_a_estrela::StatusEstacao::ExpandindoVizinhos => (Color32::from_rgb(0, 150, 255), 3.5),
            crate::algoritmo_a_estrela::StatusEstacao::Explorada => {
                if app.resultado_caminho_ui.is_some() {
                    (Color32::from_rgb(0, 255, 150), 3.0)
                } else {
                    (Color32::from_rgb(100, 150, 255), 2.5)
                }
            },
        }
    } else if app.vizinhos_sendo_analisados_ui.contains(&id_estacao) {
        (Color32::from_rgb(255, 140, 0), 2.5)
    } else if app.resultado_caminho_ui.as_ref().map(|info| 
        info.estacoes_do_caminho.iter().any(|(id, _)| *id == id_estacao)).unwrap_or(false) {
        (Color32::from_rgb(0, 150, 136), 3.0)
    } else {
        (Color32::from_rgb(150, 150, 200), 2.0)
    }
}

fn desenhar_balao_tempo(app: &MinhaAplicacaoGUI, painter: &egui::Painter, posicao: Pos2, texto: String, tamanho_fonte: egui::FontId, destacado: bool) {
    let texto_galley = painter.layout_no_wrap(
        texto.clone(),
        tamanho_fonte.clone(),
        Color32::WHITE,
    );

    let padding = 5.0 * app.zoom_nivel;
    let raio_arredondamento = 6.0 * app.zoom_nivel;
    let tamanho_balao = texto_galley.size() + Vec2::new(padding * 2.0, padding * 2.0);

    let cor_fundo = if destacado {
        Color32::from_rgba_premultiplied(0, 80, 120, 250)
    } else {
        Color32::from_rgba_premultiplied(20, 20, 20, 240)
    };

    let cor_borda = if destacado {
        Color32::from_rgb(100, 200, 255)
    } else {
        Color32::from_gray(120)
    };

    let cor_texto = if destacado {
        Color32::from_rgb(255, 255, 160)
    } else {
        Color32::WHITE
    };

    let bg_rect = egui::Rect::from_center_size(posicao, tamanho_balao);

    if destacado {
        painter.rect_filled(
            bg_rect.translate(Vec2::new(2.0, 2.0) * app.zoom_nivel),
            raio_arredondamento,
            Color32::from_rgba_premultiplied(0, 0, 0, 100),
        );
    }

    painter.rect_filled(bg_rect, raio_arredondamento, cor_fundo);
    painter.rect_stroke(bg_rect, raio_arredondamento, Stroke::new(1.5 * app.zoom_nivel, cor_borda), egui::StrokeKind::Middle);
    painter.text(posicao, egui::Align2::CENTER_CENTER, texto, tamanho_fonte, cor_texto);
}

fn desenhar_icones_baldeacao(app: &MinhaAplicacaoGUI, painter: &egui::Painter, rect_desenho: egui::Rect, caminho_info: &crate::algoritmo_a_estrela::InfoCaminho, i: usize) {
    if i < caminho_info.estacoes_do_caminho.len().saturating_sub(2) {
        let (_, linha_atual) = caminho_info.estacoes_do_caminho[i+1];
        let (_, proxima_linha) = caminho_info.estacoes_do_caminho[i+2];
        
        if linha_atual.is_some() && proxima_linha.is_some() && linha_atual != proxima_linha {
            let (id_destino, _) = caminho_info.estacoes_do_caminho[i+1];
            let pos_baldeacao = app.posicoes_estacoes_tela[id_destino] * app.zoom_nivel + 
                                app.offset_rolagem + rect_desenho.min.to_vec2() + 
                                Vec2::new(0.0, -45.0 * app.zoom_nivel);
            
            super::visual_effects::desenhar_icone_baldeacao(
                app,
                painter,
                pos_baldeacao,
                10.0 * app.zoom_nivel,
                Some((linha_atual.unwrap(), proxima_linha.unwrap()))
            );
        }
    }
}
