// KREWES = {
//     'orpheus': ['ERIC', 'SAUVE', 'JONATHAN', 'PAK', 'LIAM', 'WINNIE', 'KELLY', 'JEFFREY', 'KARL', 'ISHITA', 'VICTORIA', 'BENJAMIN', 'ARIB', 'AMELIA', 'CONSTANCE', 'IAN'],
//     'rex': ['ANYA', 'DUB-Y', 'JESSICA', 'ALVIN', 'HELENA', 'MICHELLE', 'SHENKER', 'ARI', 'STELLA', 'RENEE', 'MADELYN', 'MAC', 'RYAN', 'DRAGOS'],
//     'endymion': ['JASON', 'DEAN', 'MADDY', 'SAQIF', 'CINDY', 'YI LING', 'RUOSHUI', 'FB', 'MATTHEW', 'MAY', 'ERIN', 'MEIRU']
// }

use rand::seq::SliceRandom;
const KREWES: [&str; 42] = [
    "ERIC",
    "SAUVE",
    "JONATHAN",
    "PAK",
    "LIAM",
    "WINNIE",
    "KELLY",
    "JEFFREY",
    "KARL",
    "ISHITA",
    "VICTORIA",
    "BENJAMIN",
    "ARIB",
    "AMELIA",
    "CONSTANCE",
    "IAN",
    "ANYA",
    "DUB-Y",
    "JESSICA",
    "ALVIN",
    "HELENA",
    "MICHELLE",
    "SHENKER",
    "ARI",
    "STELLA",
    "RENEE",
    "MADELYN",
    "MAC",
    "RYAN",
    "DRAGOS",
    "JASON",
    "DEAN",
    "MADDY",
    "SAQIF",
    "CINDY",
    "YI LING",
    "RUOSHUI",
    "FB",
    "MATTHEW",
    "MAY",
    "ERIN",
    "MEIRU",
];
fn main() {
    println!("{}", KREWES.choose(&mut rand::thread_rng()).unwrap());
}
