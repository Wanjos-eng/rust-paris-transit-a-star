// Algoritmo A* para encontrar o caminho mais curto em um grafo de metrô.
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;
use std::sync::Arc; // Para compartilhar o GrafoMetro de forma segura

// Importa o GrafoMetro e os tipos necessários
use crate::grafo_metro::{GrafoMetro, IdEstacao, CorLinha, TEMPO_BALDEACAO_MINUTOS};

/// Representa o estado de um nó na fronteira (open list) do A*.
/// Inclui a estação atual, a linha pela qual se chegou a esta estação,
/// e os custos f e g. O custo h é calculado sob demanda.
#[derive(Debug, Clone)]
pub struct EstadoNoFronteira {
    pub id_estacao: IdEstacao,
    pub linha_chegada: Option<CorLinha>, // Linha pela qual se chegou a esta `id_estacao`
    pub custo_f: f32, // Custo total estimado (g + h)
    pub custo_g: f32, // Custo real desde o início até este nó
}

// Implementação de PartialEq manualmente
impl PartialEq for EstadoNoFronteira {
    fn eq(&self, other: &Self) -> bool {
        // Compara os campos relevantes. Para floats, a igualdade direta é usada aqui.
        // Para o BinaryHeap, a ordenação (Ord) é mais crítica.
        // A igualdade exata de floats pode ser complicada, mas para custos que
        // são somas de outros floats, a comparação direta pode funcionar se não houver
        // erros de arredondamento muito significativos que causem problemas lógicos.
        // Uma abordagem mais robusta para floats envolveria comparação com epsilon,
        // mas para a estrutura de A*, a ordenação na BinaryHeap é o principal.
        // Se esta struct fosse usada como chave em HashMap, a igualdade de float seria mais crítica.
        self.id_estacao == other.id_estacao &&
        self.linha_chegada == other.linha_chegada &&
        // A comparação de custo_f e custo_g aqui é para a trait PartialEq.
        // A BinaryHeap usa Ord, que tem sua própria lógica de comparação.
        (self.custo_f - other.custo_f).abs() < f32::EPSILON &&
        (self.custo_g - other.custo_g).abs() < f32::EPSILON
    }
}

// Eq pode ser implementado se PartialEq for implementado e não houver NaNs
// ou se a lógica de igualdade for reflexiva, simétrica e transitiva.
impl Eq for EstadoNoFronteira {}

impl Ord for EstadoNoFronteira {
    fn cmp(&self, other: &Self) -> Ordering {
        // Queremos uma min-heap (menor custo_f tem maior prioridade),
        // então invertemos a comparação para custo_f.
        match other.custo_f.partial_cmp(&self.custo_f) {
            Some(Ordering::Equal) => {
                // Se custo_f é igual, desempatamos por custo_g (menor é melhor)
                self.custo_g.partial_cmp(&other.custo_g)
                    .unwrap_or(Ordering::Equal)
                    // Desempate final por ID para garantir uma ordem consistente
                    .then_with(|| self.id_estacao.cmp(&other.id_estacao))
            }
            Some(order) => order,
            None => {
                // Este caso (NaN) não deveria ocorrer em custos válidos.
                // Se ocorrer, tratar como igual para não quebrar a ordenação,
                // mas idealmente, NaNs devem ser evitados nos custos.
                Ordering::Equal
            }
        }
    }
}

impl PartialOrd for EstadoNoFronteira {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


/// Guarda informações sobre o caminho encontrado.
#[derive(Debug, Clone)]
pub struct InfoCaminho {
    // Vec<(IdEstacao, LinhaUsadaParaChegarNestaEstacao)>
    // A primeira estação (início) terá Option<CorLinha> como None.
    pub estacoes_do_caminho: Vec<(IdEstacao, Option<CorLinha>)>,
    pub tempo_total_minutos: f32,
    pub baldeacoes: u32,
}

/// Representa o resultado de um único passo do algoritmo A*.
#[derive(Debug)]
pub enum ResultadoPassoAEstrela {
    EmProgresso,
    CaminhoEncontrado(InfoCaminho),
    NenhumCaminhoPossivel,
    Erro(String), // Ainda não usamos esta variante, mas está aqui para futuras extensões.
}


/// Struct principal que gerencia o estado e a lógica do algoritmo A*.
#[derive(Debug)]
pub struct SolucionadorAEstrela {
    grafo: Arc<GrafoMetro>,
    id_inicio: IdEstacao, // Adicionado para a reconstrução do caminho
    id_objetivo: IdEstacao,
    fronteira: BinaryHeap<std::cmp::Reverse<EstadoNoFronteira>>,
    explorados: HashSet<(IdEstacao, Option<CorLinha>)>,
    // Chave: (IdEstacao atual, Linha de chegada na estacao atual)
    // Valor: (IdEstacao de onde veio, Linha de chegada NAQUELA estacao anterior, CorLinha da conexao USADA para chegar aqui)
    predecessores: HashMap<(IdEstacao, Option<CorLinha>), (IdEstacao, Option<CorLinha>, CorLinha)>,
    custos_g: HashMap<(IdEstacao, Option<CorLinha>), f32>,
}

impl SolucionadorAEstrela {
    /// Cria uma nova instância do `SolucionadorAEstrela`.
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
            .unwrap_or(f32::INFINITY); // Se não houver heurística, pode ser problemático se não tratarmos infinito.
                                       // Para A*, 0.0 é uma heurística válida (admissível, mas não informativa).

