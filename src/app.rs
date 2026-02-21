use gtk4::gdk_pixbuf::{InterpType, Pixbuf};
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box, CssProvider, Frame, Label, Orientation, Picture,
    STYLE_PROVIDER_PRIORITY_APPLICATION, Separator, SizeGroup, SizeGroupMode,
};

use crate::config::{Category, Document, KeyEntry, Theme};

pub fn run(doc: Document) {
    let app = Application::builder().application_id("com.keylist").build();

    app.connect_activate(move |app| {
        build(app, &doc);
    });

    app.run_with_args(&[] as &[&str]);
}

fn load_picture(path: &str, height: i32) -> Picture {
    let pixbuf = Pixbuf::from_file(path).unwrap();
    let width = (height as f64 * pixbuf.width() as f64 / pixbuf.height() as f64) as i32;
    let scaled = pixbuf
        .scale_simple(width, height, InterpType::Hyper)
        .unwrap();
    let pic = Picture::for_paintable(&gtk4::gdk::Texture::for_pixbuf(&scaled));
    pic.set_can_shrink(false);
    pic.set_size_request(width, height);
    pic
}

fn keys_box(entries: &[KeyEntry], dark: bool, height: i32) -> Box {
    let hbox = Box::new(Orientation::Horizontal, 5);
    hbox.set_halign(gtk4::Align::Center);
    for (i, entry) in entries.iter().enumerate() {
        if i > 0 {
            hbox.append(&Label::new(Some("+")));
        }
        match entry {
            KeyEntry::Single(key) => {
                hbox.append(&load_picture(&key.to_path(dark), height));
            }
            KeyEntry::OneOf(alts) => {
                let oneof_box = Box::new(Orientation::Horizontal, 2);
                for key in alts {
                    oneof_box.append(&load_picture(&key.to_path(dark), height));
                }
                hbox.append(&oneof_box);
            }
        }
    }
    hbox
}

fn binding_row(
    description: &str,
    entries: &[KeyEntry],
    dark: bool,
    img_height: i32,
    desc_group: &SizeGroup,
    keys_group: &SizeGroup,
) -> Box {
    let desc = Label::new(Some(description));
    desc.set_halign(gtk4::Align::Center);
    desc.set_hexpand(true);
    desc.set_margin_start(8);
    desc.set_margin_end(8);

    let kb_wrapper = Box::new(Orientation::Horizontal, 0);
    kb_wrapper.set_hexpand(true);
    let kb = keys_box(entries, dark, img_height);
    kb.set_halign(gtk4::Align::Center);
    kb.set_hexpand(true);
    kb_wrapper.append(&kb);

    desc_group.add_widget(&desc);
    keys_group.add_widget(&kb_wrapper);

    let row = Box::new(Orientation::Horizontal, 16);
    row.set_css_classes(&["binding-row"]);
    row.append(&desc);
    row.append(&kb_wrapper);
    row
}

fn category_weight(cat: &Category) -> usize {
    cat.bindings.len() + if cat.name.is_some() { 1 } else { 0 }
}

fn distribute<'a>(categories: &'a [Category], columns: usize) -> Vec<Vec<&'a Category>> {
    let total: usize = categories.iter().map(category_weight).sum();
    let target = total as f32 / columns as f32;
    let mut result = vec![vec![]; columns];
    let mut col = 0;
    let mut col_weight = 0;
    for cat in categories {
        result[col].push(cat);
        col_weight += category_weight(cat);
        if col < columns - 1 && col_weight as f32 >= target * (col + 1) as f32 {
            col += 1;
        }
    }
    result
}

