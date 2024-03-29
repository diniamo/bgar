use std::{thread, time::Duration};

use chrono::{format::StrftimeItems, Local};
use gtk::{
    gdk::Screen,
    gio,
    glib::{self, clone},
    prelude::{
        ApplicationExt, ApplicationExtManual, ContainerExt, CssProviderExt, LabelExt, WidgetExt,
    },
    Application, ApplicationWindow, Box, CssProvider, Label, Orientation, StyleContext,
    STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use gtk_layer_shell::{Layer, LayerShell};
use sysinfo::{CpuRefreshKind, RefreshKind, System};

fn main() {
    let application = Application::new(Some("me.diniamo.bgar"), Default::default());
    application.connect_activate(|app| {
        let css = CssProvider::new();
        css.load_from_data(include_bytes!("../res/style.css"))
            .unwrap();
        StyleContext::add_provider_for_screen(
            &Screen::default().expect("Failed to get the default gdk screen"),
            &css,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let window = ApplicationWindow::new(app);
        window.init_layer_shell();
        window.set_namespace("bgar");
        window.set_layer(Layer::Background);

        let vbox = Box::new(Orientation::Vertical, 0);

        let time = Label::builder().name("time").build();
        let cpu = Label::builder().name("cpu").build();
        vbox.add(&time);
        vbox.add(&cpu);

        window.add(&vbox);
        window.show_all();

        let (sender, receiver) = async_channel::bounded(1);

        gio::spawn_blocking(move || {
            let format = StrftimeItems::new("%R");
            let mut system = System::new_with_specifics(
                RefreshKind::new().with_cpu(CpuRefreshKind::new().with_cpu_usage()),
            );
            let cores = system.physical_core_count().unwrap() as f32;

            loop {
                let time_label = Local::now().format_with_items(format.clone()).to_string();

                system.refresh_cpu();
                let cpu_usage = system.global_cpu_info().cpu_usage();
                let cpu_label = format!(
                    "{}% / {}%",
                    f32::round(cpu_usage / cores),
                    f32::round(cpu_usage),
                );

                sender
                    .send_blocking((time_label, cpu_label))
                    .expect("Failed to send data from update thread");
                thread::sleep(Duration::from_secs(5));
            }
        });

        glib::spawn_future_local(clone!(@weak time, @weak cpu => async move {
            while let Ok((time_label, cpu_label)) = receiver.recv().await {
                time.set_label(&time_label);
                cpu.set_label(&cpu_label);
            }
        }));
    });
    application.run();
}
