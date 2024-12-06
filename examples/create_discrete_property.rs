use anyhow::Result;
use io3d::*;

fn main() -> Result<()> {
    let src_file_name = r#"d:\raaDir\ForUserDefined\exchange\ijk.ascii"#;
    let mut property = load_discrete_property(src_file_name, 8)?;

    for value in property.iter_mut() {
        *value = *value + 111;
    }

    let dest_file_name = r#"d:\raaDir\ForUserDefined\exchange\result.ascii"#;
    save_discrete_property(dest_file_name, &property)?;

    println!("{}", "Ok-k-k!!");
    Ok(())
}
