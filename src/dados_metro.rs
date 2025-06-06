use crate::grafo_metro::*;
use std::error::Error;
use std::fs::File;
use csv::ReaderBuilder;

impl GrafoMetro {
    pub fn carregar_distancias_heuristicas(&mut self, caminho_arquivo: &str) -> Result<(), Box<dyn Error>> {
        println!("Carregando distâncias heurísticas de: {}", caminho_arquivo);
        let arquivo = File::open(caminho_arquivo)?;
        let mut leitor_csv = ReaderBuilder::new()
            .delimiter(b';')
            .has_headers(true)
            .from_reader(arquivo);

        for (id_linha_atual, resultado_linha) in leitor_csv.records().enumerate() {
            if id_linha_atual >= NUMERO_ESTACOES { break; }
            let registro = resultado_linha?;
            for id_coluna_destino in 0..NUMERO_ESTACOES {
                let indice_no_csv = id_coluna_destino + 1;
                if indice_no_csv < registro.len() {
                    match registro.get(indice_no_csv) {
                        Some(valor_str) if !valor_str.trim().is_empty() => {
                            // Trata tanto ponto quanto vírgula como separador decimal
                            let valor_normalizado = valor_str.trim().replace(',', ".").replace(';', ".");
                            match valor_normalizado.parse::<f32>() {
                                Ok(valor_f32) => {
                                    if valor_f32 >= 0.0 {
                                        self.distancias_heuristicas_km[id_linha_atual][id_coluna_destino] = Some(valor_f32);
                                        // Depuração para os primeiros valores
                                        if id_linha_atual < 2 && id_coluna_destino < 2 {
                                            println!("Heurística E{} -> E{}: {}", 
                                                 id_linha_atual+1, id_coluna_destino+1, valor_f32);
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!(
                                        "Aviso: Erro ao parsear heurística na linha {} (E{}), coluna {} (E{}): '{}' ({})",
                                        id_linha_atual + 1, id_linha_atual + 1,
                                        id_coluna_destino + 1, id_coluna_destino + 1,
                                        valor_str, e
                                    );
                                }
                            }
                        }
                        _ => {} 
                    }
                }
            }
        }
        Ok(())
    }

    pub fn carregar_conexoes(
        &mut self,
        caminho_dist_reais: &str,
        caminho_linhas_conexao: &str,
    ) -> Result<(), Box<dyn Error>> {
        println!("Carregando distâncias reais de: {}", caminho_dist_reais);
        let mut matriz_distancias_reais = vec![vec![None::<f32>; NUMERO_ESTACOES]; NUMERO_ESTACOES];
        let arquivo_dist = File::open(caminho_dist_reais)?;
        let mut leitor_dist = ReaderBuilder::new().delimiter(b';').has_headers(true).from_reader(arquivo_dist);

        for (id_linha_atual, resultado_linha) in leitor_dist.records().enumerate() {
            if id_linha_atual >= NUMERO_ESTACOES { break; }
            let registro = resultado_linha?;
            for id_coluna_destino in 0..NUMERO_ESTACOES {
                let indice_no_csv = id_coluna_destino + 1;
                if indice_no_csv < registro.len() {
                    if let Some(valor_str) = registro.get(indice_no_csv) {
                        if !valor_str.trim().is_empty() {
                            // Trata tanto ponto quanto vírgula como separador decimal
                            let valor_normalizado = valor_str.trim().replace(',', ".").replace(';', ".");
                            if let Ok(valor_f32) = valor_normalizado.parse::<f32>() {
                                // Armazenamos somente se for maior que zero (ou diferente de -1)
                                if valor_f32 > 0.0 {
                                    matriz_distancias_reais[id_linha_atual][id_coluna_destino] = Some(valor_f32);
                                    // Depuração para os primeiros valores
                                    if id_linha_atual < 2 && id_coluna_destino < 2 {
                                        println!("Distância real E{} -> E{}: {}", 
                                             id_linha_atual+1, id_coluna_destino+1, valor_f32);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        println!("Carregando linhas de conexão de: {}", caminho_linhas_conexao);
        let arquivo_linhas = File::open(caminho_linhas_conexao)?;
        let mut leitor_linhas = ReaderBuilder::new().delimiter(b';').has_headers(true).from_reader(arquivo_linhas);

        for (id_estacao_origem, resultado_linha) in leitor_linhas.records().enumerate() {
            if id_estacao_origem >= NUMERO_ESTACOES { break; }
            let registro_linha = resultado_linha?;

            for id_estacao_destino in 0..NUMERO_ESTACOES {
                if id_estacao_origem == id_estacao_destino { continue; }

                let indice_no_csv = id_estacao_destino + 1;
                if indice_no_csv < registro_linha.len() {
                    if let Some(cor_linha_str) = registro_linha.get(indice_no_csv) {
                        if !cor_linha_str.trim().is_empty() {
                            if let Ok(cor_linha_int) = cor_linha_str.trim().parse::<u8>() {
                                if cor_linha_int > 0 {
                                    let cor_da_linha = CorLinha::de_inteiro(cor_linha_int);
                                    if cor_da_linha == CorLinha::Nenhuma {
                                        eprintln!("Aviso: Cor de linha inválida ({}) entre E{} e E{}",
                                            cor_linha_int, id_estacao_origem + 1, id_estacao_destino + 1);
                                        continue;
                                    }
                                    
                                    if let Some(distancia_real_km) = matriz_distancias_reais[id_estacao_origem][id_estacao_destino] {
                                        // Tempo direto conforme o código C++ (distância * 2)
                                        let tempo_viagem_minutos = distancia_real_km * 2.0;
                                        
                                        // Depuração para conexões importantes
                                        println!("CONEXÃO: E{} -> E{} (Linha: {:?}, Dist: {:.2}km, Tempo: {:.2}min)",
                                            id_estacao_origem+1, id_estacao_destino+1, cor_da_linha, 
                                            distancia_real_km, tempo_viagem_minutos);
                                        
                                        self.lista_adjacencia[id_estacao_origem].push(Conexao {
                                            para_estacao: id_estacao_destino,
                                            cor_linha: cor_da_linha,
                                            distancia_km: distancia_real_km,
                                            tempo_minutos: tempo_viagem_minutos, // Usando a conversão direta
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Imprime o número de conexões para todas as estações
        for i in 0..NUMERO_ESTACOES {
            println!("E{} tem {} conexões", i+1, self.lista_adjacencia[i].len());
        }
        
        Ok(())
    }
}