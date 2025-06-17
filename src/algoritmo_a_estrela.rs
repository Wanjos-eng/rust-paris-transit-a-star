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
    pub fn debug_print(&self) -> String {
        format!("E{} (f={:.1}, g={:.1}, h={:.1})", 
                self.id_estacao + 1, 
                self.custo_f, 
                self.custo_g_viagem, 
                self.custo_f - self.custo_g_viagem)
    }

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

// Novo struct para detalhes da análise
#[derive(Debug, Clone)]
pub struct DetalhesAnalise {
    pub estacao_expandida: IdEstacao,
    pub vizinhos_analisados: Vec<String>,
    pub fronteira_atual: Vec<String>,
}

// Sistema de estados detalhados para visualização didática
#[derive(Debug, Clone, PartialEq)]
pub enum StatusEstacao {
    Disponivel,
    SelecionadaParaExpansao,    // Tirada da fronteira, vai ser expandida
    ExpandindoVizinhos,         // Analisando seus vizinhos
    Explorada,                  // Completamente processada
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
    pub ultima_analise: Option<DetalhesAnalise>, // Novo campo para armazenar detalhes da última análise
    pub status_estacoes: HashMap<IdEstacao, StatusEstacao>, // Status de cada estação
    pub estacao_sendo_explorada_no_momento: Option<IdEstacao>, // Estação que está sendo explorada neste momento
    pub passo_atual: usize, // Contador de passos para controle didático
    pub vizinhos_sendo_analisados: HashSet<IdEstacao>, // Vizinhos sendo analisados no passo atual
}

