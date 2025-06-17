use std::sync::Arc;
use crate::algoritmo_a_estrela::SolucionadorAEstrela;
use super::app::MinhaAplicacaoGUI;

/// Limpa todos os estados visuais do algoritmo
pub fn limpar_estado_visual(app: &mut MinhaAplicacaoGUI) {
    app.resultado_caminho_ui = None;
    app.estacao_sendo_expandida_ui = None;
    app.estacoes_exploradas_ui.clear();
    app.detalhes_analise_ui.clear();
    app.vizinhos_sendo_analisados_ui.clear();
    app.solucionador_a_estrela = None;
}

/// Inicia uma nova busca A*
pub fn iniciar_busca_a_estrela(app: &mut MinhaAplicacaoGUI) {
    if let Some(ref grafo) = app.grafo_metro {
        let grafo_arco = Arc::clone(grafo);
        let id_inicio = app.id_estacao_inicio_selecionada;
        let id_objetivo = app.id_estacao_objetivo_selecionada;
        
        // Extrair nomes das estações antes de limpar o estado
        let nome_inicio = grafo.estacoes[id_inicio].nome.clone();
        let nome_objetivo = grafo.estacoes[id_objetivo].nome.clone();
        
        // Resetar estado
        limpar_estado_visual(app);
        
        // Criar o solucionador
        let solucionador = SolucionadorAEstrela::novo(
            grafo_arco,
            id_inicio,
            app.linha_inicio_opcional,
            id_objetivo
        );
        
        app.solucionador_a_estrela = Some(solucionador);
        app.mensagem_status_ui = format!(
            "Busca iniciada: De {} para {}", 
            nome_inicio, 
            nome_objetivo
        );
    } else {
        app.mensagem_status_ui = "Erro: Grafo não carregado.".to_string();
    }
}

/// Executa o próximo passo do algoritmo A*
pub fn executar_proximo_passo_a_estrela(app: &mut MinhaAplicacaoGUI) {
    let resultado = if let Some(ref mut solucionador) = app.solucionador_a_estrela {
        Some(solucionador.proximo_passo())
    } else {
        None
    };
    
    if let Some(resultado) = resultado {
        match resultado {
            crate::algoritmo_a_estrela::ResultadoPassoAEstrela::EmProgresso => {
                let analise_dados = if let Some(ref solucionador) = app.solucionador_a_estrela {
                    solucionador.ultima_analise.clone()
                } else {
                    None
                };
                
                if let Some(analise) = analise_dados {
                    processar_passo_em_progresso_dados(app, &analise);
                }
            },
            crate::algoritmo_a_estrela::ResultadoPassoAEstrela::CaminhoEncontrado(caminho_info) => {
                processar_caminho_encontrado(app, caminho_info);
            },
            crate::algoritmo_a_estrela::ResultadoPassoAEstrela::NenhumCaminhoPossivel => {
                processar_nenhum_caminho(app);
            },
            crate::algoritmo_a_estrela::ResultadoPassoAEstrela::Erro(msg) => {
                processar_erro(app, msg);
            }
        }
    } else {
        app.mensagem_status_ui = "Erro: Nenhuma busca em andamento.".to_string();
    }
}

fn processar_passo_em_progresso_dados(app: &mut MinhaAplicacaoGUI, analise: &crate::algoritmo_a_estrela::DetalhesAnalise) {
        // A estação que foi expandida
        app.estacao_sendo_expandida_ui = Some(analise.estacao_expandida);
        
        // Adicionar às exploradas
        app.estacoes_exploradas_ui.insert(analise.estacao_expandida);
        
        // Extrair vizinhos sendo analisados
        app.vizinhos_sendo_analisados_ui.clear();
        for vizinho_info in &analise.vizinhos_analisados {
            if let Some(id_estacao) = extrair_id_estacao_de_info(vizinho_info) {
                if !app.estacoes_exploradas_ui.contains(&id_estacao) {
                    app.vizinhos_sendo_analisados_ui.insert(id_estacao);
                }
            }
        }
        
        // Atualizar detalhes da análise
        app.detalhes_analise_ui = analise.vizinhos_analisados.clone();
        
        let nome_estacao = if let Some(ref grafo) = app.grafo_metro {
            &grafo.estacoes[analise.estacao_expandida].nome
        } else {
            "Desconhecida"
        };
        
        app.mensagem_status_ui = format!(
            "Expandindo {} (E{}) - Analisando vizinhos",
            nome_estacao,
            analise.estacao_expandida + 1
        );
}

