# Prompts Archive

Este arquivo arquiva as trocas de prompts e respostas que geraram as instruções iniciais para o projeto `computers`. O objetivo é manter rastreabilidade e permitir reproduzir o contexto que levou às decisões de design, comandos e snippets fornecidos.

Meta
- Registrar, de forma íntegra e pesquisável, as prompts do usuário e as respostas geradas pelo assistente.
- Facilitar auditoria e reprodutibilidade do fluxo de trabalho (GPU passthrough → VM → inferência LLM → exposição via MCP/TCP/SSH).

Formato
- Cada entrada contém: data, autor da prompt (usuário), conteúdo da prompt, resposta do assistente (com resumo técnico) e tags rápidas.
- As transcrições são mantidas de forma literal dentro de blocos identificados, para preservar o histórico.

Entrada: 2026-03-29 — Planejamento inicial GPU passthrough + LLM
- Autor da prompt: usuário
- Canal: conversa com assistente
- Tags: passthrough, KVM, VFIO, OVMF, RTX 3050, Arch Linux, vLLM, TurboQuant, MCP, SSH, TCP

Prompt (inteiro, como enviado pelo usuário):
```rust_computers/docs/prompts.md#L1-34
**Entendi perfeitamente o que você quer, Zé!** 🙌

Você tem um **Dell G15 com RTX 3050 6 GB** no Arch Linux (host), e quer:

- Isolar / passar a RTX 3050 para uma VM (usando hypervisor **tipo 2**).
- Rodar o sistema de inferência de LLM dentro da VM.
- Expor o serviço de LLM (via **MCP** — Model Context Protocol, o novo protocolo da Anthropic de 2024 para LLMs se conectarem com ferramentas/dados externos de forma padronizada) de volta para o host via SSH ou TCP puro.
- Usar a inferência do host como se fosse local (com TurboQuant para sentir aqueles “36 GB efetivos”).

### ⚠️ Aviso importante sobre hypervisor tipo 2
**VirtualBox, VMware Workstation ou qualquer Type 2 puro NÃO funcionam** para isso.  
Eles não suportam **PCI passthrough completo** da RTX 3050 (especialmente em laptop Optimus). Você até consegue aceleração 3D básica, mas **nada de CUDA nativo** para LLM com TurboQuant/vLLM — o ganho seria mínimo ou zero.

**A solução real e que funciona hoje (2026) no Arch é KVM/QEMU + VFIO + OVMF** (libvirt/virt-manager).  
É o que **todas as guias sérias** usam (incluindo Arch Wiki). Tecnicamente é um hypervisor kernel-based, mas roda perfeitamente no seu Arch host e entrega performance quase nativa da RTX 3050 para inferência.

No seu caso é **mais fácil ainda** porque:
- Você tem iGPU Intel + dGPU NVIDIA → o host continua usando a Intel para display.
- Não precisa de monitor externo (inferência é headless).
- Dell G15 costuma ser MUXed ou semi-MUXed → passthrough da 3050 é viável.

### Walkthrough resumido e atualizado (baseado na Arch Wiki + guias 2025/2026)

1. **Preparar o host (uma vez só)**
   - Entre no BIOS → ative **Intel VT-d** + **VT-x**.
   - Edite `/etc/default/grub` e adicione:
     ```bash
     intel_iommu=on iommu=pt
     ```
     (se precisar de grupos isolados: `pcie_acs_override=downstream,multifunction`).
   - Atualize GRUB: `sudo grub-mkconfig -o /boot/grub/grub.cfg`
   - Identifique sua RTX 3050:
     ```bash
     lspci -nnk | grep -E 'VGA|3D|Display'
     ```
     Anote os IDs (ex: 10de:25a2 e o áudio 10de:xxxx).
   - Crie `/etc/modprobe.d/vfio.conf`:
     ```
     options vfio-pci ids=10de:25a2,10de:xxxx
     softdep nvidia pre: vfio-pci
     ```
   - Adicione módulos no `/etc/mkinitcpio.conf` (MODULES): `vfio_pci vfio vfio_iommu_type1`
   - `sudo mkinitcpio -P && reboot`

2. **Instalar ferramentas**
   ```bash
   sudo pacman -S qemu-desktop libvirt edk2-ovmf virt-manager dnsmasq
   sudo systemctl enable --now libvirtd
   sudo usermod -aG libvirt $USER
   ```

3. **Criar a VM (virt-manager é o mais fácil)**
   - Abra `virt-manager`.
   - Crie VM com **firmware UEFI (OVMF)**.
   - CPU: host-passthrough.
   - Adicione a GPU + dispositivo de áudio como **PCI Host Device**.
   - (Opcional mas recomendado) Esconda KVM e adicione Hyper-V vendor ID para evitar Error 43 da NVIDIA:
     ```xml
     <kvm><hidden state='on'/></kvm>
     <hyperv><vendor_id state='on' value='12345678'/></hyperv>
     ```
   - Para mobile NVIDIA (RTX 3050), adicione tabela ACPI para simular bateria (evita Error 43).

4. **Dentro da VM (guest Linux recomendado, ex: Ubuntu ou Arch)**
   - Instale drivers NVIDIA + CUDA.
   - Rode **vLLM** (melhor performance) ou fork do llama.cpp com TurboQuant.
   - Exemplo vLLM (com TurboQuant quando o fork estiver maduro):
     ```bash
     python -m vllm serve modelo --host 0.0.0.0 --port 8000 --gpu-memory-utilization 0.9
     ```
   - Para **MCP** (Model Context Protocol): rode um servidor MCP que exponha o LLM (ex: Anthropic-compatible ou bridge para OpenAI API + MCP tools). O protocolo é novo, mas já tem implementações iniciais no ecossistema Anthropic/Claude.

5. **Expor o serviço para o host**
   - **Mais simples**: TCP direto (porta 8000).
     - No host: `http://IP-DA-VM:8000/v1` (compatível OpenAI).
     - Ou use SSH tunnel: `ssh -L 8000:localhost:8000 user@vm-ip`
   - Coloque Open WebUI ou LM Studio no host apontando para o IP da VM.
   - Você vai sentir o TurboQuant + RTX 3050 isolada exatamente como queria: contexto gigante com pouca VRAM.

