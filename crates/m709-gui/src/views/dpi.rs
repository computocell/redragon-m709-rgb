use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Align, Box, CheckButton, DropDown, Label, Orientation, Scale, StringList};
use m709_core::{DpiStage, PollingRate, NUM_DPI_LEVELS};
use std::rc::Rc;

use crate::state::{save_to_hardware, SharedState};

pub fn build_dpi_tab(state: SharedState, on_change: Rc<dyn Fn()>) -> Box {
    let container = Box::new(Orientation::Vertical, 16);
    container.set_margin_top(20);
    container.set_margin_bottom(20);
    container.set_margin_start(24);
    container.set_margin_end(24);

    let title = Label::builder()
        .label("DPI & Desempenho")
        .halign(Align::Start)
        .css_classes(["title-2"])
        .build();
    container.append(&title);

    let desc = Label::builder()
        .label("Configure os 5 estágios de sensibilidade DPI e a taxa de atualização (Polling Rate) USB.")
        .halign(Align::Start)
        .css_classes(["dim-label"])
        .build();
    container.append(&desc);

    // 1. DPI Stages Card
    let dpi_card = Box::new(Orientation::Vertical, 12);
    dpi_card.set_css_classes(&["card"]);

    let dpi_header = Label::builder()
        .label("Estágios de DPI:")
        .halign(Align::Start)
        .css_classes(["heading"])
        .build();
    dpi_card.append(&dpi_header);

    for lvl in 0..NUM_DPI_LEVELS {
        let row = Box::new(Orientation::Horizontal, 12);

        let check = CheckButton::builder()
            .label(format!("Nível {}", lvl + 1))
            .build();

        let val_label = Label::builder()
            .label("1000 DPI")
            .width_chars(10)
            .halign(Align::Start)
            .build();

        let scale = Scale::with_range(Orientation::Horizontal, 200.0, 10000.0, 100.0);
        scale.set_hexpand(true);
        scale.set_draw_value(false);

        // Load initial values
        {
            let s = state.borrow();
            let p = s.config.active_profile.index();
            let stage = &s.config.profiles[p].dpi_stages[lvl];
            check.set_active(stage.enabled);
            scale.set_value(stage.dpi as f64);
            val_label.set_text(&format!("{} DPI", stage.dpi));
        }

        // Signals
        let s_clone = state.clone();
        let on_change_clone = on_change.clone();
        check.connect_toggled(move |btn| {
            let active = btn.is_active();
            let mut s = s_clone.borrow_mut();
            let p = s.config.active_profile.index();
            s.config.profiles[p].dpi_stages[lvl].enabled = active;
            s.is_dirty = true;
            drop(s);
            on_change_clone();
        });

        let s_clone = state.clone();
        let val_label_clone = val_label.clone();
        let on_change_clone = on_change.clone();
        scale.connect_value_changed(move |sc| {
            let raw_val = sc.value();
            let snapped_dpi = (raw_val / 100.0).round() as u32 * 100;
            val_label_clone.set_text(&format!("{} DPI", snapped_dpi));

            let mut s = s_clone.borrow_mut();
            let p = s.config.active_profile.index();
            let enabled = s.config.profiles[p].dpi_stages[lvl].enabled;
            s.config.profiles[p].dpi_stages[lvl] = DpiStage::new(snapped_dpi, enabled);
            s.is_dirty = true;
            drop(s);
            on_change_clone();
        });

        row.append(&check);
        row.append(&val_label);
        row.append(&scale);
        dpi_card.append(&row);
    }
    container.append(&dpi_card);

    // 2. Polling Rate Card
    let perf_card = Box::new(Orientation::Vertical, 12);
    perf_card.set_css_classes(&["card"]);

    let perf_header = Label::builder()
        .label("Taxa de Relatório USB (Polling Rate):")
        .halign(Align::Start)
        .css_classes(["heading"])
        .build();
    perf_card.append(&perf_header);

    let rates = [
        ("125 Hz (8 ms - Baixo consumo)", PollingRate::Hz125),
        ("250 Hz (4 ms)", PollingRate::Hz250),
        ("500 Hz (2 ms - Recomendado)", PollingRate::Hz500),
        ("1000 Hz (1 ms - Resposta rápida esportiva)", PollingRate::Hz1000),
    ];

    let rate_names: Vec<&str> = rates.iter().map(|(n, _)| *n).collect();
    let rate_strings = StringList::new(&rate_names);
    let rate_dropdown = DropDown::new(Some(rate_strings), None::<gtk::Expression>);

    // Set initial rate
    {
        let s = state.borrow();
        let p = s.config.active_profile.index();
        let current_rate = s.config.profiles[p].polling_rate;
        for (i, (_, r)) in rates.iter().enumerate() {
            if *r == current_rate {
                rate_dropdown.set_selected(i as u32);
                break;
            }
        }
    }

    let s_clone = state.clone();
    let on_change_clone = on_change.clone();
    rate_dropdown.connect_selected_notify(move |dd| {
        let idx = dd.selected() as usize;
        if idx < rates.len() {
            let mut s = s_clone.borrow_mut();
            let p = s.config.active_profile.index();
            s.config.profiles[p].polling_rate = rates[idx].1;
            s.is_dirty = true;
            drop(s);
            on_change_clone();
        }
    });

    let rate_row = Box::new(Orientation::Horizontal, 12);
    let rate_lbl = Label::builder()
        .label("Taxa de atualização:")
        .halign(Align::Start)
        .hexpand(true)
        .build();
    rate_row.append(&rate_lbl);
    rate_row.append(&rate_dropdown);
    perf_card.append(&rate_row);

    container.append(&perf_card);

    // Botão de Aplicar
    let action_box = Box::new(Orientation::Horizontal, 12);
    action_box.set_halign(Align::End);

    let apply_btn = gtk::Button::builder()
        .label("Aplicar DPI e Desempenho no Mouse")
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
