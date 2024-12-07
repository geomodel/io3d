use anyhow::Result;
use io3d::*;

fn main() -> Result<()> {
    let src_file_name = r#"d:\raaDir\ForUserDefined\exchange\ijk.ascii"#;
    let mut property = load_discrete_property(src_file_name, 8)?;
    let property2 = load_continuous_property(src_file_name, 8)?;

    let bw_file_name = r#"d:\raaDir\ForUserDefined\exchange\bw.ascii"#;
    let _bw1 = load_discrete_bw(bw_file_name)?;
    let _bw2 = load_continuous_bw(bw_file_name)?;

    for value in property.iter_mut() {
        if let Some(v) = value {
            *value = Some(*v + 5);
        }
    }

    let dest_file_name = r#"d:\raaDir\ForUserDefined\exchange\result.ascii"#;
    save_continuous_property(dest_file_name, &property2)?;
    save_discrete_property(dest_file_name, &property)?;

    println!("{}", "Ok-k-k!!");
    Ok(())
}
