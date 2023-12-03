#[allow(unused, dead_code)]
use numerologie::quersumme;

extern crate clap;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "Numerologie Rechner")]
#[command(author = "Bjoern <student1304@vodafonemail.de>")]
#[command(version = "1.0")]
#[command(about = "Berechnet Numerologie fuer Klienten", 
    long_about = 
    "Berechnet Numerologie basierend auf Geburtstag.\n----------------------------------------------\n
    Eins - Vier:\tStaerken vier Lebensabschnitte
    Fuenf - Acht:\tHerausforderungen zum jeweiligen Abschnitt
    Mitte:\t\tEntwicklungspotential")]
/// Birthday in DD MM JJJJ Format
struct Cli {
    /// Day of Birthday as DD
    day: i32,
    /// Month of Birthday as MM
    month: i32,
    /// Year of Birthday as JJJJ
    year: i32
}

#[derive(Debug)]
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

fn calc_num(day: i32, month: i32, year: i32) -> Numerologie {
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

    // println!("{:?}", &numerologie);
    numerologie
}

fn main() {
    let cli = Cli::parse();

    let day: i32 = cli.day;
    let month: i32 = cli.month;
    let year: i32 = cli.year;
    println!("Rechne mit:\n{:?}", cli);
    let numerologie = calc_num(day, month, year);
    println!("\nBerechnungsergebnis: \n{:?}", &numerologie);

}




#[test]
fn verify_cli(){
    use clap::CommandFactory;
    Cli::command().debug_assert();
}