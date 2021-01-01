use nalgebra::{Dynamic, DMatrix, RowVector};
use std::fs;

fn main() {
    let white_file = fs::File::open("./examples/wine_quality/winequality-white.csv").unwrap();
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(white_file);
    let dm = DMatrix::from_rows(
        reader
            .records()
            .into_iter()
            .map(|r| {
                RowVector::<f64, Dynamic, _>::from_iterator(
                    12,
                    r.unwrap()
                        .iter()
                        .map(|s| s.parse().unwrap())
                        .collect::<Vec<f64>>()
                )
            })
            .collect::<Vec<RowVector<f64, Dynamic, _>>>()
            .as_slice(),
    );
    println!("{:?}", dm.shape());
}
