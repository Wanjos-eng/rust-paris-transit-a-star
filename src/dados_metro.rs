// Trazemos todas as definições do módulo grafo_metro para o escopo deste arquivo.
// Isso inclui GrafoMetro, Estacao, CorLinha, NUMERO_ESTACOES, etc.
use crate::grafo_metro::*;
use std::error::Error;
use std::fs::File;
// A crate 'csv' precisa ser importada para usarmos suas funcionalidades.
use csv::ReaderBuilder;

// Continuamos a implementação de métodos para a struct GrafoMetro.
// Esta é uma forma de organizar o código: a definição da struct está em grafo_metro.rs,
// e os métodos relacionados ao carregamento de dados estão aqui, em dados_metro.rs.
impl GrafoMetro {
    /// Carrega os dados da Tabela 1 (distâncias diretas/heurísticas) de um arquivo CSV.
    /// Popula o campo `distancias_heuristicas_km` do `GrafoMetro`.
    ///
    /// # Argumentos
    /// * `caminho_arquivo`: Path para o arquivo CSV contendo as distâncias heurísticas.
    ///
    /// # Retorna
    /// `Ok(())` se o carregamento for bem-sucedido, ou um `Err` com uma descrição do erro.
    pub fn carregar_distancias_heuristicas(&mut self, caminho_arquivo: &str) -> Result<(), Box<dyn Error>> {
        let arquivo = File::open(caminho_arquivo)?; // Abre o arquivo. O '?' propaga erros.
        let mut leitor_csv = ReaderBuilder::new()
            .delimiter(b';') // Define o ponto-e-vírgula como delimitador de colunas.
            .has_headers(true) // Indica que o CSV tem uma linha de cabeçalho (Estacao;E1;E2;...).
            .from_reader(arquivo);

        // Itera sobre cada linha do CSV (registro).
        // `enumerate()` adiciona um índice `id_linha_atual` (0, 1, 2...).
        for (id_linha_atual, resultado_linha) in leitor_csv.records().enumerate() {
            // Garante que não lemos mais linhas do que o número de estações.
            if id_linha_atual >= NUMERO_ESTACOES {
                break;
            }
            let registro = resultado_linha?; // Obtém o registro da linha atual.

            // Itera sobre as colunas de dados desta linha (distâncias para E1, E2,...).
            // `NUMERO_ESTACOES` define quantas colunas de distância esperamos.
            for id_coluna_destino in 0..NUMERO_ESTACOES {
                // A primeira coluna do CSV é o nome da estação de origem (ex: "E1").
                // As colunas de dados numéricos começam a partir do índice 1 no `registro`.
                // Por isso, `indice_no_csv = id_coluna_destino + 1`.
                let indice_no_csv = id_coluna_destino + 1;

                if indice_no_csv < registro.len() { // Verifica se o índice é válido para o registro.
                    // Tenta obter o valor da célula como string.
                    match registro.get(indice_no_csv) {
                        Some(valor_str) if !valor_str.trim().is_empty() => {
                            // Se a string não estiver vazia, tenta convertê-la para f32.
                            // Substituímos "," por "." caso seus dados usem vírgula como separador decimal.
                            // No nosso CSV padronizado, já usamos ".", então a substituição pode não ser necessária.
                            match valor_str.trim().replace(',', ".").parse::<f32>() {
                                Ok(valor_f32) => {
                                    // Se a distância for maior que 0, armazena Some(valor).
                                    // Distância para si mesmo (0.0) será None aqui se parseada como 0.0
                                    // ou se o CSV tiver "0.0" explicitamente, será Some(0.0).
                                    // A lógica de h(n) deve tratar 0.0 para si mesmo.
                                    if valor_f32 > 0.0 || (id_linha_atual == id_coluna_destino && valor_f32 == 0.0) {
                                        self.distancias_heuristicas_km[id_linha_atual][id_coluna_destino] = Some(valor_f32);
                                    }
                                }
                                Err(e) => {
                                    // Se houver erro na conversão, imprime uma mensagem.
                                    eprintln!(
                                        "Aviso: Erro ao parsear heurística na linha {} (origem E{}), coluna {} (destino E{}): '{}' ({})",
                                        id_linha_atual + 1, id_linha_atual + 1,
                                        id_coluna_destino + 1, id_coluna_destino + 1,
                                        valor_str, e
                                    );
                                }
                            }
                        }
                        _ => { /* Célula vazia ou não encontrada, permanece None */ }
                    }
                }
            }
        }
        Ok(()) // Retorna Ok se tudo correu bem.
    }

