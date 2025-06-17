use egui::{Color32, ComboBox};
use crate::grafo_metro::NUMERO_ESTACOES;
use super::app::MinhaAplicacaoGUI;
use super::state_manager;

pub fn mostrar_painel_controles(app: &mut MinhaAplicacaoGUI, ctx: &egui::Context) {
    egui::SidePanel::left("painel_controles")
        .min_width(250.0)
        .show(ctx, |ui| {
            ui.heading("Controles A* Metrô");
            ui.separator();
            
            // Seletores de estação
            mostrar_seletores_estacao(app, ui);
            
            ui.separator();
            ui.label(egui::RichText::new("Controles de Busca")
                .size(14.0)
                .strong());
            ui.add_space(5.0);
            
            // Botões de controle principais
            mostrar_botoes_controle_principal(app, ui);
            
            // Botões de execução passo a passo
            if app.solucionador_a_estrela.is_some() {
                mostrar_controles_passo_a_passo(app, ui);
            }
            
            ui.separator();
            ui.label(&app.mensagem_status_ui);
            
            // Resumo da rota
            if let Some(info_caminho) = &app.resultado_caminho_ui {
                mostrar_resumo_rota(app, ui, info_caminho);
            }
            
            ui.separator();
            mostrar_opcoes_visualizacao(app, ui);
        });
}

fn mostrar_seletores_estacao(app: &mut MinhaAplicacaoGUI, ui: &mut egui::Ui) {
    if let Some(grafo) = &app.grafo_metro {
        let estacao_inicio_nome_atual = grafo.estacoes[app.id_estacao_inicio_selecionada].nome.clone();
        ComboBox::from_label("Estação de Início")
            .selected_text(estacao_inicio_nome_atual)
            .show_ui(ui, |ui_combo| {
                for estacao in &grafo.estacoes {
                    ui_combo.selectable_value(&mut app.id_estacao_inicio_selecionada, estacao.id, &estacao.nome);
                }
            });
            
        let estacao_objetivo_nome_atual = grafo.estacoes[app.id_estacao_objetivo_selecionada].nome.clone();
        ComboBox::from_label("Estação Objetivo")
            .selected_text(estacao_objetivo_nome_atual)
            .show_ui(ui, |ui_combo| {
                for estacao in &grafo.estacoes {
                    ui_combo.selectable_value(&mut app.id_estacao_objetivo_selecionada, estacao.id, &estacao.nome);
                }
            });
    } else {
        ui.label("Aguardando carregamento do grafo...");
    }
}

fn mostrar_botoes_controle_principal(app: &mut MinhaAplicacaoGUI, ui: &mut egui::Ui) {
    let tamanho_botao = egui::Vec2::new(200.0, 32.0);
    
    if ui.add_sized(tamanho_botao, egui::Button::new("Iniciar/Reiniciar Busca")).clicked() {
        state_manager::iniciar_busca_a_estrela(app);
    }
    
    ui.add_space(3.0);
    
    if ui.add_sized(tamanho_botao, egui::Button::new("Limpar Tudo")).clicked() {
        state_manager::limpar_estado_visual(app);
        app.mensagem_status_ui = "Estado limpo. Selecione início/fim e inicie nova busca.".to_string();
    }
}

