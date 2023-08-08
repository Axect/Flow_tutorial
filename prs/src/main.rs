use peroxide::fuga::*;

#[allow(non_snake_case)]
fn main() {
    let df = DataFrame::read_nc("../data/emit_total.nc").unwrap();
    df.print();
    let E: Vec<f64> = df["E_alp"].to_vec();
    let dNdE: Vec<f64> = df["dNdE_alp"].to_vec();

    let cs = cubic_hermite_spline(&E, &dNdE, Akima);
    let f = |x: f64| {
        let y = cs.eval(x);
        if y < 0f64 {
            0f64
        } else {
            y
        }
    };

    let E_sample = prs(f, 10_0000, (E[0], 200f64), 100, 1e-6);

    let mut df = DataFrame::new(vec![]);
    df.print();
    df.push("E", Series::new(E_sample));
    df.write_parquet("../data/emit_total.parquet", CompressionOptions::Uncompressed).expect("Can't write parquet file");
}
