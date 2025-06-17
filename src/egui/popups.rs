use std::collections::HashSet;
use std::cell::RefCell;
use egui::{Color32, Id, Pos2, Vec2};
use crate::grafo_metro::{CorLinha, GrafoMetro, IdEstacao};
use super::app::{MinhaAplicacaoGUI, PopupInfo, AcaoPopup, TipoAcaoPopup};

/// Desenha popups persistentes com informações das estações
pub fn desenhar_popups(
    app: &mut MinhaAplicacaoGUI, 
    ui: &mut egui::Ui, 
    rect_desenho: egui::Rect, 
    _grafo: &GrafoMetro
) -> Vec<AcaoPopup> {
    let mut acoes = Vec::new();
    
    let posicoes_estacoes = app.posicoes_estacoes_tela.clone();
    let zoom_nivel = app.zoom_nivel;
    let offset_rolagem = app.offset_rolagem;
    let grafo = app.grafo_metro.as_ref().map(|g| g.as_ref());
    
    let popup_data: Vec<_> = app.popups_info.iter()
        .filter(|(_, popup)| popup.visivel)
        .map(|(id, popup)| (*id, popup.clone()))
        .collect();
    
    if let Some(grafo) = grafo {
        for (id, popup) in popup_data {
            let pos_estacao = posicoes_estacoes[id] * zoom_nivel + offset_rolagem + rect_desenho.min.to_vec2();
            let offset_popup = *popup.posicao.borrow();
            let pos_popup = pos_estacao + offset_popup;
            
            desenhar_popup_persistente(grafo, ui, id, &popup, pos_popup, &mut acoes);
        }
    }
    
    acoes
}

fn desenhar_popup_persistente(
    grafo: &crate::grafo_metro::GrafoMetro,
    ui: &mut egui::Ui,
    id_estacao: IdEstacao,
    popup: &PopupInfo,
    pos_popup: Pos2,
    acoes: &mut Vec<AcaoPopup>
) {
    let _area = egui::Area::new(Id::new(format!("popup_persistente_{}", id_estacao)))
        .fixed_pos(pos_popup)
        .order(egui::Order::Foreground)
        .constrain(true)
        .show(ui.ctx(), |ui| {
            egui::Frame::popup(ui.style())
                .fill(egui::Color32::from_rgba_premultiplied(25, 30, 40, 250))
                .stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(120, 150, 200)))
                .corner_radius(8.0)
                .inner_margin(egui::Margin::same(12))
                .show(ui, |ui| {
                    ui.set_max_width(320.0);
                    ui.set_min_width(280.0);
                    
                    desenhar_cabecalho_popup(ui, id_estacao, acoes);
                    
                    ui.separator();
                    
                    desenhar_conteudo_popup(ui, popup);
                    
                    ui.add_space(6.0);
                    ui.separator();
                    
                    desenhar_rodape_popup(ui);
                });
        });
}

fn desenhar_cabecalho_popup(ui: &mut egui::Ui, id_estacao: IdEstacao, acoes: &mut Vec<AcaoPopup>) {
    let _header_response = ui.horizontal(|ui| {
        ui.label(egui::RichText::new("[INFO]")
            .size(14.0)
            .color(egui::Color32::from_rgb(120, 150, 200))
            .strong());
        
        let title_response = ui.add(egui::Label::new(
            egui::RichText::new("Detalhes da Estação")
                .size(14.0)
                .color(egui::Color32::from_rgb(120, 150, 200))
                .strong()
        ).sense(egui::Sense::drag()));
        
        if title_response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
        }
        
        if title_response.dragged() {
            acoes.push(AcaoPopup { 
                id_estacao, 
                tipo: TipoAcaoPopup::MoverDelta, 
                delta: Some(title_response.drag_delta()) 
            });
        }
        
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let close_button = egui::Button::new(
                egui::RichText::new("×")
                    .size(16.0)
                    .color(egui::Color32::WHITE)
            )
            .fill(egui::Color32::from_rgb(180, 50, 50))
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(220, 80, 80)))
            .corner_radius(4.0)
            .min_size(egui::Vec2::new(24.0, 24.0));
            
            if ui.add(close_button).clicked() {
                acoes.push(AcaoPopup { 
                    id_estacao, 
                    tipo: TipoAcaoPopup::Fechar, 
                    delta: None 
                });
            }
        });
        
        title_response
    });
    
    ui.label(egui::RichText::new("Arraste o título para mover o popup")
        .size(9.0)
        .color(egui::Color32::from_rgb(150, 150, 150))
        .italics());
}

