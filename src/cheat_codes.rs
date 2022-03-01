use rand::distributions::{Alphanumeric, DistString};
use rand::prelude::SliceRandom;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum CheatCodeKind {
    // Mandatory
    Jump,

    // Common
    Crouch,
    Attack,
    AttackDmgBoost,
    AttackFireRateBoost,
    MoveLeft,
    SpeedBoost1,
    SpeedBoost2,
    SpeedBoost3,
    Armor,
    Dash,

    // Rare
    DoubleJump,
    SpeedBoost4,
    SpeedBoost5,
    Shield,

    // Legendary
    ExtraLife,
    TempInvicibility,
    Fly,
}

// here the value is the weight for the weighted distribution
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CheatCodeRarity {
    Mandatory = 0, // weight of zero because it is not present in the distribution
    Common = 10,
    Rare = 5,
    Legendary = 2,
}

#[derive(Debug)]
pub enum CheatCodeActivationResult {
    NotFound,
    Activated(CheatCodeKind),
    AlreadyActivated(CheatCodeKind),
}
impl CheatCodeActivationResult {
    pub fn repr(&self) -> String {
        match self {
            CheatCodeActivationResult::Activated(kind) => {
                return format!("[{:?}] cheat code successfully activated", kind)
            }
            CheatCodeActivationResult::AlreadyActivated(kind) => {
                return format!("[{:?}] already activated", kind)
            }
            CheatCodeActivationResult::NotFound => {
                "cheat code not recognized by the system".to_string()
            }
        }
    }
}

#[derive(Debug)]
pub struct CheatCode {
    pub kind: CheatCodeKind,
    pub rarity: CheatCodeRarity,
    pub text: String,
    pub dependencies: Vec<CheatCodeKind>,
    pub image: String,
}

impl CheatCode {
    pub fn new(
        kind: CheatCodeKind,
        rarity: CheatCodeRarity,
        text: &str,
        dependencies: Vec<CheatCodeKind>,
        image: String,
    ) -> Self {
        Self {
            kind,
            rarity,
            text: text.to_string(),
            dependencies,
            image,
        }
    }
}

pub struct CheatCodeResource {
    pub codes: HashMap<CheatCodeKind, CheatCode>,
    activated: Vec<CheatCodeKind>,
}

impl CheatCodeResource {
    pub fn get_next_code(&self) -> CheatCodeKind {
        // first get a list of mandatory cheat codes (JUMP)
        let mandatories = self
            .codes
            .iter()
            .filter(|(kind, code)| {
                code.rarity == CheatCodeRarity::Mandatory
                    && !self.is_code_activated(&(*kind).clone())
            })
            .map(|(kind, _)| *kind)
            .collect::<Vec<CheatCodeKind>>();
        // if there is a mandatory code to be chosen, then return it
        if !mandatories.is_empty() {
            return *mandatories.choose(&mut rand::thread_rng()).unwrap();
        }

        // then we grab all the codes that haven't been activated yet
        // don't forget to check for dependencies!
        let available_codes = self
            .codes
            .iter()
            .filter(|(kind, code)| {
                let missing_deps = code
                    .dependencies
                    .iter()
                    .filter(|kind| !self.is_code_activated(kind))
                    .count();

                // if the code is not activated and has no missing deps
                // then it's a potential candidate
                missing_deps == 0 && !self.is_code_activated(kind)
            })
            .map(|(_, code)| code)
            .collect::<Vec<&CheatCode>>();

        // then return a random code based on their rarity (rarity is the weight)

        available_codes
            .choose_weighted(&mut rand::thread_rng(), |code| code.rarity as u8)
            .unwrap()
            .kind
    }

    pub fn activate_code(&mut self, text: &str) -> CheatCodeActivationResult {
        // iteration over all the existing codes
        for (_, code) in self.codes.iter() {
            // it we found a code
            if code.text.eq(text) {
                if self.is_code_activated(&code.kind) {
                    return CheatCodeActivationResult::AlreadyActivated(code.kind);
                }

                // if the code hasn't been activated do it
                self.activated.push(code.kind);
                return CheatCodeActivationResult::Activated(code.kind);
            }
        }
        CheatCodeActivationResult::NotFound
    }

    pub fn is_code_activated(&self, kind: &CheatCodeKind) -> bool {
        self.activated.contains(kind)
    }