fn processar_caminho_encontrado(app: &mut MinhaAplicacaoGUI, caminho_info: crate::algoritmo_a_estrela::InfoCaminho) {
    app.resultado_caminho_ui = Some(caminho_info.clone());
    
    // Limpar estados temporários
    app.estacao_sendo_expandida_ui = None;
    app.vizinhos_sendo_analisados_ui.clear();
    
    // Marcar estações do caminho final como exploradas
    app.estacoes_exploradas_ui.clear();
    for (id_estacao, _) in &caminho_info.estacoes_do_caminho {
        app.estacoes_exploradas_ui.insert(*id_estacao);
    }
    
    app.mensagem_status_ui = format!(
        "✅ Caminho encontrado! Tempo: {:.1} min, Baldeações: {}",
        caminho_info.tempo_total_minutos,
        caminho_info.baldeacoes
    );
    app.solucionador_a_estrela = None;
}

fn processar_nenhum_caminho(app: &mut MinhaAplicacaoGUI) {
    app.mensagem_status_ui = "❌ Não foi possível encontrar um caminho.".to_string();
    app.estacao_sendo_expandida_ui = None;
    app.vizinhos_sendo_analisados_ui.clear();
    app.solucionador_a_estrela = None;
}

fn processar_erro(app: &mut MinhaAplicacaoGUI, msg: String) {
    app.mensagem_status_ui = format!("❌ Erro: {}", msg);
    app.estacao_sendo_expandida_ui = None;
    app.vizinhos_sendo_analisados_ui.clear();
    app.solucionador_a_estrela = None;
}

/// Extrai o ID da estação de uma string de informação
fn extrair_id_estacao_de_info(vizinho_info: &str) -> Option<usize> {
    if let Some(inicio_e) = vizinho_info.find('E') {
        if let Some(pos_dois_pontos) = vizinho_info.find(':') {
            if pos_dois_pontos > inicio_e + 1 {
                let numero_str = &vizinho_info[inicio_e + 1..pos_dois_pontos];
                if let Ok(id_estacao_um_baseado) = numero_str.parse::<usize>() {
                    if id_estacao_um_baseado > 0 {
                        return Some(id_estacao_um_baseado - 1); // Converter para zero-based
                    }
                }
            }
        }
    }
    None
}

/// Atualiza o estado visual da GUI com base no solucionador atual
pub fn atualizar_estado_visual_do_solucionador(app: &mut MinhaAplicacaoGUI) {
    if let Some(ref solucionador) = app.solucionador_a_estrela {
        // Atualizar estações exploradas
        app.estacoes_exploradas_ui.clear();
        for (id_estacao, status) in &solucionador.status_estacoes {
            match status {
                crate::algoritmo_a_estrela::StatusEstacao::Explorada => {
                    app.estacoes_exploradas_ui.insert(*id_estacao);
                },
                _ => {}
            }
        }
        
        // Atualizar estação sendo explorada no momento
        app.estacao_sendo_expandida_ui = solucionador.estacao_sendo_explorada_no_momento;
        
        // Atualizar vizinhos sendo analisados
        app.vizinhos_sendo_analisados_ui = solucionador.vizinhos_sendo_analisados.clone();
        
        // Atualizar detalhes da análise se disponível
        if let Some(ref analise) = solucionador.ultima_analise {
            app.detalhes_analise_ui = analise.vizinhos_analisados.clone();
        } else {
            app.detalhes_analise_ui.clear();
        }
    }
}
