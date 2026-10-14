# Constituição — redragon-mouse

## Missão
GUI desktop Linux para controlar LEDs, perfis, DPI e botões do Redragon M709 — inspirada no OpenRGB, com protocolo USB reimplementado via engenharia reversa (REA) do projeto mouse_m908.

## Stack
| Camada | Escolha | Motivo |
|--------|---------|--------|
| Runtime | rust | Performance nativa, sem GC, binário único |
| GUI | gtk4-rs | Bindings GTK4 maduros, nativo no Linux |
| USB | rusb (libusb) | Acesso direto ao dispositivo USB HID |
| DB | none | Sem necessidade |
| Config | toml (serde) | Perfis locais em ~/.config/redragon-mouse/ |
| Secrets | — | Não aplicável |

## Decisões resolvidas
| Decisão | Resolução |
|---------|-----------|
| Linguagem | Rust (substituiu Go original) |
| GUI framework | gtk4-rs (GTK4 nativo Linux) |
| Modelo alvo | Redragon M709 (VID:PID 0x04d9:0xfc4f — via mouse_m908 -M 709) |
| Abordagem USB | Reimplementar protocolo em Rust via rusb, usando REA para RE do código C++ do mouse_m908 |
| Referência de código-fonte | ~/git/tools/mouse_m908/ (clonado e compilado) |

## Ferramentas e Integrações
| Campo | Valor |
|-------|-------|
| VCS / Work Item System | github |

Consulte `sdd/memory/mcps.md` para o status real de cada MCP configurado (`ativo`/`indisponível`) antes de assumir que ele responde. Se "VCS / Work Item System" for `azure-devops`, use `az repos pr create` (ou instrução equivalente documentada) em vez de `gh pr create`. Se `nenhum`, deixe a branch pronta e informe o usuário, sem tentar nenhum comando de VCS.

## Regras (máx. 10)
1. Sem commits diretos em main
2. Branch por feature
3. Config centralizado em <arquivo>
4. Secrets em .env (nunca commit)
5. Antes de usar lib externa, consultar context7 com versão exata — desde que `sdd/memory/mcps.md` o liste como `ativo`; se `indisponível`, usar a documentação oficial da lib
6. Toda feature tem critério executável
7. Feature quebrada em subpasta (`sdd/features/<prefixo>-ID-<nome>/`) usa uma única branch agrupando todas as subtarefas — nunca uma branch por subtarefa. Antes de criar a branch, pergunte a branch de partida (default `main`) e verifique (`git branch --list <prefixo>/ID-*`) se já existe uma branch da mesma feature/fix a retomar.
