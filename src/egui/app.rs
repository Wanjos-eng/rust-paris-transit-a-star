use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::cell::RefCell;
use egui::{Color32, Vec2};

use crate::grafo_metro::{CorLinha, GrafoMetro, IdEstacao, NUMERO_ESTACOES};
use crate::algoritmo_a_estrela::{InfoCaminho, SolucionadorAEstrela};

#[derive(Clone, Debug)]
pub struct PopupInfo {
    pub id_estacao: IdEstacao,
    pub conteudo: String,
    pub posicao: RefCell<Vec2>,
    pub visivel: bool,
    pub esta_sendo_arrastado: bool,
    pub tamanho: Vec2,
}

pub enum TipoAcaoPopup {
    Fechar,
    Iniciar,
    MoverDelta,
    Soltar,
}

pub struct AcaoPopup {
    pub id_estacao: IdEstacao,
    pub tipo: TipoAcaoPopup,
    pub delta: Option<Vec2>,
}

pub struct MinhaAplicacaoGUI {
    pub grafo_metro: Option<Arc<GrafoMetro>>,
    pub solucionador_a_estrela: Option<SolucionadorAEstrela>,
    pub id_estacao_inicio_selecionada: IdEstacao,
    pub id_estacao_objetivo_selecionada: IdEstacao,
    pub linha_inicio_opcional: Option<CorLinha>,
    pub resultado_caminho_ui: Option<InfoCaminho>,
    pub mensagem_status_ui: String,
    pub posicoes_estacoes_tela: Vec<egui::Pos2>,
    pub estacao_sendo_expandida_ui: Option<IdEstacao>,
    pub estacoes_exploradas_ui: HashSet<IdEstacao>,
    pub detalhes_analise_ui: Vec<String>,
    pub vizinhos_sendo_analisados_ui: HashSet<IdEstacao>,
    pub zoom_nivel: f32,
    pub mostrar_tempos_conexao: bool,
    pub mostrar_marcadores_estacoes: bool,
    pub mostrar_ids_estacoes: bool,
    pub offset_rolagem: Vec2,
    pub arrastando: bool,
    pub ultima_posicao_mouse: Option<egui::Pos2>,
    pub popups_info: HashMap<IdEstacao, PopupInfo>,
    pub estacoes_com_popup_automatico: HashSet<IdEstacao>,
    pub offset_arrasto_popup_atual: Option<Vec2>,
    pub estacao_sendo_arrastada: Option<IdEstacao>,
    pub ultimo_tempo_animacao: f32,
    pub ja_centralizou: bool,
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
        
        let mut posicoes = vec![egui::Pos2::ZERO; NUMERO_ESTACOES];
        let offset_x = 200.0;
        let offset_y = 150.0;
        let fator_escala = 1.4;
        
        if NUMERO_ESTACOES >= 14 {
            posicoes[0] = egui::Pos2::new(offset_x + 80.0 * fator_escala, offset_y + 250.0 * fator_escala);
            posicoes[1] = egui::Pos2::new(offset_x + 220.0 * fator_escala, offset_y + 240.0 * fator_escala);
            posicoes[2] = egui::Pos2::new(offset_x + 360.0 * fator_escala, offset_y + 230.0 * fator_escala);
            posicoes[3] = egui::Pos2::new(offset_x + 400.0 * fator_escala, offset_y + 280.0 * fator_escala);
            posicoes[4] = egui::Pos2::new(offset_x + 580.0 * fator_escala, offset_y + 350.0 * fator_escala);
            posicoes[5] = egui::Pos2::new(offset_x + 730.0 * fator_escala, offset_y + 320.0 * fator_escala);
            posicoes[6] = egui::Pos2::new(offset_x + 680.0 * fator_escala, offset_y + 390.0 * fator_escala);
            posicoes[7] = egui::Pos2::new(offset_x + 420.0 * fator_escala, offset_y + 150.0 * fator_escala);
            posicoes[8] = egui::Pos2::new(offset_x + 300.0 * fator_escala, offset_y + 130.0 * fator_escala);
            posicoes[9] = egui::Pos2::new(offset_x + 150.0 * fator_escala, offset_y + 210.0 * fator_escala);
            posicoes[10] = egui::Pos2::new(offset_x + 200.0 * fator_escala, offset_y + 50.0 * fator_escala);
            posicoes[11] = egui::Pos2::new(offset_x + 400.0 * fator_escala, offset_y + 50.0 * fator_escala);
            posicoes[12] = egui::Pos2::new(offset_x + 400.0 * fator_escala, offset_y + 480.0 * fator_escala);
            posicoes[13] = egui::Pos2::new(offset_x + 380.0 * fator_escala, offset_y + 580.0 * fator_escala);
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
            estacao_sendo_expandida_ui: None,
            estacoes_exploradas_ui: HashSet::new(),
            detalhes_analise_ui: Vec::new(),
            vizinhos_sendo_analisados_ui: HashSet::new(),
            zoom_nivel: 0.70,
            mostrar_tempos_conexao: true,
            mostrar_marcadores_estacoes: true,
            mostrar_ids_estacoes: true,
            offset_rolagem: Vec2::new(0.0, 0.0),
            arrastando: false,
            ultima_posicao_mouse: None,
            popups_info: HashMap::new(),
            estacoes_com_popup_automatico: HashSet::new(),
            offset_arrasto_popup_atual: None,
            estacao_sendo_arrastada: None,
            ultimo_tempo_animacao: 0.0,
            ja_centralizou: false,
        }
    }
}

impl eframe::App for MinhaAplicacaoGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        super::controls::mostrar_painel_controles(self, ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            let tamanho_disponivel = ui.available_size();

            if !self.ja_centralizou {
                super::navigation::centralizar_visualizacao(self, tamanho_disponivel);
                self.ja_centralizou = true;
            }

            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let response = ui.allocate_response(ui.available_size(), egui::Sense::click_and_drag());
                let rect_desenho = response.rect;
                let painter = ui.painter_at(rect_desenho);

                // Processar eventos de navegação
                if response.hovered() {
                    super::navigation::processar_eventos_navegacao(self, ui, &response, rect_desenho);
                }

                painter.rect_filled(rect_desenho, 0.0, Color32::from_gray(30));

                let grafo_clone = match &self.grafo_metro {
                    Some(grafo_arc) => grafo_arc.clone(),
                    None => return,
                };
                let grafo_ref = &*grafo_clone;
                
                super::drawing::desenhar_conexoes(self, &painter, rect_desenho, grafo_ref);
                super::drawing::desenhar_estacoes(self, &painter, rect_desenho, grafo_ref, ui);
                super::visual_effects::desenhar_marcadores_estacoes(self, &painter, rect_desenho, grafo_ref, ui);
                
                let acoes_popup = super::popups::desenhar_popups(self, ui, rect_desenho, grafo_ref);
                super::popups::processar_acoes_popup(self, acoes_popup);
            });

            let precisa_repaint = self.solucionador_a_estrela.is_some() || !self.vizinhos_sendo_analisados_ui.is_empty();
            if precisa_repaint {
                let tempo = ctx.input(|i| i.time) as f32;
                if tempo - self.ultimo_tempo_animacao > 0.16 {
                    self.ultimo_tempo_animacao = tempo;
                    ctx.request_repaint();
                }
            }
        });
    }
}