fn desenhar_conteudo_popup(ui: &mut egui::Ui, popup: &PopupInfo) {
    egui::ScrollArea::vertical()
        .max_height(300.0)
        .show(ui, |ui| {
            for linha in popup.conteudo.lines() {
                formatar_linha_popup(ui, linha);
            }
        });
}

fn formatar_linha_popup(ui: &mut egui::Ui, linha: &str) {
    if linha.trim().is_empty() {
        ui.add_space(4.0);
    } else if linha.starts_with("Estação:") || linha.starts_with("ID:") {
        ui.label(egui::RichText::new(linha)
            .size(13.0)
            .color(egui::Color32::WHITE)
            .strong());
    } else if linha.contains("Status:") {
        let cor = determinar_cor_status(linha);
        ui.label(egui::RichText::new(linha)
            .size(12.0)
            .color(cor)
            .strong());
    } else if linha.contains("CONEXÕES DISPONÍVEIS") || linha.contains("ESTAÇÕES CONECTADAS") {
        ui.add_space(6.0);
        ui.label(egui::RichText::new(linha)
            .size(12.0)
            .color(egui::Color32::from_rgb(150, 200, 255))
            .strong());
    } else if linha.starts_with("• Linha") {
        let cor = determinar_cor_linha_popup(linha);
        ui.label(egui::RichText::new(linha)
            .size(11.0)
            .color(cor));
    } else if linha.starts_with("• ") && linha.contains("min") {
        ui.label(egui::RichText::new(linha)
            .size(10.0)
            .color(egui::Color32::from_rgb(200, 200, 200)));
    } else if linha.contains("Total de conexões") || linha.contains("Use os controles") {
        ui.label(egui::RichText::new(linha)
            .size(10.0)
            .color(egui::Color32::from_rgb(180, 180, 180)));
    } else {
        ui.label(egui::RichText::new(linha)
            .size(10.0)
            .color(egui::Color32::from_rgb(220, 220, 220)));
    }
}

fn determinar_cor_status(linha: &str) -> Color32 {
    if linha.contains("INÍCIO") {
        egui::Color32::from_rgb(100, 255, 100)
    } else if linha.contains("DESTINO") {
        egui::Color32::from_rgb(255, 100, 100)
    } else if linha.contains("ROTA ENCONTRADA") {
        egui::Color32::from_rgb(100, 255, 150)
    } else if linha.contains("EXPLORADA") {
        egui::Color32::from_rgb(150, 200, 255)
    } else {
        egui::Color32::from_rgb(180, 180, 180)
    }
}

fn determinar_cor_linha_popup(linha: &str) -> Color32 {
    if linha.contains("Azul") {
        egui::Color32::from_rgb(0, 120, 255)
    } else if linha.contains("Amarela") {
        egui::Color32::from_rgb(255, 215, 0)
    } else if linha.contains("Vermelha") {
        egui::Color32::RED
    } else if linha.contains("Verde") {
        egui::Color32::from_rgb(0, 180, 0)
    } else {
        egui::Color32::GRAY
    }
}

fn desenhar_rodape_popup(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("[TIP]")
            .size(11.0)
            .color(egui::Color32::from_rgb(150, 150, 150))
            .strong());
        ui.label(egui::RichText::new("Arraste este popup para movê-lo")
            .size(9.0)
            .color(egui::Color32::from_rgb(150, 150, 150))
            .italics());
    });
}

