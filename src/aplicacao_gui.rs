use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::cell::RefCell;
use egui::{Color32, ComboBox, Pos2, Stroke, Vec2, Id};

use crate::grafo_metro::{CorLinha, GrafoMetro, IdEstacao, NUMERO_ESTACOES};
use crate::algoritmo_a_estrela::{InfoCaminho, SolucionadorAEstrela};

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
    estacao_sendo_expandida_ui: Option<IdEstacao>, // Estação que está sendo expandida atualmente
    estacoes_exploradas_ui: HashSet<IdEstacao>, // Estações já completamente exploradas (conjunto fechado)
    detalhes_analise_ui: Vec<String>, // Detalhes da análise para exibição
    vizinhos_sendo_analisados_ui: HashSet<IdEstacao>, // Vizinhos da estação sendo expandida
    zoom_nivel: f32,
    mostrar_linha_atual: bool,
    mostrar_tempos_conexao: bool, // Nova opção para controlar a visibilidade dos tempos
    offset_rolagem: Vec2,
    arrastando: bool,
    ultima_posicao_mouse: Option<Pos2>,
    popups_info: HashMap<IdEstacao, PopupInfo>,
    estacoes_com_popup_automatico: HashSet<IdEstacao>,
    offset_arrasto_popup_atual: Option<Vec2>,
    estacao_sendo_arrastada: Option<IdEstacao>,
    // Controle de atualização para reduzir piscamento
    ultimo_tempo_animacao: f32,
    // Controle de centralização seguro
    ja_centralizou: bool,

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
            estacao_sendo_expandida_ui: None,
            estacoes_exploradas_ui: HashSet::new(), // Inicializamos vazio
            detalhes_analise_ui: Vec::new(), // Inicializamos vazio
            vizinhos_sendo_analisados_ui: HashSet::new(), // Inicializamos vazio
            zoom_nivel: 0.70, // Zoom inicial mais afastado para melhor visualização
            mostrar_linha_atual: true,
            mostrar_tempos_conexao: true, // Iniciar com os tempos visíveis
            offset_rolagem: Vec2::new(0.0, 0.0), // Inicializa centralizado
            arrastando: false,
            ultima_posicao_mouse: None,
            popups_info: HashMap::new(),
            estacoes_com_popup_automatico: HashSet::new(),
            offset_arrasto_popup_atual: None,
            estacao_sendo_arrastada: None,
            ultimo_tempo_animacao: 0.0,
            ja_centralizou: false,
        }
        // Removido código duplicado do Self retornado
    }

    // Todos os métodos necessários
    fn limpar_estado_visual(&mut self) {
        // Método central para limpar todos os estados visuais do algoritmo
        self.resultado_caminho_ui = None;
        self.estacao_sendo_expandida_ui = None;
        self.estacoes_exploradas_ui.clear();
        self.detalhes_analise_ui.clear();
        self.vizinhos_sendo_analisados_ui.clear();
        self.solucionador_a_estrela = None;
    }

    fn iniciar_busca_a_estrela(&mut self) {
        if let Some(ref grafo) = self.grafo_metro {
            let grafo_arco = Arc::clone(grafo);
            let id_inicio = self.id_estacao_inicio_selecionada;
            let id_objetivo = self.id_estacao_objetivo_selecionada;
            
            // Extrair nomes das estações antes de limpar o estado
            let nome_inicio = grafo.estacoes[id_inicio].nome.clone();
            let nome_objetivo = grafo.estacoes[id_objetivo].nome.clone();
            
            // Usar o método central para resetar estado
            self.limpar_estado_visual();
            
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
                nome_inicio, 
                nome_objetivo
            );
        } else {
            self.mensagem_status_ui = "Erro: Grafo não carregado.".to_string();
        }
    }

    fn executar_proximo_passo_a_estrela(&mut self) {
        if let Some(ref mut solucionador) = self.solucionador_a_estrela {
            match solucionador.proximo_passo() {
                crate::algoritmo_a_estrela::ResultadoPassoAEstrela::EmProgresso => {
                    // Atualizar a visualização com base na análise atual
                    if let Some(ref analise) = solucionador.ultima_analise {
                        // A estação que foi expandida (retirada da fronteira e analisada)
                        self.estacao_sendo_expandida_ui = Some(analise.estacao_expandida);
                        
                        // Adicionar a estação expandida ao conjunto de exploradas
                        self.estacoes_exploradas_ui.insert(analise.estacao_expandida);
                        
                        // Extrair os vizinhos que estão sendo analisados
                        self.vizinhos_sendo_analisados_ui.clear();
                        for vizinho_info in &analise.vizinhos_analisados {
                            // Extrair ID da estação do formato "E{id}: ..."
                            if let Some(inicio_e) = vizinho_info.find('E') {
                                if let Some(pos_dois_pontos) = vizinho_info.find(':') {
                                    if pos_dois_pontos > inicio_e + 1 {
                                        let numero_str = &vizinho_info[inicio_e + 1..pos_dois_pontos];
                                        if let Ok(id_estacao_um_baseado) = numero_str.parse::<usize>() {
                                            if id_estacao_um_baseado > 0 {
                                                let id_estacao = id_estacao_um_baseado - 1; // Converter para zero-based
                                                // Só adicionar se não foi explorado ainda
                                                if !self.estacoes_exploradas_ui.contains(&id_estacao) {
                                                    self.vizinhos_sendo_analisados_ui.insert(id_estacao);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        
                        // Atualizar detalhes da análise para exibição
                        self.detalhes_analise_ui = analise.vizinhos_analisados.clone();
                        
                        let nome_estacao = if let Some(ref grafo) = self.grafo_metro {
                            &grafo.estacoes[analise.estacao_expandida].nome
                        } else {
                            "Desconhecida"
                        };
                        
                        self.mensagem_status_ui = format!(
                            "Expandindo estação: {} (E{}) - {} vizinhos analisados",
                            nome_estacao,
                            analise.estacao_expandida + 1,
                            self.vizinhos_sendo_analisados_ui.len()
                        );
                    } else {
                        self.mensagem_status_ui = "Passo em progresso, mas sem detalhes da análise.".to_string();
                    }
                },
                crate::algoritmo_a_estrela::ResultadoPassoAEstrela::CaminhoEncontrado(caminho_info) => {
                    self.resultado_caminho_ui = Some(caminho_info.clone());
                    
                    // Limpar estados temporários da busca
                    self.estacao_sendo_expandida_ui = None;
                    self.vizinhos_sendo_analisados_ui.clear();
                    
                    // Marcar todas as estações do caminho final como exploradas
                    self.estacoes_exploradas_ui.clear();
                    for (id_estacao, _) in &caminho_info.estacoes_do_caminho {
                        self.estacoes_exploradas_ui.insert(*id_estacao);
                    }
                    
                    self.mensagem_status_ui = format!(
                        "✅ Caminho encontrado! Tempo: {:.1} min, Baldeações: {}",
                        caminho_info.tempo_total_minutos,
                        caminho_info.baldeacoes
                    );
                    self.solucionador_a_estrela = None;
                },
                crate::algoritmo_a_estrela::ResultadoPassoAEstrela::NenhumCaminhoPossivel => {
                    self.mensagem_status_ui = "❌ Não foi possível encontrar um caminho.".to_string();
                    // Limpar estados temporários
                    self.estacao_sendo_expandida_ui = None;
                    self.vizinhos_sendo_analisados_ui.clear();
                    self.solucionador_a_estrela = None;
                },
                crate::algoritmo_a_estrela::ResultadoPassoAEstrela::Erro(msg) => {
                    self.mensagem_status_ui = format!("❌ Erro: {}", msg);
                    // Limpar estados temporários
                    self.estacao_sendo_expandida_ui = None;
                    self.vizinhos_sendo_analisados_ui.clear();
                    self.solucionador_a_estrela = None;
                }
            }
        } else {
            self.mensagem_status_ui = "Erro: Nenhuma busca em andamento.".to_string();
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
                                            Vec2::new(0.0, -45.0 * self.zoom_nivel); // Posicionar MAIS ACIMA da estação
                        
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

    fn mostrar_popup_vizinho_hover(&self, ui: &mut egui::Ui, pos_estacao: egui::Pos2, id_estacao: IdEstacao, grafo: &GrafoMetro) {
        // Encontrar informações do vizinho na análise atual
        if let Some(ref solucionador) = self.solucionador_a_estrela {
            if let Some(ref analise) = solucionador.ultima_analise {
                if let Some(vizinho_info) = analise.vizinhos_analisados.iter()
                    .find(|v| v.starts_with(&format!("E{}", id_estacao + 1))) {
                    
                    // Extrair informações detalhadas do vizinho
                    let estacao = &grafo.estacoes[id_estacao];
                    let estacao_nome = &estacao.nome;
                    
                    // Parsear os valores f, g, h da string de informação
                    let (valor_f, valor_g, valor_h) = self.extrair_valores_fgh(vizinho_info);
                    
                    // Posição do pop-up ajustada para não sobrepor a estação
                    let pos_popup = pos_estacao + egui::Vec2::new(25.0 * self.zoom_nivel, -80.0 * self.zoom_nivel);
                    
                    // Use um ID estável para evitar recrear o popup constantemente
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
                                    
                                    // Cabeçalho do pop-up
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
                                    
                                    // Nome completo da estação
                                    ui.label(egui::RichText::new(format!("Estação: {}", estacao_nome))
                                        .size(13.0)
                                        .color(egui::Color32::WHITE)
                                        .strong());
                                    
                                    ui.label(egui::RichText::new(format!("ID: E{}", id_estacao + 1))
                                        .size(11.0)
                                        .color(egui::Color32::from_rgb(200, 200, 200)));
                                    
                                    ui.add_space(8.0);
                                    
                                    // Valores do algoritmo A* com explicações - DESTAQUE PRINCIPAL
                                    ui.add_space(8.0);
                                    ui.label(egui::RichText::new("VALORES FUNDAMENTAIS DO A*")
                                        .size(14.0)
                                        .color(egui::Color32::from_rgb(255, 215, 0))
                                        .strong());
                                    
                                    // Frame destacado para os valores principais
                                    egui::Frame::group(ui.style())
                                        .fill(egui::Color32::from_rgba_premultiplied(50, 50, 70, 200))
                                        .stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 215, 0)))
                                        .corner_radius(6.0)
                                        .inner_margin(egui::Margin::same(10))
                                        .show(ui, |ui| {
                                            // Exibir valores em formato mais destacado e claro - Ordem: H, G, F
                                            if let Some(h) = valor_h {
                                                ui.horizontal(|ui| {
                                                    ui.label(egui::RichText::new("H")
                                                        .size(16.0)
                                                        .color(egui::Color32::from_rgb(255, 150, 150))
                                                        .strong());
                                                    ui.label(egui::RichText::new("=")
                                                        .size(14.0)
                                                        .color(egui::Color32::WHITE));
                                                    ui.label(egui::RichText::new(format!("{:.1} min", h))
                                                        .size(16.0)
                                                        .color(egui::Color32::WHITE)
                                                        .strong());
                                                    ui.separator();
                                                    ui.label(egui::RichText::new("Estimativa até Destino")
                                                        .size(11.0)
                                                        .color(egui::Color32::from_rgb(200, 200, 200)));
                                                });
                                                ui.add_space(2.0);
                                            }
                                            
                                            if let Some(g) = valor_g {
                                                ui.horizontal(|ui| {
                                                    ui.label(egui::RichText::new("G")
                                                        .size(16.0)
                                                        .color(egui::Color32::from_rgb(150, 255, 150))
                                                        .strong());
                                                    ui.label(egui::RichText::new("=")
                                                        .size(14.0)
                                                        .color(egui::Color32::WHITE));
                                                    ui.label(egui::RichText::new(format!("{:.1} min", g))
                                                        .size(16.0)
                                                        .color(egui::Color32::WHITE)
                                                        .strong());
                                                    ui.separator();
                                                    ui.label(egui::RichText::new("Tempo Real Percorrido")
                                                        .size(11.0)
                                                        .color(egui::Color32::from_rgb(200, 200, 200)));
                                                });
                                                ui.add_space(2.0);
                                            }
                                            
                                            if let Some(f) = valor_f {
                                                ui.horizontal(|ui| {
                                                    ui.label(egui::RichText::new("F")
                                                        .size(16.0)
                                                        .color(egui::Color32::from_rgb(255, 220, 150))
                                                        .strong());
                                                    ui.label(egui::RichText::new("=")
                                                        .size(14.0)
                                                        .color(egui::Color32::WHITE));
                                                    ui.label(egui::RichText::new(format!("{:.1} min", f))
                                                        .size(16.0)
                                                        .color(egui::Color32::WHITE)
                                                        .strong());
                                                    ui.separator();
                                                    ui.label(egui::RichText::new("Custo Total Estimado")
                                                        .size(11.0)
                                                        .color(egui::Color32::from_rgb(200, 200, 200)));
                                                });
                                            }
                                            
                                            // Mostrar a fórmula visual
                                            if valor_f.is_some() && valor_g.is_some() && valor_h.is_some() {
                                                ui.add_space(8.0);
                                                ui.separator();
                                                ui.add_space(4.0);
                                                
                                                // Fórmula visual destacada
                                                ui.horizontal(|ui| {
                                                    ui.label(egui::RichText::new("Fórmula:")
                                                        .size(12.0)
                                                        .color(egui::Color32::from_rgb(255, 215, 0))
                                                        .strong());
                                                    ui.label(egui::RichText::new("F")
                                                        .size(14.0)
                                                        .color(egui::Color32::from_rgb(255, 220, 150))
                                                        .strong());
                                                    ui.label(egui::RichText::new("=")
                                                        .size(12.0)
                                                        .color(egui::Color32::WHITE));
                                                    ui.label(egui::RichText::new("G")
                                                        .size(14.0)
                                                        .color(egui::Color32::from_rgb(150, 255, 150))
                                                        .strong());
                                                    ui.label(egui::RichText::new("+")
                                                        .size(12.0)
                                                        .color(egui::Color32::WHITE));
                                                    ui.label(egui::RichText::new("H")
                                                        .size(14.0)
                                                        .color(egui::Color32::from_rgb(255, 150, 150))
                                                        .strong());
                                                });
                                                
                                                ui.horizontal(|ui| {
                                                    let f_val = valor_f.unwrap();
                                                    let g_val = valor_g.unwrap();
                                                    let h_val = valor_h.unwrap();
                                                    
                                                    ui.label(egui::RichText::new("Valores:")
                                                        .size(11.0)
                                                        .color(egui::Color32::from_rgb(200, 200, 200)));
                                                    ui.label(egui::RichText::new(format!("{:.1}", f_val))
                                                        .size(12.0)
                                                        .color(egui::Color32::from_rgb(255, 220, 150))
                                                        .strong());
                                                    ui.label(egui::RichText::new("=")
                                                        .size(10.0)
                                                        .color(egui::Color32::WHITE));
                                                    ui.label(egui::RichText::new(format!("{:.1}", g_val))
                                                        .size(12.0)
                                                        .color(egui::Color32::from_rgb(150, 255, 150))
                                                        .strong());
                                                    ui.label(egui::RichText::new("+")
                                                        .size(10.0)
                                                        .color(egui::Color32::WHITE));
                                                    ui.label(egui::RichText::new(format!("{:.1}", h_val))
                                                        .size(12.0)
                                                        .color(egui::Color32::from_rgb(255, 150, 150))
                                                        .strong());
                                                });
                                            }
                                        });
                                    
                                    ui.add_space(8.0);
                                    
                                    // Informações de conectividade
                                    if let Some(conexoes) = grafo.lista_adjacencia.get(id_estacao) {
                                        ui.label(egui::RichText::new("Conexões Disponíveis:")
                                            .size(12.0)
                                            .color(egui::Color32::from_rgb(150, 255, 150))
                                            .strong());
                                        
                                        let mut linhas_conectadas: std::collections::HashSet<CorLinha> = std::collections::HashSet::new();
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
                                    
                                    // Dica de interação
                                    ui.separator();
                                    ui.label(egui::RichText::new("Clique para ver informações detalhadas")
                                        .size(9.0)
                                        .color(egui::Color32::from_rgb(150, 150, 150))
                                        .italics());
                                });
                        });
                }
            }
        }
    }
    
    fn mostrar_popup_estacao_hover(&self, ui: &mut egui::Ui, pos_estacao: egui::Pos2, id_estacao: IdEstacao, grafo: &GrafoMetro) {
        let estacao = &grafo.estacoes[id_estacao];
        
        // Posição do pop-up ajustada para não sobrepor a estação
        let pos_popup = pos_estacao + egui::Vec2::new(25.0 * self.zoom_nivel, -60.0 * self.zoom_nivel);
        
        // ID estável para o popup
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
                        
                        // Cabeçalho
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
                        
                        // Nome e ID da estação
                        ui.label(egui::RichText::new(format!("{}", estacao.nome))
                            .size(14.0)
                            .color(egui::Color32::WHITE)
                            .strong());
                        
                        ui.label(egui::RichText::new(format!("Identificador: E{}", id_estacao + 1))
                            .size(11.0)
                            .color(egui::Color32::from_rgb(200, 200, 200)));
                        
                        ui.add_space(6.0);
                        
                        // Verificar se há informações A* disponíveis para esta estação
                        let mut valores_a_star_encontrados = false;
                        if let Some(ref solucionador) = self.solucionador_a_estrela {
                            if let Some(ref analise) = solucionador.ultima_analise {
                                // Procurar informações desta estação na análise atual
                                for vizinho_info in &analise.vizinhos_analisados {
                                    if vizinho_info.starts_with(&format!("E{}", id_estacao + 1)) {
                                        let (valor_f, valor_g, valor_h) = self.extrair_valores_fgh(vizinho_info);
                                        
                                        if valor_f.is_some() || valor_g.is_some() || valor_h.is_some() {
                                            valores_a_star_encontrados = true;
                                            
                                            ui.label(egui::RichText::new("VALORES A* DISPONÍVEIS")
                                                .size(13.0)
                                                .color(egui::Color32::from_rgb(255, 215, 0))
                                                .strong());
                                            
                                            egui::Frame::group(ui.style())
                                                .fill(egui::Color32::from_rgba_premultiplied(40, 40, 60, 200))
                                                .stroke(egui::Stroke::new(1.5, egui::Color32::from_rgb(255, 215, 0)))
                                                .corner_radius(4.0)
                                                .inner_margin(egui::Margin::same(8))
                                                .show(ui, |ui| {
                                                    // Ordem: H, G, F
                                                    if let Some(h) = valor_h {
                                                        ui.horizontal(|ui| {
                                                            ui.label(egui::RichText::new("H")
                                                                .size(14.0)
                                                                .color(egui::Color32::from_rgb(255, 150, 150))
                                                                .strong());
                                                            ui.label(egui::RichText::new("=")
                                                                .size(12.0)
                                                                .color(egui::Color32::WHITE));
                                                            ui.label(egui::RichText::new(format!("{:.1} min", h))
                                                                .size(14.0)
                                                                .color(egui::Color32::WHITE)
                                                                .strong());
                                                            ui.label(egui::RichText::new("(Estimativa)")
                                                                .size(10.0)
                                                                .color(egui::Color32::from_rgb(180, 180, 180)));
                                                        });
                                                    }
                                                    
                                                    if let Some(g) = valor_g {
                                                        ui.horizontal(|ui| {
                                                            ui.label(egui::RichText::new("G")
                                                                .size(14.0)
                                                                .color(egui::Color32::from_rgb(150, 255, 150))
                                                                .strong());
                                                            ui.label(egui::RichText::new("=")
                                                                .size(12.0)
                                                                .color(egui::Color32::WHITE));
                                                            ui.label(egui::RichText::new(format!("{:.1} min", g))
                                                                .size(14.0)
                                                                .color(egui::Color32::WHITE)
                                                                .strong());
                                                            ui.label(egui::RichText::new("(Percorrido)")
                                                                .size(10.0)
                                                                .color(egui::Color32::from_rgb(180, 180, 180)));
                                                        });
                                                    }
                                                    
                                                    if let Some(f) = valor_f {
                                                        ui.horizontal(|ui| {
                                                            ui.label(egui::RichText::new("F")
                                                                .size(14.0)
                                                                .color(egui::Color32::from_rgb(255, 220, 150))
                                                                .strong());
                                                            ui.label(egui::RichText::new("=")
                                                                .size(12.0)
                                                                .color(egui::Color32::WHITE));
                                                            ui.label(egui::RichText::new(format!("{:.1} min", f))
                                                                .size(14.0)
                                                                .color(egui::Color32::WHITE)
                                                                .strong());
                                                            ui.label(egui::RichText::new("(Total)")
                                                                .size(10.0)
                                                                .color(egui::Color32::from_rgb(180, 180, 180)));
                                                        });
                                                    }
                                                });
                                            
                                            ui.add_space(4.0);
                                        }
                                        break;
                                    }
                                }
                            }
                        }
                        
                        if !valores_a_star_encontrados {
                            ui.add_space(6.0);
                        }
                        
                        // Status atual da estação
                        if id_estacao == self.id_estacao_inicio_selecionada {
                            ui.label(egui::RichText::new("Estação de INÍCIO")
                                .size(12.0)
                                .color(egui::Color32::from_rgb(100, 255, 100))
                                .strong());
                        } else if id_estacao == self.id_estacao_objetivo_selecionada {
                            ui.label(egui::RichText::new("Estação de DESTINO")
                                .size(12.0)
                                .color(egui::Color32::from_rgb(255, 100, 100))
                                .strong());
                        } else if self.estacoes_exploradas_ui.contains(&id_estacao) && self.resultado_caminho_ui.is_some() {
                            ui.label(egui::RichText::new("Parte da rota encontrada")
                                .size(12.0)
                                .color(egui::Color32::from_rgb(100, 255, 150))
                                .strong());
                        } else if self.estacoes_exploradas_ui.contains(&id_estacao) {
                            ui.label(egui::RichText::new("Sendo explorada")
                                .size(12.0)
                                .color(egui::Color32::from_rgb(150, 200, 255))
                                .strong());
                        } else {
                            ui.label(egui::RichText::new("Estação disponível")
                                .size(12.0)
                                .color(egui::Color32::from_rgb(180, 180, 180)));
                        }
                        
                        ui.add_space(6.0);
                        
                        // Informações de conectividade
                        if let Some(conexoes) = grafo.lista_adjacencia.get(id_estacao) {
                            ui.label(egui::RichText::new("Linhas disponíveis:")
                                .size(11.0)
                                .color(egui::Color32::from_rgb(150, 200, 255))
                                .strong());
                            
                            let mut linhas_conectadas: std::collections::HashSet<CorLinha> = std::collections::HashSet::new();
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
                        ui.separator();
                        ui.label(egui::RichText::new("Clique para ver mais detalhes")
                            .size(9.0)
                            .color(egui::Color32::from_rgb(150, 150, 150))
                            .italics());
                    });
            });
    }
    
    // Função auxiliar para extrair valores f, g, h da string de informação
    fn extrair_valores_fgh(&self, info: &str) -> (Option<f32>, Option<f32>, Option<f32>) {
        let mut valor_f = None;
        let mut valor_g = None;
        let mut valor_h = None;
        
        // Parsear a string no formato: "E{id}: g={value}, h={value}, f={value} - ADICIONADO"
        // Primeiro, encontrar onde começam os valores após ':'
        if let Some(pos_dois_pontos) = info.find(':') {
            let valores_parte = &info[pos_dois_pontos + 1..];
            
            // Dividir por vírgulas e processar cada parte
            for parte in valores_parte.split(',') {
                let parte = parte.trim();
                
                // Verificar se contém "g=", "h=" ou "f="
                if let Some(pos_g) = parte.find("g=") {
                    let valor_str = &parte[pos_g + 2..];
                    // Extrair apenas os dígitos e ponto decimal
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

    fn desenhar_marcadores_estacoes(&self, painter: &egui::Painter, rect_desenho: egui::Rect, grafo: &GrafoMetro, _ui: &egui::Ui) {
        // Desenha marcadores visuais acima das estações para indicar seu status
        for (i, _estacao) in grafo.estacoes.iter().enumerate() {
            let pos = self.posicoes_estacoes_tela[i] * self.zoom_nivel + self.offset_rolagem + rect_desenho.min.to_vec2();
            
            // Posição do marcador (acima da estação)
            let pos_marcador = pos + Vec2::new(0.0, -35.0 * self.zoom_nivel);
            
            // Determinar que tipo de marcador mostrar (prioridade: início/fim > caminho atual > analisando vizinhos)
            let marcador_info = if i == self.id_estacao_inicio_selecionada {
                Some(("INÍCIO", Color32::from_rgb(0, 140, 0), Color32::from_rgb(20, 80, 20)))
            } else if i == self.id_estacao_objetivo_selecionada {
                Some(("FIM", Color32::from_rgb(220, 50, 50), Color32::from_rgb(80, 20, 20)))
            } else if self.estacoes_exploradas_ui.contains(&i) && self.resultado_caminho_ui.is_some() {
                // Só mostrar "CAMINHO" verde se realmente há uma solução final
                Some(("CAMINHO", Color32::from_rgb(0, 120, 60), Color32::from_rgb(20, 60, 40)))
            } else if self.estacoes_exploradas_ui.contains(&i) && self.solucionador_a_estrela.is_some() {
                // Mostrar "EXPLORANDO" azul para caminho parcial durante a busca
                Some(("EXPLORANDO", Color32::from_rgb(60, 100, 200), Color32::from_rgb(30, 50, 100)))
            } else if self.vizinhos_sendo_analisados_ui.contains(&i) {
                // Apenas vizinhos que NÃO estão no caminho atual recebem marcador "ANALISANDO"
                Some(("ANALISANDO", Color32::from_rgb(255, 140, 0), Color32::from_rgb(120, 60, 0)))
            } else {
                None
            };
            
            // Desenhar o marcador se houver
            if let Some((texto, cor_fundo, cor_borda)) = marcador_info {
                // Calcular tamanho do texto
                let fonte = egui::FontId::proportional((9.0 * self.zoom_nivel).max(8.0)); // Tamanho mínimo para legibilidade
                let tamanho_texto = painter.ctx().fonts(|f| f.layout_no_wrap(texto.to_string(), fonte.clone(), Color32::WHITE));
                
                // Desenhar fundo arredondado
                let padding = Vec2::new(6.0, 3.0) * self.zoom_nivel;
                let rect_fundo = egui::Rect::from_center_size(
                    pos_marcador,
                    tamanho_texto.rect.size() + padding * 2.0
                );
                
                // Sombra
                painter.rect_filled(
                    rect_fundo.translate(Vec2::new(1.0, 1.0) * self.zoom_nivel),
                    3.0 * self.zoom_nivel,
                    Color32::from_rgba_premultiplied(0, 0, 0, 120)
                );
                
                // Fundo principal
                painter.rect_filled(
                    rect_fundo,
                    3.0 * self.zoom_nivel,
                    cor_fundo
                );
                
                // Borda
                painter.rect_stroke(
                    rect_fundo,
                    3.0 * self.zoom_nivel,
                    Stroke::new(1.0 * self.zoom_nivel, cor_borda),
                    egui::StrokeKind::Middle
                );
                
                // Texto
                painter.text(
                    pos_marcador,
                    egui::Align2::CENTER_CENTER,
                    texto,
                    fonte,
                    Color32::WHITE
                );
                
                // Seta apontando para a estação (mais simples e sutil)
                let pos_seta_inicio = pos_marcador + Vec2::new(0.0, rect_fundo.height() / 2.0);
                let pos_seta_fim = pos + Vec2::new(0.0, -22.0 * self.zoom_nivel);
                
                // Linha da seta
                painter.line_segment(
                    [pos_seta_inicio, pos_seta_fim],
                    Stroke::new(1.5 * self.zoom_nivel, cor_borda)
                );
                
                // Ponta da seta (triângulo pequeno)
                let tamanho_ponta = 3.0 * self.zoom_nivel;
                let ponta1 = pos_seta_fim + Vec2::new(-tamanho_ponta, -tamanho_ponta);
                let ponta2 = pos_seta_fim + Vec2::new(tamanho_ponta, -tamanho_ponta);
                
                painter.line_segment([pos_seta_fim, ponta1], Stroke::new(1.5 * self.zoom_nivel, cor_borda));
                painter.line_segment([pos_seta_fim, ponta2], Stroke::new(1.5 * self.zoom_nivel, cor_borda));
            }
        }
    }

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
            
            // Determinar se esta estação é um vizinho sendo analisado
            let e_vizinho_sendo_analisado = self.vizinhos_sendo_analisados_ui.contains(&i);
            
            // Desenhar efeito visual para vizinhos sendo analisados
            if e_vizinho_sendo_analisado {
                // Efeito estático para evitar piscamento
                painter.circle_stroke(
                    pos,
                    20.0 * self.zoom_nivel,
                    Stroke::new(2.0 * self.zoom_nivel, Color32::from_rgba_premultiplied(255, 165, 0, 160))
                );
                
                // Círculo interno estático
                painter.circle_stroke(
                    pos,
                    16.0 * self.zoom_nivel,
                    Stroke::new(1.0 * self.zoom_nivel, Color32::from_rgba_premultiplied(255, 140, 0, 100))
                );
            }
            
            // Cor de preenchimento baseada no status da estação
            let cor_preenchimento = if i == self.id_estacao_inicio_selecionada {
                // Estação de início fica verde escuro
                Color32::from_rgb(0, 60, 0)
            } else if self.estacoes_exploradas_ui.contains(&i) && self.resultado_caminho_ui.is_some() {
                // Estações do caminho FINAL (só quando há solução) ficam com verde mais claro
                Color32::from_rgb(0, 40, 20) // Verde mais sutil para diferenciação
            } else if self.estacoes_exploradas_ui.contains(&i) && self.solucionador_a_estrela.is_some() {
                // Estações do caminho PARCIAL (durante a busca) ficam com azul escuro
                Color32::from_rgb(20, 30, 50) // Azul escuro para caminho parcial
            } else {
                Color32::from_rgb(40, 42, 54) // Cor base escura para estações não visitadas
            };
            
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
            
            // Borda com cor apropriada e espessura maior - ORDEM CORRIGIDA DE PRIORIDADE
            let (cor_borda, espessura_borda) = if i == self.id_estacao_inicio_selecionada {
                (Color32::from_rgb(0, 140, 0), 3.0) // Verde mais escuro para início
            } else if i == self.id_estacao_objetivo_selecionada {
                (Color32::from_rgb(220, 50, 50), 3.0) // Vermelho para objetivo
            } else if e_vizinho_sendo_analisado {
                (Color32::from_rgb(255, 140, 0), 2.5) // PRIORIDADE ALTA: Laranja para vizinhos sendo analisados
            } else if self.estacoes_exploradas_ui.contains(&i) && self.resultado_caminho_ui.is_some() {
                (Color32::from_rgb(0, 180, 100), 2.5) // Verde-água apenas para estações do caminho FINAL
            } else if self.estacoes_exploradas_ui.contains(&i) && self.solucionador_a_estrela.is_some() {
                (Color32::from_rgb(100, 150, 255), 2.5) // Azul claro para estações do caminho PARCIAL durante busca
            } else if esta_na_solucao {
                (Color32::from_rgb(0, 150, 136), 3.0) // Verde-azul escuro para estações na solução final
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
            
            // Interação: hover para mostrar informações detalhadas, clique para popup persistente
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
                
                // Mostrar pop-up informativo SEMPRE quando hover (não só para vizinhos sendo analisados)
                if e_vizinho_sendo_analisado {
                    // Pop-up especial para vizinhos sendo analisados (com detalhes do algoritmo A*)
                    self.mostrar_popup_vizinho_hover(ui, pos, i, grafo);
                } else {
                    // Pop-up simples para estações normais
                    self.mostrar_popup_estacao_hover(ui, pos, i, grafo);
                }
            }
            
            // Processar clique na estação para abrir popup persistente
            if response.clicked() && !ui.input(|i| i.pointer.is_decidedly_dragging()) {
                // Abrir popup persistente com informações da estação
                self.abrir_popup_estacao(i, grafo);
            }
        }
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
        
        // Suporte para zoom com a roda do mouse APENAS se estiver sobre a área de desenho
        if response.hovered() {
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
    }

    fn processar_acoes_popup(&mut self, acoes: Vec<AcaoPopup>) {
        // Processa ações dos popups (fechar, mover, etc)
        for acao in acoes {
            match acao.tipo {
                TipoAcaoPopup::Fechar => {
                    if let Some(popup) = self.popups_info.get_mut(&acao.id_estacao) {
                        popup.visivel = false;
                    }
                },
                TipoAcaoPopup::Iniciar => {
                    // Lógica para iniciar popup
                },
                TipoAcaoPopup::MoverDelta => {
                    if let Some(delta) = acao.delta {
                        if let Some(popup) = self.popups_info.get_mut(&acao.id_estacao) {
                            let mut pos = popup.posicao.borrow().clone();
                            pos += delta;
                            *popup.posicao.borrow_mut() = pos;
                        }
                    }
                },
                TipoAcaoPopup::Soltar => {
                    if let Some(popup) = self.popups_info.get_mut(&acao.id_estacao) {
                        popup.esta_sendo_arrastado = false;
                    }
                }
            }
        }
    }

    fn abrir_popup_estacao(&mut self, id_estacao: IdEstacao, grafo: &GrafoMetro) {
        let estacao = &grafo.estacoes[id_estacao];
        
        // Criar conteúdo mais detalhado para o popup persistente
        let mut conteudo = format!("Estação: {}\nID: E{}\n\n", estacao.nome, id_estacao + 1);
        
        // Status da estação
        if id_estacao == self.id_estacao_inicio_selecionada {
            conteudo.push_str("Status: ESTAÇÃO DE INÍCIO\n\n");
        } else if id_estacao == self.id_estacao_objetivo_selecionada {
            conteudo.push_str("Status: ESTAÇÃO DE DESTINO\n\n");
        } else if self.estacoes_exploradas_ui.contains(&id_estacao) && self.resultado_caminho_ui.is_some() {
            conteudo.push_str("Status: PARTE DA ROTA ENCONTRADA\n\n");
        } else if self.estacoes_exploradas_ui.contains(&id_estacao) {
            conteudo.push_str("Status: SENDO EXPLORADA\n\n");
        } else {
            conteudo.push_str("Status: DISPONÍVEL\n\n");
        }
        
        // Informações de conectividade
        if let Some(conexoes) = grafo.lista_adjacencia.get(id_estacao) {
            conteudo.push_str("CONEXÕES DISPONÍVEIS:\n");
            
            let mut linhas_conectadas: std::collections::HashSet<CorLinha> = std::collections::HashSet::new();
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
            
            // Mostrar algumas conexões diretas
            conteudo.push_str("ESTAÇÕES CONECTADAS:\n");
            let mut conexoes_mostradas = 0;
            for conexao in conexoes.iter().take(5) { // Mostrar apenas as primeiras 5
                let estacao_destino = &grafo.estacoes[conexao.para_estacao];
                conteudo.push_str(&format!("• {} ({:.1} min)\n", estacao_destino.nome, conexao.tempo_minutos));
                conexoes_mostradas += 1;
            }
            
            if conexoes.len() > 5 {
                conteudo.push_str(&format!("• ... e mais {} conexões\n", conexoes.len() - 5));
            }
        }
        
        conteudo.push_str("\nUse os controles do painel lateral para\n   selecionar início e destino");
        
        let popup = PopupInfo {
            id_estacao,
            conteudo,
            posicao: RefCell::new(Vec2::new(50.0, -40.0)),
            visivel: true,
            esta_sendo_arrastado: false,
            tamanho: Vec2::new(300.0, 200.0),
        };
        
        self.popups_info.insert(id_estacao, popup);
    }

    fn desenhar_popups(&mut self, ui: &mut egui::Ui, rect_desenho: egui::Rect, _grafo: &GrafoMetro) -> Vec<AcaoPopup> {
        // Desenha popups persistentes com informações detalhadas das estações
        let mut acoes = Vec::new();
        for (id, popup) in self.popups_info.iter_mut() {
            if popup.visivel {
                let pos_estacao = self.posicoes_estacoes_tela[*id] * self.zoom_nivel + self.offset_rolagem + rect_desenho.min.to_vec2();
                let offset_popup = *popup.posicao.borrow();
                let pos_popup = pos_estacao + offset_popup;
                
                let _area = egui::Area::new(Id::new(format!("popup_persistente_{}", id)))
                    .fixed_pos(pos_popup)
                    .order(egui::Order::Foreground)
                    .constrain(true)
                    .show(ui.ctx(), |ui| {
                        // Frame customizado para o popup persistente
                        egui::Frame::popup(ui.style())
                            .fill(egui::Color32::from_rgba_premultiplied(25, 30, 40, 250))
                            .stroke(egui::Stroke::new(2.0, egui::Color32::from_rgb(120, 150, 200)))
                            .corner_radius(8.0)
                            .inner_margin(egui::Margin::same(12))
                            .show(ui, |ui| {
                                ui.set_max_width(320.0);
                                ui.set_min_width(280.0);
                                
                                // Cabeçalho do popup com título arrastável e botão de fechar melhorado
                                let _header_response = ui.horizontal(|ui| {
                                    ui.label(egui::RichText::new("[INFO]")
                                        .size(14.0)
                                        .color(egui::Color32::from_rgb(120, 150, 200))
                                        .strong());
                                    
                                    // Título que serve como área de arrasto
                                    let title_response = ui.add(egui::Label::new(
                                        egui::RichText::new("Detalhes da Estação")
                                            .size(14.0)
                                            .color(egui::Color32::from_rgb(120, 150, 200))
                                            .strong()
                                    ).sense(egui::Sense::drag()));
                                    
                                    // Mudar cursor quando sobre o título arrastável
                                    if title_response.hovered() {
                                        ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);
                                    }
                                    
                                    // Detectar arrasto no título
                                    if title_response.dragged() {
                                        acoes.push(AcaoPopup { 
                                            id_estacao: *id, 
                                            tipo: TipoAcaoPopup::MoverDelta, 
                                            delta: Some(title_response.drag_delta()) 
                                        });
                                    }
                                    
                                    // Botão de fechar melhorado no canto direito
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        // Botão X mais visível e elegante
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
                                                id_estacao: *id, 
                                                tipo: TipoAcaoPopup::Fechar, 
                                                delta: None 
                                            });
                                        }
                                    });
                                    
                                    title_response
                                });
                                
                                // Dica visual de que pode ser arrastado
                                ui.label(egui::RichText::new("Arraste o título para mover o popup")
                                    .size(9.0)
                                    .color(egui::Color32::from_rgb(150, 150, 150))
                                    .italics());
                                
                                ui.separator();
                                
                                // Conteúdo do popup em scroll area
                                egui::ScrollArea::vertical()
                                    .max_height(300.0)
                                    .show(ui, |ui| {
                                        // Dividir o conteúdo em linhas e formatar adequadamente
                                        for linha in popup.conteudo.lines() {
                                            if linha.trim().is_empty() {
                                                ui.add_space(4.0);
                                            } else if linha.starts_with("Estação:") || linha.starts_with("ID:") {
                                                // Títulos principais
                                                ui.label(egui::RichText::new(linha)
                                                    .size(13.0)
                                                    .color(egui::Color32::WHITE)
                                                    .strong());
                                            } else if linha.contains("Status:") {
                                                // Status da estação
                                                let cor = if linha.contains("INÍCIO") {
                                                    egui::Color32::from_rgb(100, 255, 100)
                                                } else if linha.contains("DESTINO") {
                                                    egui::Color32::from_rgb(255, 100, 100)
                                                } else if linha.contains("ROTA ENCONTRADA") {
                                                    egui::Color32::from_rgb(100, 255, 150)
                                                } else if linha.contains("EXPLORADA") {
                                                    egui::Color32::from_rgb(150, 200, 255)
                                                } else {
                                                    egui::Color32::from_rgb(180, 180, 180)
                                                };
                                                ui.label(egui::RichText::new(linha)
                                                    .size(12.0)
                                                    .color(cor)
                                                    .strong());
                                            } else if linha.contains("CONEXÕES DISPONÍVEIS") || linha.contains("ESTAÇÕES CONECTADAS") {
                                                // Seções principais
                                                ui.add_space(6.0);
                                                ui.label(egui::RichText::new(linha)
                                                    .size(12.0)
                                                    .color(egui::Color32::from_rgb(150, 200, 255))
                                                    .strong());
                                            } else if linha.starts_with("• Linha") {
                                                // Linhas do metrô com cores
                                                let cor = if linha.contains("Azul") {
                                                    egui::Color32::from_rgb(0, 120, 255)
                                                } else if linha.contains("Amarela") {
                                                    egui::Color32::from_rgb(255, 215, 0)
                                                } else if linha.contains("Vermelha") {
                                                    egui::Color32::RED
                                                } else if linha.contains("Verde") {
                                                    egui::Color32::from_rgb(0, 180, 0)
                                                } else {
                                                    egui::Color32::GRAY
                                                };
                                                ui.label(egui::RichText::new(linha)
                                                    .size(11.0)
                                                    .color(cor));
                                            } else if linha.starts_with("• ") && linha.contains("min") {
                                                // Conexões com tempo
                                                ui.label(egui::RichText::new(linha)
                                                    .size(10.0)
                                                    .color(egui::Color32::from_rgb(200, 200, 200)));
                                            } else if linha.contains("Total de conexões") || linha.contains("Use os controles") {
                                                // Informações adicionais
                                                ui.label(egui::RichText::new(linha)
                                                    .size(10.0)
                                                    .color(egui::Color32::from_rgb(180, 180, 180)));
                                            } else {
                                                // Texto normal
                                                ui.label(egui::RichText::new(linha)
                                                    .size(10.0)
                                                    .color(egui::Color32::from_rgb(220, 220, 220)));
                                            }
                                        }
                                    });
                                
                                ui.add_space(6.0);
                                ui.separator();
                                
                                // Rodapé com dica
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
                            });
                    });
            }
        }
        acoes
    }

    // ...existing methods...
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
                ui.label(egui::RichText::new("Controles de Busca")
                    .size(14.0)
                    .strong());
                ui.add_space(5.0);
                
                // Definir tamanho padrão para todos os botões
                let tamanho_botao = egui::Vec2::new(200.0, 32.0);
                
                // Botões principais de controle organizados verticalmente
                if ui.add_sized(tamanho_botao, egui::Button::new("Iniciar/Reiniciar Busca")).clicked() {
                    self.iniciar_busca_a_estrela();
                }
                
                ui.add_space(3.0);
                
                if ui.add_sized(tamanho_botao, egui::Button::new("Limpar Tudo")).clicked() {
                    self.limpar_estado_visual();
                    self.mensagem_status_ui = "Estado limpo. Selecione início/fim e inicie nova busca.".to_string();
                }
                
                // Botões de execução (apenas quando há busca ativa)
                if self.solucionador_a_estrela.is_some() {
                    ui.add_space(8.0);
                    ui.label(egui::RichText::new("Execução Passo a Passo")
                        .size(13.0)
                        .strong());
                    ui.add_space(5.0);
                    
                    if ui.add_sized(tamanho_botao, egui::Button::new("Próximo Passo")).clicked() {
                        self.executar_proximo_passo_a_estrela();
                    }
                    
                    ui.add_space(3.0);
                    
                    if ui.add_sized(tamanho_botao, egui::Button::new("Executar Tudo")).clicked() {
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
                }
                ui.separator();
                ui.label(&self.mensagem_status_ui);
                if let Some(info_caminho) = &self.resultado_caminho_ui {
                    ui.separator();
                    ui.heading("Resumo da Rota");
                    
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
                                            
                                            // Marca as estações de início e fim com símbolos
                                            let label_idx = if idx == 0 {
                                                format!("[INÍCIO]")
                                            } else if idx == info_caminho.estacoes_do_caminho.len() - 1 {
                                                format!("[FIM]")
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
                                                
                                                // Adiciona indicador de baldeação
                                                ui.label(egui::RichText::new(format!("[BALDEAÇÃO] {}", nome_estacao_texto.text())));
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
                ui.label(egui::RichText::new("Opções de Visualização")
                    .size(14.0)
                    .strong());
                ui.add_space(5.0);
                
                // Slider de zoom com tamanho padronizado
                ui.horizontal(|ui| {
                    ui.label("Zoom:");
                    ui.add_sized([140.0, 20.0], egui::Slider::new(&mut self.zoom_nivel, 0.5..=2.0)
                        .show_value(true)
                        .step_by(0.1));
                });
                
                ui.add_space(3.0);
                
                // Checkboxes organizados
                ui.checkbox(&mut self.mostrar_linha_atual, "Mostrar Linha Atual");
                ui.checkbox(&mut self.mostrar_tempos_conexao, "Mostrar Tempos entre Estações");
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            let tamanho_disponivel = ui.available_size();

            // Centralização segura sem unsafe
            if !self.ja_centralizou {
                self.centralizar_visualizacao(tamanho_disponivel);
                self.ja_centralizou = true;
            }

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let response = ui.allocate_response(ui.available_size(), egui::Sense::click_and_drag());
                let rect_desenho = response.rect;
                let painter = ui.painter_at(rect_desenho);

                // Só processar eventos de navegação se o mouse estiver sobre a área de desenho
                if response.hovered() {
                    self.processar_eventos_navegacao(ui, &response, rect_desenho);
                }

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
                
                // Desenhar marcadores visuais acima das estações
                self.desenhar_marcadores_estacoes(&painter, rect_desenho, grafo_ref, ui);
                
                // Desenhar popups persistentes e processar suas ações
                let acoes_popup = self.desenhar_popups(ui, rect_desenho, grafo_ref);
                self.processar_acoes_popup(acoes_popup);
            });

            // Controle de repaint mais rigoroso para evitar piscamento
            let precisa_repaint = self.solucionador_a_estrela.is_some() || !self.vizinhos_sendo_analisados_ui.is_empty();
            if precisa_repaint {
                let tempo = ctx.input(|i| i.time) as f32;
                if tempo - self.ultimo_tempo_animacao > 0.16 { // ~60 FPS máximo
                    self.ultimo_tempo_animacao = tempo;
                    ctx.request_repaint();
                }
            }
        });
    }
}