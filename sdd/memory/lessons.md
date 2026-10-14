# Lições Aprendidas — redragon-mouse

- **M709 Scroll e Keymap Registers (2026-10-14):**
  - Os botões de Scroll Up (`0xd2`) e Scroll Down (`0xd6`) são puramente de escrita no M709; a leitura `DATA_READ_3` lê 20 pacotes por perfil e cobre apenas os botões esquerdo (0), direito (1), meio (2), DPI (4), forward (6) e backward (7).
  - Tratar os 8 botões como contíguos no buffer de leitura causava a leitura de dados residuais em ScrollUp/ScrollDown, que ao serem regravados no mouse desativavam o scroll físico.
  - Correção: `parse_settings` mapeia os offsets reais `[0, 1, 2, 6, 7, 4]` e preserva `ScrollUp` e `ScrollDown` intactos.
- **GTK4 DropDown e Inscription (2026-10-14):**
  - No GTK 4.10+, o factory padrão de `GtkDropDown` cria nós do tipo `inscription`, e não `label`. Seletores como `dropdown label` ou `popover label` não afetam os itens.
  - O motor de CSS do GTK4 não aceita `!important` e descarta a linha inteira com erro `Junk at end of value`.
  - Em ambientes com tema claro do sistema (ex: ZorinBlue-Light), deve-se ativar `gtk-application-prefer-dark-theme = true` via `gtk::Settings` e injetar um `SignalListItemFactory` com `Label` explícito e classe `.dropdown-label`.
