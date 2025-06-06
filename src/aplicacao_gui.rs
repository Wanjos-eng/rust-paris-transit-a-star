// src/aplicacao_gui.rs

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::cell::RefCell;
use egui::{Color32, ComboBox, Pos2, Stroke, Vec2, Id};

use crate::grafo_metro::{CorLinha, GrafoMetro, IdEstacao, NUMERO_ESTACOES};
use crate::algoritmo_a_estrela::{EstadoNoFronteira, InfoCaminho, SolucionadorAEstrela};

#[derive(Clone, Debug)]
struct PopupInfo {
    id_estacao: IdEstacao,
    conteudo: String,
    posicao: RefCell<Vec2>, // Offset relativo à posição da estação
    visivel: bool,
    esta_sendo_arrastado: bool,
    tamanho: Vec2,
}

enum TipoAcaoPopup {
    Fechar,
    Iniciar,
    MoverDelta,
    Soltar,
}

struct AcaoPopup {
    id_estacao: IdEstacao,
    tipo: TipoAcaoPopup,
    delta: Option<Vec2>,
}

pub struct MinhaAplicacaoGUI {
    grafo_metro: Option<Arc<GrafoMetro>>,
    solucionador_a_estrela: Option<SolucionadorAEstrela>,
    id_estacao_inicio_selecionada: IdEstacao,
    id_estacao_objetivo_selecionada: IdEstacao,
    linha_inicio_opcional: Option<CorLinha>,
    resultado_caminho_ui: Option<InfoCaminho>,
    mensagem_status_ui: String,
    posicoes_estacoes_tela: Vec<Pos2>,
    no_expandido_atualmente_ui: Option<EstadoNoFronteira>,
    nos_explorados_ui: HashSet<IdEstacao>, // Novo campo para rastrear estações já exploradas
    detalhes_analise_ui: Vec<String>, // Novo campo para mostrar detalhes da análise
    vizinhos_sendo_analisados: HashSet<IdEstacao>, // Novo campo para vizinhos sendo analisados
    zoom_nivel: f32,
    mostrar_custos_detalhados: bool,
    mostrar_linha_atual: bool,
    mostrar_tempos_conexao: bool, // Nova opção para controlar a visibilidade dos tempos
    offset_rolagem: Vec2,
    arrastando: bool,
    ultima_posicao_mouse: Option<Pos2>,
    popups_info: HashMap<IdEstacao, PopupInfo>,
    estacoes_com_popup_automatico: HashSet<IdEstacao>,
    offset_arrasto_popup_atual: Option<Vec2>,
    estacao_sendo_arrastada: Option<IdEstacao>,
    // Campos para execução automática
    execucao_automatica: bool,
    velocidade_execucao: f32, // segundos entre passos
    ultimo_tempo_passo: f64,
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
        let offset_x = 200.0; // Aumentado de 150.0 para 200.0
        let offset_y = 150.0; // Aumentado de 100.0 para 150.0
        let fator_escala = 1.4; // Ajustado para um valor mais adequado
        if NUMERO_ESTACOES >= 14 {
            posicoes[0] = Pos2::new(offset_x + 80.0 * fator_escala, offset_y + 250.0 * fator_escala);
            posicoes[1] = Pos2::new(offset_x + 220.0 * fator_escala, offset_y + 240.0 * fator_escala);
            posicoes[2] = Pos2::new(offset_x + 360.0 * fator_escala, offset_y + 230.0 * fator_escala);
            posicoes[3] = Pos2::new(offset_x + 400.0 * fator_escala, offset_y + 280.0 * fator_escala);
            posicoes[4] = Pos2::new(offset_x + 580.0 * fator_escala, offset_y + 350.0 * fator_escala);
            posicoes[5] = Pos2::new(offset_x + 730.0 * fator_escala, offset_y + 320.0 * fator_escala);
            posicoes[6] = Pos2::new(offset_x + 680.0 * fator_escala, offset_y + 390.0 * fator_escala);
            posicoes[7] = Pos2::new(offset_x + 420.0 * fator_escala, offset_y + 150.0 * fator_escala);
            posicoes[8] = Pos2::new(offset_x + 300.0 * fator_escala, offset_y + 130.0 * fator_escala);
            posicoes[9] = Pos2::new(offset_x + 150.0 * fator_escala, offset_y + 210.0 * fator_escala);
            posicoes[10] = Pos2::new(offset_x + 200.0 * fator_escala, offset_y + 50.0 * fator_escala);
            posicoes[11] = Pos2::new(offset_x + 400.0 * fator_escala, offset_y + 50.0 * fator_escala);
            posicoes[12] = Pos2::new(offset_x + 400.0 * fator_escala, offset_y + 480.0 * fator_escala);
            posicoes[13] = Pos2::new(offset_x + 380.0 * fator_escala, offset_y + 580.0 * fator_escala);
        }
        
