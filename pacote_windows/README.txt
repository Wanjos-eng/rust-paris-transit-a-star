METRÔ DE PARIS - PLANEJADOR DE ROTAS A*
=======================================

Este é um aplicativo de visualização e planejamento de rotas do Metrô de Paris
usando o algoritmo A* (A-estrela).

COMO USAR:
----------
1. Execute o arquivo "metro_paris_astar.exe"
2. A aplicação abrirá uma janela gráfica com o mapa do metrô
3. Clique em uma estação para selecioná-la como origem
4. Clique em outra estação para selecioná-la como destino
5. Use os botões para controlar a busca A*:
   - "Iniciar Busca A*": Inicia o algoritmo de busca
   - "Próximo Passo": Executa um passo da busca (modo passo-a-passo)
   - "Executar Completo": Executa toda a busca de uma vez
   - "Limpar": Limpa a busca atual

CONTROLES:
----------
- Clique e arraste para mover o mapa
- Use as opções de visualização para mostrar/ocultar elementos
- Passe o mouse sobre as estações para ver informações
- A rota encontrada será destacada em cores diferentes

ARQUIVOS NECESSÁRIOS:
--------------------
- metro_paris_astar.exe (o aplicativo principal)
- data/tabela1_distancias_diretas.csv (distâncias heurísticas)
- data/tabela2_distancias_reais.csv (distâncias reais entre estações)
- data/tabela_linhas_conexao.csv (informações das linhas do metrô)

IMPORTANTE: Mantenha todos os arquivos na mesma estrutura de pastas!

REQUISITOS:
-----------
- Windows 7 ou superior
- Nenhuma instalação adicional necessária (executável independente)

DESENVOLVIDO COM:
-----------------
- Linguagem: Rust
- GUI: egui/eframe
- Algoritmo: A* (A-estrela)

Para dúvidas ou problemas, entre em contato com o desenvolvedor.
