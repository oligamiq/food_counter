/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    #[serde(skip)]
    sold_food: Vec<SoldFood>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            sold_food: vec![],
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SoldFood {
    pub name: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

impl TemplateApp {
    pub fn add_sold_food(&mut self, name: String) {
        for _ in 0..3 {
            self.sold_food.push(SoldFood {
                name: name.clone(),
                time: chrono::Utc::now(),
            });
        }

        // 保存
        let path = "sold_food.json";
        if let Err(e) = self.save_to_file(std::path::Path::new(path)) {
            eprintln!("Failed to save file: {}", e);
        }
    }

    // 名前ごとに売れた個数を返す
    pub fn sold_food_count(&self) -> Vec<(String, usize)> {
        // プレーン、チョコ、いちご、はちみつ、シナモン
        let mut count = vec![];

        for food in &["プレーン", "チョコ", "いちご", "はちみつ", "シナモン"] {
            count.push((
                food.to_string(),
                self.sold_food.iter().filter(|f| f.name == *food).count(),
            ));
        }

        count
    }

    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        let mut fonts = egui::FontDefinitions::default();

        fonts.font_data.insert(
            "NotoSansJP".to_owned(),
            egui::FontData::from_static(include_bytes!("../fonts/NotoSansJP-Regular.ttf")),
        );

        fonts
            .families
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "NotoSansJP".to_owned());

        cc.egui_ctx.set_fonts(fonts);

        Default::default()
    }

    pub fn save_to_file(&self, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
        if std::fs::exists(path)? {
            std::fs::remove_file(path)?;
        }
        let file = std::fs::File::create(path)?;
        serde_json::to_writer(file, self.sold_food.as_slice())?;

        Ok(())
    }

    pub fn load_from_file(
        &mut self,
        path: &std::path::Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if !std::fs::exists(path)? {
            return Ok(());
        }

        let file = std::fs::File::open(path)?;
        self.sold_food = serde_json::from_reader(file)?;

        Ok(())
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                        // save
                        let path = "sold_food.json";
                        if ui.button("Save").clicked() {
                            if let Err(e) = self.save_to_file(std::path::Path::new(path)) {
                                eprintln!("Failed to save file: {}", e);
                            }
                        }
                        // load
                        if ui.button("Load").clicked() {
                            if let Err(e) = self.load_from_file(std::path::Path::new(path)) {
                                eprintln!("Failed to load file: {}", e);
                            }
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let range = ui.clip_rect().width();
            let text_size = range * 0.03;

            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("sold food counter");

            // ui.horizontal(|ui| {
            //     ui.label("Write something: ");
            //     ui.text_edit_singleline(&mut self.label);
            // });

            // ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            // if ui.button("Increment").clicked() {
            //     self.value += 1.0;
            // }

            // ui.separator();

            // ui.add(egui::github_link_file!(
            //     "https://github.com/emilk/eframe_template/blob/main/",
            //     "Source code."
            // ));

            // ui.with_layout(
            //     egui::Layout::top_down_justified(egui::Align::Center),
            //     |ui| {
            ui.separator();

            // 大きく真ん中の上に配置
            // ui.heading("売れた玉数");
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new(format!("{}玉売れました", self.sold_food.len()))
                        .size(text_size * 2.0),
                );
            });

            // 下の方
            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add_space(text_size * 2.0);

                ui.horizontal(|ui| {
                    // 0.8倍の範囲にボタンを配置
                    // プレーン、チョコ、いちご、はちみつ、シナモン
                    let food_list = &["プレーン", "チョコ", "いちご", "はちみつ", "シナモン"];
                    let spacing = range * 0.8 / (food_list.len() as f32 + 1.0); // ボタンの間にスペースを加える

                    ui.add_space(range * 0.15); // 左スペースを追加してボタンを中央寄せに
                    for food in food_list {
                        let space = spacing - text_size * 4 as f32;

                        ui.add_space(space); // 左スペースを追加してボタンを中央寄せに
                        if ui
                            .button(egui::RichText::new(*food).size(text_size))
                            .clicked()
                        {
                            self.add_sold_food(food.to_string());
                        }
                    }

                    // 取り消し
                    ui.add_space(spacing * 0.2); // 左スペースを追加してボタンを中央寄せに
                    if ui
                        .button(egui::RichText::new("取り消し").size(text_size))
                        .clicked()
                    {
                        self.sold_food.pop();

                        self.save_to_file(std::path::Path::new("sold_food.json"))
                            .unwrap_or_else(|e| eprintln!("Failed to save file: {}", e));
                    }
                });
            });

            let binding = self.sold_food_count();
            let max = binding
                .iter()
                .cloned()
                .map(|(_, count)| count)
                .max()
                .unwrap_or(0);

            {
                use egui_plotter::EguiBackend;
                use plotters::prelude::*;

                ui.scope_builder(
                    egui::UiBuilder::new().max_rect({
                        // 中心に向かって0.6倍
                        let rect = ui.clip_rect();

                        // println!("rect: {:?}", rect);

                        let center = rect.center();
                        let width = (rect.width() * 0.6).max(200.0);
                        let height = (rect.height() * 0.6).max(200.0);
                        let correct = egui::vec2(width, height);
                        let correct = egui::Rect::from_center_size(center, correct);
                        correct
                    }),
                    |ui| {
                        let root = EguiBackend::new(ui).into_drawing_area();
                        let mut chart = ChartBuilder::on(&root)
                            .margin(5)
                            .x_label_area_size(0)
                            .y_label_area_size(30)
                            .build_cartesian_2d((0u32..4u32).into_segmented(), 0..max + 1)
                            .unwrap();

                        chart.configure_mesh().draw().unwrap();

                        let sold_food_count = self.sold_food_count();

                        chart
                            .draw_series(
                                Histogram::vertical(&chart)
                                    .style(RED.mix(0.5).filled())
                                    .data(
                                        sold_food_count
                                            .iter()
                                            .enumerate()
                                            .map(|(i, (_, count))| (i as u32, *count)),
                                    ),
                            )
                            .unwrap()
                            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

                        root.present().unwrap();
                    },
                );
            }

            ui.separator();
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
