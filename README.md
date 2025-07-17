# Paris Metro A* Route Planner

![Metro Paris Map](./mapa.jpg)

## 🚇 Sobre o Projeto

O Paris Metro A* Route Planner é uma aplicação Rust que utiliza o algoritmo A* para encontrar o caminho mais eficiente entre duas estações do metrô de Paris. O aplicativo conta com uma interface gráfica interativa que permite visualizar o processo de busca e exibir a rota ótima, considerando distâncias reais, tempo de viagem e trocas de linha.

## 🔍 Características

- **Algoritmo A*** - Implementação eficiente do algoritmo A* para encontrar o caminho mais rápido
- **Visualização Interativa** - Interface gráfica que permite visualizar o processo de busca em tempo real
- **Múltiplas Linhas de Metrô** - Suporte para 4 linhas diferentes (Azul, Amarela, Vermelha e Verde)
- **Cálculo de Tempo Real** - Considera distâncias reais, velocidade dos trens e tempo de baldeação
- **Visualização do Processo** - Mostra nós explorados, fronteira e decisões do algoritmo
- **Controles de Zoom** - Interface ajustável com controles de zoom e navegação

## 📊 Estrutura do Projeto

```
rust-paris-transit-a-star/
│
├── src/                     # Código fonte
│   ├── main.rs              # Ponto de entrada do programa
│   ├── algoritmo_a_estrela.rs # Implementação do algoritmo A*
│   ├── dados_metro.rs       # Funções para carregar dados do metrô
│   ├── grafo_metro.rs       # Estrutura de dados do grafo do metrô
│   └── egui/               # Módulos de interface gráfica
│       ├── app.rs          # Aplicação principal da interface
│       ├── controls.rs     # Controles da interface
│       ├── drawing.rs      # Funções de desenho
│       ├── mod.rs          # Módulo de exportação
│       ├── navigation.rs   # Controles de navegação
│       ├── popups.rs       # Janelas popup
│       ├── state_manager.rs # Gerenciamento de estado
│       └── visual_effects.rs # Efeitos visuais
│
├── data/                    # Dados do metrô de Paris
│   ├── tabela1_distancias_diretas.csv   # Distâncias heurísticas (em linha reta)
│   ├── tabela2_distancias_reais.csv     # Distâncias reais entre estações conectadas
│   └── tabela_linhas_conexao.csv        # Informações sobre conexões entre estações e linhas
│
└── mapa.jpg                 # Mapa visual do metrô de Paris
```

## 🛠️ Tecnologias Utilizadas

- **Rust** - Linguagem de programação segura e de alto desempenho
- **egui/eframe** - Framework para criação de interfaces gráficas em Rust
- **CSV** - Biblioteca para leitura de arquivos CSV

## 🚀 Compilação e Execução

### Pré-requisitos

- Rust e Cargo instalados (https://www.rust-lang.org/tools/install)

### Compilando o Projeto

1. Clone o repositório:
   ```bash
   git clone https://github.com/seu-usuario/rust-paris-transit-a-star.git
   cd rust-paris-transit-a-star
   ```

2. Compile o projeto:
   ```bash
   cargo build --release
   ```

### Executando o Programa

Execute o programa compilado:
```bash
cargo run --release
```

### Compilação para Windows (Cross-compilation)

Para compilar o projeto para Windows a partir de Linux:

1. Configure o target para Windows:
   ```bash
   rustup target add x86_64-pc-windows-gnu
   ```

2. Instale o MinGW:
   ```bash
   sudo apt install mingw-w64
   ```

3. Compile para Windows:
   ```bash
   cargo build --release --target x86_64-pc-windows-gnu
   ```

## 🖱️ Como Usar

1. Inicie o aplicativo
2. Selecione uma estação de origem usando o menu suspenso "Estação de Início"
3. Selecione uma estação de destino usando o menu suspenso "Estação de Objetivo"
4. Clique no botão "Calcular Rota" para iniciar a busca A*
5. Visualize o processo de busca e o caminho encontrado no mapa
6. Use os controles de zoom e navegação para melhor visualização
7. Consulte o painel de informações para detalhes sobre o caminho encontrado

## 🧪 Formato dos Dados

### tabela1_distancias_diretas.csv
Contém as distâncias em linha reta entre cada par de estações (heurística para A*).

### tabela2_distancias_reais.csv
Contém as distâncias reais entre estações conectadas (-1.0 para estações não conectadas diretamente).

### tabela_linhas_conexao.csv
Define as conexões entre estações e a qual linha cada conexão pertence:
- 0: Sem conexão
- 1: Linha Azul
- 2: Linha Amarela
- 3: Linha Vermelha
- 4: Linha Verde

## 📝 Licença

Este projeto está licenciado sob a Licença MIT - veja o arquivo [LICENSE](LICENSE) para detalhes.

## 🧠 Algoritmo A*

O algoritmo A* implementado usa a seguinte função de avaliação:
- f(n) = g(n) + h(n)
  - g(n): Custo real do caminho do nó inicial até o nó n
  - h(n): Estimativa heurística do custo do nó n até o objetivo
  
O algoritmo considera o tempo de baldeação entre diferentes linhas de metrô, o que o torna mais realista para aplicações de transporte público.