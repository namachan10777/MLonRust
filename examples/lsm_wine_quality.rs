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
    let (nrows, _) = dm.shape();

    let dm_train = dm.index((0..nrows/3*2, ..));
    let g_train = dm_train.index((.., ..11)).insert_column(0, 1.0);
    let y_train = dm_train.index((.., 11));

    let dm_test = dm.index((nrows/3*2.., ..));
    let g_test = dm_test.index((.., ..11)).insert_column(0, 1.0);
    let y_test = dm_test.index((.., 11));

    let a = (g_train.transpose() * g_train.clone()).try_inverse().unwrap() * g_train.transpose() * y_train;

    let y_estimated = g_test * a;
    let tss = y_test.lp_norm(2);
    let rss = (y_test - y_estimated).lp_norm(2);
    let r2 = 1.0 - rss/tss;
    println!("{:?}", r2);
}
