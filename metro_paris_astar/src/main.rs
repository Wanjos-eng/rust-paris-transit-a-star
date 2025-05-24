mod grafo_metro;   // Declara o módulo grafo_metro (arquivo grafo_metro.rs)
mod dados_metro;   // Declara o módulo dados_metro (arquivo dados_metro.rs)

// Trazemos a struct GrafoMetro para o escopo para facilitar o uso.
use grafo_metro::GrafoMetro;

fn main() {
    println!("--- Iniciando Teste de Carregamento do Grafo do Metrô ---");

    // Cria uma nova instância (mutável) do GrafoMetro.
    let mut grafo = GrafoMetro::novo();

    println!("\n[1] Carregando distâncias heurísticas (Tabela 1)...");
    // Tenta carregar as distâncias heurísticas.
    // Se ocorrer um erro, imprime o erro e encerra (panic!).
    // Em uma aplicação real, você trataria o erro de forma mais elegante.
    if let Err(e) = grafo.carregar_distancias_heuristicas("data/tabela1_distancias_diretas.csv") {
        eprintln!("ERRO CRÍTICO ao carregar 'tabela1_distancias_diretas.csv': {}", e);
        panic!(); // Encerra a aplicação em caso de erro crítico no carregamento.
    }
    println!("Distâncias heurísticas carregadas com sucesso!");
    // Pequeno teste para verificar uma distância heurística (E1 para E2)
    if let Some(dist_e1_e2_km) = grafo.distancias_heuristicas_km[0][1] {
         if let Some(tempo_e1_e2_min) = grafo.obter_tempo_heuristico_minutos(0,1) {
            println!(" > Teste Heurística: E1 (ID 0) para E2 (ID 1): {:.1} km, Tempo Estimado: {:.1} min", dist_e1_e2_km, tempo_e1_e2_min);
         }
    }


    println!("\n[2] Carregando conexões (Tabela 2 e Tabela de Linhas)...");
    // Tenta carregar as conexões reais e as linhas.
    if let Err(e) = grafo.carregar_conexoes(
        "data/tabela2_distancias_reais.csv",
        "data/tabela_linhas_conexao.csv",
    ) {
        eprintln!("ERRO CRÍTICO ao carregar conexões: {}", e);
        panic!();
    }
    println!("Conexões do metrô carregadas com sucesso!");
    // Pequeno teste para verificar conexões da Estação E1 (ID 0)
    println!(" > Conexões da Estação E1 (ID 0):");
    if grafo.estacoes.len() > 0 && grafo.lista_adjacencia.len() > 0 {
         for conexao in &grafo.lista_adjacencia[0] { // ID 0 é E1
            println!(
                "   -> Para E{} ({}), Linha: {:?}, Dist: {:.1}km, Tempo: {:.1}min",
                conexao.para_estacao + 1, // +1 para mostrar nome "E_X"
                grafo.estacoes[conexao.para_estacao].nome,
                conexao.cor_linha,
                conexao.distancia_km,
                conexao.tempo_minutos
            );
        }
    } else {
        println!("   Nenhuma estação ou lista de adjacência encontrada para teste.")
    }


    println!("\n--- Teste de Carregamento Concluído ---");
    println!("Se não houve 'panic!', os dados básicos foram carregados.");
    println!("Próximo passo: Implementar o algoritmo A* em algoritmo_a_estrela.rs!");
}