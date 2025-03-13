use eframe::emath::{pos2, vec2};
// use eframe::emath::Vec2;
use egui::{emath, Color32, RichText, TextEdit, Ui};
use crate::ui::ui_defaults::ZgadnijSlowo;

pub fn templejt_3(proxy_self:&mut ZgadnijSlowo,ui:&mut Ui,ctx: &egui::Context,słowo:&str){
    // println!("wygrałeś {}",&proxy_self.wygrałeś);
    ui.horizontal(|ui|{
            let mut kolor :Color32 = Default::default();
            match słowo.chars().next() {
                Some(first_char) if first_char.to_string() == proxy_self.litera_0_0 && proxy_self.litera_0_0.len()==1usize => {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor=Color32::GREEN;
                    proxy_self.rząd_0_0=2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_0_0) && proxy_self.litera_0_0.len()==1usize => {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor=Color32::ORANGE;
                    proxy_self.rząd_0_0=1;
                },
                Some(_)  => {
                    if proxy_self.litera_0_0.len()==1usize{
                    // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                    kolor=Color32::RED;
                    proxy_self.rząd_0_0=1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                    proxy_self.rząd_0_0=0;
                }
            }
            if proxy_self.litera_0_0.len() >=1{
                ui.add_space(proxy_self.rozmiar_spacji_litery);
                ui.label(RichText::new(proxy_self.litera_0_0.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
                ui.add_space(proxy_self.rozmiar_spacji_litery);
            }else {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_0_0).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)));
            }


            let mut kolor :Color32 = Default::default();
            match słowo.chars().nth(1) {
                Some(second_char) if second_char.to_string() == proxy_self.litera_0_1 && proxy_self.litera_0_1.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_1 jest taka sama jak 1. litera słowa.");
                    kolor=Color32::GREEN;
                    proxy_self.rząd_0_1=2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_0_1) && proxy_self.litera_0_1.len()==1usize => {
                    // println!("Litera proxy_self.litera_0_1 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor=Color32::ORANGE;
                    proxy_self.rząd_0_1=1;
                },
                Some(_) => {
                    if proxy_self.litera_0_0.len()==1usize {
                        // println!("Litera proxy_self.litera_0_1 nie występuje w słowie.");
                        kolor = Color32::RED;
                        proxy_self.rząd_0_1 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                    proxy_self.rząd_0_1=0;
                }
            }
            if proxy_self.litera_0_1.len() >=1{
                ui.add_space(proxy_self.rozmiar_spacji_litery);
                ui.label(RichText::new(proxy_self.litera_0_1.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
                ui.add_space(proxy_self.rozmiar_spacji_litery);
            }else {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_0_1).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)));
            }

            let mut kolor :Color32 = Default::default();
            match słowo.chars().nth(2) {
                Some(third_char) if third_char.to_string() == proxy_self.litera_0_2 && proxy_self.litera_0_2.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor=Color32::GREEN;
                    proxy_self.rząd_0_2=2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_0_2) && proxy_self.litera_0_2.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor=Color32::ORANGE;
                    proxy_self.rząd_0_2=1;
                },
                Some(_) => {
                    // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                    if proxy_self.litera_0_2.len()==1{
                        kolor=Color32::RED;
                        proxy_self.rząd_0_2=1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                    proxy_self.rząd_0_2=0;
                }
            }
            if proxy_self.litera_0_2.len() >=1{
                ui.add_space(proxy_self.rozmiar_spacji_litery);
                ui.label(RichText::new(proxy_self.litera_0_2.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
                ui.add_space(proxy_self.rozmiar_spacji_litery);
            }else {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_0_2).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)));
            }
        proxy_self.wygrałeś =  proxy_self.rząd_0_0 ==2 && proxy_self.rząd_0_1 ==2 && proxy_self.rząd_0_2 ==2 ;
    });

    ui.add_space(proxy_self.rozmiar_spacji_litery * 2.);

    // drugi rząd
    
    let checker_0 = proxy_self.rząd_0_0 >= 1 && proxy_self.rząd_0_1 >= 1 && proxy_self.rząd_0_2 >= 1 && !proxy_self.wygrałeś;
    if checker_0{ui.add_space(proxy_self.rozmiar_spacji_litery);}
    
    ui.horizontal(|ui|{
        // println!("{} {} {}",proxy_self.rząd_0_0,proxy_self.rząd_0_1, proxy_self.rząd_0_2);

        let mut kolor :Color32 = Default::default();
        if checker_0 {
            match słowo.chars().next() {
                Some(first_char) if first_char.to_string() == proxy_self.litera_1_0 && proxy_self.litera_1_0.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_1_0 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_1_0) && proxy_self.litera_1_0.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_1_0 = 1;
                },
                Some(_) => {
                    if proxy_self.litera_1_0.len()==1usize {
                        // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                        kolor = Color32::RED;
                        proxy_self.rząd_1_0 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                    proxy_self.rząd_1_0=0;
                }
            }
        }
        if proxy_self.litera_1_0.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_1_0.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_0, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_1_0).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });        }

        let mut kolor :Color32 = Default::default();
        if checker_0 {
            match słowo.chars().nth(1) {
                Some(second_char) if second_char.to_string() == proxy_self.litera_1_1 && proxy_self.litera_1_1.len()==1usize=> {
                    // println!("Litera proxy_self.litera_1_1 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_1_1 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_1_1) && proxy_self.litera_1_1.len()==1usize=> {
                    // println!("Litera proxy_self.litera_1_1 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_1_1 = 1;
                },
                Some(_) => {
                    if proxy_self.litera_1_1.len()==1usize {
                        // println!("Litera proxy_self.litera_1_1 nie występuje w słowie.");
                        kolor = Color32::RED;
                        proxy_self.rząd_1_1 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_1_1.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_1_1.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_0, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_1_1).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });        }

        let mut kolor :Color32 = Default::default();
        if checker_0 {
            match słowo.chars().nth(2) {
                Some(third_char) if third_char.to_string() == proxy_self.litera_1_2 && proxy_self.litera_1_2.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_1_2 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_1_2) && proxy_self.litera_1_2.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_1_2 = 1;
                },
                Some(_) => {
                    // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                    if proxy_self.litera_1_2.len()==1usize {
                        kolor = Color32::RED;
                        proxy_self.rząd_1_2 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_1_2.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_1_2.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_0, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_1_2).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });
        }
        if checker_0 { proxy_self.wygrałeś = proxy_self.rząd_1_0 == 2 && proxy_self.rząd_1_1 == 2 && proxy_self.rząd_1_2 == 2 };
    });

    ui.add_space(proxy_self.rozmiar_spacji_litery * 2.);

    // trzeci rząd

    let checker_1 = proxy_self.rząd_1_0 >= 1 && proxy_self.rząd_1_1 >= 1 && proxy_self.rząd_1_2 >= 1 && !proxy_self.wygrałeś;
    if checker_1{ui.add_space(proxy_self.rozmiar_spacji_litery);}

    ui.horizontal(|ui|{
        // println!("{} {} {}",proxy_self.rząd_1_0,proxy_self.rząd_1_1, proxy_self.rząd_1_2);

        let mut kolor :Color32 = Default::default();
        if checker_1 {
            match słowo.chars().next() {
                Some(first_char) if first_char.to_string() == proxy_self.litera_2_0 && proxy_self.litera_2_0.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_2_0 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_2_0) && proxy_self.litera_2_0.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_2_0 = 1;
                },
                Some(_) => {
                    if proxy_self.litera_2_0.len()==1usize {
                        // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                        kolor = Color32::RED;
                        proxy_self.rząd_2_0 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                    proxy_self.rząd_2_0=0;
                }
            }
        }
        if proxy_self.litera_2_0.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_2_0.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_1, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_2_0).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });        }

        let mut kolor :Color32 = Default::default();
        if checker_1 {
            match słowo.chars().nth(1) {
                Some(second_char) if second_char.to_string() == proxy_self.litera_2_1 && proxy_self.litera_2_1.len()==1usize=> {
                    // println!("Litera proxy_self.litera_2_1 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_2_1 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_2_1) && proxy_self.litera_2_1.len()==1usize=> {
                    // println!("Litera proxy_self.litera_2_1 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_2_1 = 1;
                },
                Some(_) => {
                    if proxy_self.litera_2_1.len()==1usize {
                        // println!("Litera proxy_self.litera_2_1 nie występuje w słowie.");
                        kolor = Color32::RED;
                        proxy_self.rząd_2_1 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_2_1.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_2_1.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_1, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_2_1).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });        }

        let mut kolor :Color32 = Default::default();
        if checker_1 {
            match słowo.chars().nth(2) {
                Some(third_char) if third_char.to_string() == proxy_self.litera_2_2 && proxy_self.litera_2_2.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_2_2 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_2_2) && proxy_self.litera_2_2.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_2_2 = 1;
                },
                Some(_) => {
                    // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                    if proxy_self.litera_2_2.len()==1usize {
                        kolor = Color32::RED;
                        proxy_self.rząd_2_2 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_2_2.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_2_2.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_1, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_2_2).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });
        }
        if checker_1 { proxy_self.wygrałeś = proxy_self.rząd_2_0 == 2 && proxy_self.rząd_2_1 == 2 && proxy_self.rząd_2_2 == 2 };
    });

    ui.add_space(proxy_self.rozmiar_spacji_litery * 2.);

    // czwarty rząd

    let checker_2 = proxy_self.rząd_2_0 >= 1 && proxy_self.rząd_2_1 >= 1 && proxy_self.rząd_2_2 >= 1 && !proxy_self.wygrałeś;
    if checker_2{ui.add_space(proxy_self.rozmiar_spacji_litery);}

    ui.horizontal(|ui|{
        // println!("{} {} {}",proxy_self.rząd_2_0,proxy_self.rząd_2_1, proxy_self.rząd_2_2);

        let mut kolor :Color32 = Default::default();
        if checker_2 {
            match słowo.chars().next() {
                Some(first_char) if first_char.to_string() == proxy_self.litera_3_0 && proxy_self.litera_3_0.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_3_0 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_3_0) && proxy_self.litera_3_0.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_3_0 = 1;
                },
                Some(_) => {
                    if proxy_self.litera_3_0.len()==1usize {
                        // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                        kolor = Color32::RED;
                        proxy_self.rząd_3_0 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                    proxy_self.rząd_2_0=0;
                }
            }
        }
        if proxy_self.litera_3_0.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_3_0.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_2, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_3_0).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });        }

        let mut kolor :Color32 = Default::default();
        if checker_2 {
            match słowo.chars().nth(1) {
                Some(second_char) if second_char.to_string() == proxy_self.litera_3_1 && proxy_self.litera_3_1.len()==1usize=> {
                    // println!("Litera proxy_self.litera_3_1 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_3_1 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_3_1) && proxy_self.litera_3_1.len()==1usize=> {
                    // println!("Litera proxy_self.litera_3_1 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_3_1 = 1;
                },
                Some(_) => {
                    if proxy_self.litera_3_1.len()==1usize {
                        // println!("Litera proxy_self.litera_3_1 nie występuje w słowie.");
                        kolor = Color32::RED;
                        proxy_self.rząd_3_1 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_3_1.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_3_1.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_2, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_3_1).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });        }

        let mut kolor :Color32 = Default::default();
        if checker_2 {
            match słowo.chars().nth(2) {
                Some(third_char) if third_char.to_string() == proxy_self.litera_3_2 && proxy_self.litera_3_2.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_3_2 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_3_2) && proxy_self.litera_3_2.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_3_2 = 1;
                },
                Some(_) => {
                    // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                    if proxy_self.litera_3_2.len()==1usize {
                        kolor = Color32::RED;
                        proxy_self.rząd_3_2 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_3_2.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_3_2.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        } else {
            ui.add_enabled(checker_2, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_3_2).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });
        }

    });
    if checker_2{ proxy_self.wygrałeś = proxy_self.rząd_3_0 == 2 && proxy_self.rząd_3_1 == 2 && proxy_self.rząd_3_2 == 2 } ;
}

