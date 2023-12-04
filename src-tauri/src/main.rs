// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::i32;
use serde::{Serialize, Deserialize};

#[tauri::command]
fn calculate(day:i32, month:i32, year:i32) -> Numerologie {
  calc_num(day, month, year)
}

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      match app.get_cli_matches() {
        Ok(matches) => {
          println!("{:?}", matches);
        }
      Err(_) => {}
      }
    Ok(())
    })
    .invoke_handler(tauri::generate_handler![calculate])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


#[derive(Serialize, Deserialize, Debug)]
struct Numerologie {
    eins: i32,
    zwei: i32,
    drei: i32,
    vier: i32,
    fuenf: i32,
    sechs: i32,
    sieben: i32,
    acht: i32,
    mitte: i32
}


fn quersumme(mut num: i32) -> i32 {
  while num > 9 {
      let mut sum = 0;
      while num > 0 {
          sum += num % 10;
          num /= 10;
      }
      num = sum;
  }
  num
}

fn calc_num(day: i32, month: i32, year: i32) -> Numerologie {
// fn calc_num(day: i32, month: i32, year: i32) -> serde_json {
    // Hilfsrechnungen
    let qs_d = quersumme(day);
    let qs_m = quersumme(month);
    let qs_j = quersumme(year);
    let h_1: i32 = qs_d - qs_m;
    let h_2: i32 = qs_m - qs_j;
    let h_3: i32 = qs_d - qs_j;

    // Numerologie ausrechnen
    let mut numerologie = Numerologie {
        eins: quersumme(qs_d + qs_m),
        zwei: quersumme(qs_m + qs_j),
        drei: 0,
        vier: quersumme(qs_d + qs_j),
        fuenf: quersumme(h_1.abs()),
        sechs: quersumme(h_2.abs()),
        sieben: 0,
        acht: quersumme(h_3.abs()),
        mitte: quersumme(qs_d + qs_m + qs_j),
    };
    numerologie.drei = quersumme(numerologie.eins + numerologie.zwei);
    numerologie.sieben = quersumme((numerologie.fuenf - numerologie.sechs).abs());

    println!("{:?}", &numerologie);
    numerologie
    // numerologie.into()
}