// src/main.rs
mod grafo_metro; // Declara o módulo grafo_metro (arquivo grafo_metro.rs)

fn main() {
    // Por enquanto, vamos apenas criar uma instância do grafo e imprimir para teste
    let grafo = grafo_metro::GrafoMetro::novo();
    println!("Grafo do Metrô Criado (inicialmente vazio):");
    println!("Número de estações: {}", grafo.estacoes.len());
    if let Some(id_e1) = grafo.obter_id_estacao("E1") {
        println!("ID da Estação E1: {}", id_e1);
        println!("Nome da Estação com ID {}: {}", id_e1, grafo.estacoes[id_e1].nome);
    }

    // Este é só um teste inicial. A lógica principal da aplicação virá depois.
    println!("\nPróximo passo: Implementar o carregamento de dados em dados_metro.rs!");
}