/// Processa ações dos popups
pub fn processar_acoes_popup(app: &mut MinhaAplicacaoGUI, acoes: Vec<AcaoPopup>) {
    for acao in acoes {
        match acao.tipo {
            TipoAcaoPopup::Fechar => {
                if let Some(popup) = app.popups_info.get_mut(&acao.id_estacao) {
                    popup.visivel = false;
                }
            },
            TipoAcaoPopup::Iniciar => {
                
            },
            TipoAcaoPopup::MoverDelta => {
                if let Some(delta) = acao.delta {
                    if let Some(popup) = app.popups_info.get_mut(&acao.id_estacao) {
                        let mut pos = popup.posicao.borrow().clone();
                        pos += delta;
                        *popup.posicao.borrow_mut() = pos;
                    }
                }
            },
            TipoAcaoPopup::Soltar => {
                if let Some(popup) = app.popups_info.get_mut(&acao.id_estacao) {
                    popup.esta_sendo_arrastado = false;
                }
            }
        }
    }
}

/// Abre um popup persistente para uma estação
pub fn abrir_popup_estacao(app: &mut MinhaAplicacaoGUI, id_estacao: IdEstacao, grafo: &GrafoMetro) {
    let estacao = &grafo.estacoes[id_estacao];
    
    let mut conteudo = format!("Estação: {}\nID: E{}\n\n", estacao.nome, id_estacao + 1);
    
    conteudo.push_str(&determinar_status_estacao(app, id_estacao));
    
    adicionar_informacoes_conectividade(&mut conteudo, grafo, id_estacao);
    
    conteudo.push_str("\nUse os controles do painel lateral para\n   selecionar início e destino");
    
    let popup = PopupInfo {
        id_estacao,
        conteudo,
        posicao: RefCell::new(Vec2::new(50.0, -40.0)),
        visivel: true,
        esta_sendo_arrastado: false,
        tamanho: Vec2::new(300.0, 200.0),
    };
    
    app.popups_info.insert(id_estacao, popup);
}

fn determinar_status_estacao(app: &MinhaAplicacaoGUI, id_estacao: IdEstacao) -> String {
    if id_estacao == app.id_estacao_inicio_selecionada {
        "Status: ESTAÇÃO DE INÍCIO\n\n".to_string()
    } else if id_estacao == app.id_estacao_objetivo_selecionada {
        "Status: ESTAÇÃO DE DESTINO\n\n".to_string()
    } else if app.estacoes_exploradas_ui.contains(&id_estacao) && app.resultado_caminho_ui.is_some() {
        "Status: PARTE DA ROTA ENCONTRADA\n\n".to_string()
    } else if app.estacoes_exploradas_ui.contains(&id_estacao) {
        "Status: SENDO EXPLORADA\n\n".to_string()
    } else {
        "Status: DISPONÍVEL\n\n".to_string()
    }
}

fn adicionar_informacoes_conectividade(conteudo: &mut String, grafo: &GrafoMetro, id_estacao: IdEstacao) {
    if let Some(conexoes) = grafo.lista_adjacencia.get(id_estacao) {
        conteudo.push_str("CONEXÕES DISPONÍVEIS:\n");
        
        let mut linhas_conectadas: HashSet<CorLinha> = HashSet::new();
        for conexao in conexoes {
            linhas_conectadas.insert(conexao.cor_linha);
        }
        
        for linha in &linhas_conectadas {
            let nome_linha = match linha {
                CorLinha::Azul => "Linha Azul",
                CorLinha::Amarela => "Linha Amarela", 
                CorLinha::Vermelha => "Linha Vermelha",
                CorLinha::Verde => "Linha Verde",
                _ => "Linha Desconhecida",
            };
            conteudo.push_str(&format!("• {}\n", nome_linha));
        }
        
        conteudo.push_str(&format!("\nTotal de conexões diretas: {}\n\n", conexoes.len()));
        
        conteudo.push_str("ESTAÇÕES CONECTADAS:\n");
        let mut conexoes_mostradas = 0;
        for conexao in conexoes.iter().take(5) {
            let estacao_destino = &grafo.estacoes[conexao.para_estacao];
            conteudo.push_str(&format!("• {} ({:.1} min)\n", estacao_destino.nome, conexao.tempo_minutos));
            conexoes_mostradas += 1;
        }
        
        if conexoes.len() > 5 {
            conteudo.push_str(&format!("• ... e mais {} conexões\n", conexoes.len() - 5));
        }
    }
}

