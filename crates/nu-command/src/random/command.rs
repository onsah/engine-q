use nu_engine::get_full_help;
use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    Category, IntoPipelineData, PipelineData, Signature, Value,
};

#[derive(Clone)]
pub struct RandomCommand;

impl Command for RandomCommand {
    fn name(&self) -> &str {
        "random"
    }

    fn signature(&self) -> Signature {
        Signature::build("random").category(Category::Random)
    }

    fn usage(&self) -> &str {
        "Generate a random values."
    }

    fn run(
        &self,
        engine_state: &EngineState,
        _stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<nu_protocol::PipelineData, nu_protocol::ShellError> {
        Ok(Value::String {
            val: get_full_help(
                &RandomCommand.signature(),
                &RandomCommand.examples(),
                engine_state,
            ),
            span: call.head,
        }
        .into_pipeline_data())
    }
}
