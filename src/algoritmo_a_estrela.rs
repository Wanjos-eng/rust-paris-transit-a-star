// src/algoritmo_a_estrela.rs

use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;
use std::sync::Arc;

use crate::grafo_metro::{GrafoMetro, IdEstacao, CorLinha, TEMPO_BALDEACAO_MINUTOS};

#[derive(Debug, Clone)]
pub struct EstadoNoFronteira {
    pub id_estacao: IdEstacao,
    pub linha_chegada: Option<CorLinha>,
    pub custo_f: f32,
    pub custo_g_viagem: f32, 
    pub caminho: Vec<IdEstacao>, // Adicionando o caminho percorrido até este nó
}

impl EstadoNoFronteira {
    // Helper method for debugging
    pub fn debug_print(&self) -> String {
        format!("E{} (f={:.1}, g={:.1}, h={:.1})", 
                self.id_estacao + 1, 
                self.custo_f, 
                self.custo_g_viagem, 
                self.custo_f - self.custo_g_viagem)
    }

    // Add enhanced debugging information
    pub fn debug_full(&self) -> String {
        let caminho_str = self.caminho.iter()
            .map(|&id| format!("E{}", id+1))
            .collect::<Vec<_>>()
            .join(" -> ");
        
        format!("Estado: {}, Caminho: {}", self.debug_print(), caminho_str)
    }
}

// Implementações de PartialEq, Eq, Ord, PartialOrd para EstadoNoFronteira
impl PartialEq for EstadoNoFronteira {
    fn eq(&self, other: &Self) -> bool {
        self.id_estacao == other.id_estacao &&
        self.linha_chegada == other.linha_chegada &&
        (self.custo_f - other.custo_f).abs() < f32::EPSILON &&
        (self.custo_g_viagem - other.custo_g_viagem).abs() < f32::EPSILON
    }
}
impl Eq for EstadoNoFronteira {}
impl Ord for EstadoNoFronteira {
    fn cmp(&self, other: &Self) -> Ordering {
        // Implementação min-heap direta, como no exemplo - menor custo_f tem maior prioridade
        match other.custo_f.partial_cmp(&self.custo_f) {
            Some(order) => order,
            None => Ordering::Equal
        }
    }
}
impl PartialOrd for EstadoNoFronteira {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
pub struct InfoCaminho {
    pub estacoes_do_caminho: Vec<(IdEstacao, Option<CorLinha>)>, 
    pub tempo_total_minutos: f32,
    pub baldeacoes: u32,
}

#[derive(Debug)]
pub enum ResultadoPassoAEstrela {
    EmProgresso,
    CaminhoEncontrado(InfoCaminho),
    NenhumCaminhoPossivel,
    Erro(String),
}

#[derive(Debug)]
pub struct SolucionadorAEstrela {
    grafo: Arc<GrafoMetro>,
    id_inicio: IdEstacao,
    linha_de_partida_busca: Option<CorLinha>,
    id_objetivo: IdEstacao,
    pub fronteira: BinaryHeap<EstadoNoFronteira>,
    pub explorados: HashSet<IdEstacao>, 
    custos_g_viagem_mapa: HashMap<IdEstacao, f32>, 
    predecessores_info: HashMap<IdEstacao, (IdEstacao, Option<CorLinha>, CorLinha)>,
}

impl SolucionadorAEstrela {
    pub fn novo(
        grafo_compartilhado: Arc<GrafoMetro>,
        id_inicio_param: IdEstacao,
        linha_inicial_opcional: Option<CorLinha>,
        id_objetivo_param: IdEstacao,
    ) -> Self {
        let mut fronteira_heap = BinaryHeap::new();
        let mut custos_g_map = HashMap::new();

        let custo_h_inicial = grafo_compartilhado
            .obter_tempo_heuristico_minutos(id_inicio_param, id_objetivo_param)
            .unwrap_or(0.0);

        let custo_g_viagem_inicial = 0.0;
        let custo_f_inicial = custo_g_viagem_inicial + custo_h_inicial;

        // Inicializa o caminho com a estação inicial
        let mut caminho_inicial = Vec::new();
        caminho_inicial.push(id_inicio_param);

        fronteira_heap.push(EstadoNoFronteira {
            id_estacao: id_inicio_param,
            linha_chegada: linha_inicial_opcional,
            custo_f: custo_f_inicial,
            custo_g_viagem: custo_g_viagem_inicial,
            caminho: caminho_inicial, // Inicializando o caminho
        });
        
        custos_g_map.insert(id_inicio_param, custo_g_viagem_inicial);

        Self {
            grafo: grafo_compartilhado,
            id_inicio: id_inicio_param,
            linha_de_partida_busca: linha_inicial_opcional,
            id_objetivo: id_objetivo_param,
            fronteira: fronteira_heap,
            explorados: HashSet::new(),
            custos_g_viagem_mapa: custos_g_map,
            predecessores_info: HashMap::new(),
        }
    }

