use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Align, Box, Button, Entry, Label, ListBox, Orientation, SelectionMode};
use m709_core::{Profile, ProfileManager};
use std::rc::Rc;

use crate::state::{save_to_hardware, switch_profile_hardware, SharedState};

pub fn build_profiles_tab(state: SharedState, on_change: Rc<dyn Fn()>) -> Box {
    let container = Box::new(Orientation::Vertical, 16);
    container.set_margin_top(20);
    container.set_margin_bottom(20);
    container.set_margin_start(24);
    container.set_margin_end(24);

    let title = Label::builder()
        .label("Gerenciamento de Perfis")
        .halign(Align::Start)
        .css_classes(["title-2"])
        .build();
    container.append(&title);

    let desc = Label::builder()
        .label("Alterne entre os 5 perfis da memória interna ou salve presets locais no computador em TOML.")
        .halign(Align::Start)
        .css_classes(["dim-label"])
        .build();
    container.append(&desc);

    // 1. Hardware Profile Switcher Card
    let hw_card = Box::new(Orientation::Vertical, 10);
    hw_card.set_css_classes(&["card"]);

    let hw_title = Label::builder()
        .label("Perfil Ativo no Mouse (Memória Onboard):")
        .halign(Align::Start)
        .css_classes(["heading"])
        .build();
    hw_card.append(&hw_title);

    let prof_btn_box = Box::new(Orientation::Horizontal, 8);
    let active_prof = state.borrow().config.active_profile;

    for i in 0..5 {
        let prof = Profile::from_index(i).unwrap();
        let btn = Button::builder()
            .label(format!("Perfil {}", prof.display_number()))
            .build();

        if prof == active_prof {
            btn.add_css_class("suggested-action");
        }

        let s_clone = state.clone();
        let on_change_clone = on_change.clone();
        btn.connect_clicked(move |_| {
            let _ = switch_profile_hardware(&s_clone, prof);
            on_change_clone();
        });
        prof_btn_box.append(&btn);
    }
    hw_card.append(&prof_btn_box);
    container.append(&hw_card);

    // 2. Salvar Perfil Atual Localmente
    let save_card = Box::new(Orientation::Vertical, 10);
    save_card.set_css_classes(&["card"]);

    let save_title = Label::builder()
        .label("Salvar Perfil Atual em Arquivo TOML:")
        .halign(Align::Start)
        .css_classes(["heading"])
        .build();
    save_card.append(&save_title);

    let save_row = Box::new(Orientation::Horizontal, 10);
    let name_entry = Entry::builder()
        .placeholder_text("Nome do perfil (ex: fps-gaming, trabalho)")
        .hexpand(true)
        .build();

    let save_btn = Button::builder()
        .label("Salvar Localmente")
        .css_classes(["pill"])
        .build();

    let s_clone = state.clone();
    let name_entry_clone = name_entry.clone();
    let on_change_clone = on_change.clone();
    save_btn.connect_clicked(move |_| {
        let name = name_entry_clone.text().to_string();
        if !name.trim().is_empty() {
            if let Ok(pm) = ProfileManager::new() {
                let s = s_clone.borrow();
                let p = s.config.active_profile.index();
                let prof_cfg = &s.config.profiles[p];
                if let Ok(_) = pm.save_profile(&name, Some("Salvo via Interface GTK4"), prof_cfg) {
                    name_entry_clone.set_text("");
                }
            }
        }
        on_change_clone();
    });

    save_row.append(&name_entry);
    save_row.append(&save_btn);
    save_card.append(&save_row);

    // Lista de perfis locais
    let list_title = Label::builder()
        .label("Perfis Salvos no Computador:")
        .halign(Align::Start)
        .css_classes(["heading"])
        .margin_top(8)
        .build();
    save_card.append(&list_title);

    let list_box = ListBox::builder()
        .selection_mode(SelectionMode::None)
        .css_classes(["boxed-list"])
        .build();

    if let Ok(pm) = ProfileManager::new() {
        if let Ok(profiles) = pm.list_profiles() {
            if profiles.is_empty() {
                let empty_lbl = Label::builder()
                    .label("Nenhum perfil salvo em ~/.config/redragon-mouse/")
                    .halign(Align::Start)
                    .css_classes(["dim-label"])
                    .margin_top(6)
                    .margin_bottom(6)
                    .build();
                list_box.append(&empty_lbl);
            } else {
                for prof_name in profiles {
                    let row = Box::new(Orientation::Horizontal, 10);
                    row.set_margin_top(6);
                    row.set_margin_bottom(6);
                    row.set_margin_start(8);
                    row.set_margin_end(8);

                    let p_label = Label::builder()
                        .label(&prof_name)
                        .halign(Align::Start)
                        .hexpand(true)
                        .build();

                    let load_btn = Button::builder()
                        .label("Carregar")
                        .css_classes(["flat"])
                        .build();

                    let s_clone = state.clone();
                    let on_change_clone = on_change.clone();
                    let prof_name_clone = prof_name.clone();
                    load_btn.connect_clicked(move |_| {
                        if let Ok(pm) = ProfileManager::new() {
                            if let Ok(loaded) = pm.load_profile(&prof_name_clone) {
                                let mut s = s_clone.borrow_mut();
                                let p = s.config.active_profile.index();
                                s.config.profiles[p] = loaded;
                                s.is_dirty = true;
                                drop(s);
                                if let Err(e) = save_to_hardware(&s_clone) {
                                    eprintln!("Erro ao gravar no mouse: {}", e);
                                }
                                on_change_clone();
                            }
                        }
                    });

                    row.append(&p_label);
                    row.append(&load_btn);
                    list_box.append(&row);
                }
            }
        }
    }
    save_card.append(&list_box);
    container.append(&save_card);

    container
}
