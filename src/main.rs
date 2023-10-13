use lalrpop_util::lalrpop_mod;
use robot_dsl::lexer::Lexer;
use robot_dsl::interpreter;
lalrpop_mod!(pub grammar);

#[cfg(not(test))]
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let source_code = std::fs::read_to_string("examples/simple.dsl")?;
    let lexer = Lexer::new(&source_code[..]);
    let parser = grammar::ScriptParser::new();
    let ast = parser.parse(lexer)?;
    let interpreter= Interpreter(ast);
    interpreter.execute();
    Ok(())
}