impl SolucionadorAEstrela {
    // PARTE 1: CONFIGURAÇÃO DA VIAGEM - Inicializa o algoritmo A*
    // Esta função é como abrir um app de mapas e definir origem/destino
    // Define origem, destino e prepara estruturas para busca sistemática
    pub fn novo(
        grafo_compartilhado: Arc<GrafoMetro>,
        id_inicio_param: IdEstacao,
        linha_inicial_opcional: Option<CorLinha>,
        id_objetivo_param: IdEstacao,
    ) -> Self {
        // Cria fronteira: lista ordenada de rotas parciais a serem analisadas
        // A fronteira sempre mantém as rotas mais promissoras no topo
        let mut fronteira_heap = BinaryHeap::new();
        let mut custos_g_map = HashMap::new();

        // Calcula estimativa inicial (heurística h): tempo estimado até destino
        // É como calcular "distância em linha reta" convertida para tempo
        let custo_h_inicial = grafo_compartilhado
            .obter_tempo_heuristico_minutos(id_inicio_param, id_objetivo_param)
            .unwrap_or(0.0);

        // Custo real de viagem (g): zero no ponto de partida
        let custo_g_viagem_inicial = 0.0;
        // Custo total estimado (f): soma do real + estimativa (f = g + h)
        let custo_f_inicial = custo_g_viagem_inicial + custo_h_inicial;

        // Inicializa o caminho percorrido com apenas a estação de partida
        let mut caminho_inicial = Vec::new();
        caminho_inicial.push(id_inicio_param);

        // Adiciona ponto de partida na fronteira como primeira rota a ser analisada
        // Esta é a única opção inicial para começar a busca
        fronteira_heap.push(EstadoNoFronteira {
            id_estacao: id_inicio_param,
            linha_chegada: linha_inicial_opcional,
            custo_f: custo_f_inicial,
            custo_g_viagem: custo_g_viagem_inicial,
            caminho: caminho_inicial, // Caminho inicial contém só a origem
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
            ultima_analise: None, // Inicializar como None
            status_estacoes: HashMap::new(), // Inicializar vazio
            estacao_sendo_explorada_no_momento: None, // Inicializar como None
            passo_atual: 0, // Inicializar contador de passos
            vizinhos_sendo_analisados: HashSet::new(), // Inicializar vazio
        }
    }

    // PARTE 2: BUSCA INTELIGENTE - Núcleo do algoritmo A* (versão didática)
    // Esta função explora sistematicamente as possibilidades para encontrar a rota mais eficiente
    // Executa um passo de análise de cada vez, priorizando rotas mais promissoras
    pub fn proximo_passo(&mut self) -> ResultadoPassoAEstrela {
        self.passo_atual += 1;
        println!("\n=== PASSO {} ===", self.passo_atual);
        
        // Continuar com o algoritmo normal
        if let Some(no_da_fronteira_atual) = self.fronteira.pop() {
            println!("SELECIONANDO: Estação E{} (f={:.1}, g={:.1}, h={:.1})", 
                     no_da_fronteira_atual.id_estacao + 1,
                     no_da_fronteira_atual.custo_f,
                     no_da_fronteira_atual.custo_g_viagem,
                     no_da_fronteira_atual.custo_f - no_da_fronteira_atual.custo_g_viagem);
            
            // CONDIÇÃO DE PARADA: Verificar se chegamos ao objetivo
            if no_da_fronteira_atual.id_estacao == self.id_objetivo {
                let info_caminho = self.criar_info_caminho_do_no(&no_da_fronteira_atual);
                return ResultadoPassoAEstrela::CaminhoEncontrado(info_caminho);
            }
            
            // Ignorar estações já exploradas
            if self.explorados.contains(&no_da_fronteira_atual.id_estacao) {
                println!("  Estação E{} já explorada, pulando.", no_da_fronteira_atual.id_estacao + 1);
                return ResultadoPassoAEstrela::EmProgresso;
            }
            
            // Atualizar status para "selecionada para expansão"
            self.status_estacoes.insert(no_da_fronteira_atual.id_estacao, StatusEstacao::SelecionadaParaExpansao);
            self.estacao_sendo_explorada_no_momento = Some(no_da_fronteira_atual.id_estacao);
            
            // Marcar como explorada
            self.explorados.insert(no_da_fronteira_atual.id_estacao);
            
            // Limpar vizinhos sendo analisados do passo anterior
            self.vizinhos_sendo_analisados.clear();
            
            // Prepara estruturas para armazenar detalhes da análise
            let mut vizinhos_analisados = Vec::new();
            let mut fronteira_atual = Vec::new();
            
            // EXPANSÃO: Analisa todas as estações vizinhas (conexões diretas)
            if let Some(conexoes) = self.grafo.lista_adjacencia.get(no_da_fronteira_atual.id_estacao) {
                for conexao in conexoes {
                    let id_vizinho = conexao.para_estacao;
                    
                    // Adicionar à lista de vizinhos sendo analisados
                    self.vizinhos_sendo_analisados.insert(id_vizinho);
                    
                    // Pula vizinhos já completamente explorados
                    if self.explorados.contains(&id_vizinho) {
                        println!("    Ignorando E{}: já explorado", id_vizinho + 1);
                        vizinhos_analisados.push(format!("E{}: já explorado", id_vizinho + 1));
                        continue;
                    }
                    
                    // Calcular custos para este vizinho
                    let custo_baldeacao = if let Some(linha_atual) = no_da_fronteira_atual.linha_chegada {
                        if linha_atual != conexao.cor_linha {
                            println!("      Adicionando custo de baldeação: +{}min", TEMPO_BALDEACAO_MINUTOS);
                            TEMPO_BALDEACAO_MINUTOS
                        } else {
                            0.0
                        }
                    } else {
                        0.0
                    };
                    
                    let custo_g_novo = no_da_fronteira_atual.custo_g_viagem + conexao.tempo_minutos + custo_baldeacao;
                    let custo_h = self.grafo.obter_tempo_heuristico_minutos(id_vizinho, self.id_objetivo)
                        .unwrap_or(0.0);
                    let custo_f = custo_g_novo + custo_h;
                    
                    println!("      Analisando E{}: g={:.1}, h={:.1}, f={:.1}", 
                             id_vizinho + 1, custo_g_novo, custo_h, custo_f);
                    
                    // Verificar se já existe um caminho melhor
                    let mut ja_tem_melhor_caminho = false;
                    
                    if let Some(&custo_g_registrado) = self.custos_g_viagem_mapa.get(&id_vizinho) {
                        if custo_g_registrado <= custo_g_novo {
                            ja_tem_melhor_caminho = true;
                        }
                    }
                    
                    if !ja_tem_melhor_caminho {
                        for no_fronteira in self.fronteira.iter() {
                            if no_fronteira.id_estacao == id_vizinho && no_fronteira.custo_g_viagem <= custo_g_novo {
                                ja_tem_melhor_caminho = true;
                                break;
                            }
                        }
                    }
                    
                    if !ja_tem_melhor_caminho {
                        // Registrar este novo caminho
                        self.custos_g_viagem_mapa.insert(id_vizinho, custo_g_novo);
                        self.predecessores_info.insert(
                            id_vizinho, 
                            (no_da_fronteira_atual.id_estacao, no_da_fronteira_atual.linha_chegada, conexao.cor_linha)
                        );
                        
                        // Criar novo caminho
                        let mut novo_caminho = no_da_fronteira_atual.caminho.clone();
                        novo_caminho.push(id_vizinho);
                        
                        // Adicionar na fronteira
                        let novo_no = EstadoNoFronteira {
                            id_estacao: id_vizinho,
                            linha_chegada: Some(conexao.cor_linha),
                            custo_f: custo_f,
                            custo_g_viagem: custo_g_novo,
                            caminho: novo_caminho,
                        };
                        
                        self.fronteira.push(novo_no);
                        vizinhos_analisados.push(format!("E{}: g={:.1}, h={:.1}, f={:.1} - ADICIONADO", 
                                                         id_vizinho + 1, custo_g_novo, custo_h, custo_f));
                    } else {
                        vizinhos_analisados.push(format!("E{}: já tem caminho melhor", id_vizinho + 1));
                    }
                }
            }
            
            // Atualizar status visual para "expandindo vizinhos"
            self.status_estacoes.insert(no_da_fronteira_atual.id_estacao, StatusEstacao::ExpandindoVizinhos);
            
            // Capturar estado atual da fronteira
            let mut nodes_fronteira: Vec<_> = self.fronteira.iter().collect();
            nodes_fronteira.sort_by(|a, b| a.custo_f.partial_cmp(&b.custo_f)
                .unwrap_or(std::cmp::Ordering::Equal));
            
            for node in nodes_fronteira.iter().take(5) {
                fronteira_atual.push(format!("E{}: f={:.1}", node.id_estacao + 1, node.custo_f));
            }
            
            // Armazenar detalhes da análise
            self.ultima_analise = Some(DetalhesAnalise {
                estacao_expandida: no_da_fronteira_atual.id_estacao,
                vizinhos_analisados,
                fronteira_atual,
            });
            
            self.debug_print_fronteira();
            return ResultadoPassoAEstrela::EmProgresso;
        }
        
        ResultadoPassoAEstrela::NenhumCaminhoPossivel
    }

    // PARTE 3: APRESENTAÇÃO DO RESULTADO - Constrói o itinerário final detalhado
    // Esta função é chamada quando o destino é alcançado
    // Reconstrói o caminho encontrado e calcula tempo total e baldeações
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
        
        // Primeira estação não tem linha de chegada
        if !no_final.caminho.is_empty() {
            estacoes_com_linhas.push((no_final.caminho[0], None));
        }
        
        // Processa cada trecho do caminho para calcular tempos e identificar baldeações
        for i in 1..no_final.caminho.len() {
            let id_estacao_atual = no_final.caminho[i];
            let id_estacao_anterior = no_final.caminho[i-1];
            
            let mut linha_usada: Option<CorLinha> = None;
            #[allow(unused_assignments)]
            let mut tempo_conexao = 0.0;
            
            println!("  {}: E{} -> E{} verificando conexão direta...",
                   i, id_estacao_anterior + 1, id_estacao_atual + 1);
            
            // Busca a conexão específica entre as duas estações
            if let Some(conexoes) = self.grafo.lista_adjacencia.get(id_estacao_anterior) {
                for conexao in conexoes {
                    if conexao.para_estacao == id_estacao_atual {
                        linha_usada = Some(conexao.cor_linha);
                        tempo_conexao = conexao.tempo_minutos;
                        
                        tempo_total += tempo_conexao;
                        
                        // Verifica se houve mudança de linha (baldeação)
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
        
        // Formata e exibe o resultado final
        let horas = (tempo_total as i32) / 60;
        let minutos = (tempo_total as i32) % 60;
        println!("Número de baldeações: {}", baldeacoes);
        println!("Tempo total: {} h {} min ({:.1} min)", horas, minutos, tempo_total);
        
        // Retorna estrutura com informações completas do itinerário
        InfoCaminho {
            estacoes_do_caminho: estacoes_com_linhas,
            tempo_total_minutos: tempo_total,
            baldeacoes,
        }
    }

    // Função auxiliar para depuração da fronteira atual
    pub fn debug_print_fronteira(&self) {
        println!("\nFRONTEIRA ATUAL (ordenada por f-cost crescente):");
        
        // Ordena nós por f-cost para entender as decisões do algoritmo
        let mut nodes: Vec<_> = self.fronteira.iter().collect();
        nodes.sort_by(|a, b| a.custo_f.partial_cmp(&b.custo_f)
            .unwrap_or(Ordering::Equal));
        
        for (idx, node) in nodes.iter().enumerate().take(10) {  // Mostra apenas os 10 melhores
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

    // Método especial para verificar o caminho direto
    pub fn check_direct_path(&self) {
        println!("\nVERIFICANDO CAMINHO DIRETO E6 -> E5 -> E4 -> E13:");
        let path = vec![5, 4, 3, 12]; // E6->E5->E4->E13 (índices base-zero)
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
    
    // Obtém status de uma estação
    pub fn obter_status_estacao(&self, id_estacao: IdEstacao) -> StatusEstacao {
        self.status_estacoes.get(&id_estacao).cloned().unwrap_or(StatusEstacao::Disponivel)
    }
}