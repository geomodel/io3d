use anyhow::Result;
use io3d::*;

#[test]
fn index_calculator() -> Result<()> {
    let ijk_max = IJK{ i:3,j:7,k:11 };
    let mut checker = 0;
    for k in 0..ijk_max.k {
        for j in 0..ijk_max.j {
            for i in 0..ijk_max.i {
                let arr_ind = ijk_to_array(&IJK{i,j,k},&ijk_max);
                assert!(arr_ind.unwrap() == checker);
                checker += 1;
            }
        }
    }

    Ok(())
}

#[test]
fn printer() -> Result<()> {
    let src_file_name = r#"assets/ijk.ascii"#;
    let i_property = load_discrete_property(src_file_name, 8)?;

    let ijk_max = IJK{ i:2,j:2,k:2 };

    for k in 0..ijk_max.k {
        for j in 0..ijk_max.j {
            for i in 0..ijk_max.i {
                let arr_ind = ijk_to_array(&IJK{i,j,k},&ijk_max);
                let value = i_property[arr_ind.unwrap()];
                println!("ijk = {}.{}.{} -> {:?} -> {:?}", i, j, k, arr_ind, value);
            }
        }
    }

    Ok(())
}


//  //  //  //  //  //  //  //
fn ijk_to_array(ijk: &IJK, ijk_max: &IJK) -> Option<usize> {
    let i = ijk.i;
    let j = ijk.j;
    let k = ijk.k;
    let result = i + j*ijk_max.i + k*ijk_max.i*ijk_max.j;
    Some( result.into() )
}

