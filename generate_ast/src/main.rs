use std::fs::File;
use std::io::Write;


fn define_visitor(mut file: &File, basename: &str, types: &Vec<&str>) -> std::io::Result<()> {
    file.write(b"")?;
    Ok(())
}


fn define_ast(output_path: &str, basename: &str, types: Vec<&str>) -> std::io::Result<()> {
    let mut file = File::create(format!("{}/{}.rs", output_path, basename.to_lowercase()))?;

    file.write(b"use crate::{\n")?;
    file.write(b"    token::{LiteralType, Token}\n")?;
    file.write(b"};\n")?;
    file.write(b"\n")?;

    let _ = define_visitor(&file, basename, &types);
    
    file.write(format!("pub enum {} {{\n", basename).as_bytes())?;
    
    for t in types {
        let s = t.split(":").collect::<Vec<&str>>();
        let class_name = s[0].trim();
        let fields = s[1].trim().split(", ").collect::<Vec<&str>>();
        
        file.write(format!("    {} {{\n", class_name).as_bytes())?;
        for field in fields {
            let name = field.split(" ").collect::<Vec<&str>>()[0];
            let t = field.split(" ").collect::<Vec<&str>>()[1];
            file.write(format!("        {} : {},\n", name, t).as_bytes())?;
        }
        file.write(b"    },\n")?;

    }
    file.write(b"}")?;
    Ok(())
}

fn main() {
    let output_path = "../src";
    let _ = define_ast(
        output_path,
        "Expr",
        Vec::from([
            "Binary   : left Box<Expr>, operator Token, right Box<Expr>",
            "Grouping : expression Box<Expr>",
            "Literal  : value LiteralType",
            "Unary    : operator Token , right Box<Expr>",
        ]),
    );
    return;
}
