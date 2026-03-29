# Diary — GPU passthrough & LLM inference (living document)

Data: 2026-03-29
Autor: José Gois / assisted by ChatGPT (registro automático)

Resumo objetivo
- Host: Dell G15 (Arch Linux) com Intel iGPU + NVIDIA RTX 3050 (6 GB).
- Meta: passar a RTX 3050 para uma VM (KVM/QEMU + VFIO + OVMF), rodar modelos de inferência (vLLM / TurboQuant) na VM e expor o serviço ao host via TCP/SSH (compatível MCP).
- Motivação: obter CUDA nativo na VM para throughput e economia de memória (TurboQuant) mantendo o host disponível e isolado.

Status
- Planejamento inicial completo.
- Documentos e snippets iniciais criados em `docs/` e `docs/snippets/`.
- Próximo: testar no hardware real e coletar logs (nvidia-smi, dmesg, iommu_groups).

Decisões-chave
- Não usar hypervisor tipo-2 (VirtualBox/VMware) — não suportam passthrough robusto para laptop Optimus com CUDA nativo.
- Usar KVM/QEMU com VFIO e OVMF (libvirt/virt-manager) para passthrough PCI.
- Guest recomendado: Linux (Ubuntu/Arch) para drivers NVIDIA + CUDA e ambiente inferência (vLLM / llama.cpp + TurboQuant).
- Exposição do serviço: TCP direto (IP da VM) ou túnel SSH (seguro e simples).

Registro de prompts e materiais de referência
- Prompts e conversa original arquivadas em `docs/prompts.md`.
- Snippets criados (copiar/colar com placeholders):
  - `docs/snippets/lspci_cmds.txt`
  - `docs/snippets/grub_cmds.txt`
  - `docs/snippets/vfio_conf.txt`
  - `docs/snippets/mkinitcpio_conf.txt`
  - `docs/snippets/install_libvirt.sh`
  - `docs/snippets/libvirt_snippet.xml`
  - `docs/snippets/vllm_run.txt`
  - `docs/snippets/ssh_tunnel.txt`

Checklist (pré-requisitos no host)
- [ ] Habilitar VT-x / VT-d no BIOS.
- [ ] Identificar IDs PCI da GPU (VGA + áudio).
  - Comando de referência: 
    ```rust_computers/docs/snippets/lspci_cmds.txt#L1-8
    (lspci command and example output)
    ```
- [ ] Editar `/etc/default/grub`:
  - `intel_iommu=on iommu=pt` (ou `amd_iommu=on` para AMD).
  - Regenerar GRUB: `sudo grub-mkconfig -o /boot/grub/grub.cfg`.
  - Referência snippet:
    ```rust_computers/docs/snippets/grub_cmds.txt#L1-6
    (grub kernel command and regen instructions)
    ```
- [ ] Adicionar IDs PCI ao `vfio-pci` via `/etc/modprobe.d/vfio.conf`:
  ```rust_computers/docs/snippets/vfio_conf.txt#L1-6
  (vfio config snippet)
  ```
- [ ] Incluir módulos no initramfs (`/etc/mkinitcpio.conf`, `MODULES=(vfio_pci vfio vfio_iommu_type1)`), e `sudo mkinitcpio -P`.
  - Snippet:
    ```rust_computers/docs/snippets/mkinitcpio_conf.txt#L1-8
    (mkinitcpio modules snippet)
    ```
- [ ] Instalar qemu/libvirt/ovmf/virt-manager e habilitar `libvirtd`:
  ```rust_computers/docs/snippets/install_libvirt.sh#L1-8
  (install and enable libvirt)
  ```

Notas operacionais / comandos úteis
- Bind dinâmico (temporário) — somente para testes (não substitui configuração persistente via initramfs):
  - Unbind device, set driver_override, bind to vfio-pci (use o PCI_ADDR do seu `lspci`).
- Verificações pós-boot:
  - `dmesg | grep -i vfio`
  - `ls /sys/bus/pci/devices/0000:XX:YY.Z/driver`  (ver driver atual)
  - `lspci -nnk` (confirmar `vfio-pci` ou driver preto)
