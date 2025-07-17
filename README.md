# 🚇 Paris Metro A* Route Planner

> Um sistema inteligente de planejamento de rotas para o Metrô de Paris usando o algoritmo A* (A-estrela)

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen?style=for-the-badge)]()
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey?style=for-the-badge)]()

<div align="center">
  <img src="mapa.jpg" alt="Mapa do Sistema de Metrô de Paris" width="600"/>
  <p><em>Visualização do sistema de metrô de Paris com 4 linhas e 14 estações</em></p>
</div>

## 📋 Sobre o Projeto

Este projeto implementa um **sistema inteligente de planejamento de rotas** para o Metrô de Paris, desenvolvido como parte da disciplina de **Estrutura de Dados II** com foco em **Algoritmos em Grafos**. O sistema utiliza o algoritmo A* (A-estrela) para encontrar o trajeto mais eficiente entre estações do metrô.

### 🎯 Características Principais

- **🎮 Interface Gráfica Interativa**: Desenvolvida com egui/eframe para visualização em tempo real
- **🧮 Algoritmo A* Otimizado**: Implementação eficiente do algoritmo de busca informada 
- **🚇 4 Linhas de Metrô**: Azul, Amarela, Vermelha e Verde com 14 estações interconectadas
- **⏱️ Cálculo de Tempo Real**: Considera velocidade dos trens (30km/h) e tempo de baldeação (4min)
- **📊 Visualização do Trajeto**: Mostra graficamente o melhor caminho com estatísticas detalhadas
- **🔍 Modo Passo a Passo**: Visualize o algoritmo funcionando em tempo real
- **📈 Análise de Performance**: Métricas detalhadas sobre a execução do algoritmo

### 🎓 Contexto Acadêmico

**Disciplina**: Estrutura de Dados II  
**Tema**: Algoritmos em Grafos  
**Linguagem**: Rust  
**Paradigma**: Programação Funcional com Segurança de Memória

## 🚀 Demonstração

### 🎮 Interface Principal
A aplicação oferece uma interface gráfica completa com:
- **Painel de Controle**: Seleção de origem e destino
- **Visualização do Mapa**: Representação gráfica das estações e linhas
- **Controles de Navegação**: Zoom, arrastar e rotação
- **Painel de Resultados**: Estatísticas detalhadas da rota encontrada

### 🎯 Exemplo de Uso
```
Origem: E6 (Linha Azul)
Destino: E13 (Linha Vermelha)
Resultado: E6 → E5 → E8 → E9 → E3 → E13
Tempo Total: 22.4 minutos
Baldeações: 2 (Azul→Amarela, Amarela→Vermelha)
```

## 🧮 Funcionamento do Algoritmo A*

O algoritmo A* é uma extensão do algoritmo de Dijkstra que utiliza uma heurística para guiar a busca em direção ao objetivo, tornando-a mais eficiente.

### 🔄 Fluxo do Algoritmo

1. **Inicialização**: 
   - Coloca a estação origem na lista aberta (fronteira) com f(n) = 0
   - Lista fechada (explorados) começa vazia

2. **Seleção do Nó**: 
   - Escolhe o nó com menor valor f(n) = g(n) + h(n) da lista aberta
   - Remove o nó da lista aberta e adiciona na lista fechada

3. **Teste de Objetivo**: 
   - Se o nó atual é o destino, reconstrói e retorna o caminho
   - Caso contrário, continua para expansão

4. **Expansão**: 
   - Gera todos os vizinhos (estações conectadas) do nó atual
   - Para cada vizinho, calcula os custos g(n) e h(n)

5. **Avaliação dos Vizinhos**: 
   - Se vizinho já está na lista fechada, ignora
   - Se vizinho não está na lista aberta, adiciona
   - Se vizinho já está na lista aberta com custo maior, atualiza

6. **Finalização**: 
   - Repete até encontrar o destino ou lista aberta ficar vazia
   - Retorna o caminho ótimo ou "sem solução"

### 📊 Funções de Custo

- **g(n)**: Tempo real acumulado da origem até a estação atual
  - Inclui tempo de viagem + tempo de baldeação (se necessário)
- **h(n)**: Heurística - tempo estimado da estação atual até o destino  
  - Baseada na distância euclidiana e velocidade média (30km/h)