/// Mostra popup informativo quando o mouse passa sobre um vizinho sendo analisado
pub fn mostrar_popup_vizinho_hover(
    app: &MinhaAplicacaoGUI,
    ui: &mut egui::Ui,
    pos_estacao: egui::Pos2,
    id_estacao: IdEstacao,
    grafo: &GrafoMetro
) {
    if let Some(ref solucionador) = app.solucionador_a_estrela {
        if let Some(ref analise) = solucionador.ultima_analise {
            if let Some(vizinho_info) = analise.vizinhos_analisados.iter()
                .find(|v| v.starts_with(&format!("E{}", id_estacao + 1))) {
                
                mostrar_popup_analise_astar(app, ui, pos_estacao, id_estacao, grafo, vizinho_info);
            }
        }
    }
}

/// Mostra popup informativo para estações normais
pub fn mostrar_popup_estacao_hover(
    app: &MinhaAplicacaoGUI,
    ui: &mut egui::Ui,
    pos_estacao: egui::Pos2,
    id_estacao: IdEstacao,
    grafo: &GrafoMetro
) {
    let estacao = &grafo.estacoes[id_estacao];
    let pos_popup = pos_estacao + egui::Vec2::new(25.0 * app.zoom_nivel, -60.0 * app.zoom_nivel);
    let popup_id = egui::Id::new(format!("hover_popup_normal_{}", id_estacao));
    
    egui::Area::new(popup_id)
        .fixed_pos(pos_popup)
        .order(egui::Order::Foreground)
        .constrain(false)
        .show(ui.ctx(), |ui| {
            egui::Frame::popup(ui.style())
                .fill(egui::Color32::from_rgba_premultiplied(35, 35, 35, 240))
                .stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 150, 200)))
                .corner_radius(6.0)
                .inner_margin(egui::Margin::same(10))
                .show(ui, |ui| {
                    ui.set_max_width(280.0);
                    
                    desenhar_cabecalho_hover(ui);
                    desenhar_info_basica_estacao(ui, estacao, id_estacao);
                    desenhar_status_estacao_hover(app, ui, id_estacao);
                    desenhar_conectividade_hover(ui, grafo, id_estacao);
                    desenhar_dica_interacao(ui);
                });
        });
}

fn mostrar_popup_analise_astar(
    app: &MinhaAplicacaoGUI,
    ui: &mut egui::Ui,
    pos_estacao: egui::Pos2,
    id_estacao: IdEstacao,
    grafo: &GrafoMetro,
    vizinho_info: &str
) {
    let estacao = &grafo.estacoes[id_estacao];
    let (valor_f, valor_g, valor_h) = extrair_valores_fgh(vizinho_info);
    let pos_popup = pos_estacao + egui::Vec2::new(25.0 * app.zoom_nivel, -80.0 * app.zoom_nivel);
    let popup_id = egui::Id::new(format!("stable_hover_popup_{}", id_estacao));
    
    egui::Area::new(popup_id)
        .fixed_pos(pos_popup)
        .order(egui::Order::Foreground)
        .constrain(false)
        .show(ui.ctx(), |ui| {
            egui::Frame::popup(ui.style())
                .fill(egui::Color32::from_rgba_premultiplied(35, 35, 35, 245))
                .stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 165, 0)))
                .corner_radius(8.0)
                .inner_margin(egui::Margin::same(12))
                .show(ui, |ui| {
                    ui.set_max_width(320.0);
                    
                    desenhar_cabecalho_astar(ui);
                    desenhar_info_estacao_astar(ui, estacao, id_estacao);
                    desenhar_valores_astar(ui, valor_f, valor_g, valor_h);
                    desenhar_conectividade_astar(ui, grafo, id_estacao);
                    desenhar_dica_detalhes(ui);
                });
        });
}

// Funções auxiliares para os popups de hover

fn desenhar_cabecalho_hover(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("[INFO]")
            .size(14.0)
            .color(egui::Color32::from_rgb(100, 150, 200))
            .strong());
        ui.label(egui::RichText::new("Informações da Estação")
            .size(13.0)
            .color(egui::Color32::from_rgb(100, 150, 200))
            .strong());
    });
    ui.separator();
}

