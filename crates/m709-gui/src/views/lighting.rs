use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Align, Box, Button, DropDown, Label, Orientation, Scale, StringList};
use m709_core::{LightMode, RgbColor};
use std::rc::Rc;

use crate::state::{save_to_hardware, SharedState};

pub fn build_lighting_tab(state: SharedState, on_change: Rc<dyn Fn()>) -> Box {
    let container = Box::new(Orientation::Vertical, 16);
    container.set_margin_top(20);
    container.set_margin_bottom(20);
    container.set_margin_start(24);
    container.set_margin_end(24);

    let title = Label::builder()
        .label("Controle de Iluminação RGB")
        .halign(Align::Start)
        .css_classes(["title-2"])
        .build();
    container.append(&title);

    let desc = Label::builder()
        .label("Configure os efeitos de iluminação, cores, brilho e velocidade da faixa LED do mouse.")
        .halign(Align::Start)
        .css_classes(["dim-label"])
        .build();
    container.append(&desc);

    // Frame de configurações
    let settings_box = Box::new(Orientation::Vertical, 14);
    settings_box.set_css_classes(&["card"]);

    // 1. Modo de Luz
    let mode_row = Box::new(Orientation::Horizontal, 12);
    let mode_label = Label::builder()
        .label("Efeito / Modo de Luz:")
        .halign(Align::Start)
        .hexpand(true)
        .build();
    mode_row.append(&mode_label);

    let modes = [
        ("Estático (Cor Sólida)", LightMode::Static),
        ("Respiração (Breathing)", LightMode::Breathing),
        ("Arco-íris (Rainbow Spectrum)", LightMode::Rainbow),
        ("Respiração Arco-íris", LightMode::BreathingRainbow),
        ("Onda (Wave Flow)", LightMode::Wave),
        ("Piscando (Flashing)", LightMode::Flashing),
        ("Reativo ao Clique", LightMode::ReactiveButton),
        ("Aleatório (Random)", LightMode::Random),
        ("Alternando (Alternating)", LightMode::Alternating),
        ("Trilha Reativa", LightMode::Reactive),
        ("Desligado (Off)", LightMode::Off),
    ];

    let mode_names: Vec<&str> = modes.iter().map(|(n, _)| *n).collect();
    let mode_strings = StringList::new(&mode_names);
    let mode_dropdown = DropDown::new(Some(mode_strings), None::<gtk::Expression>);
    mode_row.append(&mode_dropdown);
    settings_box.append(&mode_row);

    // 2. Cor RGB
    let color_section = Box::new(Orientation::Vertical, 8);
    let color_title = Label::builder()
        .label("Cor da Iluminação:")
        .halign(Align::Start)
        .css_classes(["heading"])
        .build();
    color_section.append(&color_title);

    let color_preview = Label::builder()
        .label("       ")
        .css_classes(["color-preview"])
        .halign(Align::Start)
        .build();

    let hex_entry = gtk::Entry::builder()
        .placeholder_text("#00FF88")
        .max_length(7)
        .width_chars(8)
        .build();

    let color_row = Box::new(Orientation::Horizontal, 12);
    color_row.append(&color_preview);
    color_row.append(&hex_entry);
    color_section.append(&color_row);

    // Paleta de presets rápidos
    let presets = [
        ("#FF0000", "Vermelho"),
        ("#00FF00", "Verde"),
        ("#0000FF", "Azul"),
        ("#00FFFF", "Ciano"),
        ("#FF00FF", "Magenta"),
        ("#FFFF00", "Amarelo"),
        ("#FF6600", "Laranja"),
        ("#9900FF", "Roxo"),
        ("#00FF88", "Verde Água"),
        ("#FFFFFF", "Branco"),
    ];

    let presets_box = Box::new(Orientation::Horizontal, 6);
    for (hex, label) in presets {
        let btn = Button::builder()
            .label(label)
            .css_classes(["flat", "preset-btn"])
            .tooltip_text(hex)
            .build();

        let s_clone = state.clone();
        let hex_clone = hex.to_string();
        let hex_entry_clone = hex_entry.clone();
        let color_preview_clone = color_preview.clone();
        let on_change_clone = on_change.clone();

        btn.connect_clicked(move |_| {
            if let Some(rgb) = RgbColor::from_hex(&hex_clone) {
                let mut s = s_clone.borrow_mut();
                let p = s.config.active_profile.index();
                s.config.profiles[p].color = rgb;
                s.is_dirty = true;
                drop(s);

                hex_entry_clone.set_text(&hex_clone);
                update_color_preview(&color_preview_clone, &hex_clone);
                on_change_clone();
            }
        });
        presets_box.append(&btn);
    }
    color_section.append(&presets_box);
    settings_box.append(&color_section);

    // 3. Brilho (1..=3)
    let brightness_row = Box::new(Orientation::Horizontal, 12);
    let brightness_label = Label::builder()
        .label("Brilho dos LEDs (1 a 3):")
        .halign(Align::Start)
        .hexpand(true)
        .build();
    brightness_row.append(&brightness_label);

    let brightness_scale = Scale::with_range(Orientation::Horizontal, 1.0, 3.0, 1.0);
    brightness_scale.set_width_request(180);
    brightness_scale.set_draw_value(true);
    brightness_scale.set_digits(0);
    brightness_row.append(&brightness_scale);
    settings_box.append(&brightness_row);

    // 4. Velocidade (1..=8)
    let speed_row = Box::new(Orientation::Horizontal, 12);
    let speed_label = Label::builder()
        .label("Velocidade do Efeito (1 a 8):")
        .halign(Align::Start)
        .hexpand(true)
        .build();
    speed_row.append(&speed_label);

    let speed_scale = Scale::with_range(Orientation::Horizontal, 1.0, 8.0, 1.0);
    speed_scale.set_width_request(180);
    speed_scale.set_draw_value(true);
    speed_scale.set_digits(0);
    speed_row.append(&speed_scale);
    settings_box.append(&speed_row);

    container.append(&settings_box);

    // Botões de Ação
    let action_box = Box::new(Orientation::Horizontal, 12);
    action_box.set_halign(Align::End);

    let apply_btn = Button::builder()
        .label("Aplicar Iluminação no Mouse")
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

    // Carregar valores iniciais
    {
        let s = state.borrow();
        let p = s.config.active_profile.index();
        let prof = &s.config.profiles[p];

        // Set mode dropdown index
        for (i, (_, lm)) in modes.iter().enumerate() {
            if *lm == prof.lightmode {
                mode_dropdown.set_selected(i as u32);
                break;
            }
        }

        let hex = prof.color.to_hex();
        hex_entry.set_text(&hex);
        update_color_preview(&color_preview, &hex);
        brightness_scale.set_value(prof.brightness as f64);
        speed_scale.set_value(prof.speed as f64);
    }

    // Signals para alterações manuais
    let s_clone = state.clone();
    let on_change_clone = on_change.clone();
    mode_dropdown.connect_selected_notify(move |dd| {
        let idx = dd.selected() as usize;
        if idx < modes.len() {
            let mut s = s_clone.borrow_mut();
            let p = s.config.active_profile.index();
            s.config.profiles[p].lightmode = modes[idx].1;
            s.is_dirty = true;
            drop(s);
            on_change_clone();
        }
    });

    let s_clone = state.clone();
    let color_prev_clone = color_preview.clone();
    let on_change_clone = on_change.clone();
    hex_entry.connect_changed(move |entry| {
        let text = entry.text().to_string();
        if let Some(rgb) = RgbColor::from_hex(&text) {
            let mut s = s_clone.borrow_mut();
            let p = s.config.active_profile.index();
            s.config.profiles[p].color = rgb;
            s.is_dirty = true;
            drop(s);
            update_color_preview(&color_prev_clone, &text);
            on_change_clone();
        }
    });

    let s_clone = state.clone();
    let on_change_clone = on_change.clone();
    brightness_scale.connect_value_changed(move |scale| {
        let val = scale.value().round() as u8;
        let mut s = s_clone.borrow_mut();
        let p = s.config.active_profile.index();
        s.config.profiles[p].brightness = val.clamp(1, 3);
        s.is_dirty = true;
        drop(s);
        on_change_clone();
    });

    let s_clone = state.clone();
    let on_change_clone = on_change.clone();
    speed_scale.connect_value_changed(move |scale| {
        let val = scale.value().round() as u8;
        let mut s = s_clone.borrow_mut();
        let p = s.config.active_profile.index();
        s.config.profiles[p].speed = val.clamp(1, 8);
        s.is_dirty = true;
        drop(s);
        on_change_clone();
    });

    container
}

fn update_color_preview(_widget: &Label, hex: &str) {
    let clean = hex.trim().trim_start_matches('#');
    let css = format!(
        ".color-preview {{ background-color: #{}; border: 1px solid #555; border-radius: 4px; padding: 4px 12px; }}",
        clean
    );
    let provider = gtk::CssProvider::new();
    provider.load_from_string(&css);
    if let Some(display) = gtk::gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
