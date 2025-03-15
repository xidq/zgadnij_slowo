// use eframe::emath::{pos2, vec2};
// use eframe::emath::Vec2;
use egui::{emath, Color32, RichText, TextEdit, Ui};
use crate::ui::ui_defaults::ZgadnijSlowo;



macro_rules! proces_słowa {
    ($litera:ident, $litera_rząd:tt, $litera_pozycja:tt, $rząd:ident, $proxy:ident, $słowo:ident, $ui:ident, $czeker:expr) => {
        let mut kolor :Color32 = Default::default();
            match $słowo.chars().nth($litera_pozycja) {
                Some(first_char) if first_char.to_string() == $proxy.$litera[$litera_rząd].$litera_pozycja && $proxy.$litera[$litera_rząd].$litera_pozycja.len() == 1usize => {
                    // println!("Litera $proxy.litera_0_0 jest taka sama jak 1. litera słowa.");
                    kolor=Color32::GREEN;
                    $proxy.$rząd[$litera_rząd].$litera_pozycja=2;
                },
                Some(_) if $słowo.contains(&$proxy.$litera[$litera_rząd].$litera_pozycja) && $proxy.$litera[$litera_rząd].$litera_pozycja.len() == 1usize => {
                    // println!("Litera $proxy.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
                    kolor=Color32::ORANGE;
                    $proxy.$rząd[$litera_rząd].$litera_pozycja=1;
                },
                Some(_)  => {
                    if $proxy.$litera[$litera_rząd].$litera_pozycja.len() == 1usize{
                    // println!("Litera $proxy.litera_0_0 nie występuje w słowie.");
                    kolor=Color32::RED;
                    $proxy.$rząd[$litera_rząd].$litera_pozycja=1;
                    }
                },
                None => {
                    // println!("Słowo jest puste.");
                    $proxy.$rząd[$litera_rząd].$litera_pozycja=0;
                }
            }
            if !$proxy.$litera[$litera_rząd].$litera_pozycja.is_empty(){
                $ui.add_space($proxy.rozmiar_spacji_litery);
                $ui.label(RichText::new($proxy.$litera[$litera_rząd].$litera_pozycja.as_str().to_uppercase()).monospace().color(kolor).size($proxy.rozmiar_litery));
                $ui.add_space($proxy.rozmiar_spacji_litery);
            }else {
                $ui.add_enabled($czeker, |ui:&mut Ui| {
                    ui.add(TextEdit::singleline(&mut $proxy.$litera[$litera_rząd].$litera_pozycja).background_color(Color32::GRAY).desired_width($proxy.rozmiar_pola_tekstowego_x).min_size(emath::vec2($proxy.rozmiar_pola_tekstowego_x, $proxy.rozmiar_pola_tekstowego_y)))
                });
            }};
}














// Yeah, I know I can do that in loops etc,
// but I wanna do this like that, coz I can
// I think loops need more memory etc


