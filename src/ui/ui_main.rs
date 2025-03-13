use egui::{pos2, vec2, Color32, Pos2, RichText, Sense, Vec2};
use rand::{random, Rng};
use crate::ui::reset::reset_wsio;
use crate::ui::slowniki::{Słowa, Wybieranie};
// use egui::WidgetText::RichText;
use crate::ui::template::{templejt_3, templejt_4, templejt_5};
use crate::ui::ui_defaults::ZgadnijSlowo;

impl eframe::App for ZgadnijSlowo {


    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::H) && i.modifiers.ctrl) {
            self.podpowiedź_toggle = !self.podpowiedź_toggle;
        }
        if ctx.input(|i| i.key_pressed(egui::Key::E) && i.modifiers.ctrl) {
            self.debug = !self.debug;
        }
        ctx.request_repaint();
        egui::CentralPanel::default().show(ctx, |ui| {


            // egui::TopBottomPanel::top("wybór_gry").resizable(false).exact_height(150.).show(ctx, |ui| {
            egui::global_theme_preference_buttons(ui);
            ui.add_space(30.);
            ui.columns(1, |column| {
                column[0].vertical_centered_justified(|ui| {
                    ui.label(egui::RichText::new("Zacznij nową grę!").size(20.));
                });
            });
            ui.add_space(15.);
            ui.columns(3, |column| {
                column[0].vertical_centered_justified(|ui| {
                    if ui.add(egui::Button::new("Trzy litery").sense(Sense::click())).clicked() {
                        self.tryb_gry = 3;
                        (self.wybór_słownictwa,self.podpowiedź) = losowe_słowo_3();


                        reset_wsio(self);
                        fn losowe_słowo_3() -> (&'static str,&'static str) {
                            let mut rng = rand::rng(); // Tworzymy generator liczb losowych
                            let index = rng.random_range(0..Słowa::trzyliterowe().słowo.len()); // Losujemy indeks w zakresie długości wektora
                            Słowa::trzyliterowe().słowo[index]
                        }
                    }
                });
                let zzzz = Słowa::trzyliterowe().słowo[1];
                column[1].vertical_centered_justified(|ui| {
                    if ui.add(egui::Button::new("Cztery litery").sense(Sense::click())).clicked() {
                        self.tryb_gry = 4;
                        (self.wybór_słownictwa,self.podpowiedź) = losowe_słowo_4();

                        reset_wsio(self);
                        fn losowe_słowo_4() -> (&'static str,&'static str) {
                            let mut rng = rand::rng(); // Tworzymy generator liczb losowych
                            let index = rng.random_range(0..Słowa::czteroliterowe().słowo.len()); // Losujemy indeks w zakresie długości wektora
                            Słowa::czteroliterowe().słowo[index]
                        }
                    }
                });
                column[2].vertical_centered_justified(|ui| {
                    if ui.add(egui::Button::new("Pięć liter").sense(Sense::click())).clicked() {
                        self.tryb_gry = 5;
                        (self.wybór_słownictwa,self.podpowiedź) = losowe_słowo_5();

                        reset_wsio(self);
                        fn losowe_słowo_5() -> (&'static str,&'static str) {
                            let mut rng = rand::rng(); // Tworzymy generator liczb losowych
                            let index = rng.random_range(0..Słowa::pięcioliterowe().słowo.len()); // Losujemy indeks w zakresie długości wektora
                            Słowa::pięcioliterowe().słowo[index]
                        }
                    }
                });
            });
            ui.add_space(15.);
            // ui.put(egui::Rect::from_center_size(pos2(250., 350.), vec2(50., 50.)), egui::Label::new(RichText::new("lll").size(50.)));
            // ui.new_child(Default::default());

        });
        let xoxo = ctx.screen_rect().size().x;
        egui::Area::new(egui::Id::new("Wygrana_tekst"))
            .fixed_pos(pos2((xoxo / 2.) - 100., 150.))
            .default_size(egui::Vec2::new(100., 100.))
            .show(ctx, |ui| {
                // ui.add(egui::Label::new(RichText::new("LOL!!!!!!!!!!").size(20.)));
                ui.allocate_space(egui::Vec2::new(250., 50.));
                match &self.wygrałeś{
                    true => {ui.add(egui::Label::new(RichText::new("Wygrałeś!!!").size(40.).color(Color32::GREEN)));},
                    false=>{}
                }

            });

        egui::Area::new(egui::Id::new("podpowiedź"))
            .fixed_pos(pos2((xoxo / 2.) - 100., 180.))
            .default_size(egui::Vec2::new(100., 100.))
            .show(ctx, |ui| {
                // ui.add(egui::Label::new(RichText::new("LOL!!!!!!!!!!").size(20.)));
                ui.allocate_space(egui::Vec2::new(250., 50.));
                match &self.podpowiedź_toggle{
                    true => {ui.add(egui::Label::new(RichText::new(self.podpowiedź).size(16.)));},
                    false=>{ui.add(egui::Label::new(RichText::new("Naciśnij Ctrl + H po podpowiedź").size(16.))); }
                }

            });




        let szerokość_zgadywanki = self.tryb_gry as f32 * 32.5;
        egui::Area::new(egui::Id::new("gra"))
            .fixed_pos(egui::pos2((xoxo / 2.) - (szerokość_zgadywanki / 2.), 300.0))
            .default_size(egui::Vec2::new(120.0, 100.0))
            .movable(false)
            .show(ctx, |ui| {
                match self.debug{
                    true => {ui.label(self.wybór_słownictwa);},
                    false => {}
                }

                match self.tryb_gry {
                    3 => { templejt_3(self, ui, ctx, self.wybór_słownictwa); },
                    4 => { templejt_4(self, ui, ctx, self.wybór_słownictwa); },
                    5 => { templejt_5(self, ui, ctx, self.wybór_słownictwa); },
                    _ => {}
                }
            });

    }
}

