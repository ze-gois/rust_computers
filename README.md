# computers

`computers` é uma crate Rust (biblioteca) para documentar e organizar experimentos e snippets relacionados a virtualização, passthrough de GPUs e configuração de ambientes de inferência (ex.: KVM/QEMU + VFIO + OVMF). O repositório serve tanto como uma crate reutilizável quanto como um diário de bordo (registro) das decisões e prompts que guiaram o processo.

Visite a documentação gerada em `docs/` (quando presente) — planejamos publicar o conteúdo via GitHub Pages para transformar o diário dinâmico em site estático.

Resumo rápido
- Crate: `computers`
- Repositório: https://github.com/ze-gois/rust_computers
- Objetivo: centralizar snippets, templates de VM, trechos de configuração e o registro das interações (prompts) usadas para gerar as instruções de passthrough e operação de LLMs em guests que usam GPU dedicada.

Conteúdo esperado
- `src/` — código da crate (atualmente contém utilitários mínimos)
- `docs/` — diário de bordo, prompts arquivados e snippets reutilizáveis
  - `docs/index.md` — índice do site/docs
  - `docs/diary.md` — entradas de diário (passos, resultados, logs)
  - `docs/prompts.md` — arquivo com as prompts/respostas relevantes (registro histórico)
  - `docs/snippets/` — trechos de configuração, exemplos de `vfio.conf`, `mkinitcpio` e trechos de XML do libvirt
- `Cargo.toml` — metadados e configuração de publicação de crate
- `LICENSE` — licença do projeto

Como construir e gerar docs localmente
- Build:
  - `cargo build`
- Testes:
  - `cargo test`
- Documentação:
  - `cargo doc --open` (gera a documentação da crate localmente)
- Publicar docs/ como site estático (planejado via GitHub Actions → `gh-pages`):
  - O fluxo recomendado será um action que copia o conteúdo de `docs/` para `gh-pages` em pushes na branch `main`.

Diário / registro das decisões
- Este repositório também é usado como um diário de bordo: cada passo crítico (ex.: ativar IOMMU, criar `vfio.conf`, editar `mkinitcpio.conf`, gerar XML do libvirt para `host-passthrough`) deve ser registrado em `docs/diary.md`.
- Incluíremos também um arquivo `docs/prompts.md` que armazena, de forma íntegra, os prompts e as respostas que conduziram as instruções — isto ajuda a manter rastreabilidade e facilita reprodutibilidade.

Contribuindo
- Abra uma issue para discutir mudanças grandes.
- Pull requests bem descritivos são bem-vindos — inclua contexto, passos para reproduzir e impacto.
- Para mudanças nos snippets de sistema (GRUB, `mkinitcpio`, `vfio.conf`, XML do libvirt) inclua sempre uma explicação das suposições (distro guest/host, endereços PCI placeholders, riscos e backups recomendados).

Notas importantes (segurança e risco)
- Modificações em GRUB, `mkinitcpio` e vinculação de drivers (`vfio`) podem tornar um sistema inoperante se aplicadas incorretamente. Sempre faça backup dos arquivos originais antes de aplicar mudanças e tenha um meio de recuperação (live USB).
- Ao aceitar snippets deste repositório, certifique-se de substituir placeholders (IDs PCI, vendor IDs, caminhos) pelos valores reais do seu hardware.

Contato
- Autor: José Gois <ze.gois.00@gmail.com>
- Repo: https://github.com/ze-gois/rust_computers