fn build_column(
    categories: &[&Category],
    dark: bool,
    img_height: i32,
    colors: &[String],
    color_offset: usize,
) -> Box {
    let col_box = Box::new(Orientation::Vertical, 8);
    let desc_group = SizeGroup::new(SizeGroupMode::Horizontal);
    let keys_group = SizeGroup::new(SizeGroupMode::Horizontal);

    let header_row = Box::new(Orientation::Horizontal, 0);
    for (text, group) in [("Function", &desc_group), ("Binding", &keys_group)] {
        let label = Label::new(Some(text));
        label.set_css_classes(&["header"]);
        label.set_halign(gtk4::Align::Center);
        label.set_hexpand(true);
        group.add_widget(&label);
        header_row.append(&label);
    }
    col_box.append(&header_row);

    let sep = Separator::new(Orientation::Horizontal);
    sep.set_margin_bottom(4);
    sep.set_size_request(-1, 3);
    col_box.append(&sep);

    let mut color_idx = color_offset;
    for category in categories {
        if let Some(name) = &category.name {
            let color = &colors[color_idx % colors.len()];
            color_idx += 1;

            let per_css = CssProvider::new();
            per_css.load_from_data(&format!(
                ".category-frame {{ border-color: {color}; }}
                 .cat-title {{ color: {color}; font-weight: bold; }}"
            ));

            let frame = Frame::new(None);
            frame.set_css_classes(&["category-frame"]);
            frame
                .style_context()
                .add_provider(&per_css, STYLE_PROVIDER_PRIORITY_APPLICATION);

            let title = Label::new(Some(name.as_str()));
            title.set_css_classes(&["cat-title"]);
            title.set_halign(gtk4::Align::Center);
            title
                .style_context()
                .add_provider(&per_css, STYLE_PROVIDER_PRIORITY_APPLICATION);

            let list = Box::new(Orientation::Vertical, 0);
            for binding in &category.bindings {
                list.append(&binding_row(
                    &binding.description,
                    &binding.keys,
                    dark,
                    img_height,
                    &desc_group,
                    &keys_group,
                ));
            }

            let wrapper = Box::new(Orientation::Vertical, 4);
            wrapper.set_margin_top(4);
            wrapper.append(&title);
            wrapper.append(&list);
            frame.set_child(Some(&wrapper));
            col_box.append(&frame);
        } else {
            for binding in &category.bindings {
                col_box.append(&binding_row(
                    &binding.description,
                    &binding.keys,
                    dark,
                    img_height,
                    &desc_group,
                    &keys_group,
                ));
            }
        }
    }
    col_box
}

pub fn build(app: &Application, doc: &Document) {
    let dark = matches!(doc.config.theme, Theme::Dark);
    let img_height = (doc.config.font_size * 1.5) as i32;

    let named_count = doc.categories.iter().filter(|c| c.name.is_some()).count();
    let colors: Vec<String> = (0..named_count)
        .map(|i| {
            let hue = (i as f32 / named_count as f32) * 360.0;
            format!(
                "rgba({},{},{},0.85)",
                (128.0 + 80.0 * hue.to_radians().sin()) as u8,
                (128.0 + 80.0 * (hue + 120.0).to_radians().sin()) as u8,
                (128.0 + 80.0 * (hue + 240.0).to_radians().sin()) as u8,
            )
        })
        .collect();

    let settings = gtk4::Settings::default().unwrap();
    settings.set_gtk_application_prefer_dark_theme(dark);

    let css = CssProvider::new();
    css.load_from_data(&format!(
        "* {{ font-family: {}; font-size: {}px; }}
        .header {{ font-weight: bold; opacity: 0.5; }}
        .category-frame {{ border-radius: 8px; border-width: 2px; background-color: transparent; }}
        .category-frame > * {{ background-color: transparent; }}
        frame > label {{ background-color: transparent; }}
        .binding-row {{ padding: 4px 8px; }}",
        doc.config.font, doc.config.font_size,
    ));
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().unwrap(),
        &css,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let columns_box = Box::new(Orientation::Horizontal, 24);
    columns_box.set_margin_top(16);
    columns_box.set_margin_bottom(16);
    columns_box.set_margin_start(16);
    columns_box.set_margin_end(16);

    let mut color_offset = 0;
    for col_cats in distribute(&doc.categories, doc.config.columns as usize) {
        let col = build_column(&col_cats, dark, img_height, &colors, color_offset);
        columns_box.append(&col);
        color_offset += col_cats.iter().filter(|c| c.name.is_some()).count();
    }

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Keylist")
        .build();
    window.set_child(Some(&columns_box));
    window.present();
}
