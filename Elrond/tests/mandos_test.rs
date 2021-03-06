
extern create loterie;
use loterie::*;
use elrond_wasm::*;
use elrond_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../output/crowdfunding.wasm",
        Box::new(|context| Box::new(CrowdfundingImpl::new(context))));
    contract_map
}

#[test]
fn test_crowdfunding1() {
parse_execute_mandos("mandos/crowdfunding-fund-too-late.scen.json", &contract_map());
}

#[test]
fn test_crowdfunding2() {
parse_execute_mandos("mandos/crowdfunding-fund.scen.json", &contract_map());
}

#[test]
fn test_crowdfunding3() {
parse_execute_mandos("mandos/crowdfunding-init.scen.json", &contract_map());
}