fn desenhar_cabecalho_astar(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("[A*]")
            .size(14.0)
            .color(egui::Color32::from_rgb(255, 165, 0))
            .strong());
        ui.label(egui::RichText::new("Análise do Vizinho")
            .size(14.0)
            .color(egui::Color32::from_rgb(255, 165, 0))
            .strong());
    });
    ui.separator();
}

fn desenhar_info_basica_estacao(ui: &mut egui::Ui, estacao: &crate::grafo_metro::Estacao, id_estacao: IdEstacao) {
    ui.label(egui::RichText::new(format!("{}", estacao.nome))
        .size(14.0)
        .color(egui::Color32::WHITE)
        .strong());
    
    ui.label(egui::RichText::new(format!("Identificador: E{}", id_estacao + 1))
        .size(11.0)
        .color(egui::Color32::from_rgb(200, 200, 200)));
    
    ui.add_space(6.0);
}

fn desenhar_info_estacao_astar(ui: &mut egui::Ui, estacao: &crate::grafo_metro::Estacao, id_estacao: IdEstacao) {
    ui.label(egui::RichText::new(format!("Estação: {}", estacao.nome))
        .size(13.0)
        .color(egui::Color32::WHITE)
        .strong());
    
    ui.label(egui::RichText::new(format!("ID: E{}", id_estacao + 1))
        .size(11.0)
        .color(egui::Color32::from_rgb(200, 200, 200)));
    
    ui.add_space(8.0);
}

fn desenhar_valores_astar(ui: &mut egui::Ui, valor_f: Option<f32>, valor_g: Option<f32>, valor_h: Option<f32>) {
    ui.label(egui::RichText::new("VALORES FUNDAMENTAIS DO A*")
        .size(14.0)
        .color(egui::Color32::from_rgb(255, 215, 0))
        .strong());
    
    egui::Frame::group(ui.style())
        .fill(egui::Color32::from_rgba_premultiplied(50, 50, 70, 200))
        .stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 215, 0)))
        .corner_radius(6.0)
        .inner_margin(egui::Margin::same(10))
        .show(ui, |ui| {
            if let Some(h) = valor_h {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("H").size(16.0).color(egui::Color32::from_rgb(255, 150, 150)).strong());
                    ui.label(egui::RichText::new("=").size(14.0).color(egui::Color32::WHITE));
                    ui.label(egui::RichText::new(format!("{:.1} min", h)).size(16.0).color(egui::Color32::WHITE).strong());
                    ui.separator();
                    ui.label(egui::RichText::new("Estimativa até Destino").size(11.0).color(egui::Color32::from_rgb(200, 200, 200)));
                });
                ui.add_space(2.0);
            }
            
            if let Some(g) = valor_g {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("G").size(16.0).color(egui::Color32::from_rgb(150, 255, 150)).strong());
                    ui.label(egui::RichText::new("=").size(14.0).color(egui::Color32::WHITE));
                    ui.label(egui::RichText::new(format!("{:.1} min", g)).size(16.0).color(egui::Color32::WHITE).strong());
                    ui.separator();
                    ui.label(egui::RichText::new("Tempo Real Percorrido").size(11.0).color(egui::Color32::from_rgb(200, 200, 200)));
                });
                ui.add_space(2.0);
            }
            
            if let Some(f) = valor_f {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("F").size(16.0).color(egui::Color32::from_rgb(255, 220, 150)).strong());
                    ui.label(egui::RichText::new("=").size(14.0).color(egui::Color32::WHITE));
                    ui.label(egui::RichText::new(format!("{:.1} min", f)).size(16.0).color(egui::Color32::WHITE).strong());
                    ui.separator();
                    ui.label(egui::RichText::new("Custo Total Estimado").size(11.0).color(egui::Color32::from_rgb(200, 200, 200)));
                });
            }
            
            // Mostrar fórmula visual
            if valor_f.is_some() && valor_g.is_some() && valor_h.is_some() {
                ui.add_space(8.0);
                ui.separator();
                ui.add_space(4.0);
                
                desenhar_formula_astar(ui, valor_f.unwrap(), valor_g.unwrap(), valor_h.unwrap());
            }
        });
    
    ui.add_space(8.0);
}