fn mostrar_controles_passo_a_passo(app: &mut MinhaAplicacaoGUI, ui: &mut egui::Ui) {
    let tamanho_botao = egui::Vec2::new(200.0, 32.0);
    
    ui.add_space(8.0);
    ui.label(egui::RichText::new("Execução Passo a Passo")
        .size(13.0)
        .strong());
    ui.add_space(5.0);
    
    // Obter informações do solucionador
    let (pode_voltar, num_passos_historico) = if let Some(ref solucionador) = app.solucionador_a_estrela {
        (solucionador.pode_voltar_passo(), solucionador.numero_passos_historico())
    } else {
        (false, 0)
    };
    
    // Botões de navegação lado a lado
    ui.horizontal(|ui| {
        let largura_nav = (tamanho_botao.x - 4.0) / 2.0;
        let tamanho_nav = egui::Vec2::new(largura_nav, tamanho_botao.y);
        
        // Botão Anterior
        let texto_ant = if num_passos_historico > 0 {
            format!("◀ Anterior ({})", num_passos_historico)
        } else {
            "◀ Anterior".to_string()
        };
        
        let btn_ant = egui::Button::new(egui::RichText::new(texto_ant).size(10.5))
            .fill(if pode_voltar { Color32::from_rgb(45, 55, 75) } else { Color32::from_rgb(35, 35, 35) })
            .stroke(egui::Stroke::new(1.5, if pode_voltar { Color32::from_rgb(100, 120, 160) } else { Color32::from_rgb(60, 60, 60) }));
        
        if ui.add_sized(tamanho_nav, btn_ant).clicked() && pode_voltar {
            let ok = if let Some(ref mut sol) = app.solucionador_a_estrela {
                sol.passo_anterior()
            } else { false };
            
            if ok {
                state_manager::atualizar_estado_visual_do_solucionador(app);
                let rest = if let Some(ref sol) = app.solucionador_a_estrela { sol.numero_passos_historico() } else { 0 };
                app.mensagem_status_ui = format!("⬅ Voltou um passo. {} restantes", rest);
            } else {
                app.mensagem_status_ui = "❌ Não é possível voltar mais.".to_string();
            }
        }
        
        ui.add_space(4.0);
        
        // Botão Próximo
        let btn_prox = egui::Button::new(egui::RichText::new("Próximo ▶").size(10.5))
            .fill(Color32::from_rgb(45, 55, 75))
            .stroke(egui::Stroke::new(1.5, Color32::from_rgb(100, 120, 160)));
        
        if ui.add_sized(tamanho_nav, btn_prox).clicked() {
            state_manager::executar_proximo_passo_a_estrela(app);
        }
    });
    
    ui.add_space(5.0);
    
    if ui.add_sized(tamanho_botao, egui::Button::new("Executar Tudo")).clicked() {
        for _i in 0..NUMERO_ESTACOES * NUMERO_ESTACOES * 2 {
            if app.solucionador_a_estrela.is_none() {
                break;
            }
            state_manager::executar_proximo_passo_a_estrela(app);
        }
        if app.solucionador_a_estrela.is_some() {
            app.mensagem_status_ui = "Executar Tudo: Limite de passos atingido.".to_string();
        }
    }
}

fn mostrar_resumo_rota(app: &MinhaAplicacaoGUI, ui: &mut egui::Ui, info_caminho: &crate::algoritmo_a_estrela::InfoCaminho) {
    ui.separator();
    ui.heading("Resumo da Rota");
    
    // Frame destacado para informações principais
    egui::Frame::group(ui.style())
        .fill(egui::Color32::from_rgb(40, 42, 54))
        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(80, 80, 100)))
        .corner_radius(egui::CornerRadius::same(8))
        .inner_margin(egui::Margin::same(8))
        .show(ui, |ui| {
            // Métricas principais
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.add(egui::Label::new(
                        egui::RichText::new(format!("{:.1}", info_caminho.tempo_total_minutos))
                            .size(24.0)
                            .color(egui::Color32::from_rgb(255, 220, 150))
                    ));
                    ui.add(egui::Label::new("minutos"));
                });
                ui.add_space(15.0);
                ui.vertical(|ui| {
                    ui.add(egui::Label::new(
                        egui::RichText::new(info_caminho.baldeacoes.to_string())
                            .size(24.0)
                            .color(if info_caminho.baldeacoes > 0 {
                                egui::Color32::from_rgb(150, 200, 255)
                            } else {
                                egui::Color32::from_rgb(150, 255, 150)
                            })
                    ));
                    ui.add(egui::Label::new("baldeações"));
                });
                ui.add_space(15.0);
                ui.vertical(|ui| {
                    ui.add(egui::Label::new(
                        egui::RichText::new(info_caminho.estacoes_do_caminho.len().to_string())
                            .size(24.0)
                            .color(egui::Color32::from_rgb(200, 200, 200))
                    ));
                    ui.add(egui::Label::new("estações"));
                });
            });
        });
    
    ui.add_space(8.0);
    ui.label(egui::RichText::new("Trajeto Completo:").strong());
    
    // Tabela de trajeto
    mostrar_tabela_trajeto(app, ui, info_caminho);
}

