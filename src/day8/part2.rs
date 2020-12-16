use crate::day8::common::{Day8Error, Program};

pub fn exo(program: Result<Program, Day8Error>) -> i16 {
    let mut acc: i16 = 0;
    match program {
        Ok(mut program) =>  {
            program.run_auto_fix();
            acc = program.get_acc();
        },
        _ => {}
    }
    acc
}