fn desenhar_formula_astar(ui: &mut egui::Ui, f_val: f32, g_val: f32, h_val: f32) {
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Fórmula:")
            .size(12.0)
            .color(egui::Color32::from_rgb(255, 215, 0))
            .strong());
        ui.label(egui::RichText::new("F").size(14.0).color(egui::Color32::from_rgb(255, 220, 150)).strong());
        ui.label(egui::RichText::new("=").size(12.0).color(egui::Color32::WHITE));
        ui.label(egui::RichText::new("G").size(14.0).color(egui::Color32::from_rgb(150, 255, 150)).strong());
        ui.label(egui::RichText::new("+").size(12.0).color(egui::Color32::WHITE));
        ui.label(egui::RichText::new("H").size(14.0).color(egui::Color32::from_rgb(255, 150, 150)).strong());
    });
    
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("Valores:").size(11.0).color(egui::Color32::from_rgb(200, 200, 200)));
        ui.label(egui::RichText::new(format!("{:.1}", f_val)).size(12.0).color(egui::Color32::from_rgb(255, 220, 150)).strong());
        ui.label(egui::RichText::new("=").size(10.0).color(egui::Color32::WHITE));
        ui.label(egui::RichText::new(format!("{:.1}", g_val)).size(12.0).color(egui::Color32::from_rgb(150, 255, 150)).strong());
        ui.label(egui::RichText::new("+").size(10.0).color(egui::Color32::WHITE));
        ui.label(egui::RichText::new(format!("{:.1}", h_val)).size(12.0).color(egui::Color32::from_rgb(255, 150, 150)).strong());
    });
}

fn desenhar_status_estacao_hover(app: &MinhaAplicacaoGUI, ui: &mut egui::Ui, id_estacao: IdEstacao) {
    if id_estacao == app.id_estacao_inicio_selecionada {
        ui.label(egui::RichText::new("🚉 ESTAÇÃO DE INÍCIO").size(13.0).color(egui::Color32::from_rgb(100, 255, 100)).strong());
    } else if id_estacao == app.id_estacao_objetivo_selecionada {
        ui.label(egui::RichText::new("🎯 ESTAÇÃO DESTINO").size(13.0).color(egui::Color32::from_rgb(255, 100, 100)).strong());
    } else if let Some(ref solucionador) = app.solucionador_a_estrela {
        match solucionador.obter_status_estacao(id_estacao) {
            crate::algoritmo_a_estrela::StatusEstacao::Disponivel => {
                ui.label(egui::RichText::new("⚪ DISPONÍVEL").size(12.0).color(egui::Color32::from_rgb(180, 180, 200)).strong());
            },
            crate::algoritmo_a_estrela::StatusEstacao::Explorada => {
                if app.resultado_caminho_ui.is_some() {
                    ui.label(egui::RichText::new("✅ PARTE DA SOLUÇÃO").size(12.0).color(egui::Color32::from_rgb(0, 255, 150)).strong());
                } else {
                    ui.label(egui::RichText::new("🔵 EXPLORADA").size(12.0).color(egui::Color32::from_rgb(100, 150, 255)).strong());
                }
            },
            _ => {}
        }
    }
    
    ui.add_space(6.0);
}

fn desenhar_conectividade_hover(ui: &mut egui::Ui, grafo: &GrafoMetro, id_estacao: IdEstacao) {
    if let Some(conexoes) = grafo.lista_adjacencia.get(id_estacao) {
        ui.label(egui::RichText::new("Linhas disponíveis:")
            .size(11.0)
            .color(egui::Color32::from_rgb(150, 200, 255))
            .strong());
        
        let mut linhas_conectadas: HashSet<CorLinha> = HashSet::new();
        for conexao in conexoes {
            linhas_conectadas.insert(conexao.cor_linha);
        }
        
        ui.horizontal_wrapped(|ui| {
            for linha in &linhas_conectadas {
                let (cor_linha, nome_linha) = match linha {
                    CorLinha::Azul => (egui::Color32::from_rgb(0, 120, 255), "Azul"),
                    CorLinha::Amarela => (egui::Color32::from_rgb(255, 215, 0), "Amarela"),
                    CorLinha::Vermelha => (egui::Color32::RED, "Vermelha"),
                    CorLinha::Verde => (egui::Color32::from_rgb(0, 180, 0), "Verde"),
                    _ => (egui::Color32::GRAY, "Outra"),
                };
                
                ui.label(egui::RichText::new(format!("● {}", nome_linha))
                    .size(10.0)
                    .color(cor_linha));
            }
        });
        
        ui.add_space(4.0);
        ui.label(egui::RichText::new(format!("Conexões diretas: {}", conexoes.len()))
            .size(10.0)
            .color(egui::Color32::from_rgb(180, 180, 180)));
    }
    
    ui.add_space(6.0);
}

