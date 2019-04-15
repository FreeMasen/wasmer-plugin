
use wasmer_plugin::*;
use example_runner_lib::Interactive;


#[wasmer_plugin]
pub fn interact(mut interactive: Interactive) -> Interactive {
    interactive.one = interactive.one.wrapping_mul(100);
    interactive.two = interactive.two.wrapping_div(8);
    interactive.three = interactive.three.repeat(2);
    interactive
}