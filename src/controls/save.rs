use std::{fs::File, io::Write, time::SystemTime};

use bevy::{
    prelude::{Res, ResMut},
    time::Time,
};

use crate::{
    config::structs::{GenerationConfig, SaveConfig},
    organism::organism_list::OrganismList,
};

use super::control_state::ControlState;

pub fn save_generation(
    ol: Res<OrganismList>,
    cs: ResMut<ControlState>,
    gc: Res<GenerationConfig>,
    sc: Res<SaveConfig>,
) {
    if cs.save {
        let json = match serde_json::to_string(&ol.builders) {
            Ok(res) => res,
            Err(err) => {
                println!("Error saving generation, {:?}", err);
                return;
            }
        };

        let path = format!(
            "{}\\{}_gen{}",
            sc.save_folder,
            chrono::offset::Local::now()
                .format("%d-%m-%Y_%H-%M")
                .to_string(),
            gc.cur_generation,
        );
        let mut file = match File::create(path) {
            Ok(f) => f,
            Err(err) => {
                println!("Error creating file to save to, {:?}", err);
                return;
            }
        };
        match file.write_all(json.as_bytes()) {
            Ok(_) => {}
            Err(err) => {
                println!("Error writing to file, {:?}", err);
                return;
            }
        }
    }
}
