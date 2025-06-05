use std::sync::Arc;
use egui::{Color32, ComboBox, Pos2, Rect, Stroke, Vec2}; 

use crate::grafo_metro::{CorLinha, GrafoMetro, IdEstacao, NUMERO_ESTACOES};
use crate::algoritmo_a_estrela::{InfoCaminho, ResultadoPassoAEstrela, SolucionadorAEstrela};

pub struct MinhaAplicacaoGUI {
    grafo_metro: Option<Arc<GrafoMetro>>,
    solucionador_a_estrela: Option<SolucionadorAEstrela>,
    
    id_estacao_inicio_selecionada: IdEstacao,
    id_estacao_objetivo_selecionada: IdEstacao,
    linha_inicio_opcional: Option<CorLinha>, 

    resultado_caminho_ui: Option<InfoCaminho>,
    mensagem_status_ui: String,
    posicoes_estacoes_tela: Vec<Pos2>,
}

impl MinhaAplicacaoGUI {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut grafo = GrafoMetro::novo();
        if let Err(e) = grafo.carregar_distancias_heuristicas("data/tabela1_distancias_diretas.csv") {
            eprintln!("ERRO GUI: Falha ao carregar distâncias heurísticas: {}", e);
        }
        if let Err(e) = grafo.carregar_conexoes(
            "data/tabela2_distancias_reais.csv",
            "data/tabela_linhas_conexao.csv",
        ) {
            eprintln!("ERRO GUI: Falha ao carregar conexões: {}", e);
        }

        let mut posicoes = vec![Pos2::ZERO; NUMERO_ESTACOES];
        let offset_x = 50.0;
        let offset_y = 30.0;
        let fator_escala = 1.0;

        if NUMERO_ESTACOES >= 14 {
            posicoes[0]  = Pos2::new(offset_x + 50.0 * fator_escala,  offset_y + 250.0 * fator_escala);
            posicoes[1]  = Pos2::new(offset_x + 200.0 * fator_escala, offset_y + 240.0 * fator_escala);
            posicoes[2]  = Pos2::new(offset_x + 350.0 * fator_escala, offset_y + 230.0 * fator_escala);
            posicoes[3]  = Pos2::new(offset_x + 400.0 * fator_escala, offset_y + 280.0 * fator_escala);
            posicoes[4]  = Pos2::new(offset_x + 580.0 * fator_escala, offset_y + 350.0 * fator_escala);
            posicoes[5]  = Pos2::new(offset_x + 730.0 * fator_escala, offset_y + 320.0 * fator_escala);
            posicoes[6]  = Pos2::new(offset_x + 650.0 * fator_escala, offset_y + 340.0 * fator_escala);
            posicoes[7]  = Pos2::new(offset_x + 420.0 * fator_escala, offset_y + 150.0 * fator_escala);
            posicoes[8]  = Pos2::new(offset_x + 300.0 * fator_escala, offset_y + 130.0 * fator_escala);
            posicoes[9]  = Pos2::new(offset_x + 150.0 * fator_escala, offset_y + 210.0 * fator_escala);
            posicoes[10] = Pos2::new(offset_x + 200.0 * fator_escala, offset_y + 50.0 * fator_escala);
            posicoes[11] = Pos2::new(offset_x + 400.0 * fator_escala, offset_y + 50.0 * fator_escala);
            posicoes[12] = Pos2::new(offset_x + 400.0 * fator_escala, offset_y + 480.0 * fator_escala);
            posicoes[13] = Pos2::new(offset_x + 380.0 * fator_escala, offset_y + 580.0 * fator_escala);
        }

