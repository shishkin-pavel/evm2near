use csv::Writer;
use near_workspaces;
use serde_json::json;

const CALC_PATH: &str = "calc.wasm";
const BENCH_PATH: &str = "bench.wasm";
const CONST_PATH: &str = "const.wasm";
const ECHO_PATH: &str = "echo.wasm";
const COLLATZ_PATH: &str = "Collatz.wasm";

/// returns average (gas_burned, gas_used) at calc contract
async fn bench_calc() -> anyhow::Result<(u64, u64)> {
    let worker = near_workspaces::sandbox().await?;
    let wasm = std::fs::read(CALC_PATH)?;
    let contract = worker.dev_deploy(&wasm).await?;

    let inputs: Vec<(i64, i64)> = vec![
        (6, 7),
        (124, 298),
        (-43222, -23422424),
        (-113444, 1344114),
        (0, 133944141),
        (31333, -144422424),
        (2424422442, 242456969),
        (-42343435435, -2444224),
        (424242424, 0),
        (0, 0),
    ];

    let mut avg_gas_burned = 0 as u64;
    let mut avg_gas_used = 0 as u64;

    for (a, b) in &inputs {
        let outcome = contract
            .call("multiply")
            .args_json(json!({
                "a": *a,
                "b": *b,
            }))
            .transact()
            .await?;

        assert!(outcome.is_success());

        avg_gas_burned += outcome.outcome().gas_burnt;
        avg_gas_used += outcome.total_gas_burnt;
    }
    Ok((
        avg_gas_burned / inputs.len() as u64,
        avg_gas_used / inputs.len() as u64,
    ))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let tera = 1000000000000 as u64;

    let mut wtr = Writer::from_path("benchmark.csv")?;
    wtr.write_record(&[
        "Contract",
        "Average gas burned",
        "Average gas used",
        "Average Tgas burned",
        "Average Tgas used",
        "Input type",
        "Number of iterations",
    ])?;

    let (calc_burned, calc_used) = bench_calc().await?;
    wtr.write_record(&[
        "Calc".to_string(),
        calc_burned.to_string(),
        calc_used.to_string(),
        (calc_burned / tera).to_string(),
        (calc_used / tera).to_string(),
        "Standard".to_string(),
        10.to_string(),
    ])?;

    wtr.flush()?;

    Ok(())
}
