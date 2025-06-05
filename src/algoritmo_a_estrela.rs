use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;
use std::sync::Arc;

// Importa as definições do nosso grafo e constantes
use crate::grafo_metro::{GrafoMetro, IdEstacao, CorLinha, TEMPO_BALDEACAO_MINUTOS};

#[derive(Debug, Clone)]
pub struct EstadoNoFronteira {
    pub id_estacao: IdEstacao,
    pub linha_chegada: Option<CorLinha>,
    pub custo_f: f32,
    pub custo_g: f32,
}

impl PartialEq for EstadoNoFronteira {
    fn eq(&self, other: &Self) -> bool {
        self.id_estacao == other.id_estacao &&
        self.linha_chegada == other.linha_chegada &&
        (self.custo_f - other.custo_f).abs() < f32::EPSILON &&
        (self.custo_g - other.custo_g).abs() < f32::EPSILON
    }
}

impl Eq for EstadoNoFronteira {}

impl Ord for EstadoNoFronteira {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.custo_f.partial_cmp(&self.custo_f) {
            Some(Ordering::Equal) => {
                self.custo_g.partial_cmp(&other.custo_g)
                    .unwrap_or(Ordering::Equal)
                    .then_with(|| self.id_estacao.cmp(&other.id_estacao))
            }
            Some(order) => order,
            None => Ordering::Equal,
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
    id_objetivo: IdEstacao,
    // CAMPOS MODIFICADOS PARA PÚBLICOS:
    pub fronteira: BinaryHeap<std::cmp::Reverse<EstadoNoFronteira>>,
    pub explorados: HashSet<(IdEstacao, Option<CorLinha>)>,
    // Mantidos privados pois são detalhes de implementação interna para reconstruir o caminho
    predecessores: HashMap<(IdEstacao, Option<CorLinha>), (IdEstacao, Option<CorLinha>, CorLinha)>,
    custos_g: HashMap<(IdEstacao, Option<CorLinha>), f32>,
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
            .unwrap_or(0.0); // Usar 0.0 se heurística não estiver disponível (A* vira Dijkstra)

        let custo_g_inicial = 0.0;
        let custo_f_inicial = custo_g_inicial + custo_h_inicial;

        fronteira_heap.push(std::cmp::Reverse(EstadoNoFronteira {
            id_estacao: id_inicio_param,
            linha_chegada: linha_inicial_opcional,
            custo_f: custo_f_inicial,
            custo_g: custo_g_inicial,
        }));

        custos_g_map.insert((id_inicio_param, linha_inicial_opcional), custo_g_inicial);

        Self {
            grafo: grafo_compartilhado,
            id_inicio: id_inicio_param,
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

            if self.explorados.contains(&estado_atual_chave) {
                return ResultadoPassoAEstrela::EmProgresso;
            }
            self.explorados.insert(estado_atual_chave.clone());

            if no_atual.id_estacao == self.id_objetivo {
                // println!("[A*] Objetivo E{} alcançado com custo g: {:.2}", self.id_objetivo + 1, no_atual.custo_g);
                let info_caminho = self.reconstruir_caminho(self.id_inicio, &no_atual);
                return ResultadoPassoAEstrela::CaminhoEncontrado(info_caminho);
            }
            
            // println!("[A*] Expandindo E{} (linha chegada: {:?}), g={:.2}, f={:.2}",
            //          no_atual.id_estacao + 1, no_atual.linha_chegada, no_atual.custo_g, no_atual.custo_f);

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
                        // println!("    [A*] Adicionando/Atualizando E{} na fronteira (linha {:?}), g={:.2}, h={:.2}, f={:.2}. Baldeação: {}",
                        //          id_vizinho+1, Some(linha_tomada_para_vizinho), custo_g_tentativo, custo_h_vizinho, custo_f_vizinho, penalidade_baldeacao_atual > 0.0);
                    }
                }
            }
            return ResultadoPassoAEstrela::EmProgresso;
        } else {
            return ResultadoPassoAEstrela::NenhumCaminhoPossivel;
        }
    }

    fn reconstruir_caminho(
        &self,
        id_inicio_busca: IdEstacao,
        estado_final_objetivo: &EstadoNoFronteira,
    ) -> InfoCaminho {
        let mut caminho_parcial: Vec<(IdEstacao, Option<CorLinha>)> = Vec::new();
        let mut baldeacoes_contadas = 0;

        let mut id_iter = estado_final_objetivo.id_estacao;
        let mut linha_chegada_iter = estado_final_objetivo.linha_chegada;

        caminho_parcial.push((id_iter, linha_chegada_iter));

        // Loop para reconstruir o caminho voltando pelos predecessores
        // A condição de parada é quando id_iter é o id_inicio_busca E não há mais como voltar (linha_chegada_iter seria o 'linha_inicial_opcional' do nó de início)
        // ou, mais simplesmente, quando não encontramos mais um predecessor no mapa (o que significa que chegamos ao início, que não tem predecessor registrado).
        loop {
            if id_iter == id_inicio_busca {
                // Se o estado atual (id_iter, linha_chegada_iter) é o estado inicial exato
                // (id_inicio_busca com a linha_inicial_opcional com que foi colocado no custos_g), paramos.
                // O nó inicial não terá uma entrada em `self.predecessores` que o tenha como *chave*.
                // A forma mais simples é verificar se `get` retorna `None`.
                let estado_inicial_original = self.custos_g.keys()
                    .find(|(id, _)| *id == id_inicio_busca)
                    .map(|k_tuple| k_tuple.clone()); // Pega o estado inicial como foi inserido

                if Some((id_iter, linha_chegada_iter)) == estado_inicial_original || linha_chegada_iter.is_none() && id_iter == id_inicio_busca {
                     // Se linha_inicial_opcional era None e chegamos ao id_inicio com linha_chegada_iter como None, é o início.
                    break;
                }
            }

            let chave_estado_atual_para_predecessor = (id_iter, linha_chegada_iter);
            if let Some(&(id_predecessor, linha_chegada_ao_predecessor, linha_usada_do_pred_para_atual)) =
                self.predecessores.get(&chave_estado_atual_para_predecessor)
            {
                caminho_parcial.push((id_predecessor, linha_chegada_ao_predecessor));

                if let Some(lc_pred) = linha_chegada_ao_predecessor {
                    if lc_pred != linha_usada_do_pred_para_atual {
                        baldeacoes_contadas += 1;
                    }
                }
                
                id_iter = id_predecessor;
                linha_chegada_iter = linha_chegada_ao_predecessor;
            } else {
                // Se não há predecessor, e não estamos no nó inicial (verificado no início do loop se id_iter == id_inicio_busca),
                // então algo está errado, ou simplesmente chegamos ao nó inicial que não tem predecessor.
                if id_iter != id_inicio_busca {
                    eprintln!(
                        "AVISO: Predecessor não encontrado para (E{}, {:?}) durante reconstrução, mas não é o início (E{}). Caminho pode estar incompleto.",
                        id_iter + 1, linha_chegada_iter, id_inicio_busca + 1
                    );
                }
                break; 
            }
        }

        caminho_parcial.reverse();

        InfoCaminho {
            estacoes_do_caminho: caminho_parcial,
            tempo_total_minutos: estado_final_objetivo.custo_g,
            baldeacoes: baldeacoes_contadas,
        }
    }
}