fn desenhar_conectividade_astar(ui: &mut egui::Ui, grafo: &GrafoMetro, id_estacao: IdEstacao) {
    if let Some(conexoes) = grafo.lista_adjacencia.get(id_estacao) {
        ui.label(egui::RichText::new("Conexões Disponíveis:")
            .size(12.0)
            .color(egui::Color32::from_rgb(150, 255, 150))
            .strong());
        
        let mut linhas_conectadas: HashSet<CorLinha> = HashSet::new();
        for conexao in conexoes {
            linhas_conectadas.insert(conexao.cor_linha);
        }
        
        ui.horizontal_wrapped(|ui| {
            for linha in &linhas_conectadas {
                let (cor_linha, nome_linha) = match linha {
                    CorLinha::Azul => (egui::Color32::from_rgb(0, 120, 255), "Azul"),
                    CorLinha::Amarela => (egui::Color32::from_rgb(255, 215, 0), "Amarela"),
                    CorLinha::Vermelha => (egui::Color32::RED, "Vermelha"),
                    CorLinha::Verde => (egui::Color32::from_rgb(0, 180, 0), "Verde"),
                    _ => (egui::Color32::GRAY, "Outra"),
                };
                
                ui.label(egui::RichText::new(format!("● {}", nome_linha))
                    .size(10.0)
                    .color(cor_linha));
            }
        });
    }
}

fn desenhar_dica_interacao(ui: &mut egui::Ui) {
    ui.separator();
    ui.label(egui::RichText::new("Clique para ver mais detalhes")
        .size(9.0)
        .color(egui::Color32::from_rgb(150, 150, 150))
        .italics());
}

fn desenhar_dica_detalhes(ui: &mut egui::Ui) {
    ui.separator();
    ui.label(egui::RichText::new("Clique para ver informações detalhadas")
        .size(9.0)
        .color(egui::Color32::from_rgb(150, 150, 150))
        .italics());
}

// Função auxiliar para extrair valores f, g, h
fn extrair_valores_fgh(info: &str) -> (Option<f32>, Option<f32>, Option<f32>) {
    let mut valor_f = None;
    let mut valor_g = None;
    let mut valor_h = None;
    
    if let Some(pos_dois_pontos) = info.find(':') {
        let valores_parte = &info[pos_dois_pontos + 1..];
        
        for parte in valores_parte.split(',') {
            let parte = parte.trim();
            
            if let Some(pos_g) = parte.find("g=") {
                let valor_str = &parte[pos_g + 2..];
                let valor_limpo = valor_str.chars()
                    .take_while(|c| c.is_ascii_digit() || *c == '.')
                    .collect::<String>();
                if let Ok(val) = valor_limpo.parse::<f32>() {
                    valor_g = Some(val);
                }
            } else if let Some(pos_h) = parte.find("h=") {
                let valor_str = &parte[pos_h + 2..];
                let valor_limpo = valor_str.chars()
                    .take_while(|c| c.is_ascii_digit() || *c == '.')
                    .collect::<String>();
                if let Ok(val) = valor_limpo.parse::<f32>() {
                    valor_h = Some(val);
                }
            } else if let Some(pos_f) = parte.find("f=") {
                let valor_str = &parte[pos_f + 2..];
                let valor_limpo = valor_str.chars()
                    .take_while(|c| c.is_ascii_digit() || *c == '.')
                    .collect::<String>();
                if let Ok(val) = valor_limpo.parse::<f32>() {
                    valor_f = Some(val);
                }
            }
        }
    }
    
    (valor_f, valor_g, valor_h)
}