pub fn templejt_4(proxy_self:&mut ZgadnijSlowo,ui:&mut Ui,ctx: &egui::Context,słowo:&str){

    ui.horizontal(|ui|{
        let mut kolor :Color32 = Default::default();
        match słowo.chars().next() {
            Some(first_char) if first_char.to_string() == proxy_self.litera_0_0 && proxy_self.litera_0_0.len()==1usize => {
                // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                kolor=Color32::GREEN;
                proxy_self.rząd_0_0=2;
            },
            Some(_) if słowo.contains(&proxy_self.litera_0_0) && proxy_self.litera_0_0.len()==1usize => {
                // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                kolor=Color32::ORANGE;
                proxy_self.rząd_0_0=1;
            },
            Some(_)  => {
                if proxy_self.litera_0_0.len()==1usize{
                    // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                    kolor=Color32::RED;
                    proxy_self.rząd_0_0=1;
                }
            },
            None => {
                // println!("Słowo jest puste.");
                proxy_self.rząd_0_0=0;
            }
        }
        if proxy_self.litera_0_0.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_0_0.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add(TextEdit::singleline(&mut proxy_self.litera_0_0).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)));
        }

        let mut kolor :Color32 = Default::default();
        match słowo.chars().nth(1) {
            Some(second_char) if second_char.to_string() == proxy_self.litera_0_1 && proxy_self.litera_0_1.len()==1usize=> {
                // println!("Litera proxy_self.litera_0_1 jest taka sama jak 1. litera słowa.");
                kolor=Color32::GREEN;
                proxy_self.rząd_0_1=2;
            },
            Some(_) if słowo.contains(&proxy_self.litera_0_1) && proxy_self.litera_0_1.len()==1usize => {
                // println!("Litera proxy_self.litera_0_1 występuje w słowie, ale nie na pierwszym miejscu.");
                kolor=Color32::ORANGE;
                proxy_self.rząd_0_1=1;
            },
            Some(_) => {
                if proxy_self.litera_0_0.len()==1usize {
                    // println!("Litera proxy_self.litera_0_1 nie występuje w słowie.");
                    kolor = Color32::RED;
                    proxy_self.rząd_0_1 = 1;
                }
            },
            None => {
                // println!("Słowo jest puste.");
                proxy_self.rząd_0_1=0;
            }
        }
        if proxy_self.litera_0_1.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_0_1.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add(TextEdit::singleline(&mut proxy_self.litera_0_1).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)));
        }

        let mut kolor :Color32 = Default::default();
        match słowo.chars().nth(2) {
            Some(third_char) if third_char.to_string() == proxy_self.litera_0_2 && proxy_self.litera_0_2.len()==1usize=> {
                // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                kolor=Color32::GREEN;
                proxy_self.rząd_0_2=2;
            },
            Some(_) if słowo.contains(&proxy_self.litera_0_2) && proxy_self.litera_0_2.len()==1usize=> {
                // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                kolor=Color32::ORANGE;
                proxy_self.rząd_0_2=1;
            },
            Some(_) => {
                // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                if proxy_self.litera_0_2.len()==1{
                    kolor=Color32::RED;
                    proxy_self.rząd_0_2=1;
                }
            },
            None => {
                // println!("Słowo jest puste.");
                proxy_self.rząd_0_2=0;
            }
        }
        if proxy_self.litera_0_2.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_0_2.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add(TextEdit::singleline(&mut proxy_self.litera_0_2).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)));
        }

        let mut kolor :Color32 = Default::default();
        match słowo.chars().nth(3) {
            Some(third_char) if third_char.to_string() == proxy_self.litera_0_3 && proxy_self.litera_0_3.len()==1usize=> {
                // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                kolor=Color32::GREEN;
                proxy_self.rząd_0_3=2;
            },
            Some(_) if słowo.contains(&proxy_self.litera_0_3) && proxy_self.litera_0_3.len()==1usize=> {
                // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                kolor=Color32::ORANGE;
                proxy_self.rząd_0_3=1;
            },
            Some(_) => {
                // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                if proxy_self.litera_0_3.len()==1{
                    kolor=Color32::RED;
                    proxy_self.rząd_0_3=1;
                }
            },
            None => {
                // println!("Słowo jest puste.");
                proxy_self.rząd_0_3=0;
            }
        }
        if proxy_self.litera_0_3.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_0_3.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add(TextEdit::singleline(&mut proxy_self.litera_0_3).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)));
        }
        proxy_self.wygrałeś =  proxy_self.rząd_0_0 == 2 && proxy_self.rząd_0_1 == 2 && proxy_self.rząd_0_2 == 2 && proxy_self.rząd_0_3 == 2 ;
    });

    ui.add_space(proxy_self.rozmiar_spacji_litery * 2.);

    // drugi rząd

    let checker_0 = proxy_self.rząd_0_0 >= 1 && proxy_self.rząd_0_1 >= 1 && proxy_self.rząd_0_2 >= 1 && proxy_self.rząd_0_3 >= 1 && !proxy_self.wygrałeś;
    if checker_0{ui.add_space(proxy_self.rozmiar_spacji_litery);}

    ui.horizontal(|ui|{
        // println!("{} {} {}",proxy_self.rząd_0_0,proxy_self.rząd_0_1, proxy_self.rząd_0_2);

        let mut kolor :Color32 = Default::default();
        if checker_0 {
            match słowo.chars().next() {
                Some(first_char) if first_char.to_string() == proxy_self.litera_1_0 && proxy_self.litera_1_0.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_1_0 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_1_0) && proxy_self.litera_1_0.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_1_0 = 1;
                },
                Some(_) => {
                    if proxy_self.litera_1_0.len()==1usize {
                        // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                        kolor = Color32::RED;
                        proxy_self.rząd_1_0 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                    proxy_self.rząd_1_0=0;
                }
            }
        }
        if proxy_self.litera_1_0.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_1_0.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_0, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_1_0).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });        }

        let mut kolor :Color32 = Default::default();
        if checker_0 {
            match słowo.chars().nth(1) {
                Some(second_char) if second_char.to_string() == proxy_self.litera_1_1 && proxy_self.litera_1_1.len()==1usize=> {
                    // println!("Litera proxy_self.litera_1_1 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_1_1 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_1_1) && proxy_self.litera_1_1.len()==1usize=> {
                    // println!("Litera proxy_self.litera_1_1 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_1_1 = 1;
                },
                Some(_) => {
                    if proxy_self.litera_1_1.len()==1usize {
                        // println!("Litera proxy_self.litera_1_1 nie występuje w słowie.");
                        kolor = Color32::RED;
                        proxy_self.rząd_1_1 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_1_1.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_1_1.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_0, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_1_1).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });        }

        let mut kolor :Color32 = Default::default();
        if checker_0 {
            match słowo.chars().nth(2) {
                Some(third_char) if third_char.to_string() == proxy_self.litera_1_2 && proxy_self.litera_1_2.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_1_2 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_1_2) && proxy_self.litera_1_2.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_1_2 = 1;
                },
                Some(_) => {
                    // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                    if proxy_self.litera_1_2.len()==1usize {
                        kolor = Color32::RED;
                        proxy_self.rząd_1_2 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_1_2.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_1_2.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_0, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_1_2).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });
        }
        let mut kolor :Color32 = Default::default();
        if checker_0 {
            match słowo.chars().nth(3) {
                Some(third_char) if third_char.to_string() == proxy_self.litera_1_3 && proxy_self.litera_1_3.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_1_3 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_1_3) && proxy_self.litera_1_3.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_1_3 = 1;
                },
                Some(_) => {
                    // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                    if proxy_self.litera_1_3.len()==1usize {
                        kolor = Color32::RED;
                        proxy_self.rząd_1_3 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_1_3.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_1_3.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_0, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_1_3).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });
        }
        if checker_0 { proxy_self.wygrałeś = proxy_self.rząd_1_0 == 2 && proxy_self.rząd_1_1 == 2 && proxy_self.rząd_1_2 == 2 && proxy_self.rząd_1_3 == 2 } ;
    });

    ui.add_space(proxy_self.rozmiar_spacji_litery * 2.);

    // trzeci rząd

    let checker_1 = proxy_self.rząd_1_0 >= 1 && proxy_self.rząd_1_1 >= 1 && proxy_self.rząd_1_2 >= 1 && proxy_self.rząd_1_3 >= 1 && !proxy_self.wygrałeś;
    if checker_1{ui.add_space(proxy_self.rozmiar_spacji_litery);}

    ui.horizontal(|ui|{
        // println!("{} {} {}",proxy_self.rząd_1_0,proxy_self.rząd_1_1, proxy_self.rząd_1_2);

        let mut kolor :Color32 = Default::default();
        if checker_1 {
            match słowo.chars().next() {
                Some(first_char) if first_char.to_string() == proxy_self.litera_2_0 && proxy_self.litera_2_0.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_2_0 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_2_0) && proxy_self.litera_2_0.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_2_0 = 1;
                },
                Some(_) => {
                    if proxy_self.litera_2_0.len()==1usize {
                        // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                        kolor = Color32::RED;
                        proxy_self.rząd_2_0 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                    proxy_self.rząd_2_0=0;
                }
            }
        }
        if proxy_self.litera_2_0.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_2_0.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_1, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_2_0).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });        }

        let mut kolor :Color32 = Default::default();
        if checker_1 {
            match słowo.chars().nth(1) {
                Some(second_char) if second_char.to_string() == proxy_self.litera_2_1 && proxy_self.litera_2_1.len()==1usize=> {
                    // println!("Litera proxy_self.litera_2_1 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_2_1 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_2_1) && proxy_self.litera_2_1.len()==1usize=> {
                    // println!("Litera proxy_self.litera_2_1 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_2_1 = 1;
                },
                Some(_) => {
                    if proxy_self.litera_2_1.len()==1usize {
                        // println!("Litera proxy_self.litera_2_1 nie występuje w słowie.");
                        kolor = Color32::RED;
                        proxy_self.rząd_2_1 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_2_1.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_2_1.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_1, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_2_1).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });        }

        let mut kolor :Color32 = Default::default();
        if checker_1 {
            match słowo.chars().nth(2) {
                Some(third_char) if third_char.to_string() == proxy_self.litera_2_2 && proxy_self.litera_2_2.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_2_2 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_2_2) && proxy_self.litera_2_2.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_2_2 = 1;
                },
                Some(_) => {
                    // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                    if proxy_self.litera_2_2.len()==1usize {
                        kolor = Color32::RED;
                        proxy_self.rząd_2_2 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_2_2.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_2_2.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_1, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_2_2).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });
        }
        
        let mut kolor :Color32 = Default::default();
        if checker_0 {
            match słowo.chars().nth(3) {
                Some(third_char) if third_char.to_string() == proxy_self.litera_2_3 && proxy_self.litera_2_3.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_2_3 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_2_3) && proxy_self.litera_2_3.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_2_3 = 1;
                },
                Some(_) => {
                    // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                    if proxy_self.litera_2_3.len()==1usize {
                        kolor = Color32::RED;
                        proxy_self.rząd_2_3 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_2_3.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_2_3.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_1, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_2_3).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });
        }
        if checker_1 { proxy_self.wygrałeś = proxy_self.rząd_2_0 == 2 && proxy_self.rząd_2_1 == 2 && proxy_self.rząd_2_2 == 2 && proxy_self.rząd_2_3 == 2 } ;

    });

    ui.add_space(proxy_self.rozmiar_spacji_litery * 2.);

    // czwarty rząd

    let checker_2 = proxy_self.rząd_2_0 >= 1 && proxy_self.rząd_2_1 >= 1 && proxy_self.rząd_2_2 >= 1 && proxy_self.rząd_2_3 >= 1 && !proxy_self.wygrałeś;
    if checker_2{ui.add_space(proxy_self.rozmiar_spacji_litery);}

    ui.horizontal(|ui|{
        // println!("{} {} {}",proxy_self.rząd_2_0,proxy_self.rząd_2_1, proxy_self.rząd_2_2);

        let mut kolor :Color32 = Default::default();
        if checker_2 {
            match słowo.chars().next() {
                Some(first_char) if first_char.to_string() == proxy_self.litera_3_0 && proxy_self.litera_3_0.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_3_0 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_3_0) && proxy_self.litera_3_0.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_3_0 = 1;
                },
                Some(_) => {
                    if proxy_self.litera_3_0.len()==1usize {
                        // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                        kolor = Color32::RED;
                        proxy_self.rząd_3_0 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_3_0.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_3_0.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_2, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_3_0).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });        }

        let mut kolor :Color32 = Default::default();
        if checker_2 {
            match słowo.chars().nth(1) {
                Some(second_char) if second_char.to_string() == proxy_self.litera_3_1 && proxy_self.litera_3_1.len()==1usize=> {
                    // println!("Litera proxy_self.litera_3_1 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_3_1 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_3_1) && proxy_self.litera_3_1.len()==1usize=> {
                    // println!("Litera proxy_self.litera_3_1 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_3_1 = 1;
                },
                Some(_) => {
                    if proxy_self.litera_3_1.len()==1usize {
                        // println!("Litera proxy_self.litera_3_1 nie występuje w słowie.");
                        kolor = Color32::RED;
                        proxy_self.rząd_3_1 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_3_1.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_3_1.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_2, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_3_1).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });        }

        let mut kolor :Color32 = Default::default();
        if checker_2 {
            match słowo.chars().nth(2) {
                Some(third_char) if third_char.to_string() == proxy_self.litera_3_2 && proxy_self.litera_3_2.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_3_2 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_3_2) && proxy_self.litera_3_2.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_3_2 = 1;
                },
                Some(_) => {
                    // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                    if proxy_self.litera_3_2.len()==1usize {
                        kolor = Color32::RED;
                        proxy_self.rząd_3_2 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_3_2.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_3_2.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        } else {
            ui.add_enabled(checker_2, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_3_2).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });
        }
        let mut kolor :Color32 = Default::default();
        if checker_0 {
            match słowo.chars().nth(3) {
                Some(third_char) if third_char.to_string() == proxy_self.litera_3_3 && proxy_self.litera_3_3.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_3_3 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_3_3) && proxy_self.litera_3_3.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_3_3 = 1;
                },
                Some(_) => {
                    // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                    if proxy_self.litera_3_3.len()==1usize {
                        kolor = Color32::RED;
                        proxy_self.rząd_3_3 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_3_3.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_3_3.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_2, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_3_3).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });
        }
        if checker_2 { proxy_self.wygrałeś = proxy_self.rząd_3_0 == 2 && proxy_self.rząd_3_1 == 2 && proxy_self.rząd_3_2 == 2 && proxy_self.rząd_3_3 == 2 } ;

    });

    ui.add_space(proxy_self.rozmiar_spacji_litery * 2.);

    // piąty rząd

    let checker_3 = proxy_self.rząd_3_0 >= 1 && proxy_self.rząd_3_1 >= 1 && proxy_self.rząd_3_2 >= 1 && proxy_self.rząd_3_3 >= 1 && !proxy_self.wygrałeś;
    if checker_3{ui.add_space(proxy_self.rozmiar_spacji_litery);}

    ui.horizontal(|ui|{
        // println!("{} {} {}",proxy_self.rząd_2_0,proxy_self.rząd_2_1, proxy_self.rząd_2_2);

        let mut kolor :Color32 = Default::default();
        if checker_3 {
            match słowo.chars().next() {
                Some(first_char) if first_char.to_string() == proxy_self.litera_4_0 && proxy_self.litera_4_0.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_4_0 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_4_0) && proxy_self.litera_4_0.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_4_0 = 1;
                },
                Some(_) => {
                    if proxy_self.litera_4_0.len()==1usize {
                        // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                        kolor = Color32::RED;
                        proxy_self.rząd_4_0 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_4_0.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_4_0.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_3, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_4_0).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });        }

        let mut kolor :Color32 = Default::default();
        if checker_3 {
            match słowo.chars().nth(1) {
                Some(second_char) if second_char.to_string() == proxy_self.litera_4_1 && proxy_self.litera_4_1.len()==1usize=> {
                    // println!("Litera proxy_self.litera_4_1 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_4_1 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_4_1) && proxy_self.litera_4_1.len()==1usize=> {
                    // println!("Litera proxy_self.litera_4_1 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_4_1 = 1;
                },
                Some(_) => {
                    if proxy_self.litera_4_1.len()==1usize {
                        // println!("Litera proxy_self.litera_4_1 nie występuje w słowie.");
                        kolor = Color32::RED;
                        proxy_self.rząd_4_1 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_4_1.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_4_1.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_3, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_4_1).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });        }

        let mut kolor :Color32 = Default::default();
        if checker_3 {
            match słowo.chars().nth(2) {
                Some(third_char) if third_char.to_string() == proxy_self.litera_4_2 && proxy_self.litera_4_2.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_4_2 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_4_2) && proxy_self.litera_4_2.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_4_2 = 1;
                },
                Some(_) => {
                    // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                    if proxy_self.litera_4_2.len()==1usize {
                        kolor = Color32::RED;
                        proxy_self.rząd_4_2 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_4_2.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_4_2.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        } else {
            ui.add_enabled(checker_3, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_4_2).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });
        }
        let mut kolor :Color32 = Default::default();
        if checker_0 {
            match słowo.chars().nth(3) {
                Some(third_char) if third_char.to_string() == proxy_self.litera_4_3 && proxy_self.litera_4_3.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_4_3 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_4_3) && proxy_self.litera_4_3.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_4_3 = 1;
                },
                Some(_) => {
                    // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                    if proxy_self.litera_4_3.len()==1usize {
                        kolor = Color32::RED;
                        proxy_self.rząd_4_3 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_4_3.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_4_3.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_3, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_4_3).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });
        }
        if checker_3 { proxy_self.wygrałeś = proxy_self.rząd_4_0 == 2 && proxy_self.rząd_4_1 == 2 && proxy_self.rząd_4_2 == 2 && proxy_self.rząd_4_3 == 2 } ;

    });

}

