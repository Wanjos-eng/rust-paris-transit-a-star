use std::collections::HashMap; // Precisaremos do HashMap para o mapeamento nome->id

// --- Constantes Globais ---
// Definem características fixas do nosso sistema de metrô.

// Número total de estações no nosso modelo (E1 a E14).
pub const NUMERO_ESTACOES: usize = 14;

// Velocidade média do trem em km/h. Usada para converter distâncias em tempo.
pub const VELOCIDADE_TREM_KMH: f32 = 30.0;

// Tempo fixo em minutos gasto para fazer uma baldeação (trocar de linha)
// dentro da mesma estação. [cite: 32]
pub const TEMPO_BALDEACAO_MINUTOS: f32 = 4.0;

// --- Enums e Tipos Personalizados ---

/// Representa as cores das linhas do metrô.
/// O 'derive' permite que o enum seja facilmente usado em debugs, copiado, comparado, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CorLinha {
    Azul = 1,
    Amarela = 2,
    Vermelha = 3,
    Verde = 4,
    Nenhuma = 0, // Usado para indicar ausência de linha específica ou erro.
}

// Implementação para o enum CorLinha
impl CorLinha {
    /// Converte um valor numérico (u8) para a CorLinha correspondente.
    /// Isso será útil ao ler os dados da "Tabela de Linhas".
    pub fn de_inteiro(valor: u8) -> Self {
        match valor {
            1 => CorLinha::Azul,
            2 => CorLinha::Amarela,
            3 => CorLinha::Vermelha,
            4 => CorLinha::Verde,
            _ => CorLinha::Nenhuma, // Qualquer outro valor resulta em Nenhuma.
        }
    }
}

/// Define um tipo `IdEstacao` como um alias para `usize`.
/// `usize` é um tipo de inteiro sem sinal que o Rust usa para indexação e tamanhos.
/// Usaremos isso para referenciar estações por um índice numérico (0 para E1, 1 para E2, etc.).
pub type IdEstacao = usize;

// --- Estruturas Principais (Structs) ---

/// Representa uma estação do metrô.
#[derive(Debug, Clone)] // Permite debug e clonagem fácil da struct.
pub struct Estacao {
    pub id: IdEstacao, // Identificador numérico único da estação.
    pub nome: String,  // Nome da estação (ex: "E1", "E2").
}

/// Representa uma conexão (aresta) entre duas estações no grafo.
#[derive(Debug, Clone)]
pub struct Conexao {
    pub para_estacao: IdEstacao, // ID da estação de destino desta conexão.
    pub cor_linha: CorLinha,     // Cor da linha de trem usada nesta conexão.
    pub distancia_km: f32,     // Distância real da conexão em quilômetros.
    pub tempo_minutos: f32,    // Tempo de viagem nesta conexão em minutos (calculado).
                               // Este será o custo g(n) parcial para esta aresta específica.
}

/// Representa o grafo completo do sistema de metrô.
/// Contém todas as estações, suas conexões, e dados auxiliares.
#[derive(Debug, Default)] // Default permite criar uma instância com valores padrão.
pub struct GrafoMetro {
    // Um vetor (lista dinâmica) de todas as structs Estacao.
    pub estacoes: Vec<Estacao>,

    // Lista de adjacência: um vetor onde cada posição `i` contém outro vetor
    // com todas as Conexoes que partem da `estacoes[i]`.
    pub lista_adjacencia: Vec<Vec<Conexao>>,

    // Matriz (vetor de vetores) para armazenar as distâncias diretas (heurísticas)
    // entre as estações, lidas da Tabela 1.
    // `Option<f32>` significa que pode haver uma distância (Some(valor)) ou não (None).
    pub distancias_heuristicas_km: Vec<Vec<Option<f32>>>,

    // Um HashMap para mapear rapidamente o nome de uma estação (String, ex: "E1")
    // para o seu IdEstacao (usize, ex: 0). Útil para consulta.
    pub nome_para_id: HashMap<String, IdEstacao>,
}

// Implementação de métodos para a struct GrafoMetro.
impl GrafoMetro {
    /// Cria e retorna uma nova instância de `GrafoMetro`.
    /// Inicializa as estações com seus nomes e IDs, e prepara as estruturas
    /// de dados internas (lista de adjacência, distâncias heurísticas) vazias
    /// ou com valores padrão.
    pub fn novo() -> Self {
        let mut estacoes_vec = Vec::with_capacity(NUMERO_ESTACOES);
        let mut nome_para_id_map = HashMap::new();

        // Cria as N estações, nomeando-as de "E1" a "EN".
        for i in 0..NUMERO_ESTACOES {
            let nome_estacao = format!("E{}", i + 1); // Formata o nome como "E1", "E2", ...
            estacoes_vec.push(Estacao {
                id: i, // O ID é o próprio índice do loop (0 para E1, 1 para E2, ...)
                nome: nome_estacao.clone(),
            });
            // Insere o mapeamento do nome para o ID no HashMap.
            nome_para_id_map.insert(nome_estacao, i);
        }

        // Retorna a nova instância de GrafoMetro.
        Self {
            estacoes: estacoes_vec,
            // A lista de adjacência começa como um vetor de N vetores vazios.
            lista_adjacencia: vec![Vec::new(); NUMERO_ESTACOES],
            // A matriz de distâncias heurísticas começa como N x N com todos os valores None.
            distancias_heuristicas_km: vec![vec![None; NUMERO_ESTACOES]; NUMERO_ESTACOES],
            nome_para_id: nome_para_id_map,
        }
    }

    /// Obtém o `IdEstacao` (numérico) a partir do nome da estação (String).
    /// Retorna `Some(IdEstacao)` se encontrada, ou `None` caso contrário.
    pub fn obter_id_estacao(&self, nome: &str) -> Option<IdEstacao> {
        // `copied()` é usado porque `get()` retorna uma referência, e IdEstacao é `Copy`.
        self.nome_para_id.get(nome).copied()
    }

    /// Calcula e retorna o tempo heurístico (estimativa h(n) para o A*) em minutos
    /// entre duas estações, baseado na distância direta e na velocidade do trem.
    /// Retorna `Some(tempo_em_minutos)` ou `None` se a distância heurística não existir.
    pub fn obter_tempo_heuristico_minutos(&self, de_estacao: IdEstacao, para_estacao: IdEstacao) -> Option<f32> {
        // Acessa a distância direta (em km) da matriz de heurísticas.
        self.distancias_heuristicas_km[de_estacao][para_estacao]
            .map(|dist_km| (dist_km / VELOCIDADE_TREM_KMH) * 60.0) // Converte km para minutos
    }
}