        Self {
            grafo_metro: Some(Arc::new(grafo)),
            posicoes_estacoes_tela: posicoes,
            id_estacao_inicio_selecionada: 5, 
            id_estacao_objetivo_selecionada: 12, 
            linha_inicio_opcional: None,
            resultado_caminho_ui: None,
            mensagem_status_ui: "Selecione início/fim e inicie a busca.".to_string(),
            solucionador_a_estrela: None,
        }
    }

    fn iniciar_busca_a_estrela(&mut self) {
        if let Some(grafo) = &self.grafo_metro {
            if self.id_estacao_inicio_selecionada == self.id_estacao_objetivo_selecionada {
                self.mensagem_status_ui = "Estação de início e objetivo são a mesma.".to_string();
                self.solucionador_a_estrela = None;
                self.resultado_caminho_ui = None;
                return;
            }

            self.solucionador_a_estrela = Some(SolucionadorAEstrela::novo(
                Arc::clone(grafo),
                self.id_estacao_inicio_selecionada,
                self.linha_inicio_opcional,
                self.id_estacao_objetivo_selecionada,
            ));
            self.resultado_caminho_ui = None;
            self.mensagem_status_ui = format!(
                "Buscando de {} para {}...",
                grafo.estacoes[self.id_estacao_inicio_selecionada].nome,
                grafo.estacoes[self.id_estacao_objetivo_selecionada].nome
            );
        } else {
            self.mensagem_status_ui = "Erro: Grafo não carregado.".to_string();
        }
    }

    fn executar_proximo_passo_a_estrela(&mut self) {
        if let Some(solucionador) = &mut self.solucionador_a_estrela {
            match solucionador.proximo_passo() {
                ResultadoPassoAEstrela::EmProgresso => {
                    self.mensagem_status_ui = "A* em progresso...".to_string();
                }
                ResultadoPassoAEstrela::CaminhoEncontrado(info) => {
                    if let Some(grafo) = &self.grafo_metro {
                         self.mensagem_status_ui = format!(
                            "Caminho de {} para {} encontrado! Tempo: {:.2} min, Baldeações: {}.",
                            grafo.estacoes[self.id_estacao_inicio_selecionada].nome,
                            grafo.estacoes[self.id_estacao_objetivo_selecionada].nome,
                            info.tempo_total_minutos, info.baldeacoes
                        );
                    } else {
                         self.mensagem_status_ui = format!(
                            "Caminho encontrado! Tempo: {:.2} min, Baldeações: {}.",
                            info.tempo_total_minutos, info.baldeacoes
                        );
                    }
                    self.resultado_caminho_ui = Some(info);
                    self.solucionador_a_estrela = None;
                }
                ResultadoPassoAEstrela::NenhumCaminhoPossivel => {
                    self.mensagem_status_ui = "Nenhum caminho possível encontrado.".to_string();
                    self.solucionador_a_estrela = None;
                }
                ResultadoPassoAEstrela::Erro(msg) => {
                    self.mensagem_status_ui = format!("Erro no A*: {}", msg);
                    self.solucionador_a_estrela = None;
                }
            }
        } else {
            self.mensagem_status_ui = "Nenhuma busca A* ativa. Clique em 'Iniciar Busca'.".to_string();
        }
    }
}