        Self {
            grafo_metro: Some(Arc::new(grafo)),
            posicoes_estacoes_tela: posicoes,
            id_estacao_inicio_selecionada: 5,  // Estação E6 como padrão inicial 
            id_estacao_objetivo_selecionada: 12, // Estação E13 como objetivo padrão
            linha_inicio_opcional: None,
            resultado_caminho_ui: None,
            mensagem_status_ui: "Selecione início/fim e inicie a busca.".to_string(),
            solucionador_a_estrela: None,
            no_expandido_atualmente_ui: None,
            nos_explorados_ui: HashSet::new(), // Inicializamos vazio
            detalhes_analise_ui: Vec::new(), // Inicializamos vazio
            vizinhos_sendo_analisados: HashSet::new(), // Inicializamos vazio
            zoom_nivel: 0.70, // Zoom inicial mais afastado para melhor visualização
            mostrar_custos_detalhados: true,
            mostrar_linha_atual: true,
            mostrar_tempos_conexao: true, // Iniciar com os tempos visíveis
            offset_rolagem: Vec2::new(0.0, 0.0), // Inicializa centralizado
            arrastando: false,
            ultima_posicao_mouse: None,
            popups_info: HashMap::new(),
            estacoes_com_popup_automatico: HashSet::new(),
            offset_arrasto_popup_atual: None,
            estacao_sendo_arrastada: None,
            // Inicializar campos de execução automática
            execucao_automatica: false,
            velocidade_execucao: 1.0, // 1 segundo entre passos por padrão
            ultimo_tempo_passo: 0.0,
        }
        // Removido código duplicado do Self retornado
    }

    // Todos os métodos necessários
    fn iniciar_busca_a_estrela(&mut self) {
        if let Some(ref grafo) = self.grafo_metro {
            let grafo_arco = Arc::clone(grafo);
            let id_inicio = self.id_estacao_inicio_selecionada;
            let id_objetivo = self.id_estacao_objetivo_selecionada;
            
            // Resetar estado do algoritmo
            self.resultado_caminho_ui = None;
            self.no_expandido_atualmente_ui = None;
            self.nos_explorados_ui.clear(); // Limpar nós explorados anteriores
            self.detalhes_analise_ui.clear(); // Limpar detalhes da análise anterior
            
            // Criar o solucionador com os parâmetros atuais da interface
            let solucionador = SolucionadorAEstrela::novo(
                grafo_arco,
                id_inicio,
                self.linha_inicio_opcional,
                id_objetivo
            );
            
            self.solucionador_a_estrela = Some(solucionador);
            self.mensagem_status_ui = format!(
                "Busca iniciada: De {} para {}", 
                grafo.estacoes[id_inicio].nome, 
                grafo.estacoes[id_objetivo].nome
            );
        } else {
            self.mensagem_status_ui = "Erro: Grafo não carregado.".to_string();
        }
    }

    fn executar_proximo_passo_a_estrela(&mut self) {
        if let Some(ref mut solucionador) = self.solucionador_a_estrela {
            match solucionador.proximo_passo() {
                crate::algoritmo_a_estrela::ResultadoPassoAEstrela::EmProgresso => {
                    // Se tem nós na fronteira, mostramos o primeiro (de menor custo)
                    if let Some(no_fronteira) = solucionador.fronteira.peek() {
                        self.no_expandido_atualmente_ui = Some(no_fronteira.clone());
                        
                        // Adiciona à nossa lista de nós explorados (para visualização)
                        // Também adicionamos os nós do solucionador
                        for id_estacao in &solucionador.explorados {
                            self.nos_explorados_ui.insert(*id_estacao);
                        }
                        
                        // Capturar os vizinhos sendo analisados se há detalhes da análise
                        self.vizinhos_sendo_analisados.clear();
                        if let Some(ref analise) = solucionador.ultima_analise {
                            // Extrair IDs das estações dos vizinhos analisados
                            for vizinho_info in &analise.vizinhos_analisados {
                                // O formato é "E{id}: ..." então vamos extrair o ID
                                if let Some(inicio_numero) = vizinho_info.find('E') {
                                    if let Some(fim_numero) = vizinho_info.find(':') {
                                        if fim_numero > inicio_numero + 1 {
                                            let numero_str = &vizinho_info[inicio_numero + 1..fim_numero];
                                            if let Ok(id_estacao_um_baseado) = numero_str.parse::<usize>() {
                                                let id_estacao = id_estacao_um_baseado - 1; // Converter para zero-based
                                                self.vizinhos_sendo_analisados.insert(id_estacao);
                                            }
                                        }
                                    }
                                }
                            }
                            
                            // Atualizar detalhes da análise para exibição
                            self.detalhes_analise_ui = analise.vizinhos_analisados.clone();
                        }
                        
                        self.mensagem_status_ui = format!(
                            "Expandindo estação: {} (f={:.1}, g={:.1}, h={:.1})",
                            no_fronteira.id_estacao + 1, // +1 para exibir baseado em 1 para o usuário
                            no_fronteira.custo_f,
                            no_fronteira.custo_g_viagem,
                            no_fronteira.custo_f - no_fronteira.custo_g_viagem
                        );
                    } else {
                        self.mensagem_status_ui = "Fronteira vazia, não há solução.".to_string();
                        self.solucionador_a_estrela = None;
                    }
                },
                crate::algoritmo_a_estrela::ResultadoPassoAEstrela::CaminhoEncontrado(caminho_info) => {
                    self.resultado_caminho_ui = Some(caminho_info.clone());
                    self.mensagem_status_ui = format!(
                        "Caminho encontrado! Tempo: {:.1} min, Baldeações: {}",
                        caminho_info.tempo_total_minutos,
                        caminho_info.baldeacoes
                    );
                    self.solucionador_a_estrela = None;
                },
                crate::algoritmo_a_estrela::ResultadoPassoAEstrela::NenhumCaminhoPossivel => {
                    self.mensagem_status_ui = "Não foi possível encontrar um caminho.".to_string();
                    self.solucionador_a_estrela = None;
                },
                crate::algoritmo_a_estrela::ResultadoPassoAEstrela::Erro(msg) => {
                    self.mensagem_status_ui = format!("Erro: {}", msg);
                    self.solucionador_a_estrela = None;
                }
            }
        } else {
            self.mensagem_status_ui = "Erro: Nenhuma busca em andamento.".to_string();
        }
    }

    fn processar_clique_estacao(&mut self, id_estacao: IdEstacao, _grafo: &GrafoMetro) {
        // Alternar entre definir como início ou fim
        if self.id_estacao_inicio_selecionada == id_estacao {
            // Se já é o início, muda para ser o objetivo
            self.id_estacao_objetivo_selecionada = id_estacao;
        } else {
            // Caso contrário, define como início
            self.id_estacao_inicio_selecionada = id_estacao;
        }
        
        // Reinicia a busca se estiver em andamento
        if self.solucionador_a_estrela.is_some() {
            self.iniciar_busca_a_estrela();
        }
    }

    fn desenhar_conexoes(&self, painter: &egui::Painter, rect_desenho: egui::Rect, grafo: &GrafoMetro) {
        // Primeiro desenhar as conexões normais (não na solução)
        for (id_origem, conexoes) in grafo.lista_adjacencia.iter().enumerate() {
            for conexao in conexoes {
                let id_destino = conexao.para_estacao;
                
                // Verificar se esta conexão faz parte da solução - só desenhar linhas que NÃO estão na solução
                let na_solucao = self.esta_na_solucao(id_origem, id_destino, conexao.cor_linha);
                if na_solucao {
                    continue; // Pular as conexões que fazem parte do caminho (serão desenhadas depois)
                }
                
                // Obtém posições das estações na tela
                let pos_origem = self.posicoes_estacoes_tela[id_origem] * self.zoom_nivel + self.offset_rolagem + rect_desenho.min.to_vec2();
                let pos_destino = self.posicoes_estacoes_tela[id_destino] * self.zoom_nivel + self.offset_rolagem + rect_desenho.min.to_vec2();
                
                // Determinar cor e espessura da linha baseada na cor da linha (mais suave para linhas que não estão na solução)
                // Aumentando a espessura de todas as linhas
                let (cor_linha, espessura) = match conexao.cor_linha {
                    CorLinha::Azul => (Color32::from_rgb(0, 120, 255), 3.5),
                    CorLinha::Amarela => (Color32::from_rgb(255, 215, 0), 3.5),
                    CorLinha::Vermelha => (Color32::RED, 3.5),
                    CorLinha::Verde => (Color32::from_rgb(0, 180, 0), 3.5),
                    _ => (Color32::GRAY, 3.0)
                };
                
                // Desenhar linha da conexão com opacidade reduzida
                painter.line_segment(
                    [pos_origem, pos_destino], 
                    Stroke::new(espessura * self.zoom_nivel, cor_linha.gamma_multiply(0.3))
                );
                
                // Se ativado, desenhar tempo da conexão (apenas para conexões não na solução)
                if self.mostrar_tempos_conexao {
                    let meio = (pos_origem + pos_destino.to_vec2()) / 2.0;
                    let texto_tempo = format!("{:.1}", conexao.tempo_minutos);
                    let tamanho_texto = egui::FontId::proportional(11.0 * self.zoom_nivel);
                    
                    self.desenhar_balao_tempo(
                        painter,
                        meio,
                        texto_tempo,
                        tamanho_texto,
                        false
                    );
                }
            }
        }
        
        // Desenhar o caminho da solução com linha destacada (verde-azul escuro)
        if let Some(ref caminho_info) = self.resultado_caminho_ui {
            // Cor verde-azul escuro vibrante para o caminho da solução
            let cor_solucao = Color32::from_rgb(0, 150, 136); // Verde-azul escuro (teal)
            let espessura_solucao = 6.5 * self.zoom_nivel; // Linha ainda mais grossa
            
            // Desenhar cada segmento do caminho
            for i in 0..caminho_info.estacoes_do_caminho.len().saturating_sub(1) {
                let (id_origem, _) = caminho_info.estacoes_do_caminho[i];
                let (id_destino, _) = caminho_info.estacoes_do_caminho[i+1];
                
                let pos_origem = self.posicoes_estacoes_tela[id_origem] * self.zoom_nivel + self.offset_rolagem + rect_desenho.min.to_vec2();
                let pos_destino = self.posicoes_estacoes_tela[id_destino] * self.zoom_nivel + self.offset_rolagem + rect_desenho.min.to_vec2();
                
                // Linha principal com brilho
                painter.line_segment(
                    [pos_origem, pos_destino], 
                    Stroke::new(espessura_solucao, cor_solucao)
                );
                
                // Efeito de brilho externo na linha (opcional)
                painter.line_segment(
                    [pos_origem, pos_destino], 
                    Stroke::new(espessura_solucao + 2.0, Color32::from_rgba_premultiplied(0, 100, 80, 40))
                );
                
                // Se ativado, desenhar tempo da conexão em destaque
                if self.mostrar_tempos_conexao {
                    // Buscar o tempo dessa conexão
                    let tempo: f32 = if let Some(conexoes) = grafo.lista_adjacencia.get(id_origem) {
                        conexoes.iter()
                               .find(|con| con.para_estacao == id_destino)
                               .map_or(0.0, |con| con.tempo_minutos)
                    } else {
                        0.0
                    };
                    
                    let meio = (pos_origem + pos_destino.to_vec2()) / 2.0;
                    let texto_tempo = format!("{:.1}", tempo);
                    let tamanho_texto = egui::FontId::proportional(12.0 * self.zoom_nivel);
                    
                    // Desenhar tempo com destaque
                    self.desenhar_balao_tempo(
                        painter,
                        meio,
                        texto_tempo,
                        tamanho_texto,
                        true
                    );
                }
                
                // Desenhar ícones de baldeação onde necessário
                if i < caminho_info.estacoes_do_caminho.len().saturating_sub(2) {
                    let (_, linha_atual) = caminho_info.estacoes_do_caminho[i+1];
                    let (_, proxima_linha) = caminho_info.estacoes_do_caminho[i+2];
                    
                    if linha_atual.is_some() && proxima_linha.is_some() && linha_atual != proxima_linha {
                        // Esta é uma baldeação - desenhar na estação intermediária
                        let pos_baldeacao = self.posicoes_estacoes_tela[id_destino] * self.zoom_nivel + 
                                            self.offset_rolagem + rect_desenho.min.to_vec2() + 
                                            Vec2::new(0.0, -25.0 * self.zoom_nivel); // Posicionar ACIMA da estação
                        
                        self.desenhar_icone_baldeacao(
                            painter,
                            pos_baldeacao,
                            10.0 * self.zoom_nivel,
                            Some((linha_atual.unwrap(), proxima_linha.unwrap()))
                        );
                    }
                }
            }
        }
    }

    fn esta_na_solucao(&self, id_origem: IdEstacao, id_destino: IdEstacao, cor_linha: CorLinha) -> bool {
        // Verifica se este trecho (origem -> destino) está na solução encontrada
        if let Some(ref info_caminho) = self.resultado_caminho_ui {
            for i in 0..info_caminho.estacoes_do_caminho.len().saturating_sub(1) {
                let (id1, _linha1) = info_caminho.estacoes_do_caminho[i];
                let (id2, linha2) = info_caminho.estacoes_do_caminho[i+1];
                
                if id1 == id_origem && id2 == id_destino {
                    return linha2.map_or(false, |l| l == cor_linha);
                }
            }
        }
        false // Não está na solução
    }

    fn desenhar_tempos_conexao(&self, painter: &egui::Painter, rect_desenho: egui::Rect, grafo: &GrafoMetro) {
        if !self.mostrar_tempos_conexao {
            return;
        }
        
        // Itera sobre todas as conexões para desenhar tempos
        for (id_origem, conexoes) in grafo.lista_adjacencia.iter().enumerate() {
            for conexao in conexoes {
                let id_destino = conexao.para_estacao;
                
                // Obtém posições na tela
                let pos_origem = self.posicoes_estacoes_tela[id_origem] * self.zoom_nivel + self.offset_rolagem + rect_desenho.min.to_vec2();
                let pos_destino = self.posicoes_estacoes_tela[id_destino] * self.zoom_nivel + self.offset_rolagem + rect_desenho.min.to_vec2();
                
                // Calcula o ponto médio para mostrar o tempo
                let pos_texto = (pos_origem + pos_destino.to_vec2()) / 2.0;
                
                // Verifica se esta conexão está na rota encontrada
                let destacar = self.esta_na_solucao(id_origem, id_destino, conexao.cor_linha);
                
                // Texto do tempo da conexão
                let texto_tempo = format!("{:.1}", conexao.tempo_minutos);
                
                self.desenhar_balao_tempo(
                    painter,
                    pos_texto,
                    texto_tempo,
                    egui::FontId::proportional(11.0 * self.zoom_nivel),
                    destacar
                );
            }
        }
    }

    fn desenhar_balao_tempo(&self, painter: &egui::Painter, posicao: Pos2, texto: String, tamanho_fonte: egui::FontId, destacado: bool) {
        let texto_galley = painter.layout_no_wrap(
            texto.clone(),
            tamanho_fonte.clone(),
            Color32::WHITE,
        );

        let padding = 5.0 * self.zoom_nivel;
        let raio_arredondamento = 6.0 * self.zoom_nivel;
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

        let bg_rect = egui::Rect::from_center_size(
            posicao,
            tamanho_balao
        );

        if destacado {
            painter.rect_filled(
                bg_rect.translate(Vec2::new(2.0, 2.0) * self.zoom_nivel),
                raio_arredondamento,
                Color32::from_rgba_premultiplied(0, 0, 0, 100),
            );
        }

        painter.rect_filled(
            bg_rect,
            raio_arredondamento,
            cor_fundo,
        );

        painter.rect_stroke(
            bg_rect,
            raio_arredondamento,
            Stroke::new(1.5 * self.zoom_nivel, cor_borda),
            egui::StrokeKind::Middle,
        );

        painter.text(
            posicao,
            egui::Align2::CENTER_CENTER,
            texto,
            tamanho_fonte,
            cor_texto,
        );
    }

    fn desenhar_icone_baldeacao(&self, painter: &egui::Painter, posicao: Pos2, tamanho: f32, linhas: Option<(CorLinha, CorLinha)>) {
        if let Some((de_linha, para_linha)) = linhas {
            // Cores para as diferentes linhas de metrô
            let cor1 = match de_linha {
                CorLinha::Azul => Color32::from_rgb(0, 120, 255),
                CorLinha::Amarela => Color32::from_rgb(255, 215, 0),
                CorLinha::Vermelha => Color32::RED,
                CorLinha::Verde => Color32::from_rgb(0, 180, 0),
                _ => Color32::GRAY,
            };

            let cor2 = match para_linha {
                CorLinha::Azul => Color32::from_rgb(0, 120, 255),
                CorLinha::Amarela => Color32::from_rgb(255, 215, 0),
                CorLinha::Vermelha => Color32::RED,
                CorLinha::Verde => Color32::from_rgb(0, 180, 0),
                _ => Color32::GRAY,
            };

            // Sombra do círculo de transferência
            painter.circle_filled(
                posicao + Vec2::new(1.0, 1.0) * self.zoom_nivel,
                tamanho + 4.0,
                Color32::from_rgba_premultiplied(0, 0, 0, 180),
            );

            // Círculo de fundo da transferência
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
            
            // Estilo mais atraente: dois semi-círculos de cores diferentes
            let raio = tamanho * 1.5;
            
            // Desenhar duas metades de cores diferentes
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
            
            // Símbolo de transferência no centro
            let seta_tam = tamanho * 0.8;
            
            // Desenha setas de transferência (círculo com setas)
            painter.circle_stroke(
                posicao, 
                seta_tam,
                Stroke::new(1.5 * self.zoom_nivel, Color32::WHITE)
            );
            
            // Setas circulares de transferência
            let angulo_inicio = std::f32::consts::PI * 0.8; // Ângulo de início da primeira seta
            let comp_seta = std::f32::consts::PI * 0.6; // Comprimento angular da seta
            
            // Primeira seta
            let ponto1 = posicao + Vec2::new(seta_tam * f32::cos(angulo_inicio), seta_tam * f32::sin(angulo_inicio));
            let ponto2 = posicao + Vec2::new(seta_tam * f32::cos(angulo_inicio + comp_seta), seta_tam * f32::sin(angulo_inicio + comp_seta));
            
            painter.line_segment(
                [ponto1, ponto2],
                Stroke::new(2.0 * self.zoom_nivel, Color32::WHITE),
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
                Stroke::new(2.0 * self.zoom_nivel, Color32::WHITE),
            );
            
            painter.line_segment(
                [
                    ponta_seta,
                    ponta_seta + Vec2::new(tam_ponta * f32::cos(angulo_ponta_2), tam_ponta * f32::sin(angulo_ponta_2)),
                ],
                Stroke::new(2.0 * self.zoom_nivel, Color32::WHITE),
            );
            
            // Segunda seta (espelhada)
            let angulo_inicio2 = angulo_inicio + std::f32::consts::PI;
            
            let ponto1_seta2 = posicao + Vec2::new(seta_tam * f32::cos(angulo_inicio2), seta_tam * f32::sin(angulo_inicio2));
            let ponto2_seta2 = posicao + Vec2::new(seta_tam * f32::cos(angulo_inicio2 + comp_seta), seta_tam * f32::sin(angulo_inicio2 + comp_seta));
            
            painter.line_segment(
                [ponto1_seta2, ponto2_seta2],
                Stroke::new(2.0 * self.zoom_nivel, Color32::WHITE),
            );
            
            // Ponta da segunda seta
            let angulo_ponta2 = angulo_inicio2 + comp_seta;
            let ponta_seta2 = posicao + Vec2::new(seta_tam * f32::cos(angulo_ponta2), seta_tam * f32::sin(angulo_ponta2));
            let angulo_ponta2_1 = angulo_ponta2 + std::f32::consts::PI * 0.8;
            let angulo_ponta2_2 = angulo_ponta2 + std::f32::consts::PI * 1.2;
            
            painter.line_segment(
                [
                    ponta_seta2,
                    ponta_seta2 + Vec2::new(tam_ponta * f32::cos(angulo_ponta2_1), tam_ponta * f32::sin(angulo_ponta2_1)),
                ],
                Stroke::new(2.0 * self.zoom_nivel, Color32::WHITE),
            );
            
            painter.line_segment(
                [
                    ponta_seta2,
                    ponta_seta2 + Vec2::new(tam_ponta * f32::cos(angulo_ponta2_2), tam_ponta * f32::sin(angulo_ponta2_2)),
                ],
                Stroke::new(2.0 * self.zoom_nivel, Color32::WHITE),
            );
            
            // Balão de informação do tempo de baldeação
            let texto_tempo = "+4.0min"; // Tempo fixo de baldeação
            
            // Cria um balão informativo com o tempo de baldeação
            let texto_galley = painter.layout_no_wrap(
                texto_tempo.to_string(),
                egui::FontId::proportional(11.0 * self.zoom_nivel),
                Color32::WHITE,
            );
            
            let padding = 4.0 * self.zoom_nivel;
            let tamanho_balao = texto_galley.size() + Vec2::new(padding * 2.0, padding * 2.0);
            // Posicionar ACIMA do ícone de baldeação (não abaixo como estava antes)
            let pos_balao = posicao + Vec2::new(0.0, -tamanho - 8.0 * self.zoom_nivel);
            
            let bg_rect = egui::Rect::from_center_size(
                pos_balao,
                tamanho_balao
            );
            
            // Fundo do balão com sombra
            painter.rect_filled(
                bg_rect.translate(Vec2::new(1.0, 1.0) * self.zoom_nivel),
                4.0 * self.zoom_nivel,
                Color32::from_rgba_premultiplied(0, 0, 0, 100),
            );
            
            // Fundo do balão
            painter.rect_filled(
                bg_rect,
                4.0 * self.zoom_nivel,
                Color32::from_rgba_premultiplied(60, 0, 60, 220),
            );
            
            // Borda do balão
            painter.rect_stroke(
                bg_rect,
                4.0 * self.zoom_nivel,
                Stroke::new(1.0 * self.zoom_nivel, Color32::from_rgba_premultiplied(255, 200, 255, 180)),
                egui::StrokeKind::Middle,
            );
            
            // Texto do tempo
            painter.text(
                pos_balao,
                egui::Align2::CENTER_CENTER,
                texto_tempo,
                egui::FontId::proportional(10.0 * self.zoom_nivel),
                Color32::WHITE,
            );
        }
    }

    // --- Métodos de visualização e interação restaurados ---
    fn desenhar_estacoes(&mut self, painter: &egui::Painter, rect_desenho: egui::Rect, grafo: &GrafoMetro, ui: &mut egui::Ui) {
        // Desenha os círculos das estações, nomes, destaques de início/fim, e permite interação
        for (i, estacao) in grafo.estacoes.iter().enumerate() {
            let pos = self.posicoes_estacoes_tela[i] * self.zoom_nivel + self.offset_rolagem + rect_desenho.min.to_vec2();
            
            // Determinar se a estação está no caminho da solução para destacá-la
            let esta_na_solucao = if let Some(ref info_caminho) = self.resultado_caminho_ui {
                info_caminho.estacoes_do_caminho.iter().any(|(id, _)| *id == i)
            } else {
                false
            };
            
            // Determinar se é a estação atualmente sendo expandida
            let esta_sendo_expandida = if let Some(ref no_atual) = self.no_expandido_atualmente_ui {
                no_atual.id_estacao == i
            } else {
                false
            };
            
            // Determinar se esta estação é um vizinho sendo analisado
            let e_vizinho_sendo_analisado = self.vizinhos_sendo_analisados.contains(&i);
            
            // Desenhar efeito de animação para estações em análise
            if esta_sendo_expandida {
                // Efeito de onda pulsante (3 círculos com opacidade decrescente)
                let tempo = ui.input(|i| i.time);
                let pulso = (tempo.sin() as f32 * 0.5 + 0.5) * 0.8 + 0.2; // Valor entre 0.2 e 1.0
                
                // Círculo de brilho externo pulsante
                painter.circle_stroke(
                    pos,
                    24.0 * self.zoom_nivel * pulso,
                    Stroke::new(2.0 * self.zoom_nivel, Color32::from_rgba_premultiplied(255, 215, 0, (180.0 * (1.0 - pulso * 0.8)) as u8))
                );
                
                // Círculo de brilho médio pulsante
                painter.circle_stroke(
                    pos,
                    19.0 * self.zoom_nivel * pulso,
                    Stroke::new(1.5 * self.zoom_nivel, Color32::from_rgba_premultiplied(255, 215, 0, (220.0 * (1.0 - pulso * 0.5)) as u8))
                );
            }
            
            // Desenhar efeito visual para vizinhos sendo analisados
            if e_vizinho_sendo_analisado {
                let tempo = ui.input(|i| i.time);
                let pulso_vizinho = (tempo * 3.0).sin() as f32 * 0.3 + 0.7; // Pulsação mais sutil
                
                // Círculo de destaque para vizinhos sendo analisados (cor laranja)
                painter.circle_stroke(
                    pos,
                    22.0 * self.zoom_nivel * pulso_vizinho,
                    Stroke::new(2.5 * self.zoom_nivel, Color32::from_rgba_premultiplied(255, 165, 0, 200)) // Laranja
                );
                
                // Círculo interno menor
                painter.circle_stroke(
                    pos,
                    16.0 * self.zoom_nivel,
                    Stroke::new(1.5 * self.zoom_nivel, Color32::from_rgba_premultiplied(255, 140, 0, 150))
                );
            }
            
            // Cor de preenchimento única para todas as estações - apenas bordas coloridas
            let cor_preenchimento = Color32::from_rgb(40, 42, 54); // Cor base escura para todas as estações
            
            // Desenhar círculo de fundo para dar profundidade (sombra)
            painter.circle_filled(
                pos + Vec2::new(1.5, 1.5) * self.zoom_nivel,
                18.0 * self.zoom_nivel,
                Color32::from_rgba_premultiplied(0, 0, 0, 160) // Sombra
            );
            
            // Círculo principal (mesmo para todas as estações)
            painter.circle_filled(
                pos,
                18.0 * self.zoom_nivel,
                cor_preenchimento
            );
            
            // Borda com cor apropriada e espessura maior
            let (cor_borda, espessura_borda) = if i == self.id_estacao_inicio_selecionada {
                (Color32::from_rgb(50, 220, 50), 3.0) // Verde para início
            } else if i == self.id_estacao_objetivo_selecionada {
                (Color32::from_rgb(220, 50, 50), 3.0) // Vermelho para objetivo
            } else if esta_na_solucao {
                (Color32::from_rgb(0, 150, 136), 3.0) // Verde-azul escuro para estações na solução
            } else if esta_sendo_expandida {
                (Color32::from_rgb(255, 215, 0), 3.0) // Amarelo-ouro para estação sendo analisada
            } else if e_vizinho_sendo_analisado {
                (Color32::from_rgb(255, 140, 0), 2.5) // Laranja para vizinhos sendo analisados
            } else {
                (Color32::from_rgb(150, 150, 200), 2.0) // Azul claro para as demais
            };
            
            painter.circle_stroke(
                pos, 
                18.0 * self.zoom_nivel, 
                Stroke::new(espessura_borda * self.zoom_nivel, cor_borda)
            );
            
            // Adicionar o identificador da estação dentro do círculo (E1, E2, etc.)
            painter.text(
                pos,
                egui::Align2::CENTER_CENTER,
                &format!("E{}", i + 1), // Usando E1, E2, etc. para identificação mais clara
                egui::FontId::proportional(12.5 * self.zoom_nivel),
                Color32::WHITE,
            );
            
            // Adicionar o nome da estação abaixo do círculo - apenas para estações relevantes
            // Só mostra nomes das estações que fazem parte da solução ou que são início/fim
            if esta_na_solucao || i == self.id_estacao_inicio_selecionada || i == self.id_estacao_objetivo_selecionada {
                painter.text(
                    pos + Vec2::new(0.0, 20.0 * self.zoom_nivel),
                    egui::Align2::CENTER_TOP,
                    &estacao.nome,
                    egui::FontId::proportional(12.0 * self.zoom_nivel),
                    Color32::WHITE
                );
            }
            
            // Mostrar informações de debug se a estação estiver sendo expandida e a opção estiver ativada
            if esta_sendo_expandida && self.mostrar_custos_detalhados {
                if let Some(ref no_atual) = self.no_expandido_atualmente_ui {
                    // Fundo para o texto de debug
                    let texto_debug = format!(
                        "f={:.1}, g={:.1}, h={:.1}",
                        no_atual.custo_f,
                        no_atual.custo_g_viagem,
                        no_atual.custo_f - no_atual.custo_g_viagem
                    );
                    
                    let texto_galley = painter.layout_no_wrap(
                        texto_debug.clone(),
                        egui::FontId::proportional(11.0 * self.zoom_nivel),
                        Color32::WHITE,
                    );
                    
                    let padding = 4.0 * self.zoom_nivel;
                    let tamanho_balao = texto_galley.size() + Vec2::new(padding * 2.0, padding * 2.0);
                    
                    let pos_texto = pos + Vec2::new(0.0, -28.0 * self.zoom_nivel);
                    let bg_rect = egui::Rect::from_center_size(
                        pos_texto,
                        tamanho_balao
                    );
                    
                    // Fundo do balão de informação
                    painter.rect_filled(
                        bg_rect,
                        4.0 * self.zoom_nivel,
                        Color32::from_rgba_premultiplied(40, 40, 0, 180),
                    );
                    
                    // Texto de debug
                    painter.text(
                        pos_texto,
                        egui::Align2::CENTER_CENTER,
                        texto_debug,
                        egui::FontId::proportional(11.0 * self.zoom_nivel),
                        Color32::YELLOW,
                    );
                }
            }
            
            // Interação: clique para abrir popup com informações detalhadas
            let area_interacao = egui::Rect::from_center_size(pos, Vec2::splat(32.0 * self.zoom_nivel));
            let response = ui.interact(
                area_interacao,
                egui::Id::new(format!("estacao_{}", i)),
                egui::Sense::click_and_drag(),
            );
            
            // Se o mouse está sobre a estação, mostrar um realce
            if response.hovered() {
                painter.circle_stroke(
                    pos, 
                    17.0 * self.zoom_nivel, 
                    Stroke::new(1.0 * self.zoom_nivel, Color32::WHITE)
                );
                
                // Mostrar pop-up informativo para vizinhos sendo analisados
                if e_vizinho_sendo_analisado {
                    self.mostrar_popup_vizinho_hover(ui, pos, i, grafo);
                }
            }
            
            // Processar clique na estação
            if response.clicked() {
                // Processar o clique na estação - Selecionar como início/fim
                self.processar_clique_estacao(i, grafo);
                
                // Abrir popup com informações da estação
                self.abrir_popup_estacao(i, grafo);
            }
        }
    }

    fn desenhar_popups(&mut self, ui: &mut egui::Ui, rect_desenho: egui::Rect, _grafo: &GrafoMetro) -> Vec<AcaoPopup> {
        // Exemplo de popup simples para cada estação visível
        let mut acoes = Vec::new();
        for (id, popup) in self.popups_info.iter_mut() {
            if popup.visivel {
                let pos = self.posicoes_estacoes_tela[*id] * self.zoom_nivel + self.offset_rolagem + rect_desenho.min.to_vec2();
                let _area = egui::Area::new(Id::new(format!("popup_{}", id)))
                    .fixed_pos(pos + Vec2::new(30.0, -30.0))
                    .show(ui.ctx(), |ui| {
                        ui.group(|ui| {
                            ui.label(&popup.conteudo);
                            if ui.button("Fechar").clicked() {
                                acoes.push(AcaoPopup { id_estacao: *id, tipo: TipoAcaoPopup::Fechar, delta: None });
                            }
                        });
                    });
            }
        }
        acoes
    }

    fn centralizar_visualizacao(&mut self, tamanho_disponivel: Vec2) {
        // Centraliza o grafo na tela considerando o zoom
        let centro = tamanho_disponivel / 2.0;
        let centro_grafo = self.posicoes_estacoes_tela.iter().fold(Vec2::ZERO, |acc, p| acc + p.to_vec2()) / (self.posicoes_estacoes_tela.len() as f32);
        self.offset_rolagem = centro - centro_grafo * self.zoom_nivel;
    }

    fn processar_eventos_navegacao(&mut self, ui: &mut egui::Ui, response: &egui::Response, rect_desenho: egui::Rect) {
        // Permite arrastar o mapa com o mouse
        if response.dragged() {
            if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
                if let Some(last) = self.ultima_posicao_mouse {
                    let delta = pos - last;
                    self.offset_rolagem += delta;
                }
                self.ultima_posicao_mouse = Some(pos);
            }
        } else {
            self.ultima_posicao_mouse = None;
        }
        
        // Suporte para zoom com a roda do mouse
        let scroll_delta = ui.input(|i| i.raw_scroll_delta.y);
        if scroll_delta != 0.0 {
            // Ajusta o zoom baseado na direção do scroll
            let fator_zoom = if scroll_delta > 0.0 { 1.1 } else { 0.9 };
            let novo_zoom = (self.zoom_nivel * fator_zoom).clamp(0.3, 2.5);
            
            // Calcula zoom relativo à posição do mouse para um efeito natural
            if let Some(pos_mouse) = ui.input(|i| i.pointer.interact_pos()) {
                let pos_antes = (pos_mouse - rect_desenho.min.to_vec2() - self.offset_rolagem) / self.zoom_nivel;
                self.zoom_nivel = novo_zoom;
                let pos_depois = (pos_mouse - rect_desenho.min.to_vec2() - self.offset_rolagem) / self.zoom_nivel;
                
                // Ajusta o deslocamento para manter o ponto sob o cursor
                self.offset_rolagem += (pos_depois - pos_antes) * self.zoom_nivel;
            } else {
                self.zoom_nivel = novo_zoom;
            }
        }
    }

    fn processar_acoes_popup(&mut self, acoes: Vec<AcaoPopup>) {
        // Processa ações dos popups (fechar, mover, etc)
        for acao in acoes {
            match acao.tipo {
                TipoAcaoPopup::Fechar => {
                    if let Some(popup) = self.popups_info.get_mut(&acao.id_estacao) {
                        popup.visivel = false;
                    }
                }
                _ => {}
            }
        }
    }

    fn verifica_baldeacao_em_conexao(&self, id_origem: IdEstacao, id_destino: IdEstacao) -> bool {
        if let Some(caminho_info) = &self.resultado_caminho_ui {
            for i in 0..caminho_info.estacoes_do_caminho.len().saturating_sub(2) {
                let (id1, _linha1) = caminho_info.estacoes_do_caminho[i];
                let (id2, linha2) = caminho_info.estacoes_do_caminho[i+1];
                let (id3, linha3) = caminho_info.estacoes_do_caminho[i+2];
                
                if (id1 == id_origem && id2 == id_destino) || (id2 == id_origem && id3 == id_destino) {
                    if linha2.is_some() && linha3.is_some() && linha2 != linha3 {
                        return true;
                    }
                }
            }
        }
        false
    }
    
    fn gerar_conteudo_popup_estacao(&self, id_estacao: IdEstacao, grafo: &GrafoMetro) -> String {
        let mut conteudo = String::new();
        
        // Nome da estação
        conteudo.push_str(&format!("Estação: {}\n", grafo.estacoes[id_estacao].nome));
        conteudo.push_str(&format!("ID: E{}\n\n", id_estacao + 1));
        
        // Papel na busca atual
        if id_estacao == self.id_estacao_inicio_selecionada {
            conteudo.push_str("Papel: INÍCIO\n");
        } else if id_estacao == self.id_estacao_objetivo_selecionada {
            conteudo.push_str("Papel: OBJETIVO\n");
        } else if let Some(ref info_caminho) = self.resultado_caminho_ui {
            if info_caminho.estacoes_do_caminho.iter().any(|(id, _)| *id == id_estacao) {
                conteudo.push_str("Papel: Parte do CAMINHO\n");
            }
        }
        
        // Informações heurísticas se disponíveis
        // Acessando o valor diretamente da matriz de distâncias heurísticas
        if let Some(distancia) = grafo.distancias_heuristicas_km[id_estacao][self.id_estacao_objetivo_selecionada] {
            conteudo.push_str(&format!("\nDistância estimada ao objetivo: {:.1} min\n", distancia));
        }
        
        // Informações do nó se estiver sendo expandido
        if let Some(ref no_atual) = self.no_expandido_atualmente_ui {
            if no_atual.id_estacao == id_estacao {
                conteudo.push_str("\n--- Expansão atual ---\n");
                conteudo.push_str(&format!("f(n) = {:.1}\n", no_atual.custo_f));
                conteudo.push_str(&format!("g(n) = {:.1}\n", no_atual.custo_g_viagem));
                conteudo.push_str(&format!("h(n) = {:.1}\n", no_atual.custo_f - no_atual.custo_g_viagem));
                
                if let Some(linha) = no_atual.linha_chegada {
                    conteudo.push_str(&format!("Linha atual: {:?}\n", linha));
                }
            }
        }
        
        // Conexões disponíveis a partir desta estação
        conteudo.push_str("\n--- Conexões ---\n");
        if let Some(conexoes) = grafo.lista_adjacencia.get(id_estacao) {
            if conexoes.is_empty() {
                conteudo.push_str("Nenhuma conexão disponível");
            } else {
                for conexao in conexoes {
                    let nome_destino = &grafo.estacoes[conexao.para_estacao].nome;
                    conteudo.push_str(&format!(
                        "→ {} ({:.1}min) [linha {:?}]\n", 
                        nome_destino, 
                        conexao.tempo_minutos,
                        conexao.cor_linha
                    ));
                }
            }
        }
        
        conteudo
    }
    
    fn abrir_popup_estacao(&mut self, id_estacao: IdEstacao, grafo: &GrafoMetro) {
        // Gerar conteúdo do popup e configurá-lo
        let popup = PopupInfo {
            id_estacao,
            conteudo: self.gerar_conteudo_popup_estacao(id_estacao, grafo),
            posicao: RefCell::new(Vec2::new(30.0, -30.0)),
            visivel: true,
            esta_sendo_arrastado: false,
            tamanho: Vec2::new(220.0, 180.0),
        };
        
        // Removemos qualquer popup existente para essa estação
        self.popups_info.insert(id_estacao, popup);
        self.estacoes_com_popup_automatico.insert(id_estacao);
    }

    // ... Adicione os outros métodos necessários
}

