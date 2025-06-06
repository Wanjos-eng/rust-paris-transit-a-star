@echo off
title Metro de Paris - Planejador de Rotas A*
echo.
echo ===================================================
echo   METRO DE PARIS - PLANEJADOR DE ROTAS A*
echo ===================================================
echo.
echo Iniciando aplicacao...
echo.

REM Verifica se o executavel existe
if not exist "metro_paris_astar.exe" (
    echo ERRO: Arquivo metro_paris_astar.exe nao encontrado!
    echo Verifique se todos os arquivos estao na mesma pasta.
    pause
    exit /b 1
)

REM Verifica se a pasta data existe
if not exist "data" (
    echo ERRO: Pasta 'data' nao encontrada!
    echo Verifique se a pasta com os arquivos CSV esta presente.
    pause
    exit /b 1
)

REM Executa o aplicativo
metro_paris_astar.exe

REM Se houver erro na execucao
if errorlevel 1 (
    echo.
    echo ERRO: O aplicativo encontrou um problema durante a execucao.
    echo Verifique se:
    echo - Todos os arquivos CSV estao na pasta 'data'
    echo - Seu Windows eh compativel (Windows 7 ou superior)
    echo - Nao ha arquivos bloqueados por antivirus
    echo.
    pause
)
