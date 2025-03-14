#[cfg(test)]
mod xoxo {
    use crate::ui::slowniki::{Słowa, Wybieranie};
    // use super::*;
    #[test]
    fn sprawdz_czy_3_sie_zgadza(){
        let mut ilość_błędów:u32 = 0;
        for (bbb,_) in Słowa::trzyliterowe().słowo{

            if bbb.len() !=3{
                println!("{}",format_args!("{} ma {} liter",bbb, bbb.len()));
                ilość_błędów += 1;
            }
        }
        assert_eq!(ilość_błędów,0u32);

    }
    #[test]
    fn sprawdz_czy_4_sie_zgadza(){
        let mut ilość_błędów:u32 = 0;
        for (bbb,_) in Słowa::czteroliterowe().słowo{

            if bbb.len() !=4{
                println!("{}",format_args!("{} ma {} liter",bbb, bbb.len()));
                ilość_błędów += 1;
            }
        }
        assert_eq!(ilość_błędów,0u32);

    }
    #[test]
    fn sprawdz_czy_5_sie_zgadza(){
        let mut ilość_błędów:u32 = 0;
        for (bbb,_) in Słowa::pięcioliterowe().słowo{

            if bbb.len() !=5{
                println!("{}",format_args!("{} ma {} liter",bbb, bbb.len()));
                ilość_błędów += 1;
            }
        }
        assert_eq!(ilość_błędów,0u32);

    }


}
