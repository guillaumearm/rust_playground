use garm_playground::libs::island;
use island::{run, AppResult, Error, ErrorList};

#[cfg(test)]
#[test]
fn map_single_char_one_island() -> AppResult<()> {
    let input_map = " # \n";
    let output_map = " 0 \n";

    assert_eq!(run(input_map)?, output_map);

    Ok(())
}

#[test]
fn map_single_char_several_island() -> AppResult<()> {
    let input_map = "   \n# # # # # # # # # # # # # # # # \n";
    let output_map = "   \n0 1 2 3 4 5 6 7 8 9 a b c d e f \n";

    assert_eq!(run(input_map)?, output_map);

    Ok(())
}

#[test]
fn map_empty() -> AppResult<()> {
    let input_map = "   \n   \n   \n";

    assert_eq!(run(input_map)?, input_map);

    Ok(())
}

#[test]
fn map_simple() -> AppResult<()> {
    let input_map = "\
###   #            
 ###  # #          
      ####         
         ##        
 ######   #        
          #        
";
    let output_map = "\
000   1            
 000  1 1          
      1111         
         11        
 222222   1        
          1        
";

    assert_eq!(run(input_map)?, output_map);

    Ok(())
}

#[test]
fn map_complex() -> AppResult<()> {
    let input_map = "\
###########################################     
#                              # #     ## #     
#                              # ##   ##  #     
#    #  # ### #   #   #####    #  ## ##   #     
#    #  # #   #   #   #   #    #   ###    #     
#    #### ### #   #   #   #    #   ###    #     
#    #  # #   #   #   #   #    #  ## ##   #     
#    #  # ### ### ### #####    # ##   ##  #     
#                              ###     ## #     
###########################################     
";
    let output_map = "\
0000000000000000000000000000000000000000000     
0                              0 0     00 0     
0                              0 00   00  0     
0    1  1 222 3   4   55555    0  00 00   0     
0    1  1 2   3   4   5   5    0   000    0     
0    1111 222 3   4   5   5    0   000    0     
0    1  1 2   3   4   5   5    0  00 00   0     
0    1  1 222 333 444 55555    0 00   00  0     
0                              000     00 0     
0000000000000000000000000000000000000000000     
";
    assert_eq!(run(input_map)?, output_map);

    Ok(())
}

mod errors {
    use super::*;

    #[test]
    fn invalid_map_empty() {
        let result = run("");

        assert_eq!(result, Err(ErrorList(vec![Error::EmptyMap])));
    }

    #[test]
    fn invalid_map() {
        let result = run("01[\n\n]23");

        assert_eq!(
            result,
            Err(ErrorList(vec![
                Error::InvalidChar {
                    char: '[',
                    line: 1,
                    col: 3
                },
                Error::EmptyLine(2),
                Error::InvalidChar {
                    char: ']',
                    line: 3,
                    col: 1
                },
            ]))
        );
    }
}