pub fn templejt(proxy_self:&mut ZgadnijSlowo,ui:&mut Ui,_ctx: &egui::Context,słowo:&str,ilość:u8){
    // println!("wygrałeś {}",&proxy_self.wygrałeś);
    ui.horizontal(|ui|{
    


        //     let mut kolor :Color32 = Default::default();
        //     match słowo.chars().next() {
        //         Some(first_char) if first_char.to_string() == proxy_self.litera_0_0 && proxy_self.litera_0_0.len() == 1usize => {
        //             // println!("Litera proxy_self.litera_0_0 jest taka sama jak 1. litera słowa.");
        //             kolor=Color32::GREEN;
        //             proxy_self.rząd_0_0=2;
        //         },
        //         Some(_) if słowo.contains(&proxy_self.litera_0_0) && proxy_self.litera_0_0.len() == 1usize => {
        //             // println!("Litera proxy_self.litera_0_0 występuje w słowie, ale nie na pierwszym miejscu.");
        //             kolor=Color32::ORANGE;
        //             proxy_self.rząd_0_0=1;
        //         },
        //         Some(_)  => {
        //             if proxy_self.litera_0_0.len() == 1usize{
        //             // println!("Litera proxy_self.litera_0_0 nie występuje w słowie.");
        //             kolor=Color32::RED;
        //             proxy_self.rząd_0_0=1;
        //             }
        //         },
        //         None => {
        //             // println!("Słowo jest puste.");
        //             proxy_self.rząd_0_0=0;
        //         }
        //     }
        //     if !proxy_self.litera_0_0.is_empty(){
        //         ui.add_space(proxy_self.rozmiar_spacji_litery);
        //         ui.label(RichText::new(proxy_self.litera_0_0.as_str().to_uppercase()).monospace().color(kolor).size(proxy_self.rozmiar_litery));
        //         ui.add_space(proxy_self.rozmiar_spacji_litery);
        //     }else {
        //         ui.add(TextEdit::singleline(&mut proxy_self.litera_0_0).background_color(Color32::GRAY).desired_width(proxy_self.rozmiar_pola_tekstowego_x).min_size(emath::vec2(proxy_self.rozmiar_pola_tekstowego_x, proxy_self.rozmiar_pola_tekstowego_y)));
        //     }
        match ilość{
            1=>{
                proces_słowa!(litery,0,0, rzędy,proxy_self,słowo,ui, true);
            },
            2=>{
                proces_słowa!(litery,0,0, rzędy,proxy_self,słowo,ui, true);
                proces_słowa!(litery,0,1, rzędy,proxy_self,słowo,ui, true);
            },
            3=>{
                proces_słowa!(litery,0,0, rzędy,proxy_self,słowo,ui, true);
                proces_słowa!(litery,0,1, rzędy,proxy_self,słowo,ui, true);
                proces_słowa!(litery,0,2, rzędy,proxy_self,słowo,ui, true);
            }
            4=>{
                proces_słowa!(litery,0,0, rzędy,proxy_self,słowo,ui, true);
                proces_słowa!(litery,0,1, rzędy,proxy_self,słowo,ui, true);
                proces_słowa!(litery,0,2, rzędy,proxy_self,słowo,ui, true);
                proces_słowa!(litery,0,3, rzędy,proxy_self,słowo,ui, true);
            },
            5=>{
                proces_słowa!(litery,0,0, rzędy,proxy_self,słowo,ui, true);
                proces_słowa!(litery,0,1, rzędy,proxy_self,słowo,ui, true);
                proces_słowa!(litery,0,2, rzędy,proxy_self,słowo,ui, true);
                proces_słowa!(litery,0,3, rzędy,proxy_self,słowo,ui, true);
                proces_słowa!(litery,0,4, rzędy,proxy_self,słowo,ui, true);
            },
            6=>{
                proces_słowa!(litery,0,0, rzędy,proxy_self,słowo,ui, true);
                proces_słowa!(litery,0,1, rzędy,proxy_self,słowo,ui, true);
                proces_słowa!(litery,0,2, rzędy,proxy_self,słowo,ui, true);
                proces_słowa!(litery,0,3, rzędy,proxy_self,słowo,ui, true);
                proces_słowa!(litery,0,4, rzędy,proxy_self,słowo,ui, true);
                proces_słowa!(litery,0,5, rzędy,proxy_self,słowo,ui, true);
            },
            _=>{}
        }

        
        proxy_self.wygrałeś = match ilość{
            1=> { proxy_self.rzędy[0].0 == 2 },
            2=> { proxy_self.rzędy[0].0 == 2 && proxy_self.rzędy[0].1 == 2 },
            3=> { proxy_self.rzędy[0].0 == 2 && proxy_self.rzędy[0].1 == 2 && proxy_self.rzędy[0].2 == 2 },
            4=> { proxy_self.rzędy[0].0 == 2 && proxy_self.rzędy[0].1 == 2 && proxy_self.rzędy[0].2 == 2 && proxy_self.rzędy[0].3 == 2 },
            5=> { proxy_self.rzędy[0].0 == 2 && proxy_self.rzędy[0].1 == 2 && proxy_self.rzędy[0].2 == 2 && proxy_self.rzędy[0].3 == 2 && proxy_self.rzędy[0].4 == 2 },
            6=> { proxy_self.rzędy[0].0 == 2 && proxy_self.rzędy[0].1 == 2 && proxy_self.rzędy[0].2 == 2 && proxy_self.rzędy[0].3 == 2 && proxy_self.rzędy[0].4 == 2 && proxy_self.rzędy[0].5 == 2 },
            _ => {false}
        };
        
    });

    ui.add_space(proxy_self.rozmiar_spacji_litery * 2.);

    // drugi rząd

    let checker_0 =match ilość{
        1=> { proxy_self.rzędy[0].0 >= 1 && !proxy_self.wygrałeś},
        2=> { proxy_self.rzędy[0].0 >=1 && proxy_self.rzędy[0].1 >=1 && !proxy_self.wygrałeś},
        3=> { proxy_self.rzędy[0].0 >=1 && proxy_self.rzędy[0].1 >=1 && proxy_self.rzędy[0].2 >=1 && !proxy_self.wygrałeś},
        4=> { proxy_self.rzędy[0].0 >=1 && proxy_self.rzędy[0].1 >=1 && proxy_self.rzędy[0].2 >=1 && proxy_self.rzędy[0].3 >=1 && !proxy_self.wygrałeś},
        5=> { proxy_self.rzędy[0].0 >=1 && proxy_self.rzędy[0].1 >=1 && proxy_self.rzędy[0].2 >=1 && proxy_self.rzędy[0].3 >=1 && proxy_self.rzędy[0].4 >=1 && !proxy_self.wygrałeś},
        6=> { proxy_self.rzędy[0].0 >=1 && proxy_self.rzędy[0].1 >=1 && proxy_self.rzędy[0].2 >=1 && proxy_self.rzędy[0].3 >=1 && proxy_self.rzędy[0].4 >=1 && proxy_self.rzędy[0].5 >=1 && !proxy_self.wygrałeś},
        _ => {false}
    };
    
    if checker_0{ui.add_space(proxy_self.rozmiar_spacji_litery);}

    ui.horizontal(|ui|{
        // println!("{} {} {}",proxy_self.rząd_0_0,proxy_self.rząd_0_1, proxy_self.rząd_0_2);

        match ilość{
            1=>{
                proces_słowa!(litery,1,0, rzędy,proxy_self,słowo,ui, checker_0);
            },
            2=>{
                proces_słowa!(litery,1,0, rzędy,proxy_self,słowo,ui, checker_0);
                proces_słowa!(litery,1,1, rzędy,proxy_self,słowo,ui, checker_0);
            },
            3=>{
                proces_słowa!(litery,1,0, rzędy,proxy_self,słowo,ui, checker_0);
                proces_słowa!(litery,1,1, rzędy,proxy_self,słowo,ui, checker_0);
                proces_słowa!(litery,1,2, rzędy,proxy_self,słowo,ui, checker_0);
            }
            4=>{
                proces_słowa!(litery,1,0, rzędy,proxy_self,słowo,ui, checker_0);
                proces_słowa!(litery,1,1, rzędy,proxy_self,słowo,ui, checker_0);
                proces_słowa!(litery,1,2, rzędy,proxy_self,słowo,ui, checker_0);
                proces_słowa!(litery,1,3, rzędy,proxy_self,słowo,ui, checker_0);
            },
            5=>{
                proces_słowa!(litery,1,0, rzędy,proxy_self,słowo,ui, checker_0);
                proces_słowa!(litery,1,1, rzędy,proxy_self,słowo,ui, checker_0);
                proces_słowa!(litery,1,2, rzędy,proxy_self,słowo,ui, checker_0);
                proces_słowa!(litery,1,3, rzędy,proxy_self,słowo,ui, checker_0);
                proces_słowa!(litery,1,4, rzędy,proxy_self,słowo,ui, checker_0);
            },
            6=>{
                proces_słowa!(litery,1,0, rzędy,proxy_self,słowo,ui, checker_0);
                proces_słowa!(litery,1,1, rzędy,proxy_self,słowo,ui, checker_0);
                proces_słowa!(litery,1,2, rzędy,proxy_self,słowo,ui, checker_0);
                proces_słowa!(litery,1,3, rzędy,proxy_self,słowo,ui, checker_0);
                proces_słowa!(litery,1,4, rzędy,proxy_self,słowo,ui, checker_0);
                proces_słowa!(litery,1,5, rzędy,proxy_self,słowo,ui, checker_0);
            },
            _=>{}
        }

        
        if checker_0 {proxy_self.wygrałeś =  match ilość{
            1=> { proxy_self.rzędy[1].0 == 2 },
            2=> { proxy_self.rzędy[1].0 == 2 && proxy_self.rzędy[1].1 == 2 },
            3=> { proxy_self.rzędy[1].0 == 2 && proxy_self.rzędy[1].1 == 2 && proxy_self.rzędy[1].2 == 2 },
            4=> { proxy_self.rzędy[1].0 == 2 && proxy_self.rzędy[1].1 == 2 && proxy_self.rzędy[1].2 == 2 && proxy_self.rzędy[1].3 == 2 },
            5=> { proxy_self.rzędy[1].0 == 2 && proxy_self.rzędy[1].1 == 2 && proxy_self.rzędy[1].2 == 2 && proxy_self.rzędy[1].3 == 2 && proxy_self.rzędy[1].4 == 2 },
            6=> { proxy_self.rzędy[1].0 == 2 && proxy_self.rzędy[1].1 == 2 && proxy_self.rzędy[1].2 == 2 && proxy_self.rzędy[1].3 == 2 && proxy_self.rzędy[1].4 == 2 && proxy_self.rzędy[1].5 == 2 },
            _ => {false}
            }; 
        };
    });

    ui.add_space(proxy_self.rozmiar_spacji_litery * 2.);

    // trzeci rząd

    let checker_1 =match ilość{
        1=> { proxy_self.rzędy[1].0 >= 1 && !proxy_self.wygrałeś},
        2=> { proxy_self.rzędy[1].0 >=1 && proxy_self.rzędy[1].1 >=1 && !proxy_self.wygrałeś},
        3=> { proxy_self.rzędy[1].0 >=1 && proxy_self.rzędy[1].1 >=1 && proxy_self.rzędy[1].2 >=1 && !proxy_self.wygrałeś},
        4=> { proxy_self.rzędy[1].0 >=1 && proxy_self.rzędy[1].1 >=1 && proxy_self.rzędy[1].2 >=1 && proxy_self.rzędy[1].3 >=1 && !proxy_self.wygrałeś},
        5=> { proxy_self.rzędy[1].0 >=1 && proxy_self.rzędy[1].1 >=1 && proxy_self.rzędy[1].2 >=1 && proxy_self.rzędy[1].3 >=1 && proxy_self.rzędy[1].4 >=1 && !proxy_self.wygrałeś},
        6=> { proxy_self.rzędy[1].0 >=1 && proxy_self.rzędy[1].1 >=1 && proxy_self.rzędy[1].2 >=1 && proxy_self.rzędy[1].3 >=1 && proxy_self.rzędy[1].4 >=1 && proxy_self.rzędy[1].5 >=1 && !proxy_self.wygrałeś},
        _ => {false}
    };
    
    if checker_1{ui.add_space(proxy_self.rozmiar_spacji_litery);}

    ui.horizontal(|ui|{
        // println!("{} {} {}",proxy_self.rząd_1_0,proxy_self.rząd_1_1, proxy_self.rząd_1_2);

        match ilość{
            1=>{
                proces_słowa!(litery,2,0, rzędy,proxy_self,słowo,ui, checker_1);
            },
            2=>{
                proces_słowa!(litery,2,0, rzędy,proxy_self,słowo,ui, checker_1);
                proces_słowa!(litery,2,1, rzędy,proxy_self,słowo,ui, checker_1);
            },
            3=>{
                proces_słowa!(litery,2,0, rzędy,proxy_self,słowo,ui, checker_1);
                proces_słowa!(litery,2,1, rzędy,proxy_self,słowo,ui, checker_1);
                proces_słowa!(litery,2,2, rzędy,proxy_self,słowo,ui, checker_1);
            }
            4=>{
                proces_słowa!(litery,2,0, rzędy,proxy_self,słowo,ui, checker_1);
                proces_słowa!(litery,2,1, rzędy,proxy_self,słowo,ui, checker_1);
                proces_słowa!(litery,2,2, rzędy,proxy_self,słowo,ui, checker_1);
                proces_słowa!(litery,2,3, rzędy,proxy_self,słowo,ui, checker_1);
            },
            5=>{
                proces_słowa!(litery,2,0, rzędy,proxy_self,słowo,ui, checker_1);
                proces_słowa!(litery,2,1, rzędy,proxy_self,słowo,ui, checker_1);
                proces_słowa!(litery,2,2, rzędy,proxy_self,słowo,ui, checker_1);
                proces_słowa!(litery,2,3, rzędy,proxy_self,słowo,ui, checker_1);
                proces_słowa!(litery,2,4, rzędy,proxy_self,słowo,ui, checker_1);
            },
            6=>{
                proces_słowa!(litery,2,0, rzędy,proxy_self,słowo,ui, checker_1);
                proces_słowa!(litery,2,1, rzędy,proxy_self,słowo,ui, checker_1);
                proces_słowa!(litery,2,2, rzędy,proxy_self,słowo,ui, checker_1);
                proces_słowa!(litery,2,3, rzędy,proxy_self,słowo,ui, checker_1);
                proces_słowa!(litery,2,4, rzędy,proxy_self,słowo,ui, checker_1);
                proces_słowa!(litery,2,5, rzędy,proxy_self,słowo,ui, checker_1);
            },
            _=>{}
        }
        
        
        if checker_1 {
            proxy_self.wygrałeś = match ilość{
                1=> { proxy_self.rzędy[2].0 == 2 },
                2=> { proxy_self.rzędy[2].0 == 2 && proxy_self.rzędy[2].1 == 2 },
                3=> { proxy_self.rzędy[2].0 == 2 && proxy_self.rzędy[2].1 == 2 && proxy_self.rzędy[2].2 == 2 },
                4=> { proxy_self.rzędy[2].0 == 2 && proxy_self.rzędy[2].1 == 2 && proxy_self.rzędy[2].2 == 2 && proxy_self.rzędy[2].3 == 2 },
                5=> { proxy_self.rzędy[2].0 == 2 && proxy_self.rzędy[2].1 == 2 && proxy_self.rzędy[2].2 == 2 && proxy_self.rzędy[2].3 == 2 && proxy_self.rzędy[2].4 == 2 },
                6=> { proxy_self.rzędy[2].0 == 2 && proxy_self.rzędy[2].1 == 2 && proxy_self.rzędy[2].2 == 2 && proxy_self.rzędy[2].3 == 2 && proxy_self.rzędy[2].4 == 2 && proxy_self.rzędy[2].5 == 2 },
                _ => {false}
            };
        };
    });

    ui.add_space(proxy_self.rozmiar_spacji_litery * 2.);

    // czwarty rząd

    let checker_2 =match ilość{
        1=> { proxy_self.rzędy[2].0 >= 1 && !proxy_self.wygrałeś},
        2=> { proxy_self.rzędy[2].0 >=1 && proxy_self.rzędy[2].1 >=1 && !proxy_self.wygrałeś},
        3=> { proxy_self.rzędy[2].0 >=1 && proxy_self.rzędy[2].1 >=1 && proxy_self.rzędy[2].2 >=1 && !proxy_self.wygrałeś},
        4=> { proxy_self.rzędy[2].0 >=1 && proxy_self.rzędy[2].1 >=1 && proxy_self.rzędy[2].2 >=1 && proxy_self.rzędy[2].3 >=1 && !proxy_self.wygrałeś},
        5=> { proxy_self.rzędy[2].0 >=1 && proxy_self.rzędy[2].1 >=1 && proxy_self.rzędy[2].2 >=1 && proxy_self.rzędy[2].3 >=1 && proxy_self.rzędy[2].4 >=1 && !proxy_self.wygrałeś},
        6=> { proxy_self.rzędy[2].0 >=1 && proxy_self.rzędy[2].1 >=1 && proxy_self.rzędy[2].2 >=1 && proxy_self.rzędy[2].3 >=1 && proxy_self.rzędy[2].4 >=1 && proxy_self.rzędy[2].5 >=1 && !proxy_self.wygrałeś},
        _ => {false}
    };
    if checker_2{ui.add_space(proxy_self.rozmiar_spacji_litery);}

    ui.horizontal(|ui|{
        // println!("{} {} {}",proxy_self.rząd_2_0,proxy_self.rząd_2_1, proxy_self.rząd_2_2);

        match ilość{
            1=>{
                proces_słowa!(litery,3,0, rzędy,proxy_self,słowo,ui, checker_2);
            },
            2=>{
                proces_słowa!(litery,3,0, rzędy,proxy_self,słowo,ui, checker_2);
                proces_słowa!(litery,3,1, rzędy,proxy_self,słowo,ui, checker_2);
            },
            3=>{
                proces_słowa!(litery,3,0, rzędy,proxy_self,słowo,ui, checker_2);
                proces_słowa!(litery,3,1, rzędy,proxy_self,słowo,ui, checker_2);
                proces_słowa!(litery,3,2, rzędy,proxy_self,słowo,ui, checker_2);
            }
            4=>{
                proces_słowa!(litery,3,0, rzędy,proxy_self,słowo,ui, checker_2);
                proces_słowa!(litery,3,1, rzędy,proxy_self,słowo,ui, checker_2);
                proces_słowa!(litery,3,2, rzędy,proxy_self,słowo,ui, checker_2);
                proces_słowa!(litery,3,3, rzędy,proxy_self,słowo,ui, checker_2);
            },
            5=>{
                proces_słowa!(litery,3,0, rzędy,proxy_self,słowo,ui, checker_2);
                proces_słowa!(litery,3,1, rzędy,proxy_self,słowo,ui, checker_2);
                proces_słowa!(litery,3,2, rzędy,proxy_self,słowo,ui, checker_2);
                proces_słowa!(litery,3,3, rzędy,proxy_self,słowo,ui, checker_2);
                proces_słowa!(litery,3,4, rzędy,proxy_self,słowo,ui, checker_2);
            },
            6=>{
                proces_słowa!(litery,3,0, rzędy,proxy_self,słowo,ui, checker_2);
                proces_słowa!(litery,3,1, rzędy,proxy_self,słowo,ui, checker_2);
                proces_słowa!(litery,3,2, rzędy,proxy_self,słowo,ui, checker_2);
                proces_słowa!(litery,3,3, rzędy,proxy_self,słowo,ui, checker_2);
                proces_słowa!(litery,3,4, rzędy,proxy_self,słowo,ui, checker_2);
                proces_słowa!(litery,3,5, rzędy,proxy_self,słowo,ui, checker_2);
            },
            _=>{}
        }

    });
    if checker_2{proxy_self.wygrałeś = match ilość{
        1=> { proxy_self.rzędy[3].0 == 2 },
        2=> { proxy_self.rzędy[3].0 == 2 && proxy_self.rzędy[3].1 == 2 },
        3=> { proxy_self.rzędy[3].0 == 2 && proxy_self.rzędy[3].1 == 2 && proxy_self.rzędy[3].2 == 2 },
        4=> { proxy_self.rzędy[3].0 == 2 && proxy_self.rzędy[3].1 == 2 && proxy_self.rzędy[3].2 == 2 && proxy_self.rzędy[3].3 == 2 },
        5=> { proxy_self.rzędy[3].0 == 2 && proxy_self.rzędy[3].1 == 2 && proxy_self.rzędy[3].2 == 2 && proxy_self.rzędy[3].3 == 2 && proxy_self.rzędy[3].4 == 2 },
        6=> { proxy_self.rzędy[3].0 == 2 && proxy_self.rzędy[3].1 == 2 && proxy_self.rzędy[3].2 == 2 && proxy_self.rzędy[3].3 == 2 && proxy_self.rzędy[3].4 == 2 && proxy_self.rzędy[3].5 == 2 },
        _ => {false}
        };
    };
    ui.add_space(proxy_self.rozmiar_spacji_litery * 2.);

    // czwarty rząd

    let checker_3 =match ilość{
        1=> { proxy_self.rzędy[3].0 >= 1 && !proxy_self.wygrałeś},
        2=> { proxy_self.rzędy[3].0 >=1 && proxy_self.rzędy[3].1 >=1 && !proxy_self.wygrałeś},
        3=> { proxy_self.rzędy[3].0 >=1 && proxy_self.rzędy[3].1 >=1 && proxy_self.rzędy[3].2 >=1 && !proxy_self.wygrałeś},
        4=> { proxy_self.rzędy[3].0 >=1 && proxy_self.rzędy[3].1 >=1 && proxy_self.rzędy[3].2 >=1 && proxy_self.rzędy[3].3 >=1 && !proxy_self.wygrałeś},
        5=> { proxy_self.rzędy[3].0 >=1 && proxy_self.rzędy[3].1 >=1 && proxy_self.rzędy[3].2 >=1 && proxy_self.rzędy[3].3 >=1 && proxy_self.rzędy[3].4 >=1 && !proxy_self.wygrałeś},
        6=> { proxy_self.rzędy[3].0 >=1 && proxy_self.rzędy[3].1 >=1 && proxy_self.rzędy[3].2 >=1 && proxy_self.rzędy[3].3 >=1 && proxy_self.rzędy[3].4 >=1 && proxy_self.rzędy[3].5 >=1 && !proxy_self.wygrałeś},
        _ => {false}
    };
    if checker_3{ui.add_space(proxy_self.rozmiar_spacji_litery);}

    ui.horizontal(|ui|{
        // println!("{} {} {}",proxy_self.rząd_2_0,proxy_self.rząd_2_1, proxy_self.rząd_2_2);

        match ilość{
            1=>{
                proces_słowa!(litery,4,0, rzędy,proxy_self,słowo,ui, checker_3);
            },
            2=>{
                proces_słowa!(litery,4,0, rzędy,proxy_self,słowo,ui, checker_3);
                proces_słowa!(litery,4,1, rzędy,proxy_self,słowo,ui, checker_3);
            },
            3=>{
                proces_słowa!(litery,4,0, rzędy,proxy_self,słowo,ui, checker_3);
                proces_słowa!(litery,4,1, rzędy,proxy_self,słowo,ui, checker_3);
                proces_słowa!(litery,4,2, rzędy,proxy_self,słowo,ui, checker_3);
            }
            4=>{
                proces_słowa!(litery,4,0, rzędy,proxy_self,słowo,ui, checker_3);
                proces_słowa!(litery,4,1, rzędy,proxy_self,słowo,ui, checker_3);
                proces_słowa!(litery,4,2, rzędy,proxy_self,słowo,ui, checker_3);
                proces_słowa!(litery,4,3, rzędy,proxy_self,słowo,ui, checker_3);
            },
            5=>{
                proces_słowa!(litery,4,0, rzędy,proxy_self,słowo,ui, checker_3);
                proces_słowa!(litery,4,1, rzędy,proxy_self,słowo,ui, checker_3);
                proces_słowa!(litery,4,2, rzędy,proxy_self,słowo,ui, checker_3);
                proces_słowa!(litery,4,3, rzędy,proxy_self,słowo,ui, checker_3);
                proces_słowa!(litery,4,4, rzędy,proxy_self,słowo,ui, checker_3);
            },
            6=>{
                proces_słowa!(litery,4,0, rzędy,proxy_self,słowo,ui, checker_3);
                proces_słowa!(litery,4,1, rzędy,proxy_self,słowo,ui, checker_3);
                proces_słowa!(litery,4,2, rzędy,proxy_self,słowo,ui, checker_3);
                proces_słowa!(litery,4,3, rzędy,proxy_self,słowo,ui, checker_3);
                proces_słowa!(litery,4,4, rzędy,proxy_self,słowo,ui, checker_3);
                proces_słowa!(litery,4,5, rzędy,proxy_self,słowo,ui, checker_3);
            },
            _=>{}
        }

    });
    if checker_3{proxy_self.wygrałeś = match ilość{
        1=> { proxy_self.rzędy[4].0 == 2 },
        2=> { proxy_self.rzędy[4].0 == 2 && proxy_self.rzędy[4].1 == 2 },
        3=> { proxy_self.rzędy[4].0 == 2 && proxy_self.rzędy[4].1 == 2 && proxy_self.rzędy[4].2 == 2 },
        4=> { proxy_self.rzędy[4].0 == 2 && proxy_self.rzędy[4].1 == 2 && proxy_self.rzędy[4].2 == 2 && proxy_self.rzędy[4].3 == 2 },
        5=> { proxy_self.rzędy[4].0 == 2 && proxy_self.rzędy[4].1 == 2 && proxy_self.rzędy[4].2 == 2 && proxy_self.rzędy[4].3 == 2 && proxy_self.rzędy[4].4 == 2 },
        6=> { proxy_self.rzędy[4].0 == 2 && proxy_self.rzędy[4].1 == 2 && proxy_self.rzędy[4].2 == 2 && proxy_self.rzędy[4].3 == 2 && proxy_self.rzędy[4].4 == 2 && proxy_self.rzędy[4].5 == 2 },
        _ => {false}
    };
    };
}