impl eframe::App for MinhaAplicacaoGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Lógica de execução automática
        if self.execucao_automatica && self.solucionador_a_estrela.is_some() {
            let tempo_atual = ctx.input(|i| i.time);
            if tempo_atual - self.ultimo_tempo_passo >= self.velocidade_execucao as f64 {
                self.executar_proximo_passo_a_estrela();
                self.ultimo_tempo_passo = tempo_atual;
                
                // Parar execução automática se o algoritmo terminou
                if self.solucionador_a_estrela.is_none() {
                    self.execucao_automatica = false;
                }
                
                // Solicitar novo frame para manter a animação
                ctx.request_repaint();
            }
        }
        
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
                        for _i in 0..NUMERO_ESTACOES * NUMERO_ESTACOES * 2 {
                            if self.solucionador_a_estrela.is_none() {
                                break;
                            }
                            self.executar_proximo_passo_a_estrela();
                        }
                        if self.solucionador_a_estrela.is_some() {
                            self.mensagem_status_ui = "Executar Tudo: Limite de passos atingido.".to_string();
                        }
                    }
                    
                    ui.separator();
                    ui.label("Execução Automática:");
                    
                    // Botão Play/Pause
                    let botao_texto = if self.execucao_automatica { "⏸ Pausar" } else { "▶ Executar" };
                    if ui.button(botao_texto).clicked() {
                        self.execucao_automatica = !self.execucao_automatica;
                        if self.execucao_automatica {
                            // Inicializar tempo quando começar execução automática
                            self.ultimo_tempo_passo = ui.ctx().input(|i| i.time);
                        }
                    }
                    
                    // Controle de velocidade
                    ui.label("Velocidade (segundos entre passos):");
                    ui.add(egui::Slider::new(&mut self.velocidade_execucao, 0.1..=3.0)
                        .step_by(0.1)
                        .text("s"));
                    
                    // Indicador visual do estado
                    if self.execucao_automatica {
                        ui.label(egui::RichText::new("🔄 Executando automaticamente...")
                            .color(egui::Color32::from_rgb(150, 255, 150)));
                    } else {
                        ui.label(egui::RichText::new("⏸ Pausado")
                            .color(egui::Color32::from_rgb(200, 200, 200)));
                    }
                }
                ui.separator();
                ui.label(&self.mensagem_status_ui);
                if let Some(info_caminho) = &self.resultado_caminho_ui {
                    ui.separator();
                    ui.heading("📍 Resumo da Rota");
                    
                    // Adiciona um quadro com fundo para destacar as informações principais
                    egui::Frame::group(ui.style())
                        .fill(egui::Color32::from_rgb(40, 42, 54))  // Mesmo tom escuro usado nas estações
                        .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(80, 80, 100)))
                        .corner_radius(egui::CornerRadius::same(8))
                        .inner_margin(egui::Margin::same(8))
                        .show(ui, |ui| {
                            // Métricas principais em destaque
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
                    
                    if let Some(grafo) = &self.grafo_metro {
                        // Frame com scroll para o trajeto
                        egui::ScrollArea::vertical()
                            .max_height(200.0)
                            .show(ui, |ui| {
                                // Tabela de trajeto
                                egui::Grid::new("grid_trajeto")
                                    .num_columns(3)
                                    .striped(true)
                                    .spacing([8.0, 4.0])
                                    .show(ui, |ui| {
                                        // Cabeçalho da tabela
                                        ui.add(egui::Label::new(egui::RichText::new("#").strong()));
                                        ui.add(egui::Label::new(egui::RichText::new("Estação").strong()));
                                        ui.add(egui::Label::new(egui::RichText::new("Linha").strong()));
                                        ui.end_row();
                                        
                                        // Conteúdo da tabela
                                        let mut linha_anterior: Option<CorLinha> = None;
                                        for (idx, (id_est, linha_chegada_op)) in info_caminho.estacoes_do_caminho.iter().enumerate() {
                                            let nome_est = &grafo.estacoes[*id_est].nome;
                                            
                                            // Marca as estações de início e fim com ícones
                                            let label_idx = if idx == 0 {
                                                format!("🏁")
                                            } else if idx == info_caminho.estacoes_do_caminho.len() - 1 {
                                                format!("🎯")
                                            } else {
                                                format!("{}", idx + 1)
                                            };
                                            
                                            ui.label(label_idx);
                                            
                                            // Destaca estações com baldeação
                                            let mut nome_estacao_texto = egui::RichText::new(nome_est);
                                            if linha_chegada_op.is_some() && linha_anterior.is_some() && 
                                               *linha_chegada_op != linha_anterior {
                                                nome_estacao_texto = nome_estacao_texto
                                                    .strong()
                                                    .color(egui::Color32::from_rgb(255, 220, 150));
                                                
                                                // Adiciona ícone de baldeação
                                                ui.label(egui::RichText::new(format!("🔄 {}", nome_estacao_texto.text())));
                                            } else {
                                                ui.label(nome_estacao_texto);
                                            }
                                            
                                            // Formata o nome da linha com a cor correspondente
                                            let texto_linha = match linha_chegada_op {
                                                Some(cor) => {
                                                    let cor_linha = match cor {
                                                        CorLinha::Azul => egui::Color32::from_rgb(0, 120, 255),
                                                        CorLinha::Amarela => egui::Color32::from_rgb(255, 215, 0),
                                                        CorLinha::Vermelha => egui::Color32::RED,
                                                        CorLinha::Verde => egui::Color32::from_rgb(0, 180, 0),
                                                        _ => egui::Color32::GRAY,
                                                    };
                                                    egui::RichText::new(format!("{:?}", cor)).color(cor_linha)
                                                },
                                                None => egui::RichText::new("Partida").italics(),
                                            };
                                            ui.label(texto_linha);
                                            
                                            ui.end_row();
                                            
                                            // Armazena a linha atual para comparação na próxima iteração
                                            linha_anterior = *linha_chegada_op;
                                        }
                                    });
                            });
                    }
                }
                ui.separator();
                ui.label("Opções de Visualização");
                ui.add(egui::Slider::new(&mut self.zoom_nivel, 0.5..=2.0)
                    .text("Zoom")
                    .step_by(0.1));
                ui.checkbox(&mut self.mostrar_custos_detalhados, "Mostrar Custos Detalhados");
                ui.checkbox(&mut self.mostrar_linha_atual, "Mostrar Linha Atual");
                ui.checkbox(&mut self.mostrar_tempos_conexao, "Mostrar Tempos entre Estações"); // Nova opção
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            let tamanho_disponivel = ui.available_size();

            static mut JA_CENTRALIZOU: bool = false;
            unsafe {
                if !JA_CENTRALIZOU {
                    self.centralizar_visualizacao(tamanho_disponivel);
                    JA_CENTRALIZOU = true;
                }
            }

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let response = ui.allocate_response(ui.available_size(), egui::Sense::click_and_drag());
                let rect_desenho = response.rect;
                let painter = ui.painter_at(rect_desenho);

                self.processar_eventos_navegacao(ui, &response, rect_desenho);

                painter.rect_filled(rect_desenho, 0.0, Color32::from_gray(30));

                if self.zoom_nivel > 1.2 {
                    let tamanho_grade = 50.0 * self.zoom_nivel;
                    let cor_grade = Color32::from_gray(45);
                    let tracos = Stroke::new(1.0, cor_grade);

                    let mut x = (rect_desenho.min.x + self.offset_rolagem.x) % tamanho_grade;
                    while x < rect_desenho.max.x {
                        painter.line_segment(
                            [Pos2::new(x, rect_desenho.min.y), Pos2::new(x, rect_desenho.max.y)],
                            tracos
                        );
                        x += tamanho_grade;
                    }

                    let mut y = (rect_desenho.min.y + self.offset_rolagem.y) % tamanho_grade;
                    while y < rect_desenho.max.y {
                        painter.line_segment(
                            [Pos2::new(rect_desenho.min.x, y), Pos2::new(rect_desenho.max.x, y)],
                            tracos
                        );
                        y += tamanho_grade;
                    }
                }

                // First get a clone of the Arc to avoid borrow checker issues
                let grafo_clone = match &self.grafo_metro {
                    Some(grafo_arc) => grafo_arc.clone(),
                    None => return,
                };
                
                // Dereference the Arc to get the GrafoMetro
                let grafo_ref = &*grafo_clone;
                
                // Now do all operations with immutable self first
                self.desenhar_conexoes(&painter, rect_desenho, grafo_ref);
                
                // Then do operations with mutable self
                self.desenhar_estacoes(&painter, rect_desenho, grafo_ref, ui);
                
                // Coletando informações para os popups - previne o erro de borrow checker
                let mut acoes_popup = Vec::new();
                for (id, popup) in &mut self.popups_info {
                    if popup.visivel {
                        let pos = self.posicoes_estacoes_tela[*id] * self.zoom_nivel + self.offset_rolagem + rect_desenho.min.to_vec2();
                        let popup_id = *id;
                        let popup_content = popup.conteudo.clone();
                        let should_close = egui::Area::new(Id::new(format!("popup_{}", id)))
                            .fixed_pos(pos + Vec2::new(30.0, -30.0))
                            .show(ui.ctx(), |ui| {
                                ui.group(|ui| {
                                    ui.label(&popup_content);
                                    ui.button("Fechar").clicked()
                                }).inner
                            }).inner;
                        
                        if should_close {
                            acoes_popup.push(AcaoPopup { 
                                id_estacao: popup_id, 
                                tipo: TipoAcaoPopup::Fechar, 
                                delta: None 
                            });
                        }
                    }
                }
                
                // Processando ações após o loop
                self.processar_acoes_popup(acoes_popup);
            });

            if self.solucionador_a_estrela.is_some() {
                ctx.request_repaint();
            }
        });
    }
}