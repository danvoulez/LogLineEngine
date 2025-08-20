# Task List for OpenAI Codex

Projeto: LogLineEngine

Objetivo: Criar um motor universal de execução `.lll`, com rastreabilidade, execução modular, spans e fluxo, em Rust.
Arquitetura mínima: 500 arquivos, multi-crate, modular e escalável.

---

## 0. Setup de workspace
1. Crie uma pasta chamada `logline_motor/`
2. Dentro dela, inicialize um workspace com `cargo init --lib`
3. Crie um arquivo `Cargo.toml` com:
   - `workspace.members` listando os subcrates: `core`, `parser`, `execution`, `action`, `flow`, `vault`, `provenance`, `llm`, `simulate`, `ui_blocks`, `cli`, `api`, `wasm`, `codex`
4. Crie cada subpasta com seu respectivo `Cargo.toml` inicial e `lib.rs`

## 1. Núcleo core/
Objetivo: definir tipos, estruturas e contexto de execução do motor.
1. Crie os diretórios: `core/span/`, `core/context/`, `core/state/`, `core/types/`
2. Em `core/span/`, implemente:
   - `span.rs`: struct `Span` com id, input, output, error, timestamp
   - `span_id.rs`: UUID com checksum (use `uuid` + `sha2`)
   - `span_meta.rs`, `span_tags.rs`, `span_status.rs`
3. Em `core/context/`, crie:
   - `context.rs`: `ExecutionContext` com memória temporária
   - `scope.rs`: escopo de execução isolado
4. Em `core/state/`, crie:
   - `state.rs`: estado persistente
   - `snapshot.rs`, `diff.rs`
5. Em `core/types/`, crie:
   - `identifiers.rs`, `timestamps.rs`, `contracts.rs`, `values.rs`

## 2. Parser .lll/
Objetivo: interpretar scripts `.lll` em AST para execução.
1. Crie os diretórios: `parser/tokenizer/`, `parser/ast/`, `parser/validator/`
2. Em `tokenizer/`:
   - `tokenizer.rs`, `ident.rs`, `symbol.rs`, `string.rs`, `number.rs`
3. Em `ast/`:
   - `node.rs`, `action_node.rs`, `flow_node.rs`, `block_node.rs`
4. Crie `loader.rs` para entrada `.lll` (arquivo/string)
5. Crie `validator.rs` para validação sintática e semântica

## 3. Executor
Objetivo: executar ações `.lll` como spans.
1. Crie `executor.rs` com loop principal:
   - carrega `.lll`
   - executa ação por ação
   - registra cada como `Span`
2. Crie módulos: `runner.rs`, `dispatcher.rs`, `parallel.rs`, `retry.rs`, `fallback.rs`

## 4. Ações (action/)
Objetivo: definir todos os tipos de ações executáveis.
1. Crie `action/mod.rs` com trait `Action`
2. Crie subpastas:
   - `register/` com `create.rs`, `transition.rs`, `complete.rs`
   - `simulate/` com `run.rs`, `seed.rs`, `env.rs`
   - `llm/` com `ask.rs`, `embed.rs`, `summarize.rs`, `classify.rs`
   - `flow/` com `pause.rs`, `resume.rs`, `next.rs`
   - `aux/` com `log.rs`, `assert.rs`, `wait.rs`

## 5. Flow Engine
Objetivo: controlar trajetória e transições de workflow.
1. Crie `flow_engine.rs`, `workflow.rs`, `flow_step.rs`
2. Crie `conditions/` com `if.rs`, `when.rs`, `match.rs`, `switch.rs`, `loop.rs`
3. Crie `state_machine.rs` com enums de estados

## 6. Vault
Objetivo: memória de estado persistente, segura e versionável.
1. Crie drivers:
   - `sled.rs`, `sqlite.rs`, `remote_api.rs`, `in_memory.rs`
2. Crie:
   - `vault.rs`, `vault_cache.rs`, `vault_index.rs`, `vault_snapshot.rs`

## 7. Proveniência
Objetivo: garantir integridade, auditoria e assinatura de spans.
1. Crie:
   - `logger.rs`, `signer.rs`, `verifier.rs`, `checksum.rs`
   - `auditor.rs`, `trace.rs`, `provenance_snapshot.rs`

## 8. Simulação
Objetivo: executar ambientes simulados como ações válidas.
1. Crie `engine.rs`
2. Crie:
   - `physics.rs`, `economy.rs`, `social.rs`, `bio.rs`
   - `simulate_env.rs`, `simulate_seed.rs`, `simulate_record.rs`

## 9. LLM
Objetivo: acionar modelos locais ou externos para raciocínio, sumarização, embedding.
1. Crie:
   - `llm_client.rs`, `prompt.rs`, `embedding.rs`, `completion.rs`
   - `context_window.rs`, `token_filter.rs`, `llm_memory.rs`

## 10. UI Blocks
Objetivo: gerar interface executável a partir da trajetória.
1. Crie:
   - `text.rs`, `table.rs`, `timeline.rs`, `map.rs`, `choice.rs`
   - `block.rs` com enum `BlockType`
   - `renderer.rs` com output JSON

## 11. Codex (autonomia semântica)
Objetivo: IA que interpreta contexto e sugere `.lll` executável.
1. Crie `codex/` com:
   - `intent.rs`, `planner.rs`, `reasoning.rs`, `completion.rs`, `translator.rs`
   - `span_suggestion.rs`, `context_injector.rs`

## 12. Interface (CLI, API, WASM)
1. `cli/`: `main.rs`, `args.rs`, `interactive.rs`
2. `api/`: `server.rs`, `routes/`, `handlers.rs`, `health.rs`
3. `wasm/`: `bindings.rs`, `runner.rs`

## Finalização
1. Criar `.lll` de exemplo: `hello.lll`, `simulacao.lll`, `workflow.lll`
2. Criar testes unitários e integração
3. Testar build com `cargo check`, `cargo test`, `cargo run`

## Observações
- Use `#[derive(Debug, Clone, Serialize, Deserialize)]` sempre que aplicável
- Mantenha `mod.rs` ou `mod modname;` para cada subdiretório
- Evite dependências pesadas; prefira: `serde`, `uuid`, `chrono`, `sha2`, `ed25519-dalek`, `tokio`, `sled`, `reqwest`, `anyhow`
- Prefira granularidade de arquivos extrema
- Ao sugerir `.lll`, gere também `Span` previsto
- Tudo que for gerado, tem que ser assinado, registrado e auditável
