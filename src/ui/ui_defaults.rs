pub struct ZgadnijSlowo{
    pub wybór_słownictwa:&'static str,
    pub podpowiedź:&'static str,
    pub tryb_gry:u8,
    pub podpowiedź_toggle:bool,
    pub debug:bool,
    pub wygrałeś:bool,

    pub rząd_0_0:u8,
    pub rząd_0_1:u8,
    pub rząd_0_2:u8,
    pub rząd_0_3:u8,
    pub rząd_0_4:u8,
    pub rząd_0_5:u8,

    pub rząd_1_0:u8,
    pub rząd_1_1:u8,
    pub rząd_1_2:u8,
    pub rząd_1_3:u8,
    pub rząd_1_4:u8,
    pub rząd_1_5:u8,

    pub rząd_2_0:u8,
    pub rząd_2_1:u8,
    pub rząd_2_2:u8,
    pub rząd_2_3:u8,
    pub rząd_2_4:u8,
    pub rząd_2_5:u8,

    pub rząd_3_0:u8,
    pub rząd_3_1:u8,
    pub rząd_3_2:u8,
    pub rząd_3_3:u8,
    pub rząd_3_4:u8,
    pub rząd_3_5:u8,

    pub rząd_4_0:u8,
    pub rząd_4_1:u8,
    pub rząd_4_2:u8,
    pub rząd_4_3:u8,
    pub rząd_4_4:u8,
    pub rząd_4_5:u8,

    pub rząd_5_0:u8,
    pub rząd_5_1:u8,
    pub rząd_5_2:u8,
    pub rząd_5_3:u8,
    pub rząd_5_4:u8,
    pub rząd_5_5:u8,

    pub litera_0_0:String,
    pub litera_0_1:String,
    pub litera_0_2:String,
    pub litera_0_3:String,
    pub litera_0_4:String,
    pub litera_0_5:String,

    pub litera_1_0:String,
    pub litera_1_1:String,
    pub litera_1_2:String,
    pub litera_1_3:String,
    pub litera_1_4:String,
    pub litera_1_5:String,

    pub litera_2_0:String,
    pub litera_2_1:String,
    pub litera_2_2:String,
    pub litera_2_3:String,
    pub litera_2_4:String,
    pub litera_2_5:String,

    pub litera_3_0:String,
    pub litera_3_1:String,
    pub litera_3_2:String,
    pub litera_3_3:String,
    pub litera_3_4:String,
    pub litera_3_5:String,

    pub litera_4_0:String,
    pub litera_4_1:String,
    pub litera_4_2:String,
    pub litera_4_3:String,
    pub litera_4_4:String,
    pub litera_4_5:String,

    pub litera_5_0:String,
    pub litera_5_1:String,
    pub litera_5_2:String,
    pub litera_5_3:String,
    pub litera_5_4:String,
    pub litera_5_5:String,

    pub rozmiar_litery:f32,
    pub rozmiar_pola_tekstowego_x:f32,
    pub rozmiar_pola_tekstowego_y:f32,
    pub rozmiar_spacji_litery:f32,
}

impl Default for ZgadnijSlowo {

    fn default() -> Self {
        ZgadnijSlowo {
            wybór_słownictwa:"",
            podpowiedź:"",
            podpowiedź_toggle:false,
            wygrałeś:false,
            debug:false,
            tryb_gry: 0,
            rząd_0_0:0,
            rząd_0_1:0,
            rząd_0_2:0,
            rząd_0_3:0,
            rząd_0_4:0,
            rząd_0_5:0,

            rząd_1_0:0,
            rząd_1_1:0,
            rząd_1_2:0,
            rząd_1_3:0,
            rząd_1_4:0,
            rząd_1_5:0,

            rząd_2_0:0,
            rząd_2_1:0,
            rząd_2_2:0,
            rząd_2_3:0,
            rząd_2_4:0,
            rząd_2_5:0,

            rząd_3_0:0,
            rząd_3_1:0,
            rząd_3_2:0,
            rząd_3_3:0,
            rząd_3_4:0,
            rząd_3_5:0,

            rząd_4_0:0,
            rząd_4_1:0,
            rząd_4_2:0,
            rząd_4_3:0,
            rząd_4_4:0,
            rząd_4_5:0,

            rząd_5_0:0,
            rząd_5_1:0,
            rząd_5_2:0,
            rząd_5_3:0,
            rząd_5_4:0,
            rząd_5_5:0,

            litera_0_0:String::new(),
            litera_0_1:String::new(),
            litera_0_2:String::new(),
            litera_0_3:String::new(),
            litera_0_4:String::new(),
            litera_0_5:String::new(),
            
            litera_1_0:String::new(),
            litera_1_1:String::new(),
            litera_1_2:String::new(),
            litera_1_3:String::new(),
            litera_1_4:String::new(),
            litera_1_5:String::new(),

            litera_2_0:String::new(),
            litera_2_1:String::new(),
            litera_2_2:String::new(),
            litera_2_3:String::new(),
            litera_2_4:String::new(),
            litera_2_5:String::new(),

            litera_3_0:String::new(),
            litera_3_1:String::new(),
            litera_3_2:String::new(),
            litera_3_3:String::new(),
            litera_3_4:String::new(),
            litera_3_5:String::new(),

            litera_4_0:String::new(),
            litera_4_1:String::new(),
            litera_4_2:String::new(),
            litera_4_3:String::new(),
            litera_4_4:String::new(),
            litera_4_5:String::new(),

            litera_5_0:String::new(),
            litera_5_1:String::new(),
            litera_5_2:String::new(),
            litera_5_3:String::new(),
            litera_5_4:String::new(),
            litera_5_5:String::new(),

            rozmiar_litery:30.,
            rozmiar_pola_tekstowego_x:20.,
            rozmiar_pola_tekstowego_y:40.,
            rozmiar_spacji_litery:5.,
        }
    }
}
impl ZgadnijSlowo {

    pub fn name() -> &'static str {
        concat!("Zgadnij słowo v", env!("CARGO_PKG_VERSION"))
    }

}