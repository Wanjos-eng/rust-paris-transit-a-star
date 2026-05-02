<!-- PROJECT_METADATA
{
  "title": "Paris Metro — A* Route Planner",
  "short_description": "Implementação do algoritmo A* em Rust puro para roteamento no metrô de Paris, com interface gráfica egui que visualiza o processo de busca em tempo real.",
  "primary_stack": ["Rust", "egui", "eframe", "CSV"],
  "architecture": "Desktop App",
  "detail_description": "Este projeto é uma implementação completa do algoritmo A* (A-estrela) em Rust puro, aplicado a um problema real de roteamento no metrô de Paris. O grafo do metrô é construído a partir de tabelas CSV reais contendo: distâncias euclidean entre estações (heurística admissível), distâncias reais pelas linhas (custo de aresta), e conexões entre as 4 linhas com tempo de baldeação calculado separadamente. O diferencial técnico está na visualização interativa construída com egui/eframe: enquanto o A* executa, a interface renderiza em tempo real os nós explorados (cinza), a fronteira aberta (amarelo) e o caminho final encontrado (verde), permitindo visualizar exatamente como o algoritmo toma suas decisões. O cálculo de tempo considera a velocidade média real dos trens do metrô de Paris (entre 25-35 km/h) e acrescenta o tempo de baldeação quando há troca de linha. Toda a lógica de busca foi implementada do zero sem bibliotecas de grafos externas — estrutura de heap mínimo, mapa de visitados e reconstrução de caminho por backtracking via mapa de predecessores.",
  "images": [],
  "cover_image": "",
  "live_url": ""
}
-->

# Paris Metro — A* Route Planner

Implementação do algoritmo A* em Rust para encontrar o caminho mais eficiente entre estações do metrô de Paris, com interface gráfica interativa que visualiza o processo de busca.

## O que o Projeto Faz

- **Algoritmo A* do zero** — heap mínimo, mapa de visitados, backtracking por predecessores
- **Visualização em tempo real** — nós explorados (cinza), fronteira (amarelo), caminho final (verde)
- **Dados reais** — 4 linhas com distâncias e tempos extraídos de CSVs reais do metrô de Paris
- **Heurística admissível** — distância euclidiana garante solução ótima
- **Cálculo temporal** — velocidade real dos trens + tempo de baldeação entre linhas

## Stack Técnica

| Camada | Tecnologia |
|--------|-----------|
| Linguagem | Rust (safe, zero GC) |
| Interface Gráfica | egui + eframe (immediate mode GUI) |
| Dados | CSV (distâncias reais, conexões, heurísticas) |
| Algoritmo | A* com heurística euclidiana admissível |

## Estrutura

```
src/
├── algoritmo_a_estrela.rs      # Implementação A* — heap, visited, path reconstruction
├── grafo_metro.rs              # Estrutura de grafo com arestas e pesos
├── dados_metro.rs              # Carregamento e parsing dos CSVs
└── egui/
    ├── app.rs                  # Loop principal da UI
    ├── drawing.rs              # Renderização do mapa e estados do algoritmo
    ├── state_manager.rs        # Estado da busca (explored/frontier/path)
    └── visual_effects.rs       # Animações e efeitos visuais

data/
├── tabela1_distancias_diretas.csv    # Heurísticas (distância euclidiana)
├── tabela2_distancias_reais.csv      # Pesos reais das arestas
└── tabela_linhas_conexao.csv         # Conexões e baldeações entre linhas
```

## Como Executar

```bash
cargo run --release
```

![Mapa do Metrô de Paris](./mapa.jpg)
