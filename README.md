# LogLineMotor

O **LogLineMotor** é um motor universal de trajetória executável, escrito em Rust, que transforma scripts `.lll` em ações com valor rastreável, proveniência forte e impacto real.

## 📌 Visão Geral

Este projeto é o núcleo do ecossistema LogLine. Ele interpreta `.lll` — uma linguagem de metacontratos vivos — e executa cada ação como um `Span`:

- Cada `Span` é auditável, assinável, com input/output/erro.
- O motor possui suporte completo a fluxo (`flow_id`, `workflow_state`) e transições autorizadas.
- É capaz de orquestrar humanos, LLMs, simulações e sistemas externos.
- É modular, multi-crate, e escalável para ambientes locais, offline, cloud e WASM.

## 🧱 Estrutura

O projeto é composto por múltiplos subcrates:

```
crates/
├── core/          # Span, Context, State
├── parser/        # Tokenização e AST de .lll
├── action/        # Ações executáveis: register, simulate, llm
├── flow/          # Workflow, transitions, state machine
├── vault/         # Persistência segura
├── provenance/    # Logger, assinatura, auditoria
├── llm/           # Integração com modelos de linguagem
├── simulate/      # Motores de simulação
├── ui_blocks/     # Saída visual estruturada
├── codex/         # Agente LLM-aware de raciocínio dirigido
```

## 🛠️ Execução local

Requisitos:
- Rust stable
- Cargo

Para rodar localmente:

```bash
cargo build
cargo run -p cli
```

## 🚧 Status
Versão: 0.1.0  
Progresso: Bootstrapping