- Ao criar VM via `virt-manager`:
  - Firmware: OVMF (UEFI).
  - CPU: `host-passthrough`.
  - Adicionar GPU e dispositivo de áudio como `PCI Host Device`.
  - Evitar Error 43 da NVIDIA: esconder KVM e setar vendor_id (ex.: `12345678`) no XML:
    ```rust_computers/docs/snippets/libvirt_snippet.xml#L1-40
    (libvirt XML snippet)
    ```

Guest (VM) — setup de inferência
- Instale drivers NVIDIA e CUDA no guest (ex.: `nvidia-driver-535` ou driver atual recomendado).
- Validar: `nvidia-smi` (deve mostrar a GPU passada).
- Instalar ambiente de inferência (Python, vLLM, TurboQuant/llama.cpp fork se necessário).
- Exemplo rápido para vLLM (expor porta 8000):
  ```rust_computers/docs/snippets/vllm_run.txt#L1-6
  (vllm serve example)
  ```
- Expor porta para host:
  - SSH tunnel (recomendado para testes): 
    ```rust_computers/docs/snippets/ssh_tunnel.txt#L1-4
    (ssh tunnel example)
    ```
  - Ou configure bridge/net para a VM e use IP direto `http://VM_IP:8000`.

MCP (Model Context Protocol) — notas
- Objetivo: oferecer endpoint compatível com MCP no guest que o host (ou apps no host) possam consumir.
- Estratégia inicial:
  - Rodar vLLM / serviço HTTP que implementa endpoints compatíveis (OpenAI-like ou adaptador MCP).
  - Implementar/instalar um pequeno bridge se necessário (OpenAI <-> MCP adapter).
- Segurança: quando abrir portas diretamente, aplicar firewall / TLS ou usar túnel SSH.

Riscos e mitigação
- Alterações em GRUB e initramfs podem deixar o host não bootável. Sempre:
  - Fazer backup de `/etc/default/grub` e `/etc/mkinitcpio.conf`.
  - Ter live-USB de recuperação à mão.
- Erro 43 (NVIDIA) na VM é comum sem `host-passthrough` + XML tweaks — veja a seção de ajustes XML.
- Substituir todos os placeholders (IDs PCI, domain/bus/slot/function) antes de aplicar.

Logs a coletar durante testes (incluir em entradas futuras)
- `dmesg` — buscar por `vfio`, `iommu`, `ERROR`, `nvidia`.
- `journalctl -b` — entradas relevantes ao boot.
- `lspci -nnk` — antes/depois de bind.
- `nvidia-smi` — no guest (após boot da VM).
- Resultado de `curl` / health endpoints do servidor de inferência.

Checklist pós-implantação (quando a VM estiver rodando e o serviço exposto)
- [ ] `nvidia-smi` dentro do guest mostra GPU e processos.
- [ ] `vLLM` (ou outro servidor) responde a `curl` /health.
- [ ] Latência e throughput mínimos dentro das expectativas para o modelo alvo.
- [ ] Registro de logs anexado a esta entrada (`docs/logs/` — criar se necessário).
- [ ] Atualizar `docs/prompts.md` com observações sobre divergências e ajustes.

Próximos passos (curto prazo)
1. Gerar template de VM XML com placeholders claros (domain/bus/slot/function e vendor/device IDs).
2. Testar um ciclo completo em hardware: aplicar `vfio.conf`, rebuild initramfs, reboot, confirmar que GPU fica bound ao vfio no host, criar VM, iniciar guest e validar `nvidia-smi`.
3. Instalar e validar vLLM no guest; expor via SSH tunnel e testar do host.
4. Coletar e anexar logs aqui (`docs/logs/`), e criar uma entrada de incidente se qualquer passo falhar.
5. (Opcional) Adicionar GitHub Action para publicar `docs/` em GitHub Pages automaticamente.

Notas históricas / referência rápida
- Conversa original e planejamento inicial estão arquivados em `docs/prompts.md`.
- Fragmentos de comandos e exemplos prontos em `docs/snippets/` (substitua placeholders).
- Use este diário para anexar resultados concretos de cada teste: data, comandos executados, saída relevante, e ação subsequente.

Registro de mudanças (última atualização)
- 2026-03-29: Entrada inicial criada com planejamento, checklist e snippets referenciados.

Fim da entrada.