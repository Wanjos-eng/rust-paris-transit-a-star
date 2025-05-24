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
    let mut grafo = GrafoMetro::novo();

    if let Err(e) = grafo.carregar_distancias_heuristicas("data/tabela1_distancias_diretas.csv") {
        eprintln!("ERRO CRÍTICO ao carregar 'tabela1_distancias_diretas.csv': {}", e);
        panic!();
    }
    if let Err(e) = grafo.carregar_conexoes(
        "data/tabela2_distancias_reais.csv",
        "data/tabela_linhas_conexao.csv",
    ) {
        eprintln!("ERRO CRÍTICO ao carregar conexões: {}", e);
        panic!();
    }
    println!("Dados do grafo carregados com sucesso.\n");

    // --- Testando o Solucionador A* ---
    println!("--- Iniciando Teste do Algoritmo A* ---");

    // Compartilha o grafo com Arc para que possa ser usado por múltiplas partes de forma segura
    let grafo_compartilhado = Arc::new(grafo);

    // Caso de teste: E6 (ID 5) linha Azul -> E13 (ID 12)
    // No nosso modelo, a "linha azul" para E6 ao iniciar pode ser representada por uma das
    // linhas de conexão que E6 possui, ou None se não quisermos forçar uma linha inicial.
    // Vamos supor que E6 (ID 5) se conecta a E5 (ID 4) pela linha Azul (CorLinha::Azul)
    // e essa é a "linha de entrada" em E6.
    // A Tabela de Linhas mostra E6-E5 (ou E5-E6) na linha 1 (Azul).
    // A linha de chegada na estação inicial pode ser None se ela não veio de uma conexão anterior.
    let id_inicio = 5; // E6
    let id_objetivo = 12; // E13

    // Para o caso "estação 6 linha azula", precisamos saber QUAL linha azul.
    // Se E6 está na linha azul porque se conecta a E5 (ID 4) via linha Azul,
    // e o usuário "entra" em E6 por essa linha, então `linha_inicial_opcional`
    // poderia ser `Some(grafo_metro::CorLinha::Azul)`.
    // Por simplicidade, vamos iniciar sem uma linha de chegada específica na estação de início.
    // O algoritmo considerará a primeira linha tomada a partir do nó inicial.
    let mut solucionador = SolucionadorAEstrela::novo(
        Arc::clone(&grafo_compartilhado), // Clona o Arc, não o grafo em si
        id_inicio,
        None, // Começamos na estação E6, sem uma "linha de chegada" anterior.
              // A primeira baldeação (se houver) será ao sair de E6 para a primeira conexão.
        id_objetivo,
    );

    println!(
        "Iniciando busca de {} para {}", // Correção
        grafo_compartilhado.estacoes[id_inicio].nome,
        grafo_compartilhado.estacoes[id_objetivo].nome
    );

    // Executa alguns passos do A*
    for i in 0..50 { // Limita o número de passos para teste
        match solucionador.proximo_passo() {
            ResultadoPassoAEstrela::EmProgresso => {
                println!("[Passo {}] A* em progresso...", i + 1);
            }
            ResultadoPassoAEstrela::CaminhoEncontrado(info_caminho) => {
                println!("\n!!! CAMINHO ENCONTRADO !!!");
                println!("   Tempo total: {:.2} minutos", info_caminho.tempo_total_minutos);
                // TODO: Imprimir o caminho detalhado quando a reconstrução estiver pronta
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
        // Para não sobrecarregar o terminal com a fronteira em cada passo:
        // if let Some(topo_fronteira) = solucionador.fronteira.peek() {
        //     println!("    Topo da fronteira: E{} (f={:.2})", topo_fronteira.0.id_estacao + 1, topo_fronteira.0.custo_f);
        // }
    }
    println!("\n--- Teste A* Concluído ---");
}