### Dicas extras para sua RTX 3050 6 GB + TurboQuant
- Use guest Linux (menos overhead que Windows).
- Ative hugepages no host e na VM para mais velocidade.
- Ferramentas que facilitam: `gpu-passthrough-manager` (AUR) ou `vm-curator` (AUR, TUI moderna de 2026).
- Se quiser zero dor de cabeça no futuro → considere rodar tudo direto no host (sem VM). O Arch + NVIDIA-open + CUDA + TurboQuant já entrega o “upgrade de 36 GB” que você sentiu. A VM só vale se você realmente precisa de isolamento total.

Quer que eu monte o **passo a passo completo com comandos exatos** (personalizado para Dell G15 RTX 3050) ou prefere primeiro o link da Arch Wiki + script pronto?

Ou quer foco em como configurar o **serviço MCP** dentro da VM?

Me fala onde quer aprofundar que eu te entrego o próximo pedaço pronto para copiar! 🚀
```

Assistant response (resumo e ações sugeridas):
```rust_computers/docs/prompts.md#L35-200
Resumo da resposta do assistente:
- Confirmou que hypervisor tipo-2 é inadequado; recomendou KVM/QEMU + VFIO + OVMF.
- Listou o fluxo completo: habilitar IOMMU no GRUB, identificar IDs PCI da GPU, bind para `vfio-pci`, incluir módulos em `mkinitcpio.conf`, regenerar initramfs e reboot.
- Sugeriu instalar `qemu`, `libvirt`, `edk2-ovmf`, `virt-manager`, habilitar `libvirtd`.
- Indicou como criar a VM (UEFI/OVMF, CPU host-passthrough, adicionar hostdev PCI para GPU e áudio).
- Forneceu snippet de ajuste XML (esconder KVM e setar `vendor_id`) para evitar Error 43.
- Recomenda guest Linux, instalar drivers NVIDIA + CUDA, rodar vLLM ou outra stack (ex.: llama.cpp + TurboQuant) e expor via porta 8000 ou via túnel SSH.
- Ofereceu opções: (1) gerar passo-a-passo completo com comandos exatos; (2) criar script automático; (3) focar na configuração do serviço MCP dentro da VM.

Ações sugeridas / próximos artefatos (por prioridade)
1. Template de VM XML com placeholders (domain/bus/slot/function e IDs PCI) em `docs/snippets/`.
2. Workflow GitHub Action (`ci/gh-pages.yml`) para publicar `docs/` como site estático.
3. Scripts de coleta de logs / checks (ex.: `collect_logs.sh`) para anexar resultados de teste (nvidia-smi, dmesg, iommu_groups).
4. Guia passo-a-passo completo (comandos copy/paste) específico para Dell G15 RTX 3050 no Arch — incluí-lo em `docs/diary.md` ou como `docs/guides/dell-g15-passthrough.md`.

Observações (rastreabilidade)
- Este arquivo (`docs/prompts.md`) preserva a prompt e o resumo da resposta. Se novos prompts forem enviados para expandir o guia, adicione novas entradas cronológicas com data, autor e a transcrição completa.
- Ao aplicar snippets no sistema real, sempre crie um backup (`/etc/default/grub`, `/etc/mkinitcpio.conf`) e um ponto de recuperação (live USB).

FIM da entrada de 2026-03-29.
```

Registro de uso
- Para adicionar uma nova transcrição: copie o template acima, altere a data, cole a prompt original e a resposta, e faça commit em `docs/prompts.md`.
- Recomenda-se usar uma convenção de commits do tipo `docs(prompts): add 2026-03-29 passthrough planning` para manter rastreabilidade no git.
