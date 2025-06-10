# Aplicação de Planejamento de Rotas do Metrô de Paris - Algoritmo A*

## 📖 Visão Geral

Esta aplicação é um **planejador de rotas inteligente** para o sistema de metrô de Paris, implementado em Rust com interface gráfica. Utiliza o algoritmo A* (A-star) para encontrar as rotas mais eficientes entre estações, considerando tempo de viagem, baldeações e conectividade da rede.

---

## 🏗️ Arquitetura da Aplicação

### **Módulos Principais:**

1. **`main.rs`** - Ponto de entrada da aplicação
2. **`grafo_metro.rs`** - Estrutura de dados do sistema de metrô  
3. **`algoritmo_a_estrela.rs`** - Implementação do algoritmo A* otimizado
4. **`aplicacao_gui.rs`** - Interface gráfica interativa
5. **`dados_metro.rs`** - Gerenciamento de dados das estações

---

## 🔄 Funcionamento Passo a Passo

### **Etapa 1: Inicialização da Aplicação**
```
1. Carregamento dos dados do metrô a partir de arquivos CSV:
   - tabela1_distancias_diretas.csv: Distâncias heurísticas entre todas as estações
   - tabela2_distancias_reais.csv: Tempos reais de viagem entre estações conectadas
   - tabela_linhas_conexao.csv: Informações sobre linhas e conexões

2. Construção do grafo do metrô:
   - 14 estações (E1 a E14)
   - 4 linhas coloridas (Azul, Amarela, Vermelha, Verde)
   - Conexões bidirecionais com tempos específicos

3. Abertura da interface gráfica (1024x768 pixels)
```

### **Etapa 2: Interface do Usuário**
```
Painel Lateral Esquerdo:
├── Controles de Busca
│   ├── ComboBox: Seleção da estação de início
│   ├── ComboBox: Seleção da estação de destino
│   ├── Botão: "Iniciar/Reiniciar Busca" (200x32px)
│   └── Botão: "Limpar Tudo" (200x32px)
│
├── Execução Passo a Passo (quando busca ativa)
│   ├── Botão: "Próximo Passo" (200x32px)
│   └── Botão: "Executar Tudo" (200x32px)
│
├── Opções de Visualização
│   ├── Slider: Controle de zoom (0.5x a 2.0x)
│   ├── Checkbox: "Mostrar Linha Atual"
│   └── Checkbox: "Mostrar Tempos entre Estações"
│
└── Resumo da Rota (quando encontrada)
    ├── Tempo total em minutos
    ├── Número de baldeações
    ├── Número de estações
    └── Trajeto completo detalhado

Área Central:
└── Mapa interativo do metrô
    ├── Estações representadas como círculos
    ├── Conexões entre estações como linhas coloridas
    ├── Animação em tempo real do algoritmo A*
    └── Popups informativos com dados do algoritmo
```

### **Etapa 3: Algoritmo A* em Ação**

#### **3.1 Preparação da Busca**
```
1. Validação das estações selecionadas
2. Inicialização das estruturas de dados:
   - Fronteira (BinaryHeap): Nós a serem explorados
   - Conjunto Fechado (HashSet): Nós já processados
   - Mapa de custos g: Custo real do início até cada nó
   
3. Adição do nó inicial à fronteira
```

#### **3.2 Execução Iterativa**
```
Para cada passo do algoritmo:

1. SELEÇÃO DO MELHOR NÓ:
   - Remove o nó com menor f-cost da fronteira
   - f = g + h (custo real + heurística)
   
2. VERIFICAÇÃO DE OBJETIVO:
   - Se nó atual = destino → SUCESSO
   - Reconstrói e retorna o caminho completo
   
3. EXPANSÃO DOS VIZINHOS:
   Para cada estação conectada:
   ├── Calcula novo custo g
   ├── Adiciona tempo de baldeação (4min) se mudança de linha
   ├── Aplica sistema de penalidades inteligentes:
   │   ├── Grau 1 (terminais): max(50.0, h*2.0) minutos
   │   ├── Grau 2 (baixa conectividade): h*0.3
   │   ├── Grau 3 (média conectividade): h*0.1
   │   └── Grau 4+ (boa conectividade): sem penalidade
   ├── Calcula heurística h (distância direta ao destino)
   ├── Computa f = g + h + penalidades
   └── Adiciona à fronteira se melhor caminho
   
4. ATUALIZAÇÃO VISUAL:
   - Destaca estação sendo expandida
   - Mostra vizinhos sendo analisados
   - Exibe valores f, g, h em popups
   - Atualiza estatísticas em tempo real
```

