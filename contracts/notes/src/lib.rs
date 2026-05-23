#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data tanaman
#[contracttype]
#[derive(Clone, Debug)]
pub struct Plant {
    id: u64,
    name: String,        // nama tanaman
    species: String,     // spesies/jenis
    status: String,      // "seedling" | "growing" | "blooming" | "harvested"
    notes: String,       // catatan perawatan
}

// Storage key
const PLANT_DATA: Symbol = symbol_short!("PLANTDATA");

#[contract]
pub struct PlantDiaryContract;

#[contractimpl]
impl PlantDiaryContract {

    // READ — ambil semua tanaman
    pub fn get_plants(env: Env) -> Vec<Plant> {
        env.storage().instance().extend_ttl(100, 100);
        env.storage()
            .instance()
            .get(&PLANT_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // CREATE — tanam tanaman baru
    pub fn add_plant(env: Env, name: String, species: String, notes: String) -> String {
        let mut plants: Vec<Plant> = env
            .storage()
            .instance()
            .get(&PLANT_DATA)
            .unwrap_or(Vec::new(&env));

        let plant = Plant {
            id: env.ledger().sequence() as u64,
            name,
            species,
            status: String::from_str(&env, "seedling"),
            notes,
        };

        plants.push_back(plant);

        env.storage().instance().set(&PLANT_DATA, &plants);
        env.storage().instance().extend_ttl(100, 100);

        String::from_str(&env, "Tanaman berhasil ditanam!")
    }

    // UPDATE — update status & catatan tanaman
    pub fn update_plant(env: Env, id: u64, status: String, notes: String) -> String {
        let mut plants: Vec<Plant> = env
            .storage()
            .instance()
            .get(&PLANT_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..plants.len() {
            let plant = plants.get(i).unwrap();
            if plant.id == id {
                let updated = Plant {
                    id: plant.id,
                    name: plant.name,
                    species: plant.species,
                    status,
                    notes,
                };
                plants.set(i, updated);

                env.storage().instance().set(&PLANT_DATA, &plants);
                env.storage().instance().extend_ttl(100, 100);

                return String::from_str(&env, "Tanaman berhasil diupdate!");
            }
        }

        String::from_str(&env, "Tanaman tidak ditemukan")
    }

    // DELETE — hapus tanaman
    pub fn remove_plant(env: Env, id: u64) -> String {
        let mut plants: Vec<Plant> = env
            .storage()
            .instance()
            .get(&PLANT_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..plants.len() {
            if plants.get(i).unwrap().id == id {
                plants.remove(i);

                env.storage().instance().set(&PLANT_DATA, &plants);
                env.storage().instance().extend_ttl(100, 100);

                return String::from_str(&env, "Tanaman berhasil dihapus");
            }
        }

        String::from_str(&env, "Tanaman tidak ditemukan")
    }
}

mod test;