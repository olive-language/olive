mod lexer;
mod parser;
mod semantic;
mod mir;
mod borrow_check;
mod codegen;
mod span;

use lexer::Lexer;
use parser::Parser;
use semantic::{Resolver, TypeChecker};
use mir::MirBuilder;
use borrow_check::BorrowChecker;
use codegen::cranelift::CraneliftCodegen;
use std::{fs, process, path::Path, collections::{HashSet, HashMap}};
use clap::Parser as ClapParser;
use ariadne::{Report, ReportKind, Label, Source};

#[derive(ClapParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // Input file (.liv)
    file: String,

    // Just check the code, don't run it
    #[arg(short, long)]
    check: bool,

    // Print the AST for debugging
    #[arg(long)]
    emit_ast: bool,

    // Print the MIR blocks
    #[arg(long)]
    emit_mir: bool,
}

// Pretty-print semantic errors using Ariadne.
fn report_error(sources: &HashMap<usize, (String, String)>, msg: &str, span: span::Span) {
    let (filename, source) = sources.get(&span.file_id).expect("file not found in sources");
    let _ = Report::build(ReportKind::Error, (filename.as_str(), span.start..span.end))
        .with_message(msg)
        .with_label(Label::new((filename.as_str(), span.start..span.end)).with_message(msg))
        .finish()
        .print((filename.as_str(), Source::from(source)));
}

// Walk the imports and parse everything into a flat list of statements.
fn load_and_parse(filename: &str, loaded: &mut HashSet<String>, file_id_counter: &mut usize, sources: &mut HashMap<usize, (String, String)>) -> Vec<parser::Stmt> {
    let current_file_id = *file_id_counter;
    *file_id_counter += 1;

    let source = fs::read_to_string(filename).unwrap_or_else(|e| {
        eprintln!("error reading {}: {e}", filename);
        process::exit(1);
    });
    
    sources.insert(current_file_id, (filename.to_string(), source.clone()));

    let tokens = match Lexer::new(&source, current_file_id).tokenise() {
        Ok(t) => t,
        Err(e) => {
            report_error(sources, &e.message, span::Span { file_id: current_file_id, line: e.line, col: e.col, start: e.start, end: e.end });
            process::exit(1);
        }
    };

    let program = match Parser::new(tokens).parse_program() {
        Ok(p) => p,
        Err(e) => {
            report_error(sources, &e.message, span::Span { file_id: current_file_id, line: e.line, col: e.col, start: e.start, end: e.end });
            process::exit(1);
        }
    };

    let mut all_stmts = Vec::new();
    let parent_dir = Path::new(filename).parent().unwrap_or(Path::new("."));

    for stmt in program.stmts {
        match &stmt.kind {
            parser::StmtKind::Import(parts) => {
                let mod_name = parts.join("/");
                let mod_path = parent_dir.join(format!("{}.liv", mod_name));
                let path_str = mod_path.to_string_lossy().to_string();
                
                if !loaded.contains(&path_str) {
                    loaded.insert(path_str.clone());
                    let mut imported_stmts = load_and_parse(&path_str, loaded, file_id_counter, sources);
                    imported_stmts.retain(|s| matches!(s.kind, parser::StmtKind::Fn { .. } | parser::StmtKind::Class { .. }));
                    all_stmts.extend(imported_stmts);
                }
            }
            _ => all_stmts.push(stmt),
        }
    }

    all_stmts
}

fn main() {
    let cli = Cli::parse();
    let filename = &cli.file;

    let mut loaded = HashSet::new();
    loaded.insert(filename.to_string());
    let mut file_id_counter = 0;
    let mut sources = HashMap::new();
    let combined_stmts = load_and_parse(filename, &mut loaded, &mut file_id_counter, &mut sources);
    let program = parser::Program { stmts: combined_stmts };

    if cli.emit_ast {
        println!("{:#?}", program);
    }

    let mut resolver = Resolver::new();
    resolver.resolve_program(&program);

    if !resolver.errors.is_empty() {
        for e in &resolver.errors {
            report_error(&sources, &format!("{}", e), e.span());
        }
        process::exit(1);
    }

    let mut type_checker = TypeChecker::new();
    type_checker.check_program(&program);

    if !type_checker.errors.is_empty() {
        for e in &type_checker.errors {
            report_error(&sources, &format!("{}", e), e.span());
        }
        process::exit(1);
    }

    let mut mir_builder = MirBuilder::new(&type_checker.expr_types);
    mir_builder.build_program(&program);

    if cli.emit_mir {
        println!("{:#?}", mir_builder.functions);
    }

    // Ownership/borrow checking.
    for func in &mir_builder.functions {
        let mut checker = BorrowChecker::new(func);
        checker.check();
        if !checker.errors.is_empty() {
            for e in &checker.errors {
                 match e {
                     semantic::SemanticError::Custom { msg, span } => {
                          report_error(&sources, &format!("borrow error in {}: {}", func.name, msg), *span);
                     }
                     _ => report_error(&sources, &format!("borrow error in {}: {}", func.name, e), e.span()),
                 }
            }
            process::exit(1);
        }
    }

    if cli.check {
        println!("Check finished successfully.");
        return;
    }

    let mut codegen = CraneliftCodegen::new(&mir_builder.functions);
    codegen.generate();
    codegen.finalize();

    // Execute the entry point.
    if let Some(main_ptr) = codegen.get_function("__main__") {
        let main_fn: extern "C" fn() -> i64 = unsafe { std::mem::transmute(main_ptr) };
        let _result = main_fn();
    } else {
        println!("No `main` function found to execute.");
    }
}