impl eframe::App for MinhaAplicacaoGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("painel_controles")
            .min_width(250.0)
            .show(ctx, |ui| {
            ui.heading("Controles A* Metrô");
            ui.separator();

            if let Some(grafo) = &self.grafo_metro {
                let estacao_inicio_nome_atual = grafo.estacoes[self.id_estacao_inicio_selecionada].nome.clone();
                ComboBox::from_label("Estação de Início")
                    .selected_text(estacao_inicio_nome_atual)
                    .show_ui(ui, |ui_combo| {
                        for estacao in &grafo.estacoes {
                            ui_combo.selectable_value(&mut self.id_estacao_inicio_selecionada, estacao.id, &estacao.nome);
                        }
                    });

                let estacao_objetivo_nome_atual = grafo.estacoes[self.id_estacao_objetivo_selecionada].nome.clone();
                ComboBox::from_label("Estação Objetivo")
                    .selected_text(estacao_objetivo_nome_atual)
                    .show_ui(ui, |ui_combo| {
                        for estacao in &grafo.estacoes {
                            ui_combo.selectable_value(&mut self.id_estacao_objetivo_selecionada, estacao.id, &estacao.nome);
                        }
                    });

            } else {
                ui.label("Aguardando carregamento do grafo...");
            }
            
            ui.separator();

            if ui.button("Iniciar/Reiniciar Busca").clicked() {
                self.iniciar_busca_a_estrela();
            }

            if self.solucionador_a_estrela.is_some() {
                if ui.button("Próximo Passo").clicked() {
                    self.executar_proximo_passo_a_estrela();
                }
                if ui.button("Executar Tudo").clicked() {
                    // CORREÇÃO APLICADA AQUI: NUMERO_ESTACOES usado diretamente
                    for _i in 0..NUMERO_ESTACOES * NUMERO_ESTACOES { 
                        if self.solucionador_a_estrela.is_none() { break; } 
                        self.executar_proximo_passo_a_estrela();
                    }
                     if self.solucionador_a_estrela.is_some() { 
                        self.mensagem_status_ui = "Executar Tudo: Limite de passos atingido.".to_string();
                    }
                }
            }
            
            ui.separator();
            ui.label(&self.mensagem_status_ui);

            if let Some(info_caminho) = &self.resultado_caminho_ui {
                ui.separator();
                ui.label("--- Resultado do Caminho ---");
                ui.label(format!("Tempo Total: {:.2} min", info_caminho.tempo_total_minutos));
                ui.label(format!("Baldeações: {}", info_caminho.baldeacoes));
                ui.add_space(5.0);
                ui.label("Trajeto:");
                if let Some(grafo) = &self.grafo_metro {
                    for (idx, (id_est, linha_chegada_op)) in info_caminho.estacoes_do_caminho.iter().enumerate() {
                        let nome_est = &grafo.estacoes[*id_est].nome;
                        let info_linha = match linha_chegada_op {
                            Some(cor) => format!("{:?}", cor),
                            None => "N/A (Partida)".to_string(),
                        };
                        ui.label(format!("  {}. {}: [{}]", idx + 1, nome_est, info_linha));
                    }
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let response = ui.allocate_response(ui.available_size(), egui::Sense::hover());
                let rect_desenho = response.rect; // Rect é usado aqui
                let painter = ui.painter_at(rect_desenho);
                painter.rect_filled(rect_desenho, 0.0, Color32::from_gray(30)); 

                if let Some(grafo) = &self.grafo_metro {
                    // --- 1. Desenhar Conexões ---
                    for id_origem in 0..NUMERO_ESTACOES { // Usa NUMERO_ESTACOES importado
                        if let Some(conexoes) = grafo.lista_adjacencia.get(id_origem) {
                            for conexao in conexoes {
                                let id_destino = conexao.para_estacao;
                                if id_origem < self.posicoes_estacoes_tela.len() && id_destino < self.posicoes_estacoes_tela.len() {
                                    let pos_origem_tela = rect_desenho.min + self.posicoes_estacoes_tela[id_origem].to_vec2();
                                    let pos_destino_tela = rect_desenho.min + self.posicoes_estacoes_tela[id_destino].to_vec2();
                                    
                                    let mut cor_da_linha_conexao = match conexao.cor_linha {
                                        CorLinha::Azul => Color32::from_rgb(0, 120, 255),
                                        CorLinha::Amarela => Color32::from_rgb(255, 215, 0),
                                        CorLinha::Vermelha => Color32::RED,
                                        CorLinha::Verde => Color32::from_rgb(0, 180, 0),
                                        CorLinha::Nenhuma => Color32::DARK_GRAY,
                                    };
                                    let mut largura_linha = 2.0;

                                    if let Some(caminho_info) = &self.resultado_caminho_ui {
                                        for i in 0..caminho_info.estacoes_do_caminho.len().saturating_sub(1) {
                                            let (id_est_caminho_A, _linha_chegada_A) = caminho_info.estacoes_do_caminho[i];
                                            let (id_est_caminho_B, linha_chegada_B_op) = caminho_info.estacoes_do_caminho[i+1];

                                            if let Some(linha_chegada_B) = linha_chegada_B_op {
                                                 if (id_est_caminho_A == id_origem && id_est_caminho_B == id_destino && conexao.cor_linha == linha_chegada_B) ||
                                                    (id_est_caminho_A == id_destino && id_est_caminho_B == id_origem && conexao.cor_linha == linha_chegada_B) 
                                                 {
                                                    cor_da_linha_conexao = Color32::CYAN; 
                                                    largura_linha = 4.0;
                                                    break;
                                                 }
                                            }
                                        }
                                    }

                                    painter.line_segment(
                                        [pos_origem_tela, pos_destino_tela],
                                        Stroke::new(largura_linha, cor_da_linha_conexao),
                                    );
                                }
                            }
                        }
                    }

                    // --- 2. Desenhar Estações ---
                    let raio_estacao = 8.0;
                    for id_estacao_desenhada in 0..NUMERO_ESTACOES { // Usa NUMERO_ESTACOES importado
                        if id_estacao_desenhada < self.posicoes_estacoes_tela.len() {
                            let pos_centro_tela = rect_desenho.min + self.posicoes_estacoes_tela[id_estacao_desenhada].to_vec2();
                            let nome_estacao = &grafo.estacoes[id_estacao_desenhada].nome;

                            let mut cor_fundo_estacao = Color32::from_gray(80);
                            let mut cor_borda_estacao = Color32::from_gray(150);
                            let mut texto_custo_f = String::new();
                            let mut largura_borda = 1.5;

                            if let Some(solucionador) = &self.solucionador_a_estrela {
                                let mut esta_explorada_com_alguma_linha = false;
                                for (id_explorado, _linha_explorada) in &solucionador.explorados {
                                    if *id_explorado == id_estacao_desenhada {
                                        esta_explorada_com_alguma_linha = true;
                                        break;
                                    }
                                }
                                if esta_explorada_com_alguma_linha {
                                    cor_fundo_estacao = Color32::from_gray(50); 
                                    cor_borda_estacao = Color32::from_gray(100);
                                }

                                for item_fronteira_invertido in solucionador.fronteira.iter() {
                                    let item_fronteira = &item_fronteira_invertido.0;
                                    if item_fronteira.id_estacao == id_estacao_desenhada {
                                        cor_fundo_estacao = Color32::GOLD;
                                        cor_borda_estacao = Color32::KHAKI;
                                        texto_custo_f = format!(" f={:.0}", item_fronteira.custo_f);
                                        break;
                                    }
                                }
                            }
                            
                            if id_estacao_desenhada == self.id_estacao_inicio_selecionada {
                                cor_borda_estacao = Color32::LIGHT_GREEN;
                                largura_borda = 3.0;
                            } else if id_estacao_desenhada == self.id_estacao_objetivo_selecionada {
                                cor_borda_estacao = Color32::LIGHT_RED;
                                largura_borda = 3.0;
                            }

                            if let Some(caminho_info) = &self.resultado_caminho_ui {
                                for (id_no_caminho, _linha_chegada) in &caminho_info.estacoes_do_caminho {
                                    if *id_no_caminho == id_estacao_desenhada {
                                        cor_fundo_estacao = Color32::from_rgb(100, 180, 255); 
                                        cor_borda_estacao = Color32::WHITE;
                                        largura_borda = 3.0;
                                        break;
                                    }
                                }
                            }

                            painter.circle_filled(pos_centro_tela, raio_estacao, cor_fundo_estacao);
                            painter.circle_stroke(pos_centro_tela, raio_estacao, Stroke::new(largura_borda, cor_borda_estacao));
                            
                            painter.text(
                                pos_centro_tela + Vec2::new(0.0, -raio_estacao - 3.0),
                                egui::Align2::CENTER_BOTTOM,
                                format!("{}{}", nome_estacao, texto_custo_f),
                                egui::FontId::proportional(10.0),
                                Color32::WHITE,
                            );
                        }
                    }
                }
            });

            if self.solucionador_a_estrela.is_some() {
                 ctx.request_repaint(); 
            }
        });
    }
}