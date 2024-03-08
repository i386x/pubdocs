use enumset::{EnumSet, EnumSetType};
use clap::{ArgAction, Parser, ValueEnum};

#[derive(EnumSetType, ValueEnum, Debug)]
enum TestSet {
    /// Unit tests
    Unit,
    /// Integration tests
    Integ,
}

#[derive(Parser, Debug)]
struct Args {
    /// Specify a test set to run
    ///
    /// A test set is a one of `unit` (for unit tests) or `integ` (for integration
    /// tests). A comma separated list is accepted. The option can be given
    /// multiple times on the command line (its values are appended to the result
    /// list). No option given means all test sets are run (the default behavior)
    #[arg(long, value_delimiter = ',', action = ArgAction::Append, verbatim_doc_comment)]
    testset: Vec<TestSet>,

    /// Specify a test set to exclude
    ///
    /// A test set is a one of `unit` (for unit tests) or `integ` (for integration
    /// tests). A comma separated list is accepted. The option can be given
    /// multiple times on the command line (its values are appended to the result
    /// list)
    #[arg(long, short = 'x', value_delimiter = ',', action = ArgAction::Append, verbatim_doc_comment)]
    exclude: Vec<TestSet>,
}

#[derive(Debug)]
struct Context {
    testset: EnumSet<TestSet>,
    opts: Vec<String>,
}

impl Context {
    fn new() -> Self {
        Self {
            testset: EnumSet::<TestSet>::new(),
            opts: Vec::<String>::new(),
        }
    }
}

impl TryFrom<&Args> for Context {
    type Error = anyhow::Error;

    fn try_from(args: &Args) -> anyhow::Result<Self> {
        let mut context = Self::new();

        args.testset.iter().for_each(|v| { context.testset.insert(*v); });
        if context.testset.is_empty() {
            context.testset.insert_all(EnumSet::<TestSet>::ALL);
        }
        args.exclude.iter().for_each(|v| { context.testset.remove(*v); });

        context.opts.extend(["--foo".to_string(), "bar".to_string()]);

        Ok(context)
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        println!("Context dropped");
    }
}

fn main() {
    let args = Args::parse();
    let context = Context::try_from(&args);

    println!("{context:#?}");
}
