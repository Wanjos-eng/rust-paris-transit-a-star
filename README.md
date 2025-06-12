# 🚇 Paris Metro A* Route Planner

> Um sistema inteligente de planejamento de rotas para o Metrô de Paris usando o algoritmo A* (A-estrela)

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)

![Mapa do Metrô](mapa.jpg)

## 📋 Sobre o Projeto

Este projeto foi desenvolvido como parte da disciplina de **Estrutura de Dados II** com foco em **Algoritmos em Grafos**. O sistema auxilia usuários do metrô de Paris a encontrar o trajeto mais rápido entre estações, utilizando o algoritmo A* para busca informada em grafos.

### 🎯 Características Principais

- **Interface Gráfica Interativa**: Desenvolvida com egui/eframe
- **Algoritmo A* Otimizado**: Implementação eficiente para busca de caminhos
- **4 Linhas de Metrô**: Azul, Amarela, Vermelha e Verde
- **Cálculo de Tempo Real**: Considera velocidade dos trens (30km/h) e tempo de baldeação (4min)
- **Visualização do Trajeto**: Mostra graficamente o melhor caminho

## 🧮 Funcionamento do Algoritmo A*

O algoritmo A* funciona da seguinte forma:

1. **Inicialização**: Coloca a estação origem na lista aberta com f = 0
2. **Seleção**: Escolhe o nó com menor valor f (f = g + h) da lista aberta
3. **Expansão**: Explora todos os vizinhos do nó atual, calculando seus custos
4. **Avaliação**: Para cada vizinho, calcula g (tempo real) + h (tempo estimado)
5. **Finalização**: Repete até encontrar a estação destino ou esgotar possibilidades

Onde:
- **g**: Tempo estimado entre a estação de origem e a estação atual
- **h**: Tempo heurístico entre a estação atual e a estação de destino
- **f**: Função de avaliação total (f = g + h)

## 🗺️ Estrutura do Metrô

### Linhas Disponíveis
- 🔵 **Linha Azul** (cor=1): Estações 1, 2, 3, 4, 5, 6
- 🟡 **Linha Amarela** (cor=2): Estações 2, 5, 7, 8, 9, 10
- 🔴 **Linha Vermelha** (cor=3): Estações 3, 9, 11, 13
- 🟢 **Linha Verde** (cor=4): Estações 4, 8, 12, 13, 14

### Exemplo de Busca
**Caso de Teste**: E6 (Linha Azul) → E13 (Linha Vermelha)
- Estação Inicial: E6 (Linha Azul)
- Estação Final: E13 (Linha Vermelha)
- Requer baldeação entre linhas

## 🚀 Como Executar

### Pré-requisitos
- [Rust](https://rustup.rs/) (versão mais recente)
- Sistema operacional: Linux, Windows ou macOS

### Instalação e Execução

```bash
# Clone o repositório
git clone https://github.com/seu-usuario/rust-paris-transit-a-star.git

# Entre no diretório
cd rust-paris-transit-a-star

# Compile e execute
cargo run --release
```

### Compilação para Diferentes Plataformas

```bash
# Linux (padrão)
cargo build --release

# Windows
cargo build --release --target x86_64-pc-windows-gnu

# Executar em modo desenvolvimento
cargo run
```

## 📊 Dados do Sistema

### Tabelas de Distâncias

O sistema utiliza duas tabelas principais:

1. **Tabela 1 - Distâncias Diretas**: Distâncias em linha reta entre estações
2. **Tabela 2 - Distâncias Reais**: Distâncias reais das conexões existentes

### Tabela de Conexões

```
Matriz 14x14 representando as conexões entre estações:
- 0: Sem conexão
- 1: Linha Azul
- 2: Linha Amarela  
- 3: Linha Vermelha
- 4: Linha Verde
```

## 🏗️ Arquitetura do Projeto

```
src/
├── main.rs                 # Ponto de entrada da aplicação
├── algoritmo_a_estrela.rs   # Implementação do algoritmo A*
├── aplicacao_gui.rs         # Interface gráfica com egui
├── dados_metro.rs           # Carregamento dos dados CSV
└── grafo_metro.rs           # Estrutura do grafo do metrô

data/
├── tabela1_distancias_diretas.csv    # Distâncias em linha reta
├── tabela2_distancias_reais.csv      # Distâncias reais
└── tabela_linhas_conexao.csv         # Matriz de conexões
```

## 🛠️ Tecnologias Utilizadas

- **[Rust](https://www.rust-lang.org/)**: Linguagem de programação principal
- **[egui](https://github.com/emilk/egui)**: Framework para interface gráfica
- **[eframe](https://github.com/emilk/egui/tree/master/crates/eframe)**: Framework de aplicação para egui
- **[csv](https://docs.rs/csv/)**: Biblioteca para leitura de arquivos CSV

## 📈 Funcionalidades

- ✅ Seleção interativa de estação origem e destino
- ✅ Cálculo automático da rota mais rápida
- ✅ Visualização gráfica do trajeto
- ✅ Informações detalhadas sobre tempo de viagem
- ✅ Suporte a baldeações entre linhas
- ✅ Interface responsiva e moderna

## 🔧 Parâmetros do Sistema

- **Velocidade média dos trens**: 30 km/h
- **Tempo de baldeação**: 4 minutos
- **Total de estações**: 14
- **Total de linhas**: 4

## 📚 Estrutura de Dados

O projeto utiliza as seguintes estruturas principais:

- **Grafo**: Representação das estações e conexões
- **Nó A***: Estrutura para o algoritmo com valores f, g, h
- **Heap Binária**: Para otimizar a seleção do próximo nó
- **HashMap**: Para acesso rápido às estações

## 🤝 Contribuição

Contribuições são bem-vindas! Sinta-se à vontade para:

1. Fazer fork do projeto
2. Criar uma branch para sua feature (`git checkout -b feature/MinhaFeature`)
3. Commit suas mudanças (`git commit -m 'Adiciona MinhaFeature'`)
4. Push para a branch (`git push origin feature/MinhaFeature`)
5. Abrir um Pull Request

## 📝 Licença

Este projeto está sob a licença MIT. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.

## 👥 Autores

- **Seu Nome** - *Desenvolvimento inicial* - [@seu-usuario](https://github.com/seu-usuario)

## 🎓 Contexto Acadêmico

Este projeto foi desenvolvido como parte da disciplina de **Estrutura de Dados II**, com foco no estudo e implementação de **Algoritmos em Grafos**, especificamente o algoritmo A* para busca informada em grafos ponderados.

---

⭐ Se este projeto foi útil para você, considere dar uma estrela!