- **f(n)**: Função de avaliação total (f = g + h)
  - Prioriza estações com menor custo total estimado

### ⚡ Otimizações Implementadas

- **Heap Binária**: Para seleção eficiente do próximo nó (O(log n))
- **HashMap**: Para acesso rápido às estações (O(1))
- **Heurística Admissível**: Garante solução ótima
- **Detecção de Baldeação**: Penaliza trocas de linha adequadamente

## 🗺️ Estrutura do Sistema de Metrô

### 🚇 Rede de Transporte

O sistema modela uma versão simplificada do metrô de Paris com:
- **14 Estações**: Numeradas de E1 a E14
- **4 Linhas**: Cada uma com cor específica e trajeto próprio
- **Conexões Inteligentes**: Baldeações automáticas entre linhas

### 🎨 Linhas Disponíveis

| Linha | Cor | Estações | Descrição |
|-------|-----|----------|-----------|
| 🔵 **Linha Azul** | `cor=1` | E1, E2, E3, E4, E5, E6 | Linha principal horizontal |
| 🟡 **Linha Amarela** | `cor=2` | E2, E5, E7, E8, E9, E10 | Conexão diagonal |
| 🔴 **Linha Vermelha** | `cor=3` | E3, E9, E11, E13 | Linha vertical |
| 🟢 **Linha Verde** | `cor=4` | E4, E8, E12, E13, E14 | Linha de conexão |

### 🔗 Pontos de Baldeação

Estações que conectam múltiplas linhas:
- **E2**: Azul ↔ Amarela
- **E3**: Azul ↔ Vermelha  
- **E4**: Azul ↔ Verde
- **E5**: Azul ↔ Amarela
- **E8**: Amarela ↔ Verde
- **E9**: Amarela ↔ Vermelha
- **E13**: Vermelha ↔ Verde

### 📈 Exemplo de Rota Complexa

**Caso de Teste**: E6 (Linha Azul) → E13 (Linha Vermelha)

```
Caminho Ótimo Encontrado:
E6 [Azul] → E5 [Azul] → E8 [Amarela] → E9 [Amarela] → E3 [Vermelha] → E13 [Vermelha]

Detalhes:
- Tempo Total: 22.4 minutos
- Baldeações: 2 (Azul→Amarela em E8, Amarela→Vermelha em E9)  
- Distância Total: 47.2 km
- Estações Percorridas: 6
```

## 🚀 Instalação e Execução

### 📋 Pré-requisitos

#### 1. 🦀 Rust Toolchain (Obrigatório)

**Instalação via rustup (Recomendado):**
```bash
# Linux/macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Verificar instalação
rustc --version
cargo --version
```

