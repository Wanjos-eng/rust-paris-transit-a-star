// src/algoritmo_a_estrela.rs

use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;
use std::sync::Arc;

use crate::grafo_metro::{GrafoMetro, IdEstacao, CorLinha, TEMPO_BALDEACAO_MINUTOS, Estacao, Conexao};

#[derive(Debug, Clone)] // Removido PartialEq daqui, implementaremos manualmente
pub struct EstadoNoFronteira {
    pub id_estacao: IdEstacao,
    pub linha_chegada: Option<CorLinha>,
    pub custo_f: f32,
    pub custo_g: f32,
}

// Implementação de PartialEq manualmente
impl PartialEq for EstadoNoFronteira {
    fn eq(&self, other: &Self) -> bool {
        // Compara os campos relevantes. Para floats, a igualdade direta é usada aqui.
        // Se precisássemos de comparação de float com epsilon, seria mais complexo.
        // Para o BinaryHeap, a ordenação (Ord) é mais crítica.
        self.id_estacao == other.id_estacao &&
        self.linha_chegada == other.linha_chegada &&
        self.custo_f == other.custo_f && // Cuidado com comparações diretas de float se NaN for possível
        self.custo_g == other.custo_g
    }
}

// Eq pode ser implementado se PartialEq for implementado e não houver NaNs
// ou se a lógica de igualdade for reflexiva, simétrica e transitiva.
// Como estamos evitando NaNs nos custos, podemos declarar Eq.
impl Eq for EstadoNoFronteira {}

impl Ord for EstadoNoFronteira {
    fn cmp(&self, other: &Self) -> Ordering {
        // Queremos uma min-heap (menor custo_f tem maior prioridade),
        // então invertemos a comparação para custo_f.
        match other.custo_f.partial_cmp(&self.custo_f) {
            Some(Ordering::Equal) => {
                // Se custo_f é igual, podemos desempatar por custo_g (menor é melhor)
                // ou outro critério para estabilidade/determinismo.
                self.custo_g.partial_cmp(&other.custo_g)
                    .unwrap_or(Ordering::Equal)
                    // Desempate final por ID para garantir uma ordem consistente
                    .then_with(|| self.id_estacao.cmp(&other.id_estacao))
            }
            Some(order) => order,
            None => Ordering::Equal, // Trata o caso de NaN, embora não esperemos NaN aqui.
                                     // Ou poderia ser Ordering::Less ou Ordering::Greater dependendo da política.
                                     // Para BinaryHeap, é importante que seja consistente.
        }
    }
}

impl PartialOrd for EstadoNoFronteira {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// ... (resto do código de algoritmo_a_estrela.rs, incluindo InfoCaminho, ResultadoPassoAEstrela, SolucionadorAEstrela) ...
// A struct SolucionadorAEstrela e sua implementação permanecem como antes.
// A mudança principal foi na struct EstadoNoFronteira e suas implementações de traits.


/// Guarda informações sobre o caminho encontrado.
#[derive(Debug, Clone)]
pub struct InfoCaminho {
    pub estacoes_do_caminho: Vec<(IdEstacao, Option<CorLinha>)>, // (estacao, linha_de_chegada_na_estacao)
    pub tempo_total_minutos: f32,
    pub baldeacoes: u32,
}

/// Representa o resultado de um único passo do algoritmo A*.
#[derive(Debug)]
pub enum ResultadoPassoAEstrela {
    EmProgresso,                         // O algoritmo ainda está buscando.
    CaminhoEncontrado(InfoCaminho),      // O caminho para o objetivo foi encontrado.
    NenhumCaminhoPossivel,               // A fronteira ficou vazia e o objetivo não foi alcançado.
    Erro(String),                        // Ocorreu algum erro durante a execução.
}


/// Struct principal que gerencia o estado e a lógica do algoritmo A*.
#[derive(Debug)]
pub struct SolucionadorAEstrela {
    grafo: Arc<GrafoMetro>, 
    id_objetivo: IdEstacao,
    fronteira: BinaryHeap<std::cmp::Reverse<EstadoNoFronteira>>,
    explorados: HashSet<(IdEstacao, Option<CorLinha>)>,
    predecessores: HashMap<(IdEstacao, Option<CorLinha>), (IdEstacao, Option<CorLinha>, CorLinha)>,
    custos_g: HashMap<(IdEstacao, Option<CorLinha>), f32>,
}

impl SolucionadorAEstrela {
    pub fn novo(
        grafo_compartilhado: Arc<GrafoMetro>,
        id_inicio: IdEstacao,
        linha_inicial_opcional: Option<CorLinha>, 
        id_objetivo_param: IdEstacao,
    ) -> Self {
        let mut fronteira_heap = BinaryHeap::new();
        let mut custos_g_map = HashMap::new();

        let custo_h_inicial = grafo_compartilhado
            .obter_tempo_heuristico_minutos(id_inicio, id_objetivo_param)
            .unwrap_or(f32::INFINITY); 

        let custo_g_inicial = 0.0;
        let custo_f_inicial = custo_g_inicial + custo_h_inicial;

        fronteira_heap.push(std::cmp::Reverse(EstadoNoFronteira {
            id_estacao: id_inicio,
            linha_chegada: linha_inicial_opcional,
            custo_f: custo_f_inicial,
            custo_g: custo_g_inicial,
        }));

        custos_g_map.insert((id_inicio, linha_inicial_opcional), custo_g_inicial);

        Self {
            grafo: grafo_compartilhado,
            id_objetivo: id_objetivo_param,
            fronteira: fronteira_heap,
            explorados: HashSet::new(),
            predecessores: HashMap::new(),
            custos_g: custos_g_map,
        }
    }