pub fn templejt_5(proxy_self:&mut ZgadnijSlowo,ui:&mut Ui,ctx: &egui::Context,słowo:&str){

    ui.horizontal(|ui|{
        let mut kolor :Color32 = Default::default();
        match słowo.chars().next() {
            Some(first_char) if first_char.to_string() == proxy_self.litera_0_0 && proxy_self.litera_0_0.len()==1usize => {
                // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                kolor=Color32::GREEN;
                proxy_self.rząd_0_0=2;
            },
            Some(_) if słowo.contains(&proxy_self.litera_0_0) && proxy_self.litera_0_0.len()==1usize => {
                // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                kolor=Color32::ORANGE;
                proxy_self.rząd_0_0=1;
            },
            Some(_)  => {
                if proxy_self.litera_0_0.len()==1usize{
                    // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                    kolor=Color32::RED;
                    proxy_self.rząd_0_0=1;
                }
            },
            None => {
                // println!("Słowo jest puste.");
                proxy_self.rząd_0_0=0;
            }
        }
        if proxy_self.litera_0_0.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_0_0.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add(TextEdit::singleline(&mut proxy_self.litera_0_0).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)));
        }

        let mut kolor :Color32 = Default::default();
        match słowo.chars().nth(1) {
            Some(second_char) if second_char.to_string() == proxy_self.litera_0_1 && proxy_self.litera_0_1.len()==1usize=> {
                // println!("Litera proxy_self.litera_0_1 jest taka sama jak 1. litera słowa.");
                kolor=Color32::GREEN;
                proxy_self.rząd_0_1=2;
            },
            Some(_) if słowo.contains(&proxy_self.litera_0_1) && proxy_self.litera_0_1.len()==1usize => {
                // println!("Litera proxy_self.litera_0_1 występuje w słowie, ale nie na pierwszym miejscu.");
                kolor=Color32::ORANGE;
                proxy_self.rząd_0_1=1;
            },
            Some(_) => {
                if proxy_self.litera_0_0.len()==1usize {
                    // println!("Litera proxy_self.litera_0_1 nie występuje w słowie.");
                    kolor = Color32::RED;
                    proxy_self.rząd_0_1 = 1;
                }
            },
            None => {
                // println!("Słowo jest puste.");
                proxy_self.rząd_0_1=0;
            }
        }
        if proxy_self.litera_0_1.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_0_1.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add(TextEdit::singleline(&mut proxy_self.litera_0_1).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)));
        }

        let mut kolor :Color32 = Default::default();
        match słowo.chars().nth(2) {
            Some(third_char) if third_char.to_string() == proxy_self.litera_0_2 && proxy_self.litera_0_2.len()==1usize=> {
                // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                kolor=Color32::GREEN;
                proxy_self.rząd_0_2=2;
            },
            Some(_) if słowo.contains(&proxy_self.litera_0_2) && proxy_self.litera_0_2.len()==1usize=> {
                // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                kolor=Color32::ORANGE;
                proxy_self.rząd_0_2=1;
            },
            Some(_) => {
                // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                if proxy_self.litera_0_2.len()==1{
                    kolor=Color32::RED;
                    proxy_self.rząd_0_2=1;
                }
            },
            None => {
                // println!("Słowo jest puste.");
                proxy_self.rząd_0_2=0;
            }
        }
        if proxy_self.litera_0_2.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_0_2.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add(TextEdit::singleline(&mut proxy_self.litera_0_2).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)));
        }

        let mut kolor :Color32 = Default::default();
        match słowo.chars().nth(3) {
            Some(third_char) if third_char.to_string() == proxy_self.litera_0_3 && proxy_self.litera_0_3.len()==1usize=> {
                // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                kolor=Color32::GREEN;
                proxy_self.rząd_0_3=2;
            },
            Some(_) if słowo.contains(&proxy_self.litera_0_3) && proxy_self.litera_0_3.len()==1usize=> {
                // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                kolor=Color32::ORANGE;
                proxy_self.rząd_0_3=1;
            },
            Some(_) => {
                // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                if proxy_self.litera_0_3.len()==1{
                    kolor=Color32::RED;
                    proxy_self.rząd_0_3=1;
                }
            },
            None => {
                // println!("Słowo jest puste.");
                proxy_self.rząd_0_3=0;
            }
        }
        if proxy_self.litera_0_3.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_0_3.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add(TextEdit::singleline(&mut proxy_self.litera_0_3).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)));
        }
        let mut kolor :Color32 = Default::default();
        match słowo.chars().nth(4) {
            Some(third_char) if third_char.to_string() == proxy_self.litera_0_4 && proxy_self.litera_0_4.len()==1usize=> {
                // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                kolor=Color32::GREEN;
                proxy_self.rząd_0_4=2;
            },
            Some(_) if słowo.contains(&proxy_self.litera_0_4) && proxy_self.litera_0_4.len()==1usize=> {
                // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                kolor=Color32::ORANGE;
                proxy_self.rząd_0_4=1;
            },
            Some(_) => {
                // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                if proxy_self.litera_0_4.len()==1{
                    kolor=Color32::RED;
                    proxy_self.rząd_0_4=1;
                }
            },
            None => {
                // println!("Słowo jest puste.");
                proxy_self.rząd_0_4=0;
            }
        }
        if proxy_self.litera_0_4.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_0_4.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add(TextEdit::singleline(&mut proxy_self.litera_0_4).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)));
        }
        proxy_self.wygrałeś =  proxy_self.rząd_0_0 == 2 && proxy_self.rząd_0_1 == 2 && proxy_self.rząd_0_2 == 2 && proxy_self.rząd_0_3 == 2 && proxy_self.rząd_0_4 == 2 ;
    });

    ui.add_space(proxy_self.rozmiar_spacji_litery * 2.);

    // drugi rząd

    let checker_0 = proxy_self.rząd_0_0 >= 1 && proxy_self.rząd_0_1 >= 1 && proxy_self.rząd_0_2 >= 1 && proxy_self.rząd_0_3 >= 1 && proxy_self.rząd_0_4 >= 1 && !proxy_self.wygrałeś;
    if checker_0{ui.add_space(proxy_self.rozmiar_spacji_litery);}

    ui.horizontal(|ui|{
        // println!("{} {} {}",proxy_self.rząd_0_0,proxy_self.rząd_0_1, proxy_self.rząd_0_2);

        let mut kolor :Color32 = Default::default();
        if checker_0 {
            match słowo.chars().next() {
                Some(first_char) if first_char.to_string() == proxy_self.litera_1_0 && proxy_self.litera_1_0.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_1_0 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_1_0) && proxy_self.litera_1_0.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_1_0 = 1;
                },
                Some(_) => {
                    if proxy_self.litera_1_0.len()==1usize {
                        // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                        kolor = Color32::RED;
                        proxy_self.rząd_1_0 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                    proxy_self.rząd_1_0=0;
                }
            }
        }
        if proxy_self.litera_1_0.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_1_0.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_0, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_1_0).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });        }

        let mut kolor :Color32 = Default::default();
        if checker_0 {
            match słowo.chars().nth(1) {
                Some(second_char) if second_char.to_string() == proxy_self.litera_1_1 && proxy_self.litera_1_1.len()==1usize=> {
                    // println!("Litera proxy_self.litera_1_1 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_1_1 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_1_1) && proxy_self.litera_1_1.len()==1usize=> {
                    // println!("Litera proxy_self.litera_1_1 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_1_1 = 1;
                },
                Some(_) => {
                    if proxy_self.litera_1_1.len()==1usize {
                        // println!("Litera proxy_self.litera_1_1 nie występuje w słowie.");
                        kolor = Color32::RED;
                        proxy_self.rząd_1_1 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_1_1.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_1_1.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_0, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_1_1).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });        }

        let mut kolor :Color32 = Default::default();
        if checker_0 {
            match słowo.chars().nth(2) {
                Some(third_char) if third_char.to_string() == proxy_self.litera_1_2 && proxy_self.litera_1_2.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_1_2 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_1_2) && proxy_self.litera_1_2.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_1_2 = 1;
                },
                Some(_) => {
                    // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                    if proxy_self.litera_1_2.len()==1usize {
                        kolor = Color32::RED;
                        proxy_self.rząd_1_2 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_1_2.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_1_2.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_0, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_1_2).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });
        }
        let mut kolor :Color32 = Default::default();
        if checker_0 {
            match słowo.chars().nth(3) {
                Some(third_char) if third_char.to_string() == proxy_self.litera_1_3 && proxy_self.litera_1_3.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_1_3 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_1_3) && proxy_self.litera_1_3.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_1_3 = 1;
                },
                Some(_) => {
                    // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                    if proxy_self.litera_1_3.len()==1usize {
                        kolor = Color32::RED;
                        proxy_self.rząd_1_3 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_1_3.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_1_3.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_0, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_1_3).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });
        }
        let mut kolor :Color32 = Default::default();
        if checker_0 {
            match słowo.chars().nth(4) {
                Some(third_char) if third_char.to_string() == proxy_self.litera_1_4 && proxy_self.litera_1_4.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_1_4 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_1_4) && proxy_self.litera_1_4.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_1_4 = 1;
                },
                Some(_) => {
                    // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                    if proxy_self.litera_1_4.len()==1usize {
                        kolor = Color32::RED;
                        proxy_self.rząd_1_4 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_1_4.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_1_4.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_0, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_1_4).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });
        }
        if checker_0 { proxy_self.wygrałeś = proxy_self.rząd_1_0 == 2 && proxy_self.rząd_1_1 == 2 && proxy_self.rząd_1_2 == 2 && proxy_self.rząd_1_3 == 2 && proxy_self.rząd_1_4 == 2 } ;
    });

    ui.add_space(proxy_self.rozmiar_spacji_litery * 2.);

    // trzeci rząd

    let checker_1 = proxy_self.rząd_1_0 >= 1 && proxy_self.rząd_1_1 >= 1 && proxy_self.rząd_1_2 >= 1 && proxy_self.rząd_1_3 >= 1 && proxy_self.rząd_1_4 >= 1  && !proxy_self.wygrałeś;
    if checker_1{ui.add_space(proxy_self.rozmiar_spacji_litery);}

    ui.horizontal(|ui|{
        // println!("{} {} {}",proxy_self.rząd_1_0,proxy_self.rząd_1_1, proxy_self.rząd_1_2);

        let mut kolor :Color32 = Default::default();
        if checker_1 {
            match słowo.chars().next() {
                Some(first_char) if first_char.to_string() == proxy_self.litera_2_0 && proxy_self.litera_2_0.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_2_0 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_2_0) && proxy_self.litera_2_0.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_2_0 = 1;
                },
                Some(_) => {
                    if proxy_self.litera_2_0.len()==1usize {
                        // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                        kolor = Color32::RED;
                        proxy_self.rząd_2_0 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                    proxy_self.rząd_2_0=0;
                }
            }
        }
        if proxy_self.litera_2_0.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_2_0.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_1, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_2_0).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });        }

        let mut kolor :Color32 = Default::default();
        if checker_1 {
            match słowo.chars().nth(1) {
                Some(second_char) if second_char.to_string() == proxy_self.litera_2_1 && proxy_self.litera_2_1.len()==1usize=> {
                    // println!("Litera proxy_self.litera_2_1 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_2_1 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_2_1) && proxy_self.litera_2_1.len()==1usize=> {
                    // println!("Litera proxy_self.litera_2_1 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_2_1 = 1;
                },
                Some(_) => {
                    if proxy_self.litera_2_1.len()==1usize {
                        // println!("Litera proxy_self.litera_2_1 nie występuje w słowie.");
                        kolor = Color32::RED;
                        proxy_self.rząd_2_1 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_2_1.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_2_1.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_1, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_2_1).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });        }

        let mut kolor :Color32 = Default::default();
        if checker_1 {
            match słowo.chars().nth(2) {
                Some(third_char) if third_char.to_string() == proxy_self.litera_2_2 && proxy_self.litera_2_2.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_2_2 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_2_2) && proxy_self.litera_2_2.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_2_2 = 1;
                },
                Some(_) => {
                    // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                    if proxy_self.litera_2_2.len()==1usize {
                        kolor = Color32::RED;
                        proxy_self.rząd_2_2 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_2_2.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_2_2.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_1, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_2_2).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });
        }

        let mut kolor :Color32 = Default::default();
        if checker_0 {
            match słowo.chars().nth(3) {
                Some(third_char) if third_char.to_string() == proxy_self.litera_2_3 && proxy_self.litera_2_3.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_2_3 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_2_3) && proxy_self.litera_2_3.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_2_3 = 1;
                },
                Some(_) => {
                    // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                    if proxy_self.litera_2_3.len()==1usize {
                        kolor = Color32::RED;
                        proxy_self.rząd_2_3 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_2_3.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_2_3.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_1, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_2_3).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });
        }

        let mut kolor :Color32 = Default::default();
        if checker_0 {
            match słowo.chars().nth(4) {
                Some(third_char) if third_char.to_string() == proxy_self.litera_2_4 && proxy_self.litera_2_4.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_2_4 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_2_4) && proxy_self.litera_2_4.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_2_4 = 1;
                },
                Some(_) => {
                    // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                    if proxy_self.litera_2_4.len()==1usize {
                        kolor = Color32::RED;
                        proxy_self.rząd_2_4 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_2_4.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_2_4.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_1, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_2_4).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });
        }
        if checker_1 { proxy_self.wygrałeś = proxy_self.rząd_2_0 == 2 && proxy_self.rząd_2_1 == 2 && proxy_self.rząd_2_2 == 2 && proxy_self.rząd_2_3 == 2 && proxy_self.rząd_2_4 == 2 } ;

    });

    ui.add_space(proxy_self.rozmiar_spacji_litery * 2.);

    // czwarty rząd

    let checker_2 = proxy_self.rząd_2_0 >= 1 && proxy_self.rząd_2_1 >= 1 && proxy_self.rząd_2_2 >= 1 && proxy_self.rząd_2_3 >= 1 && proxy_self.rząd_2_4 >= 1 && !proxy_self.wygrałeś;
    if checker_2{ui.add_space(proxy_self.rozmiar_spacji_litery);}

    ui.horizontal(|ui|{
        // println!("{} {} {}",proxy_self.rząd_2_0,proxy_self.rząd_2_1, proxy_self.rząd_2_2);

        let mut kolor :Color32 = Default::default();
        if checker_2 {
            match słowo.chars().next() {
                Some(first_char) if first_char.to_string() == proxy_self.litera_3_0 && proxy_self.litera_3_0.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_3_0 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_3_0) && proxy_self.litera_3_0.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_3_0 = 1;
                },
                Some(_) => {
                    if proxy_self.litera_3_0.len()==1usize {
                        // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                        kolor = Color32::RED;
                        proxy_self.rząd_3_0 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_3_0.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_3_0.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_2, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_3_0).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });        }

        let mut kolor :Color32 = Default::default();
        if checker_2 {
            match słowo.chars().nth(1) {
                Some(second_char) if second_char.to_string() == proxy_self.litera_3_1 && proxy_self.litera_3_1.len()==1usize=> {
                    // println!("Litera proxy_self.litera_3_1 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_3_1 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_3_1) && proxy_self.litera_3_1.len()==1usize=> {
                    // println!("Litera proxy_self.litera_3_1 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_3_1 = 1;
                },
                Some(_) => {
                    if proxy_self.litera_3_1.len()==1usize {
                        // println!("Litera proxy_self.litera_3_1 nie występuje w słowie.");
                        kolor = Color32::RED;
                        proxy_self.rząd_3_1 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_3_1.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_3_1.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_2, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_3_1).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });        }

        let mut kolor :Color32 = Default::default();
        if checker_2 {
            match słowo.chars().nth(2) {
                Some(third_char) if third_char.to_string() == proxy_self.litera_3_2 && proxy_self.litera_3_2.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_3_2 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_3_2) && proxy_self.litera_3_2.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_3_2 = 1;
                },
                Some(_) => {
                    // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                    if proxy_self.litera_3_2.len()==1usize {
                        kolor = Color32::RED;
                        proxy_self.rząd_3_2 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_3_2.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_3_2.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        } else {
            ui.add_enabled(checker_2, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_3_2).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });
        }
        let mut kolor :Color32 = Default::default();
        if checker_0 {
            match słowo.chars().nth(3) {
                Some(third_char) if third_char.to_string() == proxy_self.litera_3_3 && proxy_self.litera_3_3.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_3_3 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_3_3) && proxy_self.litera_3_3.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_3_3 = 1;
                },
                Some(_) => {
                    // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                    if proxy_self.litera_3_3.len()==1usize {
                        kolor = Color32::RED;
                        proxy_self.rząd_3_3 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_3_3.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_3_3.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_2, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_3_3).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });
        }

        let mut kolor :Color32 = Default::default();
        if checker_0 {
            match słowo.chars().nth(4) {
                Some(third_char) if third_char.to_string() == proxy_self.litera_3_4 && proxy_self.litera_3_4.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_3_4 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_3_4) && proxy_self.litera_3_4.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_3_4 = 1;
                },
                Some(_) => {
                    // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                    if proxy_self.litera_3_4.len()==1usize {
                        kolor = Color32::RED;
                        proxy_self.rząd_3_4 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_3_4.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_3_4.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_2, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_3_4).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });
        }
        if checker_2 { proxy_self.wygrałeś = proxy_self.rząd_3_0 == 2 && proxy_self.rząd_3_1 == 2 && proxy_self.rząd_3_2 == 2 && proxy_self.rząd_3_3 == 2 && proxy_self.rząd_3_4 == 2 } ;

    });

    ui.add_space(proxy_self.rozmiar_spacji_litery * 2.);

    // piąty rząd

    let checker_3 = proxy_self.rząd_3_0 >= 1 && proxy_self.rząd_3_1 >= 1 && proxy_self.rząd_3_2 >= 1 && proxy_self.rząd_3_3 >= 1 && proxy_self.rząd_3_4 >= 1 && !proxy_self.wygrałeś;
    if checker_3{ui.add_space(proxy_self.rozmiar_spacji_litery);}

    ui.horizontal(|ui|{
        // println!("{} {} {}",proxy_self.rząd_2_0,proxy_self.rząd_2_1, proxy_self.rząd_2_2);

        let mut kolor :Color32 = Default::default();
        if checker_3 {
            match słowo.chars().next() {
                Some(first_char) if first_char.to_string() == proxy_self.litera_4_0 && proxy_self.litera_4_0.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_4_0 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_4_0) && proxy_self.litera_4_0.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_4_0 = 1;
                },
                Some(_) => {
                    if proxy_self.litera_4_0.len()==1usize {
                        // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                        kolor = Color32::RED;
                        proxy_self.rząd_4_0 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_4_0.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_4_0.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_3, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_4_0).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });        }

        let mut kolor :Color32 = Default::default();
        if checker_3 {
            match słowo.chars().nth(1) {
                Some(second_char) if second_char.to_string() == proxy_self.litera_4_1 && proxy_self.litera_4_1.len()==1usize=> {
                    // println!("Litera proxy_self.litera_4_1 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_4_1 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_4_1) && proxy_self.litera_4_1.len()==1usize=> {
                    // println!("Litera proxy_self.litera_4_1 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_4_1 = 1;
                },
                Some(_) => {
                    if proxy_self.litera_4_1.len()==1usize {
                        // println!("Litera proxy_self.litera_4_1 nie występuje w słowie.");
                        kolor = Color32::RED;
                        proxy_self.rząd_4_1 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_4_1.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_4_1.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_3, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_4_1).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });        }

        let mut kolor :Color32 = Default::default();
        if checker_3 {
            match słowo.chars().nth(2) {
                Some(third_char) if third_char.to_string() == proxy_self.litera_4_2 && proxy_self.litera_4_2.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_4_2 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_4_2) && proxy_self.litera_4_2.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_4_2 = 1;
                },
                Some(_) => {
                    // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                    if proxy_self.litera_4_2.len()==1usize {
                        kolor = Color32::RED;
                        proxy_self.rząd_4_2 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_4_2.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_4_2.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        } else {
            ui.add_enabled(checker_3, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_4_2).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });
        }
        let mut kolor :Color32 = Default::default();
        if checker_0 {
            match słowo.chars().nth(3) {
                Some(third_char) if third_char.to_string() == proxy_self.litera_4_3 && proxy_self.litera_4_3.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_4_3 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_4_3) && proxy_self.litera_4_3.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_4_3 = 1;
                },
                Some(_) => {
                    // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                    if proxy_self.litera_4_3.len()==1usize {
                        kolor = Color32::RED;
                        proxy_self.rząd_4_3 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_4_3.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_4_3.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_3, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_4_3).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });
        }
        let mut kolor :Color32 = Default::default();
        if checker_0 {
            match słowo.chars().nth(4) {
                Some(third_char) if third_char.to_string() == proxy_self.litera_4_4 && proxy_self.litera_4_4.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor = Color32::GREEN;
                    proxy_self.rząd_4_4 = 2;
                },
                Some(_) if słowo.contains(&proxy_self.litera_4_4) && proxy_self.litera_4_4.len()==1usize=> {
                    // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor = Color32::ORANGE;
                    proxy_self.rząd_4_4 = 1;
                },
                Some(_) => {
                    // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
                    if proxy_self.litera_4_4.len()==1usize {
                        kolor = Color32::RED;
                        proxy_self.rząd_4_4 = 1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                }
            }
        }
        if proxy_self.litera_4_4.len() >=1{
            ui.add_space(proxy_self.rozmiar_spacji_litery);
            ui.label(RichText::new(proxy_self.litera_4_4.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
            ui.add_space(proxy_self.rozmiar_spacji_litery);
        }else {
            ui.add_enabled(checker_3, |ui:&mut Ui| {
                ui.add(TextEdit::singleline(&mut proxy_self.litera_4_4).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)))
            });
        }
        if checker_3 { proxy_self.wygrałeś = proxy_self.rząd_4_0 == 2 && proxy_self.rząd_4_1 == 2 && proxy_self.rząd_4_2 == 2 && proxy_self.rząd_4_3 == 2 && proxy_self.rząd_4_4 == 2 } ;

    });

}