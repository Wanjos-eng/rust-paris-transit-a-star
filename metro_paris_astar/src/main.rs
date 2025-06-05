// src/main.rs
mod grafo_metro;
mod dados_metro;
mod algoritmo_a_estrela; // Declara o novo módulo

use grafo_metro::GrafoMetro;
// Importa o SolucionadorAEstrela e ResultadoPassoAEstrela para uso
use algoritmo_a_estrela::{SolucionadorAEstrela, ResultadoPassoAEstrela, InfoCaminho};
use std::sync::Arc; // Para usar Arc com o grafo

fn main() {
    println!("--- Iniciando Teste de Carregamento do Grafo do Metrô ---");

    // Cria uma nova instância (mutável) do GrafoMetro.
    let mut grafo = GrafoMetro::novo();

    // Carrega as distâncias heurísticas.
    if let Err(e) = grafo.carregar_distancias_heuristicas("data/tabela1_distancias_diretas.csv") {
        eprintln!("ERRO CRÍTICO ao carregar 'tabela1_distancias_diretas.csv': {}", e);
        panic!();
    }
    // Carrega as conexões reais e as linhas.
    if let Err(e) = grafo.carregar_conexoes(
        "data/tabela2_distancias_reais.csv",
        "data/tabela_linhas_conexao.csv",
    ) {
        eprintln!("ERRO CRÍTICO ao carregar conexões: {}", e);
        panic!();
    }
    println!("Dados do grafo carregados com sucesso.");

    // --- DEBUG RÁPIDO: Imprime algumas conexões carregadas ---
    // Descomente para verificar se as conexões estão sendo carregadas como esperado.
    /*
    for id_estacao_origem in 0..grafo_metro::NUMERO_ESTACOES {
        if !grafo.lista_adjacencia[id_estacao_origem].is_empty() {
            println!("Conexões de {}:", grafo.estacoes[id_estacao_origem].nome);
            for conexao in &grafo.lista_adjacencia[id_estacao_origem] {
                println!(
                    "  -> Para {} (Linha: {:?}, Tempo: {:.1} min)",
                    grafo.estacoes[conexao.para_estacao].nome,
                    conexao.cor_linha,
                    conexao.tempo_minutos
                );
            }
        }
    }
    */
    println!("\n--- Teste de Carregamento Concluído ---\n");


    // --- Testando o Solucionador A* ---
    println!("--- Iniciando Teste do Algoritmo A* ---");

    // Compartilha o grafo com Arc para que possa ser usado de forma segura
    let grafo_compartilhado = Arc::new(grafo);

    // Caso de teste: E6 (ID 5) -> E13 (ID 12)
    let id_inicio = 5; // E6
    let id_objetivo = 12; // E13

    // A linha inicial pode ser None, significando que o A* considera a primeira
    // linha tomada a partir da estação de início para calcular a primeira possível baldeação.
    let linha_de_partida_opcional = None; // Exemplo: grafo_metro::CorLinha::Azul se soubéssemos

    let mut solucionador = SolucionadorAEstrela::novo(
        Arc::clone(&grafo_compartilhado),
        id_inicio,
        linha_de_partida_opcional, 
        id_objetivo,
    );

    println!(
        "Iniciando busca de {} para {}", // Corrigido para não duplicar "E"
        grafo_compartilhado.estacoes[id_inicio].nome,
        grafo_compartilhado.estacoes[id_objetivo].nome
    );

    // Executa os passos do A*
    for i in 0..100 { // Limite de passos para evitar loop infinito em caso de erro
        match solucionador.proximo_passo() {
            ResultadoPassoAEstrela::EmProgresso => {
                // Comente esta linha para uma saída mais limpa quando o caminho for longo
                // println!("[Passo {}] A* em progresso...", i + 1);
            }
            ResultadoPassoAEstrela::CaminhoEncontrado(info_caminho) => {
                println!("\n!!! CAMINHO ENCONTRADO !!!");
                println!("   Tempo total: {:.2} minutos", info_caminho.tempo_total_minutos);
                println!("   Número de Baldeações: {}", info_caminho.baldeacoes);
                println!("   Trajeto (Estação [Linha de Chegada nela]):");

                for (indice_no_trajeto, (id_estacao_no_trajeto, linha_chegada_opcional)) in info_caminho.estacoes_do_caminho.iter().enumerate() {
                    let nome_estacao = &grafo_compartilhado.estacoes[*id_estacao_no_trajeto].nome;
                    let info_linha = match linha_chegada_opcional {
                        Some(cor) => format!("{:?}", cor),
                        None => "N/A (Início)".to_string(),
                    };

                    if indice_no_trajeto == 0 {
                         println!("     {}. Partida: {} [{}]",
                            indice_no_trajeto + 1,
                            nome_estacao,
                            info_linha // Linha pela qual "entramos" na primeira estação (geralmente None)
                        );
                    } else {
                        // A linha_chegada_opcional aqui é a linha do TRECHO que chegou nesta estação
                        println!("     {}. Para: {} [Chegou pela Linha: {}]",
                            indice_no_trajeto + 1,
                            nome_estacao,
                            info_linha
                        );
                    }
                }
                break; // Sai do loop
            }
            ResultadoPassoAEstrela::NenhumCaminhoPossivel => {
                println!("\nXXX NENHUM CAMINHO POSSÍVEL ENCONTRADO XXX");
                break; // Sai do loop
            }
            ResultadoPassoAEstrela::Erro(msg_erro) => {
                eprintln!("\nERRO NO A*: {}", msg_erro);
                break; // Sai do loop
            }
        }
        if i == 99 { // Limite de passos de segurança
            println!("Atingido limite de 100 passos sem encontrar o objetivo.");
        }
    }
    println!("\n--- Teste A* Concluído ---");
}