#### **3.3 Sistema de Penalidades Inteligentes**
```
PROBLEMA RESOLVIDO: 
- Nós terminais (becos sem saída) eram incorretamente priorizados
- Exemplo: E7 (f=39.6) escolhido antes de E4 (f=53.2)

SOLUÇÃO IMPLEMENTADA:
- Análise da conectividade de cada estação
- Penalidades graduadas baseadas no grau de conectividade
- Proteção para não penalizar o destino final
- Logs detalhados para debugging

RESULTADO:
- E4 agora corretamente priorizado (f=53.2 vs E7 f=89.6)
- Caminhos mais eficientes encontrados
- Comportamento mais intuitivo do algoritmo
```

### **Etapa 4: Visualização Interativa**

#### **4.1 Estados Visuais das Estações**
```
- INÍCIO: Círculo verde com borda destacada
- DESTINO: Círculo vermelho com borda destacada  
- EXPLORANDO: Círculo azul (parte do caminho sendo analisado)
- CAMINHO FINAL: Círculo verde-azul escuro
- ANALISANDO: Círculo laranja (vizinho sendo processado)
- PADRÃO: Círculo cinza
```

#### **4.2 Elementos Interativos**
```
- Hover sobre estações: Popup com informações A*
- Clique em estações: Popup persistente com detalhes
- Arrasto do mapa: Navegação livre
- Zoom com scroll: Controle de escala (0.5x a 2.0x)
- Popups arrastáveis: Organização da interface
```

#### **4.3 Animação do Algoritmo**
```
- Expansão passo a passo visualizada
- Valores f, g, h mostrados em tempo real
- Highlight de estações ativas
- Linha do caminho final destacada em verde-azul
- Ícones de baldeação com tempo (+4.0min)
```

### **Etapa 5: Resultado Final**

#### **5.1 Caminho Encontrado**
```
Informações exibidas:
├── Tempo total (em minutos)
├── Número de baldeações
├── Número de estações
├── Trajeto passo a passo:
│   ├── [INÍCIO] Nome da estação
│   ├── Estações intermediárias  
│   ├── [BALDEAÇÃO] Mudanças de linha destacadas
│   └── [FIM] Estação de destino
└── Linhas utilizadas com cores correspondentes
```

#### **5.2 Análise de Performance**
```
- Número de nós expandidos
- Tempo de execução do algoritmo
- Eficiência da heurística
- Qualidade da solução encontrada
```

---

## 🎯 Principais Funcionalidades

### **✅ Algoritmo A* Otimizado**
- Sistema de penalidades para nós terminais
- Heurística admissível baseada em distâncias reais
- Tratamento inteligente de baldeações
- Busca eficiente com heap binário

### **✅ Interface Gráfica Profissional**
- Botões padronizados de mesmo tamanho (200x32px)
- Layout organizado e intuitivo
- Popups informativos sem emojis
- Controles de zoom e visualização

### **✅ Visualização em Tempo Real**
- Animação passo a passo do algoritmo
- Estados visuais claros das estações
- Popups com dados do algoritmo A*
- Navegação interativa do mapa

### **✅ Análise Detalhada**
- Logs de debug do algoritmo
- Estatísticas de performance
- Comparação de rotas alternativas
- Informações de conectividade

---

## 📊 Dados do Sistema

### **Estações:** 14 (E1 a E14)
### **Linhas:** 4 (Azul, Amarela, Vermelha, Verde)  
### **Tempo de Baldeação:** 4.0 minutos
### **Arquivos de Dados:**
- `tabela1_distancias_diretas.csv` - Matriz 14x14 de distâncias heurísticas
- `tabela2_distancias_reais.csv` - Tempos reais entre estações conectadas
- `tabela_linhas_conexao.csv` - Mapeamento de conexões por linha

---

## 🚀 Exemplo de Uso Típico

```
1. Usuário abre a aplicação
2. Seleciona estação de início: "E6 - République"
3. Seleciona estação de destino: "E13 - Châtelet"
4. Clica em "Iniciar/Reiniciar Busca"
5. Algoritmo executa com visualização em tempo real
6. Resultado: E6 → E5 → E4 → E13 (61.6 min, 1 baldeação)
7. Usuário pode ver detalhes do caminho e estatísticas
```

---

## 🛠️ Tecnologias Utilizadas

- **Rust** - Linguagem de programação principal
- **egui/eframe** - Framework de interface gráfica
- **Algoritmo A*** - Busca heurística otimizada
- **CSV** - Formato de dados das estações
- **Estruturas de dados eficientes** - HashMap, BinaryHeap, HashSet

---

## 📈 Melhorias Implementadas (Versão 1.0)

1. **Correção crítica:** Sistema de penalidades para nós terminais
2. **Interface limpa:** Remoção de emojis unicode dos popups  
3. **Botões padronizados:** Layout mais profissional
4. **Visualização aprimorada:** Controles organizados e consistentes

A aplicação representa uma implementação completa e profissional de um sistema de planejamento de rotas, combinando algoritmos eficientes com uma interface gráfica intuitiva e informativa.
