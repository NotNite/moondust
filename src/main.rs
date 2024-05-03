use anyhow::Context;
use full_moon::ast::{Block, Stmt};
use std::{
    env::args,
    io::{stdin, Read},
};

fn clean_block(block: Block) -> Block {
    let mut new_stmts = Vec::new();

    for (stmt, token_ref) in block.stmts_with_semicolon() {
        match stmt {
            Stmt::TypeDeclaration(_) | Stmt::ExportedTypeDeclaration(_) => {
                // do nothing
            }

            Stmt::LocalAssignment(local_assignment) => {
                let local_assignment = local_assignment.clone().with_type_specifiers(vec![]);
                new_stmts.push((Stmt::LocalAssignment(local_assignment), token_ref.clone()));
            }

            Stmt::FunctionDeclaration(function) => {
                let body = function.body().clone();
                let block = body.block().clone();
                let body = body
                    .with_type_specifiers(vec![])
                    .with_block(clean_block(block))
                    .with_return_type(None);
                let function = function.clone().with_body(body);
                new_stmts.push((Stmt::FunctionDeclaration(function), token_ref.clone()));
            }

            Stmt::LocalFunction(local_function) => {
                let body = local_function.body().clone();
                let block = body.block().clone();
                let body = body
                    .with_type_specifiers(vec![])
                    .with_block(clean_block(block))
                    .with_return_type(None);
                let local_function = local_function.clone().with_body(body);
                new_stmts.push((Stmt::LocalFunction(local_function), token_ref.clone()));
            }

            Stmt::Do(r#do) => {
                let block = r#do.block().clone();
                let block = clean_block(block);
                let r#do = r#do.clone().with_block(block);
                new_stmts.push((Stmt::Do(r#do), token_ref.clone()));
            }

            Stmt::If(r#if) => {
                let block = r#if.block().clone();
                let block = clean_block(block);
                let r#if = r#if.clone().with_block(block);
                new_stmts.push((Stmt::If(r#if), token_ref.clone()));
            }

            Stmt::While(r#while) => {
                let block = r#while.block().clone();
                let block = clean_block(block);
                let r#while = r#while.clone().with_block(block);
                new_stmts.push((Stmt::While(r#while), token_ref.clone()));
            }

            Stmt::Repeat(repeat) => {
                let block = repeat.block().clone();
                let block = clean_block(block);
                let repeat = repeat.clone().with_block(block);
                new_stmts.push((Stmt::Repeat(repeat), token_ref.clone()));
            }

            _ => {
                new_stmts.push((stmt.clone(), token_ref.clone()));
            }
        }
    }

    block.clone().with_stmts(new_stmts)
}

fn main() -> anyhow::Result<()> {
    let input = args()
        .nth(1)
        .map(|path| {
            std::fs::read_to_string(path)
                .context("Failed to read from file")
                .unwrap()
        })
        .unwrap_or_else(|| {
            let mut buf = String::new();
            stdin()
                .read_to_string(&mut buf)
                .context("Failed to read from stdin")
                .unwrap();
            buf
        });

    let ast = full_moon::parse(&input)?;
    let nodes = ast.nodes().clone();
    let nodes = clean_block(nodes);
    let ast = ast.with_nodes(nodes);
    let new = full_moon::print(&ast);
    println!("{}", new);

    Ok(())
}