    pub fn proximo_passo(&mut self) -> ResultadoPassoAEstrela {
        if let Some(no_da_fronteira_atual) = self.fronteira.pop() {
            println!("EXPANDINDO: Estação E{} (f={:.1}, g={:.1}, h={:.1})", 
                     no_da_fronteira_atual.id_estacao + 1,
                     no_da_fronteira_atual.custo_f,
                     no_da_fronteira_atual.custo_g_viagem,
                     no_da_fronteira_atual.custo_f - no_da_fronteira_atual.custo_g_viagem);
            
            // Verificamos se chegamos ao objetivo
            if no_da_fronteira_atual.id_estacao == self.id_objetivo {
                // Caminho encontrado! Reconstruímos o caminho e retornamos
                let info_caminho = self.criar_info_caminho_do_no(&no_da_fronteira_atual);
                return ResultadoPassoAEstrela::CaminhoEncontrado(info_caminho);
            }
            
            // Similar ao exemplo, ignoramos estações já exploradas
            if self.explorados.contains(&no_da_fronteira_atual.id_estacao) {
                println!("  Estação E{} já explorada, pulando.", no_da_fronteira_atual.id_estacao + 1);
                return ResultadoPassoAEstrela::EmProgresso;
            }
            
            // Marcamos como explorada
            self.explorados.insert(no_da_fronteira_atual.id_estacao);
            
            println!("  Explorando conexões da estação E{}", no_da_fronteira_atual.id_estacao + 1);
            
            // Recuperamos as conexões da estação atual
            if let Some(conexoes) = self.grafo.lista_adjacencia.get(no_da_fronteira_atual.id_estacao) {
                for conexao in conexoes {
                    let id_vizinho = conexao.para_estacao;
                    
                    // Não exploramos vizinhos já visitados
                    if self.explorados.contains(&id_vizinho) {
                        println!("    Ignorando E{}: já explorado", id_vizinho + 1);
                        continue;
                    }
                    
                    // Adicionar informação de direção para depuração
                    let heuristica_atual = self.grafo.obter_tempo_heuristico_minutos(
                        no_da_fronteira_atual.id_estacao, self.id_objetivo).unwrap_or(0.0);
                    let heuristica_vizinho = self.grafo.obter_tempo_heuristico_minutos(
                        id_vizinho, self.id_objetivo).unwrap_or(0.0);
                    let direcao = if heuristica_vizinho < heuristica_atual {
                        "APROXIMANDO"
                    } else if heuristica_vizinho > heuristica_atual {
                        "AFASTANDO"
                    } else {
                        "LATERAL"
                    };
                    
                    println!("    Analisando E{} via linha {:?} ({} do objetivo)", 
                             id_vizinho + 1, conexao.cor_linha, direcao);
                    
                    // Calcular custo de baldeação se necessário
                    let custo_baldeacao = if let Some(linha_atual) = no_da_fronteira_atual.linha_chegada {
                        if linha_atual != conexao.cor_linha {
                            println!("      Adicionando custo de baldeação: +{}min", TEMPO_BALDEACAO_MINUTOS);
                            TEMPO_BALDEACAO_MINUTOS
                        } else {
                            0.0
                        }
                    } else {
                        0.0 // Primeira estação não tem baldeação
                    };
                    
                    // CORREÇÃO: Aqui está o erro! Precisamos usar o valor correto do tempo de viagem
                    let custo_g_novo = no_da_fronteira_atual.custo_g_viagem + conexao.tempo_minutos + custo_baldeacao;
                    let custo_h = self.grafo.obter_tempo_heuristico_minutos(id_vizinho, self.id_objetivo)
                        .unwrap_or(0.0);
                    let custo_f = custo_g_novo + custo_h;
                    
                    println!("      Custos: g_acumulado={:.1} + viagem={:.1} + baldeação={:.1} = {:.1}, h={:.1}, f={:.1}", 
                             no_da_fronteira_atual.custo_g_viagem, // Valor acumulado anterior
                             conexao.tempo_minutos,               // Tempo da conexão atual
                             custo_baldeacao,                     // Custo de baldeação, se houver
                             custo_g_novo,                        // Novo valor acumulado
                             custo_h,
                             custo_f);
                    
                    // Verificar se já temos um caminho melhor para esta estação na fronteira
                    let mut ja_tem_melhor_caminho = false;
                    for no_fronteira in self.fronteira.iter() {
                        if no_fronteira.id_estacao == id_vizinho && no_fronteira.custo_g_viagem <= custo_g_novo {
                            ja_tem_melhor_caminho = true;
                            println!("      Já existe um caminho melhor na fronteira com g={:.1}", no_fronteira.custo_g_viagem);
                            break;
                        }
                    }
                    
                    if let Some(&custo_g_registrado) = self.custos_g_viagem_mapa.get(&id_vizinho) {
                        if custo_g_registrado <= custo_g_novo {
                            ja_tem_melhor_caminho = true;
                            println!("      Já existe um caminho melhor registrado com g={:.1}", custo_g_registrado);
                        }
                    }
                    
                    if !ja_tem_melhor_caminho {
                        // Atualizamos o mapa de custos g para uso na reconstrução do caminho
                        self.custos_g_viagem_mapa.insert(id_vizinho, custo_g_novo);
                        
                        // Guardamos informação do predecessor 
                        self.predecessores_info.insert(
                            id_vizinho, 
                            (no_da_fronteira_atual.id_estacao, no_da_fronteira_atual.linha_chegada, conexao.cor_linha)
                        );
                        
                        // Criamos um novo caminho adicionando o vizinho atual
                        let mut novo_caminho = no_da_fronteira_atual.caminho.clone();
                        novo_caminho.push(id_vizinho);
                        
                        // Mostrar o caminho completo até agora para depuração
                        let caminho_str = novo_caminho.iter()
                            .map(|&id| format!("E{}", id+1))
                            .collect::<Vec<_>>()
                            .join(" -> ");
                            
                        println!("      Novo caminho: {} (custo g={:.1})", caminho_str, custo_g_novo);
                        
                        // Criamos um novo nó para a fronteira
                        let novo_no = EstadoNoFronteira {
                            id_estacao: id_vizinho,
                            linha_chegada: Some(conexao.cor_linha),
                            custo_f: custo_f,
                            custo_g_viagem: custo_g_novo,
                            caminho: novo_caminho,
                        };
                        
                        println!("      Adicionando à fronteira: E{} (f={:.1}, g={:.1}, h={:.1})", 
                                id_vizinho + 1, custo_f, custo_g_novo, custo_h);
                        self.fronteira.push(novo_no);
                    }
                }
            }
            
            // Depois de processar, mostramos a fronteira atualizada
            self.debug_print_fronteira();
            
            ResultadoPassoAEstrela::EmProgresso
        } else {
            ResultadoPassoAEstrela::NenhumCaminhoPossivel
        }
    }