fn mostrar_tabela_trajeto(app: &MinhaAplicacaoGUI, ui: &mut egui::Ui, info_caminho: &crate::algoritmo_a_estrela::InfoCaminho) {
    if let Some(grafo) = &app.grafo_metro {
        egui::ScrollArea::vertical()
            .max_height(200.0)
            .show(ui, |ui| {
                egui::Grid::new("grid_trajeto")
                    .num_columns(3)
                    .striped(true)
                    .spacing([8.0, 4.0])
                    .show(ui, |ui| {
                        // Cabeçalho
                        ui.add(egui::Label::new(egui::RichText::new("#").strong()));
                        ui.add(egui::Label::new(egui::RichText::new("Estação").strong()));
                        ui.add(egui::Label::new(egui::RichText::new("Linha").strong()));
                        ui.end_row();
                        
                        // Conteúdo
                        let mut linha_anterior: Option<crate::grafo_metro::CorLinha> = None;
                        for (idx, (id_est, linha_chegada_op)) in info_caminho.estacoes_do_caminho.iter().enumerate() {
                            let nome_est = &grafo.estacoes[*id_est].nome;
                            
                            let label_idx = if idx == 0 {
                                "[INÍCIO]".to_string()
                            } else if idx == info_caminho.estacoes_do_caminho.len() - 1 {
                                "[FIM]".to_string()
                            } else {
                                format!("{}", idx + 1)
                            };
                            
                            ui.label(label_idx);
                            
                            // Destacar baldeações
                            let mut nome_estacao_texto = egui::RichText::new(nome_est);
                            if linha_chegada_op.is_some() && linha_anterior.is_some() && 
                               *linha_chegada_op != linha_anterior {
                                nome_estacao_texto = nome_estacao_texto
                                    .strong()
                                    .color(egui::Color32::from_rgb(255, 220, 150));
                                ui.label(egui::RichText::new(format!("[BALDEAÇÃO] {}", nome_estacao_texto.text())));
                            } else {
                                ui.label(nome_estacao_texto);
                            }
                            
                            // Nome da linha com cor
                            let texto_linha = match linha_chegada_op {
                                Some(cor) => {
                                    let cor_linha = match cor {
                                        crate::grafo_metro::CorLinha::Azul => egui::Color32::from_rgb(0, 120, 255),
                                        crate::grafo_metro::CorLinha::Amarela => egui::Color32::from_rgb(255, 215, 0),
                                        crate::grafo_metro::CorLinha::Vermelha => egui::Color32::RED,
                                        crate::grafo_metro::CorLinha::Verde => egui::Color32::from_rgb(0, 180, 0),
                                        _ => egui::Color32::GRAY,
                                    };
                                    egui::RichText::new(format!("{:?}", cor)).color(cor_linha)
                                },
                                None => egui::RichText::new("Partida").italics(),
                            };
                            ui.label(texto_linha);
                            ui.end_row();
                            
                            linha_anterior = *linha_chegada_op;
                        }
                    });
            });
    }
}

fn mostrar_opcoes_visualizacao(app: &mut MinhaAplicacaoGUI, ui: &mut egui::Ui) {
    ui.label(egui::RichText::new("Opções de Visualização")
        .size(14.0)
        .strong());
    ui.add_space(5.0);
    
    // Controle de zoom
    ui.horizontal(|ui| {
        ui.label("Zoom:");
        ui.add_sized([140.0, 20.0], egui::Slider::new(&mut app.zoom_nivel, 0.5..=2.0)
            .show_value(true)
            .step_by(0.1));
    });
    
    ui.add_space(5.0);
    
    // Opções de visualização
    ui.label(egui::RichText::new("Exibir:")
        .size(12.0)
        .color(Color32::LIGHT_GRAY));
    
    ui.horizontal(|ui| {
        ui.checkbox(&mut app.mostrar_tempos_conexao, "⏱ Tempos");
        ui.add_space(10.0);
        ui.checkbox(&mut app.mostrar_marcadores_estacoes, "🏷 Status");
        ui.add_space(10.0);
        ui.checkbox(&mut app.mostrar_ids_estacoes, "🏷 IDs");
    });
}
