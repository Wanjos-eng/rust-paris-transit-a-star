use std::collections::HashMap;

pub const NUMERO_ESTACOES: usize = 14;
pub const VELOCIDADE_TREM_KMH: f32 = 30.0 / 2.0; // Ajustado para corresponder ao tempo esperado (em C++ tempo = distância * 2)
pub const TEMPO_BALDEACAO_MINUTOS: f32 = 4.0; // Esta constante será usada no algoritmo_a_estrela.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CorLinha {
    Azul = 1,
    Amarela = 2,
    Vermelha = 3,
    Verde = 4,
    Nenhuma = 0,
}

impl CorLinha {
    pub fn de_inteiro(valor: u8) -> Self {
        match valor {
            1 => CorLinha::Azul,
            2 => CorLinha::Amarela,
            3 => CorLinha::Vermelha,
            4 => CorLinha::Verde,
            _ => CorLinha::Nenhuma,
        }
    }
}

pub type IdEstacao = usize;

#[derive(Debug, Clone)]
pub struct Estacao {
    pub id: IdEstacao, // Este campo será lido/usado ao exibir informações ou na lógica da GUI
    pub nome: String,
}

#[derive(Debug, Clone)]
pub struct Conexao {
    pub para_estacao: IdEstacao,
    pub cor_linha: CorLinha,
    pub distancia_km: f32, // Este campo pode ser usado para exibir informações detalhadas do trajeto
    pub tempo_minutos: f32,
}

#[derive(Debug, Default)]
pub struct GrafoMetro {
    pub estacoes: Vec<Estacao>,
    pub lista_adjacencia: Vec<Vec<Conexao>>,
    pub distancias_heuristicas_km: Vec<Vec<Option<f32>>>,
    // Este campo nome_para_id será usado se implementarmos seleção de estação por nome na GUI
    // ou em outras funcionalidades de busca por nome.
    pub nome_para_id: HashMap<String, IdEstacao>,
}

impl GrafoMetro {
    pub fn novo() -> Self {
        let mut estacoes_vec = Vec::with_capacity(NUMERO_ESTACOES);
        let mut nome_para_id_map = HashMap::new();

        for i in 0..NUMERO_ESTACOES {
            let nome_estacao = format!("E{}", i + 1);
            estacoes_vec.push(Estacao {
                id: i,
                nome: nome_estacao.clone(),
            });
            nome_para_id_map.insert(nome_estacao, i);
        }

        Self {
            estacoes: estacoes_vec,
            lista_adjacencia: vec![Vec::new(); NUMERO_ESTACOES],
            distancias_heuristicas_km: vec![vec![None; NUMERO_ESTACOES]; NUMERO_ESTACOES],
            nome_para_id: nome_para_id_map,
        }
    }

    pub fn obter_id_estacao(&self, nome: &str) -> Option<IdEstacao> {
        self.nome_para_id.get(nome).copied()
    }

    pub fn obter_tempo_heuristico_minutos(&self, de_estacao: IdEstacao, para_estacao: IdEstacao) -> Option<f32> {
        self.distancias_heuristicas_km[de_estacao][para_estacao]
            .map(|dist_km| dist_km * 2.0) // Direto para minutos, conforme o código
    }
}