    // Novo método para criar InfoCaminho diretamente do nó final
    fn criar_info_caminho_do_no(&self, no_final: &EstadoNoFronteira) -> InfoCaminho {
        let mut estacoes_com_linhas = Vec::new();
        let mut tempo_total = 0.0;
        let mut baldeacoes = 0;
        let _linha_atual: Option<CorLinha> = self.linha_de_partida_busca;
        
        println!("\nDETALHES DO CAMINHO ENCONTRADO:");
        let caminho_str = no_final.caminho.iter()
            .map(|&id| format!("E{}", id+1))
            .collect::<Vec<_>>()
            .join(" -> ");
        println!("Caminho: {}", caminho_str);
        
        if !no_final.caminho.is_empty() {
            estacoes_com_linhas.push((no_final.caminho[0], None));
        }
        
        for i in 1..no_final.caminho.len() {
            let id_estacao_atual = no_final.caminho[i];
            let id_estacao_anterior = no_final.caminho[i-1];
            
            let mut linha_usada: Option<CorLinha> = None;
            // Tempo da conexão será preenchido ao encontrar a conexão correta
            #[allow(unused_assignments)]
            let mut tempo_conexao = 0.0;
            
            println!("  {}: E{} -> E{} verificando conexão direta...",
                   i, id_estacao_anterior + 1, id_estacao_atual + 1);
            
            if let Some(conexoes) = self.grafo.lista_adjacencia.get(id_estacao_anterior) {
                for conexao in conexoes {
                    if conexao.para_estacao == id_estacao_atual {
                        linha_usada = Some(conexao.cor_linha);
                        tempo_conexao = conexao.tempo_minutos;
                        
                        tempo_total += tempo_conexao;
                        
                        if i > 1 {
                            let linha_anterior = estacoes_com_linhas[i-1].1;
                            if linha_anterior != linha_usada {
                                baldeacoes += 1;
                                tempo_total += TEMPO_BALDEACAO_MINUTOS;
                                println!("  Baldeação em E{}: {:?} -> {:?} (+{}min)",
                                       id_estacao_anterior + 1, linha_anterior, linha_usada, TEMPO_BALDEACAO_MINUTOS);
                            }
                        }
                        
                        println!("    Encontrada conexão direta: via linha {:?}, tempo={:.1}min",
                               linha_usada.unwrap_or(CorLinha::Nenhuma), tempo_conexao);
                        
                        println!("  E{} -> E{} | Linha: {:?} | Tempo: {:.1}min | Total: {:.1}min",
                               id_estacao_anterior + 1, id_estacao_atual + 1, 
                               linha_usada.unwrap_or(CorLinha::Nenhuma), 
                               tempo_conexao, tempo_total);
                        
                        break;
                    }
                }
            } else {
                println!("  ERRO: Nenhuma conexão encontrada de E{} para E{}!",
                       id_estacao_anterior + 1, id_estacao_atual + 1);
            }
            
            estacoes_com_linhas.push((id_estacao_atual, linha_usada));
        }
        
        let horas = (tempo_total as i32) / 60;
        let minutos = (tempo_total as i32) % 60;
        println!("Número de baldeações: {}", baldeacoes);
        println!("Tempo total: {} h {} min ({:.1} min)", horas, minutos, tempo_total);
        
        InfoCaminho {
            estacoes_do_caminho: estacoes_com_linhas,
            tempo_total_minutos: tempo_total,
            baldeacoes,
        }
    }