**Windows:**
1. Baixe e execute o instalador: [rustup.rs](https://rustup.rs/)
2. Siga as instruções do instalador
3. Reinicie o prompt de comando

#### 2. 🖥️ Dependências do Sistema

**Ubuntu/Debian:**
```bash
sudo apt update
sudo apt install build-essential pkg-config libfontconfig1-dev
sudo apt install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
sudo apt install libxkbcommon-dev libssl-dev
```

**Fedora/RHEL:**
```bash
sudo dnf groupinstall "Development Tools"
sudo dnf install pkg-config fontconfig-devel openssl-devel
sudo dnf install libxkbcommon-devel libX11-devel
```

**macOS:**
```bash
# Instalar Xcode command line tools
xcode-select --install

# Opcional: pkg-config via Homebrew
brew install pkg-config
```

**Windows:**
- Visual Studio Build Tools 2019+ ou Visual Studio Community
- Alternativamente: MSYS2/MinGW-w64

### 🚀 Executar o Projeto

#### Método Principal (Recomendado)
```bash
# 1. Clone o repositório
git clone https://github.com/Wanjos-eng/rust-paris-transit-a-star.git

# 2. Entre no diretório
cd rust-paris-transit-a-star

# 3. Execute o projeto
cargo run --release
```

#### Compilar Executável (Opcional)
```bash
# Compilar versão otimizada
cargo build --release

# Executar o binário
./target/release/metro_paris_astar  # Linux/macOS
.\target\release\metro_paris_astar.exe  # Windows
```

### 🔧 Verificação do Projeto

```bash
# Verificar se compila sem erros
cargo check

# Ver warnings e sugestões
cargo clippy

# Formatar código
cargo fmt

# Limpar builds anteriores
cargo clean
```

### 📦 Requisitos Mínimos

- **RAM**: 2GB (4GB recomendado)
- **Espaço**: 500MB para projeto + dependências
- **OS**: Linux (Ubuntu 18.04+), macOS 10.12+, Windows 10+
- **GPU**: Suporte a OpenGL 3.0+ (placa integrada serve)

### 🐛 Solução de Problemas

**Erro de compilação no Linux:**
```bash
# Instalar dependências ausentes
sudo apt install libfontconfig1-dev pkg-config
```

**Erro de linker no Windows:**
```bash
# Instalar Visual Studio Build Tools
# https://visualstudio.microsoft.com/visual-cpp-build-tools/
```

**Performance lenta:**
```bash
# Sempre usar release para melhor performance
cargo run --release
```

## 🎮 Como Usar a Interface

### 🚀 Iniciando a Aplicação
```bash
# Execute o comando no diretório do projeto
cargo run --release
```

### 🎯 Planejando uma Rota

#### 1. **Seleção de Estações**
- Use o dropdown **"Estação de Origem"** no painel lateral esquerdo
- Selecione a **"Estação de Destino"** no dropdown correspondente
- As estações são numeradas de E1 a E14

#### 2. **Executando o Algoritmo**
- Clique em **"Iniciar A*"** para começar a busca
- Use **"Próximo Passo"** para visualizar o algoritmo passo a passo
- Ou clique **"Executar Tudo"** para ver o resultado final

#### 3. **Navegação no Mapa**
- **Zoom**: Use a roda do mouse ou o slider no painel
- **Arrastar**: Clique e arraste com o botão esquerdo do mouse
- **Informações**: Clique em qualquer estação para ver detalhes

### 🔍 Controles Disponíveis

| Controle | Função |
|----------|---------|
| **Iniciar A*** | Inicia a busca do melhor caminho |
| **Próximo Passo** | Avança um passo no algoritmo |
| **Executar Tudo** | Executa o algoritmo completo |
| **Limpar Tudo** | Remove marcadores e reinicia |
| **Mostrar Tempos** | Exibe tempos nas conexões |
| **Mostrar Status** | Mostra estado das estações |
| **Controle de Zoom** | Ajusta o nível de aproximação |

### 📊 Interpretando os Resultados

#### **Cores das Estações**
- 🟢 **Verde**: Estação de origem
- 🔴 **Vermelho**: Estação de destino
- 🟡 **Amarelo**: Estação sendo analisada
- 🔵 **Azul**: Estação na lista aberta (fronteira)
- ⚫ **Cinza**: Estação já explorada

#### **Caminho Encontrado**
- **Linha Verde Grossa**: Caminho ótimo encontrado
- **Informações no Painel**: Tempo total, baldeações, distância
- **Lista de Estações**: Trajeto completo com linhas utilizadas

### 🎯 Exemplo Prático

```
Origem: E1 (Linha Azul)
Destino: E14 (Linha Verde)

Resultado:
E1 → E2 → E5 → E8 → E14
Tempo Total: 18.7 minutos
Baldeações: 2
Linhas: Azul → Amarela → Verde
```

## 📊 Dados do Sistema

### 🗃️ Estrutura de Dados

O projeto utiliza três arquivos CSV principais:

#### 1. **Distâncias Diretas** (`tabela1_distancias_diretas.csv`)
```csv
;E1;E2;E3;E4;E5;E6;E7;E8;E9;E10;E11;E12;E13;E14
E1;0.0;10.0;18.5;24.8;36.4;38.8;35.8;25.4;17.6;9.1;16.7;27.3;27.6;29.8
E2;10.0;0.0;8.5;14.8;26.6;29.1;26.1;17.3;10.0;3.5;15.5;20.9;19.1;21.8
...
```
- **Função**: Heurística para o algoritmo A*
- **Conteúdo**: Distâncias em linha reta entre todas as estações
- **Uso**: Cálculo de h(n) - estimativa de custo até o destino

#### 2. **Distâncias Reais** (`tabela2_distancias_reais.csv`)
```csv
;E1;E2;E3;E4;E5;E6;E7;E8;E9;E10;E11;E12;E13;E14
E1;0.0;12.5;0.0;0.0;0.0;0.0;0.0;0.0;0.0;0.0;0.0;0.0;0.0;0.0
E2;12.5;0.0;8.2;0.0;0.0;0.0;0.0;0.0;15.3;18.7;0.0;0.0;0.0;0.0
...
```
- **Função**: Distâncias reais das conexões existentes
- **Conteúdo**: Distâncias apenas entre estações conectadas (0.0 = sem conexão)
- **Uso**: Cálculo de g(n) - custo real acumulado

#### 3. **Conexões por Linha** (`tabela_linhas_conexao.csv`)
```csv
;E1;E2;E3;E4;E5;E6;E7;E8;E9;E10;E11;E12;E13;E14
E1;0;1;0;0;0;0;0;0;0;0;0;0;0;0
E2;1;0;1;0;0;0;0;0;2;2;0;0;0;0
...
```
- **Função**: Matriz de conectividade por linha
- **Valores**: 0=sem conexão, 1=Azul, 2=Amarela, 3=Vermelha, 4=Verde
- **Uso**: Detecção de baldeações e validação de rotas

### 📏 Parâmetros do Sistema

| Parâmetro | Valor | Descrição |
|-----------|-------|-----------|
| **Velocidade dos Trens** | 30 km/h | Velocidade média entre estações |
| **Tempo de Baldeação** | 4 minutos | Tempo para trocar de linha |
| **Total de Estações** | 14 | Estações numeradas de E1 a E14 |
| **Total de Linhas** | 4 | Azul, Amarela, Vermelha, Verde |
| **Algoritmo** | A* | Busca informada com heurística |

## 🏗️ Arquitetura do Projeto

### 📁 Estrutura de Diretórios

```
rust-paris-transit-a-star/
├── 📄 Cargo.toml                      # Configuração do projeto e dependências
├── 📄 Cargo.lock                      # Lock das versões das dependências
├── 📄 README.md                       # Documentação principal
├── 📄 LICENSE                         # Licença MIT
├── 🖼️ mapa.jpg                        # Imagem ilustrativa do metrô
├── 📁 src/                           # Código fonte principal
│   ├── 📄 main.rs                    # → Ponto de entrada da aplicação
│   ├── 📄 algoritmo_a_estrela.rs     # → Implementação completa do A*
│   ├── 📄 dados_metro.rs             # → Carregamento e parsing dos CSVs
│   ├── 📄 grafo_metro.rs             # → Estrutura do grafo e conexões
│   └── 📁 egui/                      # → Módulos da interface gráfica
│       ├── 📄 mod.rs                 # → Declarações públicas dos módulos
│       ├── 📄 app.rs                 # → Estrutura principal da aplicação
│       ├── 📄 controls.rs            # → Painel lateral de controles
│       ├── 📄 drawing.rs             # → Renderização do grafo visual
│       ├── 📄 navigation.rs          # → Zoom, arrastar, interações
│       ├── 📄 popups.rs              # → Tooltips e janelas informativas
│       ├── 📄 state_manager.rs       # → Estado do algoritmo A*
│       └── 📄 visual_effects.rs      # → Marcadores e efeitos visuais
├── 📁 data/                          # Dados do sistema de metrô
│   ├── 📄 tabela1_distancias_diretas.csv    # → Heurística (linha reta)
│   ├── 📄 tabela2_distancias_reais.csv      # → Distâncias das conexões
│   └── 📄 tabela_linhas_conexao.csv         # → Matriz de conectividade
└── 📁 target/ 🚫                     # Binários compilados (NÃO no Git)
    ├── 📁 debug/                     # → Builds de desenvolvimento
    └── 📁 release/                   # → Builds otimizados
```

### 🧩 Módulos e Responsabilidades

#### **Core do Sistema**
- **`main.rs`**: Inicialização da aplicação egui e loop principal
- **`algoritmo_a_estrela.rs`**: Lógica completa do algoritmo A* com heap binária
- **`grafo_metro.rs`**: Estrutura de dados do grafo, nós e conexões
- **`dados_metro.rs`**: Leitura dos CSVs e construção das estruturas de dados

#### **Interface Gráfica (egui/)**
- **`app.rs`**: Estrutura principal (`MetroApp`) e loop de renderização
- **`controls.rs`**: Painel lateral com dropdowns, botões e configurações
- **`drawing.rs`**: Renderização das estações, linhas e caminhos no canvas
- **`navigation.rs`**: Controles de zoom, arrastar e transformações de coordenadas
- **`popups.rs`**: Janelas modais com informações detalhadas das estações
- **`state_manager.rs`**: Gerenciamento de estado do algoritmo (executando, pausado, etc.)
- **`visual_effects.rs`**: Marcadores visuais, cores e animações

### 🔄 Fluxo de Dados

```
┌─ Início da Aplicação ─┐
│                       │
▼                       │
📄 main.rs              │
├── Inicializa eframe   │
├── Carrega dados CSV   │
└── Cria MetroApp       │
                        │
▼                       │
🎮 Interface Gráfica    │
├── Renderiza mapa      │
├── Processa eventos    │
└── Atualiza estado     │
                        │
▼                       │
🧮 Algoritmo A*         │
├── Recebe origem/dest  │
├── Executa busca       │
├── Retorna caminho     │
└── Atualiza visual     │
                        │
▼                       │
📊 Exibição Resultado   │
├── Caminho ótimo       │
├── Tempo total         │
└── Estatísticas        └─┘
```

## 🛠️ Tecnologias e Dependências

### 🦀 Rust Ecosystem
- **[Rust](https://www.rust-lang.org/)** `1.70+`: Linguagem de programação principal
- **[Cargo](https://doc.rust-lang.org/cargo/)**: Sistema de build e gerenciador de dependências

### 🖼️ Interface Gráfica
- **[egui](https://github.com/emilk/egui)** `0.31.1`: Framework para interface gráfica imediata
- **[eframe](https://github.com/emilk/egui/tree/master/crates/eframe)** `0.31.1`: Framework de aplicação para egui

### 📊 Processamento de Dados
- **[csv](https://docs.rs/csv/)** `1.3.1`: Biblioteca para leitura de arquivos CSV

### 🏗️ Estruturas de Dados
- **[std::collections](https://doc.rust-lang.org/std/collections/)**: HashMap, HashSet, BinaryHeap
- **[std::rc::Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html)**: Compartilhamento de referências

### 📦 Configuração das Dependências

```toml
[package]
name = "metro_paris_astar"
version = "0.1.0"
edition = "2024"

[dependencies]
csv = "1.3.1"       # Para ler arquivos CSV
eframe = "0.31.1"   # Para a interface gráfica com egui (framework)
egui = "0.31.1"     # Biblioteca de interface gráfica imediata

# Para Windows: compilação estática
[target.x86_64-pc-windows-gnu.dependencies]
winapi = { version = "0.3", features = ["everything"] }
```

## 🤝 Contribuição

Contribuições são muito bem-vindas! Este é um projeto educacional em constante evolução.

### 🔧 Para Desenvolvedores

#### Configuração do Ambiente
```bash
# Clone o repositório
git clone https://github.com/Wanjos-eng/rust-paris-transit-a-star.git
cd rust-paris-transit-a-star

# Instale ferramentas de desenvolvimento
rustup component add rustfmt rust-analyzer clippy

# Verifique o código
cargo check
cargo clippy
cargo fmt
```

#### Como Contribuir
1. **Fork** do projeto no GitHub
2. **Clone** seu fork
3. **Crie uma branch** para sua feature: `git checkout -b feature/MinhaFeature`
4. **Implemente** suas mudanças
5. **Teste** suas mudanças: `cargo test && cargo run --release`
6. **Commit** suas mudanças: `git commit -m 'Adiciona MinhaFeature'`
7. **Push** para sua branch: `git push origin feature/MinhaFeature`
8. **Abra um Pull Request**

### 💡 Ideias para Contribuições

- ✨ Adicionar mais estações e linhas do metrô real de Paris
- 🎨 Melhorar a interface visual e animações
- 📊 Implementar outros algoritmos de busca (Dijkstra, BFS, DFS)
- 🔧 Adicionar testes automatizados
- 📱 Criar versão para mobile/web com egui
- 🌐 Internacionalização (i18n) para múltiplos idiomas
- 📈 Adicionar métricas de performance do algoritmo
- 🗺️ Integração com dados reais da RATP (metrô de Paris)

### 📝 Padrões de Código
- Use `cargo fmt` antes de cada commit
- Execute `cargo clippy` para verificar boas práticas
- Mantenha comentários em português para consistência
- Siga as convenções de nomenclatura do Rust

## 🆘 Suporte e Dúvidas

### 📞 Onde Buscar Ajuda
- **Issues do GitHub**: Para bugs e solicitações de features
- **Discussions**: Para dúvidas gerais sobre o projeto
- **Rust Community**: [Forum oficial do Rust](https://users.rust-lang.org/)

### 🐛 Reportando Bugs
Ao reportar um bug, inclua:
1. **Sistema operacional** e versão
2. **Versão do Rust** (`rustc --version`)
3. **Passos para reproduzir** o problema
4. **Mensagens de erro** completas
5. **Screenshots** se relevante

### 💬 Perguntas Frequentes

**P: O programa não abre no Linux**  
R: Instale as dependências: `sudo apt install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev`

**P: A compilação demora muito**  
R: Normal na primeira vez. Use `cargo build --release` para compilações futuras mais rápidas.

**P: Posso usar outros algoritmos?**  
R: Sim! O projeto foi estruturado para facilitar a adição de novos algoritmos de busca.

**P: Como adicionar mais estações?**  
R: Edite os arquivos CSV na pasta `data/` seguindo o formato existente.

## 🎓 Contexto Educacional

### 📚 Objetivos de Aprendizagem
Este projeto demonstra:
- **Algoritmos de Busca em Grafos**: Implementação prática do A*
- **Estruturas de Dados Avançadas**: Uso de heaps, hashmaps e grafos
- **Programação Funcional**: Padrões do Rust para segurança de memória
- **Interface Gráfica**: Desenvolvimento de UI responsiva com egui
- **Engenharia de Software**: Organização modular e documentação

### 🎯 Aplicações Práticas
- Sistemas de navegação GPS
- Planejamento de rotas em jogos
- Otimização de redes de transporte
- Algoritmos de roteamento em redes

### 📖 Conceitos Abordados
- **Busca Informada vs. Não-informada**
- **Heurísticas admissíveis**
- **Complexidade temporal e espacial**
- **Otimalidade de algoritmos**
- **Programação orientada a eventos (GUI)**

### 👨‍🏫 Para Educadores
Este projeto pode ser usado como:
- Exemplo prático de implementação do A*
- Base para exercícios de otimização
- Demonstração de aplicações reais de grafos
- Introdução ao desenvolvimento em Rust

## 📝 Licença

Este projeto está licenciado sob a **Licença MIT** - veja o arquivo [LICENSE](LICENSE) para detalhes completos.

### 📋 Resumo da Licença
- ✅ **Uso comercial** permitido
- ✅ **Modificação** permitida  
- ✅ **Distribuição** permitida
- ✅ **Uso privado** permitido
- ❌ **Garantia** não fornecida
- ❌ **Responsabilidade** do autor limitada

## 👥 Autores e Reconhecimentos

### 👨‍💻 Desenvolvimento Principal
- **Wanjos-eng** - *Desenvolvimento inicial e arquitetura* - [@Wanjos-eng](https://github.com/Wanjos-eng)

### 🙏 Agradecimentos
- **Rust Foundation** - Pela linguagem incrível
- **egui Community** - Pelo framework de interface
- **RATP** - Pelos dados inspiradores do metrô de Paris
- **Professores e Colegas** - Pelo feedback e sugestões

### 🏆 Inspirações
- Algoritmos clássicos de busca em grafos
- Sistemas reais de navegação urbana
- Visualizações interativas de algoritmos

---

<div align="center">

### 🌟 Se este projeto foi útil para você, considere dar uma estrela! ⭐

**Desenvolvido com ❤️ em Rust para fins educacionais**

![Rust](https://img.shields.io/badge/Made%20with-Rust-orange?style=for-the-badge&logo=rust)
![MIT License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)
![Open Source](https://img.shields.io/badge/Open%20Source-❤️-red?style=for-the-badge)

</div>