    pub fn proximo_passo(&mut self) -> ResultadoPassoAEstrela {
        if let Some(std::cmp::Reverse(no_atual)) = self.fronteira.pop() {
            let estado_atual_chave = (no_atual.id_estacao, no_atual.linha_chegada);

            if no_atual.id_estacao == self.id_objetivo {
                println!("[A*] Objetivo E{} alcançado com custo g: {:.2}", self.id_objetivo + 1, no_atual.custo_g);
                let caminho_info_temp = InfoCaminho {
                    estacoes_do_caminho: vec![(no_atual.id_estacao, no_atual.linha_chegada)], 
                    tempo_total_minutos: no_atual.custo_g,
                    baldeacoes: 0, 
                };
                return ResultadoPassoAEstrela::CaminhoEncontrado(caminho_info_temp);
            }

            if self.explorados.contains(&estado_atual_chave) {
                return ResultadoPassoAEstrela::EmProgresso; 
            }

            self.explorados.insert(estado_atual_chave);
            
            if let Some(conexoes_do_no_atual) = self.grafo.lista_adjacencia.get(no_atual.id_estacao) {
                for conexao_adjacente in conexoes_do_no_atual {
                    let id_vizinho = conexao_adjacente.para_estacao;
                    let linha_tomada_para_vizinho = conexao_adjacente.cor_linha;
                    let estado_vizinho_chave = (id_vizinho, Some(linha_tomada_para_vizinho));

                    if self.explorados.contains(&estado_vizinho_chave) {
                        continue;
                    }

                    let mut penalidade_baldeacao_atual = 0.0;
                    if let Some(linha_pela_qual_chegamos_no_no_atual) = no_atual.linha_chegada {
                        if linha_pela_qual_chegamos_no_no_atual != linha_tomada_para_vizinho {
                            penalidade_baldeacao_atual = TEMPO_BALDEACAO_MINUTOS;
                        }
                    }
                    
                    let custo_g_tentativo = no_atual.custo_g + conexao_adjacente.tempo_minutos + penalidade_baldeacao_atual;
                    let custo_g_anterior_para_vizinho = self.custos_g.get(&estado_vizinho_chave).cloned().unwrap_or(f32::INFINITY);

                    if custo_g_tentativo < custo_g_anterior_para_vizinho {
                        self.predecessores.insert(
                            estado_vizinho_chave,
                            (no_atual.id_estacao, no_atual.linha_chegada, linha_tomada_para_vizinho)
                        );
                        self.custos_g.insert(estado_vizinho_chave, custo_g_tentativo);

                        let custo_h_vizinho = self.grafo
                            .obter_tempo_heuristico_minutos(id_vizinho, self.id_objetivo)
                            .unwrap_or(0.0); 
                                            
                        let custo_f_vizinho = custo_g_tentativo + custo_h_vizinho;

                        self.fronteira.push(std::cmp::Reverse(EstadoNoFronteira {
                            id_estacao: id_vizinho,
                            linha_chegada: Some(linha_tomada_para_vizinho),
                            custo_f: custo_f_vizinho,
                            custo_g: custo_g_tentativo,
                        }));
                    }
                }
            }
            return ResultadoPassoAEstrela::EmProgresso;
        } else {
            return ResultadoPassoAEstrela::NenhumCaminhoPossivel;
        }
    }
}