    // Debugging helper to print the current frontier
    pub fn debug_print_fronteira(&self) {
        println!("\nFRONTEIRA ATUAL (ordenada por f-cost crescente):");
        
        // Sort nodes by f-cost to understand the algorithm's decisions
        let mut nodes: Vec<_> = self.fronteira.iter().collect();
        nodes.sort_by(|a, b| a.custo_f.partial_cmp(&b.custo_f)
            .unwrap_or(Ordering::Equal));
        
        for (idx, node) in nodes.iter().enumerate().take(10) {  // Show just top 10 entries
            let caminho_str = node.caminho.iter()
                .map(|&id| format!("E{}", id+1))
                .collect::<Vec<_>>()
                .join(" -> ");
                
            println!("  {}. f={:.1} g={:.1} h={:.1} | E{} | {}",
                     idx + 1, 
                     node.custo_f,
                     node.custo_g_viagem,
                     node.custo_f - node.custo_g_viagem,
                     node.id_estacao + 1,
                     caminho_str);
        }
        
        if nodes.len() > 10 {
            println!("  ... e mais {} nós", nodes.len() - 10);
        }
        println!();
    }

    // Special method to verify the direct path
    pub fn check_direct_path(&self) {
        println!("\nVERIFICANDO CAMINHO DIRETO E6 -> E5 -> E4 -> E13:");
        let path = vec![5, 4, 3, 12]; // E6->E5->E4->E13 (zero-based indices)
        let mut total_time = 0.0;
        let mut transfers = 0;
        let mut last_line: Option<CorLinha> = None;
        
        for i in 0..(path.len()-1) {
            let from = path[i];
            let to = path[i+1];
            let edge = self.grafo.lista_adjacencia[from].iter()
                .find(|c| c.para_estacao == to);
            
            if let Some(conn) = edge {
                total_time += conn.tempo_minutos;
                
                if let Some(prev_line) = last_line {
                    if prev_line != conn.cor_linha {
                        transfers += 1;
                        total_time += TEMPO_BALDEACAO_MINUTOS;
                        println!("  Baldeação em E{}: {:?} -> {:?} (+4min)",
                                 from+1, prev_line, conn.cor_linha);
                    }
                }
                
                println!("  E{} -> E{}: Linha {:?}, {:.1}min (total: {:.1}min)",
                         from+1, to+1, conn.cor_linha, conn.tempo_minutos, total_time);
                
                last_line = Some(conn.cor_linha);
            } else {
                println!("  ERRO: Conexão não encontrada entre E{} e E{}", from+1, to+1);
            }
        }
        
        println!("Caminho E6 -> E5 -> E4 -> E13:");
        println!("  Tempo total: {:.1}min ({} h {} min)", 
                 total_time, 
                 (total_time as i32) / 60, 
                 (total_time as i32) % 60);
        println!("  Baldeações: {}", transfers);
    }