    /// Carrega os dados da Tabela 2 (distâncias reais) e da Tabela de Linhas (cores das conexões)
    /// de arquivos CSV e popula a `lista_adjacencia` do `GrafoMetro`.
    pub fn carregar_conexoes(
        &mut self,
        caminho_dist_reais: &str,
        caminho_linhas_conexao: &str,
    ) -> Result<(), Box<dyn Error>> {
        // --- Passo 1: Ler as distâncias reais (Tabela 2) para uma matriz temporária ---
        let mut matriz_distancias_reais = vec![vec![None::<f32>; NUMERO_ESTACOES]; NUMERO_ESTACOES];
        let arquivo_dist = File::open(caminho_dist_reais)?;
        let mut leitor_dist = ReaderBuilder::new().delimiter(b';').has_headers(true).from_reader(arquivo_dist);

        for (id_linha_atual, resultado_linha) in leitor_dist.records().enumerate() {
            if id_linha_atual >= NUMERO_ESTACOES { break; }
            let registro = resultado_linha?;
            
            // println!("[DEBUG LEITURA TABELA2] Linha {}: {:?}", id_linha_atual + 1, registro);
            
            for id_coluna_destino in 0..NUMERO_ESTACOES {
                let indice_no_csv = id_coluna_destino + 1;
                if indice_no_csv < registro.len() {
                    if let Some(valor_str) = registro.get(indice_no_csv) {
                        // println!("[DEBUG TABELA2] Origem E{} -> Destino E{}: '{}'", 
                        //          id_linha_atual + 1, id_coluna_destino + 1, valor_str);
                        if !valor_str.trim().is_empty() {
                            if let Ok(valor_f32) = valor_str.trim().replace(',', ".").parse::<f32>() {
                                // println!("[DEBUG TABELA2] Parseia ok: {} ({})", valor_f32, valor_f32 > 0.0);
                                if valor_f32 > 0.0 {
                                    matriz_distancias_reais[id_linha_atual][id_coluna_destino] = Some(valor_f32);
                                    // println!("[DEBUG TABELA2] Adicionou distância E{} -> E{}: {:?}", 
                                    //          id_linha_atual + 1, id_coluna_destino + 1, valor_f32);
                                }
                            }
                        }
                    }
                }
            }
        }

        // Mantivemos este debug específico para E1->E2 e E2->E1 como referência útil
        println!("[DEBUG DADOS_METRO] matriz_distancias_reais[0][1] (E1->E2): {:?}", matriz_distancias_reais[0][1]);
        println!("[DEBUG DADOS_METRO] matriz_distancias_reais[1][0] (E2->E1): {:?}", matriz_distancias_reais[1][0]);

        // --- Passo 2: Ler as cores das linhas (Tabela de Linhas) e criar as Conexoes ---
        let arquivo_linhas = File::open(caminho_linhas_conexao)?;
        let mut leitor_linhas = ReaderBuilder::new().delimiter(b';').has_headers(true).from_reader(arquivo_linhas);

        // println!("\n[DEBUG LINHAS] Iniciando leitura da tabela de linhas\n");

        for (id_estacao_origem, resultado_linha) in leitor_linhas.records().enumerate() {
            if id_estacao_origem >= NUMERO_ESTACOES { break; }
            let registro_linha = resultado_linha?;
            
            // println!("[DEBUG LINHAS] Linha {}: {:?}", id_estacao_origem + 1, registro_linha);

            for id_estacao_destino in 0..NUMERO_ESTACOES {
                if id_estacao_origem == id_estacao_destino { continue; } // Não há conexões para si mesmo.

                let indice_no_csv = id_estacao_destino + 1;
                if indice_no_csv < registro_linha.len() {
                    if let Some(cor_linha_str) = registro_linha.get(indice_no_csv) {
                        // println!("[DEBUG LINHAS] E{} -> E{}: '{}'", 
                        //          id_estacao_origem + 1, id_estacao_destino + 1, cor_linha_str);
                        
                        if !cor_linha_str.trim().is_empty() {
                            if let Ok(cor_linha_int) = cor_linha_str.trim().parse::<u8>() {
                                // println!("[DEBUG LINHAS] Cor da linha: {}", cor_linha_int);
                                
                                if cor_linha_int > 0 { // Se o valor é > 0, existe uma linha.
                                    let cor_da_linha = CorLinha::de_inteiro(cor_linha_int);
                                    
                                    // println!("[DEBUG LINHAS] Converteu para enum: {:?}", cor_da_linha);
                                    
                                    if cor_da_linha == CorLinha::Nenhuma {
                                        eprintln!("Aviso: Cor de linha inválida ({}) entre E{} e E{}",
                                            cor_linha_int, id_estacao_origem + 1, id_estacao_destino + 1);
                                        continue;
                                    }

                                    // Verifica se existe uma distância real correspondente para esta conexão.
                                    // println!("[DEBUG LINHAS] Buscando distância real E{} -> E{}: {:?}", 
                                    //          id_estacao_origem + 1, id_estacao_destino + 1,
                                    //          matriz_distancias_reais[id_estacao_origem][id_estacao_destino]);
                                    
                                    if let Some(distancia_real_km) = matriz_distancias_reais[id_estacao_origem][id_estacao_destino] {
                                        let tempo_viagem_minutos = (distancia_real_km / VELOCIDADE_TREM_KMH) * 60.0;

                                        // Mantemos este debug como útil para informação sobre conexões adicionadas
                                        println!("[DEBUG LINHAS] ADICIONANDO CONEXÃO: E{} -> E{} (Cor {:?}, Dist: {:.2}km, Tempo: {:.2}min)",
                                                id_estacao_origem + 1, id_estacao_destino + 1, 
                                                cor_da_linha, distancia_real_km, tempo_viagem_minutos);
                                        
                                        self.lista_adjacencia[id_estacao_origem].push(Conexao {
                                            para_estacao: id_estacao_destino,
                                            cor_linha: cor_da_linha,
                                            distancia_km: distancia_real_km,
                                            tempo_minutos: tempo_viagem_minutos,
                                        });
                                    } else {
                                        // Mantemos este aviso sobre falta de correspondência entre linhas e distâncias
                                        println!("[DEBUG LINHAS] FALHOU: Sem distância real para E{} -> E{}", 
                                                 id_estacao_origem + 1, id_estacao_destino + 1);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Mantemos este resumo final que é muito útil para diagnóstico rápido
        println!("\n[DEBUG DADOS_METRO] Todas conexões para E1 (ID 0):");
        for (i, conexao) in self.lista_adjacencia[0].iter().enumerate() {
            println!("  Conexão {}: {:?}", i+1, conexao);
        }
        
        println!("[DEBUG DADOS_METRO] Conexões carregadas para E1 (ID 0): {:?}", self.lista_adjacencia[0]);
        Ok(())
    }
}