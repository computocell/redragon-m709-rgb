use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{
    Align, Application, ApplicationWindow, Box, Button, HeaderBar, Label, Orientation,
    Stack, StackSwitcher,
};
use m709_core::MODEL_NAME;
use std::rc::Rc;

use crate::state::{refresh_from_hardware, save_to_hardware, switch_profile_hardware, SharedState};
use crate::views::{
    buttons::build_buttons_tab, dpi::build_dpi_tab, lighting::build_lighting_tab,
    profiles::build_profiles_tab,
};

pub struct MainWindow {
    pub window: ApplicationWindow,
}

impl MainWindow {
    pub fn new(app: &Application, state: SharedState) -> Self {
        // Inicializa o estado lendo do hardware se disponível
        let _ = refresh_from_hardware(&state);

        let window = ApplicationWindow::builder()
            .application(app)
            .title(&format!("{} Control", MODEL_NAME))
            .default_width(850)
            .default_height(680)
            .build();

        // 1. HeaderBar
        let header = HeaderBar::new();
        window.set_titlebar(Some(&header));

        // Status badge
        let status_badge = Label::builder()
            .label("Verificando...")
            .css_classes(["badge"])
            .build();
        header.pack_start(&status_badge);

        // Botão de Reconectar / Atualizar
        let refresh_btn = Button::builder()
            .icon_name("view-refresh-symbolic")
            .tooltip_text("Recarregar dados do mouse USB")
            .build();
        header.pack_start(&refresh_btn);

        // Seletor rápido de perfil no Header
        let profile_box = Box::new(Orientation::Horizontal, 4);
        let profile_lbl = Label::builder().label("Perfil:").build();
        profile_box.append(&profile_lbl);

        let prof_btns: Vec<Button> = (0..5)
            .map(|i| {
                let b = Button::builder()
                    .label(format!("{}", i + 1))
                    .tooltip_text(format!("Ativar Perfil {}", i + 1))
                    .css_classes(["flat", "circular"])
                    .build();
                profile_box.append(&b);
                b
            })
            .collect();
        header.pack_end(&profile_box);

        // Botão de Salvar Global
        let save_all_btn = Button::builder()
            .label("Gravar no Mouse")
            .css_classes(["suggested-action"])
            .tooltip_text("Grava todas as alterações na memória interna EEPROM do mouse")
            .build();
        header.pack_end(&save_all_btn);

        // 2. Container Principal
        let root_box = Box::new(Orientation::Vertical, 0);
        window.set_child(Some(&root_box));

        // Stack de Abas
        let stack = Stack::new();
        stack.set_transition_type(gtk::StackTransitionType::Crossfade);
        stack.set_vexpand(true);

        // Stack Switcher (Barra de navegação de abas no topo)
        let switcher = StackSwitcher::new();
        switcher.set_stack(Some(&stack));
        switcher.set_halign(Align::Center);
        switcher.set_margin_top(12);
        switcher.set_margin_bottom(8);
        root_box.append(&switcher);

        // Content Area Container (para reconstrução dinâmica de abas)
        let content_box = Box::new(Orientation::Vertical, 0);
        content_box.set_vexpand(true);
        content_box.append(&stack);
        root_box.append(&content_box);

        // Status Bar Footer
        let footer = Box::new(Orientation::Horizontal, 12);
        footer.set_css_classes(&["footer"]);
        footer.set_margin_start(16);
        footer.set_margin_end(16);
        footer.set_margin_top(8);
        footer.set_margin_bottom(8);

        let footer_label = Label::builder()
            .label(&state.borrow().status_message)
            .halign(Align::Start)
            .hexpand(true)
            .build();
        footer.append(&footer_label);
        root_box.append(&footer);

        // Reconstrução de abas e UI state updater
        let state_c = state.clone();
        let footer_label_c = footer_label.clone();
        let status_badge_c = status_badge.clone();
        let prof_btns_c = prof_btns.clone();

        let update_ui = Rc::new(move || {
            let s = state_c.borrow();
            footer_label_c.set_text(&s.status_message);

            if s.is_connected {
                status_badge_c.set_text("● Conectado");
                status_badge_c.set_css_classes(&["badge", "badge-success"]);
            } else {
                status_badge_c.set_text("○ Desconectado");
                status_badge_c.set_css_classes(&["badge", "badge-danger"]);
            }

            let active_idx = s.config.active_profile.index();
            for (i, btn) in prof_btns_c.iter().enumerate() {
                if i == active_idx {
                    btn.set_css_classes(&["suggested-action", "circular"]);
                } else {
                    btn.set_css_classes(&["flat", "circular"]);
                }
            }
        });

        // Constrói as 4 abas no Stack
        let populate_tabs = {
            let stack = stack.clone();
            let state = state.clone();
            let update_ui = update_ui.clone();

            Rc::new(move || {
                let current_tab = stack.visible_child_name();

                // Remove abas antigas
                while let Some(child) = stack.first_child() {
                    stack.remove(&child);
                }

                let lighting_tab = build_lighting_tab(state.clone(), update_ui.clone());
                stack.add_titled(&lighting_tab, Some("lighting"), "Iluminação RGB");

                let dpi_tab = build_dpi_tab(state.clone(), update_ui.clone());
                stack.add_titled(&dpi_tab, Some("dpi"), "DPI & Desempenho");

                let buttons_tab = build_buttons_tab(state.clone(), update_ui.clone());
                stack.add_titled(&buttons_tab, Some("buttons"), "Botões");

                let profiles_tab = build_profiles_tab(state.clone(), update_ui.clone());
                stack.add_titled(&profiles_tab, Some("profiles"), "Perfis");

                if let Some(tab_name) = current_tab {
                    stack.set_visible_child_name(&tab_name);
                }
            })
        };

        populate_tabs();
        update_ui();

        // Conectar signals de header buttons
        for (i, btn) in prof_btns.iter().enumerate() {
            let state_c = state.clone();
            let prof = m709_core::Profile::from_index(i).unwrap();
            let update_ui_c = update_ui.clone();
            let pop_tabs_c = populate_tabs.clone();
            btn.connect_clicked(move |_| {
                let _ = switch_profile_hardware(&state_c, prof);
                pop_tabs_c();
                update_ui_c();
            });
        }

        // Refresh hardware signal
        let state_c = state.clone();
        let update_ui_c = update_ui.clone();
        let pop_tabs_c = populate_tabs.clone();
        refresh_btn.connect_clicked(move |_| {
            let _ = refresh_from_hardware(&state_c);
            pop_tabs_c();
            update_ui_c();
        });

        // Save All button
        let state_c = state.clone();
        let update_ui_c = update_ui.clone();
        save_all_btn.connect_clicked(move |_| {
            if let Err(e) = save_to_hardware(&state_c) {
                eprintln!("Erro ao gravar no mouse: {}", e);
            }
            update_ui_c();
        });

        Self { window }
    }

    pub fn present(&self) {
        self.window.present();
    }
}
