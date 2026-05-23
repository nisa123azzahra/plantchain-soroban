#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Ledger, Env, String};

#[test]
fn test_add_plant() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PlantDiaryContract);
    let client = PlantDiaryContractClient::new(&env, &contract_id);

    env.ledger().set_sequence_number(1);

    let result = client.add_plant(
        &String::from_str(&env, "Mawar"),
        &String::from_str(&env, "Rosa"),
        &String::from_str(&env, "Siram setiap pagi"),
    );

    assert_eq!(result, String::from_str(&env, "Tanaman berhasil ditanam!"));

    let plants = client.get_plants();
    assert_eq!(plants.len(), 1);
    assert_eq!(plants.get(0).unwrap().status, String::from_str(&env, "seedling"));
}

#[test]
fn test_update_plant() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PlantDiaryContract);
    let client = PlantDiaryContractClient::new(&env, &contract_id);

    env.ledger().set_sequence_number(5);

    client.add_plant(
        &String::from_str(&env, "Tomat"),
        &String::from_str(&env, "Solanum lycopersicum"),
        &String::from_str(&env, "Pupuk seminggu sekali"),
    );

    let plants = client.get_plants();
    let id = plants.get(0).unwrap().id;

    let result = client.update_plant(
        &id,
        &String::from_str(&env, "blooming"),
        &String::from_str(&env, "Sudah mulai berbuah!"),
    );

    assert_eq!(result, String::from_str(&env, "Tanaman berhasil diupdate!"));

    let updated = client.get_plants();
    assert_eq!(updated.get(0).unwrap().status, String::from_str(&env, "blooming"));
}

#[test]
fn test_remove_plant() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PlantDiaryContract);
    let client = PlantDiaryContractClient::new(&env, &contract_id);

    env.ledger().set_sequence_number(10);

    client.add_plant(
        &String::from_str(&env, "Kangkung"),
        &String::from_str(&env, "Ipomoea aquatica"),
        &String::from_str(&env, "Mudah tumbuh"),
    );

    let plants = client.get_plants();
    let id = plants.get(0).unwrap().id;

    let result = client.remove_plant(&id);
    assert_eq!(result, String::from_str(&env, "Tanaman berhasil dihapus"));

    let remaining = client.get_plants();
    assert_eq!(remaining.len(), 0);
}

#[test]
fn test_remove_not_found() {
    let env = Env::default();
    let contract_id = env.register_contract(None, PlantDiaryContract);
    let client = PlantDiaryContractClient::new(&env, &contract_id);

    let result = client.remove_plant(&9999u64);
    assert_eq!(result, String::from_str(&env, "Tanaman tidak ditemukan"));
}