    pub fn new() -> Self {
        let mut codes: HashMap<CheatCodeKind, CheatCode> = HashMap::new();

        // Mandatory
        insert_cheat(
            &mut codes,
            CheatCodeKind::Jump,
            CheatCodeRarity::Mandatory,
            vec![],
            "jump.png",
        );

        // Common
        insert_cheat(
            &mut codes,
            CheatCodeKind::Crouch,
            CheatCodeRarity::Common,
            vec![],
            "crouch.png",
        );
        insert_cheat(
            &mut codes,
            CheatCodeKind::Attack,
            CheatCodeRarity::Common,
            vec![],
            "attack.png",
        );
        insert_cheat(
            &mut codes,
            CheatCodeKind::AttackDmgBoost,
            CheatCodeRarity::Common,
            vec![CheatCodeKind::Attack],
            "attack_dmg_boost.png",
        );
        insert_cheat(
            &mut codes,
            CheatCodeKind::AttackFireRateBoost,
            CheatCodeRarity::Common,
            vec![CheatCodeKind::Attack],
            "attack_fr_boost.png",
        );
        insert_cheat(
            &mut codes,
            CheatCodeKind::MoveLeft,
            CheatCodeRarity::Common,
            vec![],
            "move_left.png",
        );
        insert_cheat(
            &mut codes,
            CheatCodeKind::SpeedBoost1,
            CheatCodeRarity::Common,
            vec![],
            "speed.png",
        );
        insert_cheat(
            &mut codes,
            CheatCodeKind::SpeedBoost2,
            CheatCodeRarity::Common,
            vec![CheatCodeKind::SpeedBoost1],
            "speed.png",
        );
        insert_cheat(
            &mut codes,
            CheatCodeKind::SpeedBoost3,
            CheatCodeRarity::Common,
            vec![CheatCodeKind::SpeedBoost1, CheatCodeKind::SpeedBoost2],
            "speed.png",
        );
        insert_cheat(
            &mut codes,
            CheatCodeKind::Armor,
            CheatCodeRarity::Common,
            vec![],
            "armor.png",
        );
        insert_cheat(
            &mut codes,
            CheatCodeKind::Dash,
            CheatCodeRarity::Common,
            vec![],
            "dash.png",
        );

        // Rare
        insert_cheat(
            &mut codes,
            CheatCodeKind::DoubleJump,
            CheatCodeRarity::Rare,
            vec![CheatCodeKind::Jump],
            "double_jump.png",
        );
        insert_cheat(
            &mut codes,
            CheatCodeKind::SpeedBoost4,
            CheatCodeRarity::Rare,
            vec![
                CheatCodeKind::SpeedBoost1,
                CheatCodeKind::SpeedBoost2,
                CheatCodeKind::SpeedBoost3,
            ],
            "speed.png",
        );
        insert_cheat(
            &mut codes,
            CheatCodeKind::SpeedBoost5,
            CheatCodeRarity::Rare,
            vec![
                CheatCodeKind::SpeedBoost1,
                CheatCodeKind::SpeedBoost2,
                CheatCodeKind::SpeedBoost3,
                CheatCodeKind::SpeedBoost4,
            ],
            "speed.png",
        );
        insert_cheat(
            &mut codes,
            CheatCodeKind::Shield,
            CheatCodeRarity::Rare,
            vec![CheatCodeKind::Jump],
            "shield.png",
        );

        // Legendary
        insert_cheat(
            &mut codes,
            CheatCodeKind::ExtraLife,
            CheatCodeRarity::Legendary,
            vec![],
            "extra_life.png",
        );
        insert_cheat(
            &mut codes,
            CheatCodeKind::TempInvicibility,
            CheatCodeRarity::Legendary,
            vec![CheatCodeKind::Armor, CheatCodeKind::Shield],
            "temp_invincibility.png",
        );
        insert_cheat(
            &mut codes,
            CheatCodeKind::Fly,
            CheatCodeRarity::Legendary,
            vec![CheatCodeKind::Jump, CheatCodeKind::DoubleJump],
            "fly.png",
        );

        Self {
            codes,
            activated: Vec::new(),
        }
    }
}

fn insert_cheat(
    codes: &mut HashMap<CheatCodeKind, CheatCode>,
    kind: CheatCodeKind,
    rarity: CheatCodeRarity,
    dependencies: Vec<CheatCodeKind>,
    image_path: &str,
) {
    codes.insert(
        kind,
        CheatCode::new(
            kind,
            rarity,
            &generate_random_code(rarity),
            dependencies,
            image_path.to_string(),
        ),
    );
}

pub fn generate_random_code(rarity: CheatCodeRarity) -> String {
    // length is based on the rarity
    let length = match rarity {
        CheatCodeRarity::Mandatory => 4,
        CheatCodeRarity::Common => 4,
        CheatCodeRarity::Rare => 6,
        CheatCodeRarity::Legendary => 8,
    };

    Alphanumeric.sample_string(&mut rand::thread_rng(), length)
}