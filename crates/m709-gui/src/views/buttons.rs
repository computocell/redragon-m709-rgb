use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Align, Box, Button, DropDown, Label, Orientation, StringList};
use m709_core::{ButtonAction, PhysicalButton};
use std::rc::Rc;

use crate::state::{save_to_hardware, SharedState};

pub fn build_buttons_tab(state: SharedState, on_change: Rc<dyn Fn()>) -> Box {
    let container = Box::new(Orientation::Vertical, 16);
    container.set_margin_top(20);
    container.set_margin_bottom(20);
    container.set_margin_start(24);
    container.set_margin_end(24);

    let title = Label::builder()
        .label("Mapeamento de Botões")
        .halign(Align::Start)
        .css_classes(["title-2"])
        .build();
    container.append(&title);

    let desc = Label::builder()
        .label("Remapeie os botões físicos e o comportamento da roda de rolagem do Redragon M709.")
        .halign(Align::Start)
        .css_classes(["dim-label"])
        .build();
    container.append(&desc);

    let card = Box::new(Orientation::Vertical, 10);
    card.set_css_classes(&["card"]);

    let actions = Rc::new([
        ("Clique Esquerdo", ButtonAction::LeftClick),
        ("Clique Direito", ButtonAction::RightClick),
        ("Clique do Meio", ButtonAction::MiddleClick),
        ("Avançar (Navegador)", ButtonAction::Forward),
        ("Voltar (Navegador)", ButtonAction::Backward),
        ("Ciclo de DPI", ButtonAction::DpiCycle),
        ("Aumentar DPI (+)", ButtonAction::DpiUp),
        ("Diminuir DPI (-)", ButtonAction::DpiDown),
        ("Scroll Up (Rolar Cima)", ButtonAction::ScrollUp),
        ("Scroll Down (Rolar Baixo)", ButtonAction::ScrollDown),
        ("Trocar Perfil", ButtonAction::ProfileSwitch),
        ("Toggle LED do DPI", ButtonAction::DpiLedToggle),
        ("Desativado", ButtonAction::None),
    ]);
    let action_names: Vec<&str> = actions.iter().map(|(n, _)| *n).collect();

    for (btn_idx, btn) in PhysicalButton::ALL.iter().enumerate() {
        let row = Box::new(Orientation::Horizontal, 12);

        let btn_label = Label::builder()
            .label(btn.name())
            .halign(Align::Start)
            .hexpand(true)
            .build();

        let str_list = StringList::new(&action_names);
        let dropdown = DropDown::new(Some(str_list), None::<gtk::Expression>);
        dropdown.set_width_request(220);

        // Set initial selected action
        {
            let s = state.borrow();
            let p = s.config.active_profile.index();
            let current_action = &s.config.profiles[p].keymap[btn_idx];
            for (act_idx, (_, act)) in actions.iter().enumerate() {
                if act == current_action {
                    dropdown.set_selected(act_idx as u32);
                    break;
                }
            }
        }

        let s_clone = state.clone();
        let on_change_clone = on_change.clone();
        let actions_clone = actions.clone();
        dropdown.connect_selected_notify(move |dd| {
            let selected_act = dd.selected() as usize;
            if selected_act < actions_clone.len() {
                let mut s = s_clone.borrow_mut();
                let p = s.config.active_profile.index();
                s.config.profiles[p].keymap[btn_idx] = actions_clone[selected_act].1.clone();
                s.is_dirty = true;
                drop(s);
                on_change_clone();
            }
        });

        row.append(&btn_label);
        row.append(&dropdown);
        card.append(&row);
    }

    container.append(&card);

    // Botoes de Ação
    let action_box = Box::new(Orientation::Horizontal, 12);
    action_box.set_halign(Align::End);

    let reset_btn = Button::builder()
        .label("Restaurar Padrão de Fábrica")
        .css_classes(["destructive-action", "pill"])
        .build();

    let s_clone = state.clone();
    let on_change_clone = on_change.clone();
    reset_btn.connect_clicked(move |_| {
        let mut s = s_clone.borrow_mut();
        let p = s.config.active_profile.index();
        s.config.profiles[p].keymap = [
            ButtonAction::LeftClick,
            ButtonAction::RightClick,
            ButtonAction::MiddleClick,
            ButtonAction::Forward,
            ButtonAction::Backward,
            ButtonAction::DpiCycle,
            ButtonAction::ScrollUp,
            ButtonAction::ScrollDown,
        ];
        s.is_dirty = true;
        drop(s);
        if let Err(e) = save_to_hardware(&s_clone) {
            eprintln!("Erro ao gravar: {}", e);
        }
        on_change_clone();
    });
    action_box.append(&reset_btn);

    let apply_btn = Button::builder()
        .label("Aplicar Botões no Mouse")
        .css_classes(["suggested-action", "pill"])
        .build();

    let s_clone = state.clone();
    let on_change_clone = on_change.clone();
    apply_btn.connect_clicked(move |_| {
        if let Err(e) = save_to_hardware(&s_clone) {
            eprintln!("Erro ao gravar no mouse: {}", e);
        }
        on_change_clone();
    });
    action_box.append(&apply_btn);

    container.append(&action_box);

    container
}
