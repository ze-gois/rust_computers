# Computers — Documentação

Bem-vindo à documentação do projeto `computers`.  
Este site funciona como um diário de bordo, repositório de snippets e central de conhecimento para workflows práticos de virtualização, GPU passthrough e execução de modelos de inferência (LLMs) em guests com GPUs dedicadas.

Resumo rápido
- Crate: `computers`  
- Repositório: `https://github.com/ze-gois/rust_computers`  
- Objetivo: centralizar guias reproduzíveis, trechos de configuração (snippets) e o diário de decisões/prompts que geraram as instruções.

Navegação
- `docs/diary.md` — Diário de bordo: entradas com data, objetivo, ações e resultados.  
- `docs/prompts.md` — Arquivo com as prompts/respostas usados para gerar instruções (rastreabilidade).  
- `docs/snippets/` — Snippets reutilizáveis (ex.: `vfio.conf`, comandos `lspci`, trechos de XML do libvirt, scripts de instalação).  
- `README.md` — Visão geral da crate e instruções de build/publish.

Como publicar como GitHub Pages
- Opção simples (recomendado durante desenvolvimento): use a branch `main` e configure GitHub Pages para servir da pasta `/docs`. Faça commits em `main` com atualizações em `docs/` e a página será atualizada automaticamente pelo GitHub.
- Fluxo alternativo (mais controlado): publicar para a branch `gh-pages`. Podemos adicionar um GitHub Action que empacota `docs/` e faz push para `gh-pages` em cada push na `main`. Se desejar, eu posso gerar o arquivo de workflow `ci/gh-pages.yml`.

Estrutura do conteúdo
- `docs/index.md` — índice e visão geral (este arquivo).  
- `docs/diary.md` — entradas ordenadas cronologicamente (cada entrada registra comandos executados, resultados, logs e links para snippets).  
- `docs/prompts.md` — registro íntegro das interações que produziram as instruções (útil para auditoria e reprodução).  
- `docs/snippets/` — arquivos de configuração e pequenos scripts, prontos para copiar/colar (com placeholders claros e notas de segurança).

Como gerar/visualizar localmente
- Documentação da crate (API Rust):
  - `cargo doc --open` — gera a documentação da crate e abre no navegador.
- Conteúdo estático (`docs/`) é Markdown simples; você pode visualizá-lo com um servidor estático:
  - `python -m http.server 8000 --directory docs` (ou use qualquer servidor estático).
- Sugestão: mantenha `docs/` estritamente como conteúdo pronto para GitHub Pages (minimize arquivos gerados).

Boas práticas e segurança
- Sempre registre backups dos arquivos originais antes de aplicar trechos que alterem `GRUB`, `mkinitcpio` ou drivers (ex.: `vfio`).  
- Substitua todos os placeholders (IDs PCI, vendor IDs, caminhos) pelos valores do seu hardware antes de aplicar.  
- Inclua notas de risco nas entradas do diário quando um passo pode deixar o sistema temporariamente não inicializável.

Próximos passos sugeridos
- Gerar um template de VM XML com placeholders (para `virt-manager` / `virsh`) e colocá-lo em `docs/snippets/`.  
- Adicionar checks automatizados (scripts) que coletam `nvidia-smi`, `dmesg | grep -i vfio` e os resultados dos comandos usados, e anexam os logs ao diário.  
- Criar um workflow GitHub Action para publicar `docs/` automaticamente (opcionalmente com validação de links).

Contribuindo
- Abra uma issue antes de mudanças grandes.  
- PRs para snippets e correções de texto são bem-vindos; para alterações em comandos de sistema, inclua contexto, plataforma (host/guest distro) e riscos.  
- Para experimentos que modificam `Cargo.toml` ou código, inclua testes quando aplicável.

Contato
- Autor: José Gois <ze.gois.00@gmail.com>  
- Repo: `https://github.com/ze-gois/rust_computers`

Se quiser, eu já gero:
- um template de workflow (`ci/gh-pages.yml`) para publicar `docs/`, e/ou  
- um template de VM XML com placeholders para sua RTX 3050 (pronto para substituir `domain/bus/slot/function` e IDs PCI).  
Diz qual você prefere e eu gero o arquivo seguinte.