    // Adicionar um método para validar a rota específica
    pub fn validar_rota_especifica(&self) {
        // Testar especificamente a rota E6 -> E5 -> E4 -> E13
        let estacoes = [5, 4, 3, 12]; // Índices zero-based 
        let mut tempo_total = 0.0;
        let mut baldeacoes = 0;
        let mut linha_anterior: Option<CorLinha> = None;
        
        println!("\n=== VALIDAÇÃO DA ROTA E6 -> E5 -> E4 -> E13 ===");
        
        for i in 0..estacoes.len()-1 {
            let id_origem = estacoes[i];
            let id_destino = estacoes[i+1];
            
            // Encontra a conexão correta
            let mut conexao_tempo = 0.0;
            let mut cor_linha = CorLinha::Nenhuma;
            let mut encontrou = false;
            
            if let Some(conexoes) = self.grafo.lista_adjacencia.get(id_origem) {
                for conexao in conexoes {
                    if conexao.para_estacao == id_destino {
                        conexao_tempo = conexao.tempo_minutos;
                        cor_linha = conexao.cor_linha;
                        encontrou = true;
                        break;
                    }
                }
            }
            
            if encontrou {
                tempo_total += conexao_tempo;
                
                // Verificar baldeação
                if let Some(linha) = linha_anterior {
                    if linha != cor_linha {
                        baldeacoes += 1;
                        tempo_total += TEMPO_BALDEACAO_MINUTOS;
                        println!("  Baldeação em E{}: {:?} -> {:?} (+{}min)",
                               id_origem + 1, linha, cor_linha, TEMPO_BALDEACAO_MINUTOS);
                    }
                }
                
                println!("  E{} -> E{}: Linha {:?}, Tempo={:.1}min, Total acumulado={:.1}min",
                       id_origem + 1, id_destino + 1, cor_linha, conexao_tempo, tempo_total);
                
                linha_anterior = Some(cor_linha);
            } else {
                println!("  ERRO: Não existe conexão direta de E{} para E{}!",
                       id_origem + 1, id_destino + 1);
            }
        }
        
        println!("TEMPO TOTAL: {:.1}min ({} h {} min)", 
               tempo_total, 
               (tempo_total as i32) / 60, 
               (tempo_total as i32) % 60);
        println!("BALDEAÇÕES: {}", baldeacoes);
        println!("=================================================\n");
    }
    
    // Chamar este método de validação durante a inicialização
    pub fn verificar_dados(&self) {
        self.validar_rota_especifica();
    }
}