        let custo_g_inicial = 0.0;
        let custo_f_inicial = custo_g_inicial + custo_h_inicial;

        fronteira_heap.push(std::cmp::Reverse(EstadoNoFronteira {
            id_estacao: id_inicio_param,
            linha_chegada: linha_inicial_opcional, // Linha pela qual "entramos" na estação de início (pode ser None)
            custo_f: custo_f_inicial,
            custo_g: custo_g_inicial,
        }));

        custos_g_map.insert((id_inicio_param, linha_inicial_opcional), custo_g_inicial);

        Self {
            grafo: grafo_compartilhado,
            id_inicio: id_inicio_param, // Armazena o ID de início
            id_objetivo: id_objetivo_param,
            fronteira: fronteira_heap,
            explorados: HashSet::new(),
            predecessores: HashMap::new(),
            custos_g: custos_g_map,
        }
    }

    /// Executa um único passo do algoritmo A*.
    pub fn proximo_passo(&mut self) -> ResultadoPassoAEstrela {
        if let Some(std::cmp::Reverse(no_atual)) = self.fronteira.pop() {
            let estado_atual_chave = (no_atual.id_estacao, no_atual.linha_chegada);

            // Se já exploramos este estado (estação + linha de chegada), pulamos.
            // Fazemos isso ANTES de verificar se é o objetivo para evitar reprocessar um caminho
            // para o objetivo que já foi encontrado por um custo maior e depois re-adicionado à fronteira.
            // No entanto, para garantir que pegamos o MENOR custo g para o objetivo,
            // esta verificação pode ser feita após a checagem do objetivo se a heurística for consistente.
            // Com heurística admissível, a primeira vez que o objetivo é retirado da fronteira é o caminho ótimo.
            if self.explorados.contains(&estado_atual_chave) {
                // println!("[A*] Estado ({:?}, {:?}) já explorado, pulando.", no_atual.id_estacao +1, no_atual.linha_chegada);
                return ResultadoPassoAEstrela::EmProgresso;
            }
            self.explorados.insert(estado_atual_chave.clone()); // Clonamos para inserir, pois `estado_atual_chave` é uma tupla de referências

            // Se o nó atual é o objetivo, encontramos o caminho!
            if no_atual.id_estacao == self.id_objetivo {
                println!("[A*] Objetivo E{} alcançado com custo g: {:.2}", self.id_objetivo + 1, no_atual.custo_g);
                let info_caminho = self.reconstruir_caminho(self.id_inicio, &no_atual);
                return ResultadoPassoAEstrela::CaminhoEncontrado(info_caminho);
            }
            
            // Adiciona o estado atual à lista de explorados.
            // Movido para cima para evitar reprocessamento desnecessário de vizinhos se já explorado.
            // self.explorados.insert(estado_atual_chave); // Já inserido

            // println!("[A*] Expandindo E{} (linha chegada: {:?}), g={:.2}, f={:.2}",
            //          no_atual.id_estacao + 1, no_atual.linha_chegada, no_atual.custo_g, no_atual.custo_f);

            if let Some(conexoes_do_no_atual) = self.grafo.lista_adjacencia.get(no_atual.id_estacao) {
                for conexao_adjacente in conexoes_do_no_atual {
                    let id_vizinho = conexao_adjacente.para_estacao;
                    let linha_tomada_para_vizinho = conexao_adjacente.cor_linha;
                    // O estado do vizinho é definido pela estação e pela linha USADA PARA CHEGAR NELE.
                    let estado_vizinho_chave = (id_vizinho, Some(linha_tomada_para_vizinho));

                    // Se o vizinho já foi explorado com esta linha de chegada, pulamos.
                    // Esta verificação é importante para grafos com ciclos.
                    if self.explorados.contains(&estado_vizinho_chave) {
                        // println!("    [A*] Vizinho E{} (linha {:?}) já explorado.", id_vizinho + 1, Some(linha_tomada_para_vizinho));
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
                            estado_vizinho_chave, // (id_vizinho, Some(linha_tomada_para_vizinho))
                            (no_atual.id_estacao, no_atual.linha_chegada, linha_tomada_para_vizinho)
                        );
                        self.custos_g.insert(estado_vizinho_chave, custo_g_tentativo);

                        let custo_h_vizinho = self.grafo
                            .obter_tempo_heuristico_minutos(id_vizinho, self.id_objetivo)
                            .unwrap_or(0.0); // Heurística zero se não definida.
                                            
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

    /// Reconstrói o caminho do nó objetivo de volta ao nó inicial usando o mapa de predecessores.
    /// Também calcula o número de baldeações.
    fn reconstruir_caminho(
        &self,
        id_inicio_busca: IdEstacao, // ID da estação inicial da busca
        estado_final_objetivo: &EstadoNoFronteira, // O estado do nó objetivo quando alcançado
    ) -> InfoCaminho {
        let mut caminho_parcial: Vec<(IdEstacao, Option<CorLinha>)> = Vec::new();
        let mut baldeacoes_contadas = 0;

        let mut id_iter = estado_final_objetivo.id_estacao;
        let mut linha_chegada_iter = estado_final_objetivo.linha_chegada;

        // Adiciona o objetivo primeiro
        caminho_parcial.push((id_iter, linha_chegada_iter));

        while id_iter != id_inicio_busca || (id_iter == id_inicio_busca && linha_chegada_iter != self.custos_g.keys().find(|(id, _)| *id == id_inicio_busca).map(|(_, line)| *line).unwrap_or(None) ) {
             // O loop continua enquanto não chegamos ao exato estado inicial (id E linha de chegada)
             // Isso é importante se o nó inicial foi adicionado à fronteira com um Some(linha_chegada) específico.
             // Se o nó inicial foi adicionado com None, então `id_iter != id_inicio_busca` é suficiente.
             // A condição acima é um pouco complexa para o estado inicial, simplificando:
             // O loop deve parar quando id_iter é id_inicio_busca E não há mais predecessores para o estado inicial.
             // O estado inicial não terá um predecessor na `self.predecessores` se foi apenas colocado na fronteira.

            let chave_estado_atual_para_predecessor = (id_iter, linha_chegada_iter);

            if let Some(&(id_predecessor, linha_chegada_ao_predecessor, linha_usada_do_pred_para_atual)) =
                self.predecessores.get(&chave_estado_atual_para_predecessor)
            {
                // Adiciona o predecessor ao caminho.
                // A linha_chegada_ao_predecessor é a linha pela qual chegamos ao predecessor.
                caminho_parcial.push((id_predecessor, linha_chegada_ao_predecessor));

                if let Some(lc_pred) = linha_chegada_ao_predecessor {
                    if lc_pred != linha_usada_do_pred_para_atual {
                        // Baldeação ocorreu na estação `id_predecessor` para pegar a `linha_usada_do_pred_para_atual`.
                        baldeacoes_contadas += 1;
                    }
                }
                
                id_iter = id_predecessor;
                linha_chegada_iter = linha_chegada_ao_predecessor;

                // Condição de parada mais robusta: se o id_iter é o início E a linha_chegada_iter
                // corresponde à linha_chegada original do nó inicial (que pode ser None).
                // Se o nó inicial foi adicionado com (id_inicio, None) ao custos_g,
                // então quando linha_chegada_iter se torna None e id_iter é id_inicio, paramos.
                // Se o `id_inicio_param` em `SolucionadorAEstrela::novo` foi adicionado a `custos_g`
                // com `linha_inicial_opcional`, precisamos checar contra isso.
                // No nosso caso, o predecessor do nó inicial não estará no mapa `predecessores`.
                // O loop vai parar naturalmente quando `get` retornar `None`.
            } else {
                // Chegamos ao início ou a um ponto sem predecessor (deve ser o início)
                if id_iter != id_inicio_busca {
                     eprintln!(
                        "ERRO: Predecessor não encontrado para ({:?}, {:?}) antes de atingir o início da busca ({}). Caminho parcial.",
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