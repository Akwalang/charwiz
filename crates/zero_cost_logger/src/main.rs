use zero_cost_logger::*;

fn main() {
    log!("Starting <$>&Charwiz</> logger test");

    debug!("Debug message: <i+>green italic</> and <i->red italic</>");
    log!("Info message: <+>green</>, <!>yellow</>, <&>cyan</>");
    warn!("Warning: <!>something looks suspicious</>");
    error!("<$>Plugin Transformer</>: Lua scripts could not be loaded");

    new_line!();

    log!("Plain text without markup still works");
}
