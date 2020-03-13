#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

type LString = std::ffi::CString;

#[derive(Debug, Serialize)]
pub struct ArchetypeName {
    pub name: String,
    pub internal: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Class {
    pub name: String,
    pub display_name: String,
    pub display_help: String,
    pub allowed_origins: Vec<String>,
    pub special_restrictions: Vec<String>,
    pub store_requires: String,
    pub locked_tooltip: String,
    pub product_code: String,
    pub reduction_class: String,
    pub reduce_as_archvillain: bool,
    pub level_up_respecs: Vec<u32>,
    pub display_short_help: String,
    pub icon: String,
    pub primary_category: String,
    pub secondary_category: String,
    pub power_pool_category: String,
    pub epic_pool_category: String,
    pub attrib_min: Vec<CharacterAttributes>,
    pub attrib_base: Vec<CharacterAttributes>,
    pub strength_min: Vec<CharacterAttributes>,
    pub resistance_min: Vec<CharacterAttributes>,
    pub atrrib_dimin_str_in: Vec<CharacterAttributes>,
    pub atrrib_dimin_str_out: Vec<CharacterAttributes>,
    pub atrrib_dimin_cur_in: Vec<CharacterAttributes>,
    pub atrrib_dimin_cur_out: Vec<CharacterAttributes>,
    pub atrrib_dimin_res_in: Vec<CharacterAttributes>,
    pub atrrib_dimin_res_out: Vec<CharacterAttributes>,
    pub attrib_max_table: Vec<CharacterAttributesTable>,
    pub attrib_max_max_table: Vec<CharacterAttributesTable>,
    pub strength_max_table: Vec<CharacterAttributesTable>,
    pub resistance_max_table: Vec<CharacterAttributesTable>,
    pub mod_table: Vec<NamedTable>,
    pub connect_hp_and_status: bool,
}

impl Class {
    pub fn fix_strings(&mut self, pmessages: &std::collections::HashMap<String, String>) {
        // TODO: Switch to proper error handling
        self.display_name = pmessages.get(&self.display_name).unwrap().to_string();
        self.display_help = pmessages.get(&self.display_help).unwrap().to_string();
        self.display_short_help = pmessages.get(&self.display_short_help).unwrap().to_string();
        if !self.locked_tooltip.is_empty() {
            self.locked_tooltip = pmessages
                .get(&self.locked_tooltip)
                .unwrap_or_else(|| panic!("Not found: {}", self.locked_tooltip))
                .to_string();
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterAttributes {
    pub damage_type00: f32,
    pub damage_type01: f32,
    pub damage_type02: f32,
    pub damage_type03: f32,
    pub damage_type04: f32,
    pub damage_type05: f32,
    pub damage_type06: f32,
    pub damage_type07: f32,
    pub damage_type08: f32,
    pub damage_type09: f32,
    pub damage_type10: f32,
    pub damage_type11: f32,
    pub damage_type12: f32,
    pub damage_type13: f32,
    pub damage_type14: f32,
    pub damage_type15: f32,
    pub damage_type16: f32,
    pub damage_type17: f32,
    pub damage_type18: f32,
    pub damage_type19: f32,
    pub hit_points: f32,
    pub absorb: f32,
    pub endurance: f32,
    pub insight: f32,
    pub rage: f32,
    pub to_hit: f32,
    pub defense_type00: f32,
    pub defense_type01: f32,
    pub defense_type02: f32,
    pub defense_type03: f32,
    pub defense_type04: f32,
    pub defense_type05: f32,
    pub defense_type06: f32,
    pub defense_type07: f32,
    pub defense_type08: f32,
    pub defense_type09: f32,
    pub defense_type10: f32,
    pub defense_type11: f32,
    pub defense_type12: f32,
    pub defense_type13: f32,
    pub defense_type14: f32,
    pub defense_type15: f32,
    pub defense_type16: f32,
    pub defense_type17: f32,
    pub defense_type18: f32,
    pub defense_type19: f32,
    pub defense: f32,
    pub speed_running: f32,
    pub speed_flying: f32,
    pub speed_swimming: f32,
    pub speed_jumping: f32,
    pub jump_height: f32,
    pub movement_control: f32,
    pub movement_friction: f32,
    pub stealth: f32,
    pub stealth_radius: f32,
    pub stealth_radius_player: f32,
    pub perception_radius: f32,
    pub regeneration: f32,
    pub recovery: f32,
    pub insight_recovery: f32,
    pub threat_level: f32,
    pub taunt: f32,
    pub placate: f32,
    pub confused: f32,
    pub afraid: f32,
    pub terrorized: f32,
    pub held: f32,
    pub immobilized: f32,
    pub stunned: f32,
    pub sleep: f32,
    pub fly: f32,
    pub jumppack: f32,
    pub teleport: f32,
    pub untouchable: f32,
    pub intangible: f32,
    pub only_affects_self: f32,
    pub experience_gain: f32,
    pub influence_gain: f32,
    pub prestige_gain: f32,
    pub null_bool: f32,
    pub knockup: f32,
    pub knockback: f32,
    pub repel: f32,
    pub accuracy: f32,
    pub radius: f32,
    pub arc: f32,
    pub range: f32,
    pub time_to_activate: f32,
    pub recharge_time: f32,
    pub interrupt_time: f32,
    pub endurance_discount: f32,
    pub insight_discount: f32,
    pub meter: f32,
    pub elusivity00: f32,
    pub elusivity01: f32,
    pub elusivity02: f32,
    pub elusivity03: f32,
    pub elusivity04: f32,
    pub elusivity05: f32,
    pub elusivity06: f32,
    pub elusivity07: f32,
    pub elusivity08: f32,
    pub elusivity09: f32,
    pub elusivity10: f32,
    pub elusivity11: f32,
    pub elusivity12: f32,
    pub elusivity13: f32,
    pub elusivity14: f32,
    pub elusivity15: f32,
    pub elusivity16: f32,
    pub elusivity17: f32,
    pub elusivity18: f32,
    pub elusivity19: f32,
    pub elusivity_base: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CharacterAttributesTable {
    pub damage_type00: Vec<f32>,
    pub damage_type01: Vec<f32>,
    pub damage_type02: Vec<f32>,
    pub damage_type03: Vec<f32>,
    pub damage_type04: Vec<f32>,
    pub damage_type05: Vec<f32>,
    pub damage_type06: Vec<f32>,
    pub damage_type07: Vec<f32>,
    pub damage_type08: Vec<f32>,
    pub damage_type09: Vec<f32>,
    pub damage_type10: Vec<f32>,
    pub damage_type11: Vec<f32>,
    pub damage_type12: Vec<f32>,
    pub damage_type13: Vec<f32>,
    pub damage_type14: Vec<f32>,
    pub damage_type15: Vec<f32>,
    pub damage_type16: Vec<f32>,
    pub damage_type17: Vec<f32>,
    pub damage_type18: Vec<f32>,
    pub damage_type19: Vec<f32>,
    pub hit_points: Vec<f32>,
    pub endurance: Vec<f32>,
    pub insight: Vec<f32>,
    pub rage: Vec<f32>,
    pub to_hit: Vec<f32>,
    pub defense_type00: Vec<f32>,
    pub defense_type01: Vec<f32>,
    pub defense_type02: Vec<f32>,
    pub defense_type03: Vec<f32>,
    pub defense_type04: Vec<f32>,
    pub defense_type05: Vec<f32>,
    pub defense_type06: Vec<f32>,
    pub defense_type07: Vec<f32>,
    pub defense_type08: Vec<f32>,
    pub defense_type09: Vec<f32>,
    pub defense_type10: Vec<f32>,
    pub defense_type11: Vec<f32>,
    pub defense_type12: Vec<f32>,
    pub defense_type13: Vec<f32>,
    pub defense_type14: Vec<f32>,
    pub defense_type15: Vec<f32>,
    pub defense_type16: Vec<f32>,
    pub defense_type17: Vec<f32>,
    pub defense_type18: Vec<f32>,
    pub defense_type19: Vec<f32>,
    pub defense: Vec<f32>,
    pub speed_running: Vec<f32>,
    pub speed_flying: Vec<f32>,
    pub speed_swimming: Vec<f32>,
    pub speed_jumping: Vec<f32>,
    pub jump_height: Vec<f32>,
    pub movement_control: Vec<f32>,
    pub movement_friction: Vec<f32>,
    pub stealth: Vec<f32>,
    pub stealth_radius: Vec<f32>,
    pub stealth_radius_player: Vec<f32>,
    pub perception_radius: Vec<f32>,
    pub regeneration: Vec<f32>,
    pub recovery: Vec<f32>,
    pub insight_recovery: Vec<f32>,
    pub threat_level: Vec<f32>,
    pub taunt: Vec<f32>,
    pub placate: Vec<f32>,
    pub confused: Vec<f32>,
    pub afraid: Vec<f32>,
    pub terrorized: Vec<f32>,
    pub held: Vec<f32>,
    pub immobilized: Vec<f32>,
    pub stunned: Vec<f32>,
    pub sleep: Vec<f32>,
    pub fly: Vec<f32>,
    pub jumppack: Vec<f32>,
    pub teleport: Vec<f32>,
    pub untouchable: Vec<f32>,
    pub intangible: Vec<f32>,
    pub only_affects_self: Vec<f32>,
    pub experience_gain: Vec<f32>,
    pub influence_gain: Vec<f32>,
    pub prestige_gain: Vec<f32>,
    pub null_bool: Vec<f32>,
    pub knockup: Vec<f32>,
    pub knockback: Vec<f32>,
    pub repel: Vec<f32>,
    pub accuracy: Vec<f32>,
    pub radius: Vec<f32>,
    pub arc: Vec<f32>,
    pub range: Vec<f32>,
    pub time_to_activate: Vec<f32>,
    pub recharge_time: Vec<f32>,
    pub interrupt_time: Vec<f32>,
    pub endurance_discount: Vec<f32>,
    pub insight_discount: Vec<f32>,
    pub meter: Vec<f32>,
    pub elusivity00: Vec<f32>,
    pub elusivity01: Vec<f32>,
    pub elusivity02: Vec<f32>,
    pub elusivity03: Vec<f32>,
    pub elusivity04: Vec<f32>,
    pub elusivity05: Vec<f32>,
    pub elusivity06: Vec<f32>,
    pub elusivity07: Vec<f32>,
    pub elusivity08: Vec<f32>,
    pub elusivity09: Vec<f32>,
    pub elusivity10: Vec<f32>,
    pub elusivity11: Vec<f32>,
    pub elusivity12: Vec<f32>,
    pub elusivity13: Vec<f32>,
    pub elusivity14: Vec<f32>,
    pub elusivity15: Vec<f32>,
    pub elusivity16: Vec<f32>,
    pub elusivity17: Vec<f32>,
    pub elusivity18: Vec<f32>,
    pub elusivity19: Vec<f32>,
    pub defense_whats_this: Vec<f32>, // Does this override the defense above?
    pub absorb: Vec<f32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NamedTable {
    pub name: String,
    pub values: Vec<f32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PowerCategory {
    pub source_file: String,
    pub name: String,
    pub display_name: String,
    pub display_help: String,
    pub display_short_help: String,
    pub powersets: Vec<String>,
}

impl PowerCategory {
    pub fn fix_strings(&mut self, pmessages: &std::collections::HashMap<String, String>) {
        // TODO: Switch to proper error handling
        if !self.display_name.is_empty() {
            self.display_name = pmessages.get(&self.display_name).unwrap().to_string();
        }
        if !self.display_help.is_empty() {
            self.display_help = pmessages.get(&self.display_help).unwrap().to_string();
        }
        if !self.display_short_help.is_empty() {
            self.display_short_help = pmessages.get(&self.display_short_help).unwrap().to_string();
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PowerRedirect {
    pub power: String,
    pub requires: Vec<String>,
    pub show_in_info: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Power {
    pub full_name: String,
    pub crcfull_name: u32,
    pub source_file: String,
    pub name: String,
    pub source_name: String,
    pub system: u32,
    pub auto_issue: bool,
    pub auto_issue_save_level: bool,
    pub free: bool,
    pub display_name: String,
    pub display_help: String,
    pub display_short_help: String,
    // display_caster_help: String,
    // display_caster_short_help: String,
    pub display_target_help: String,
    pub display_target_short_help: String,
    pub display_attacker_attack: String,
    pub display_attacker_attack_floater: String,
    pub display_attacker_hit: String,
    pub display_victim_hit: String,
    pub display_confirm: String,
    pub float_rewarded: String,
    pub display_power_defense_float: String,
    pub icon_name: String,
    #[serde(rename = "type")]
    pub power_type: u32,
    pub num_allowed: u32,
    pub attack_types: Vec<AttackType>,
    // pub attack_types: Vec<u32>,
    pub buy_requires: Vec<String>,
    pub activate_requires: Vec<String>,
    pub slot_requires: Vec<String>,
    pub target_requires: Vec<String>,
    pub reward_requires: Vec<String>,
    pub auction_requires: Vec<String>,
    pub reward_fallback: String,
    pub accuracy: f32,
    pub near_ground: bool,
    pub target_near_ground: bool,
    pub castable_after_death: u32,
    pub cast_through_hold: bool,
    pub cast_through_sleep: bool,
    pub cast_through_stun: bool,
    pub cast_through_terrorize: bool,
    pub toggle_ignore_hold: bool,
    pub toggle_ignore_sleep: bool,
    pub toggle_ignore_stun: bool,
    pub ignore_level_bought: bool,
    pub shoot_through_untouchable: bool,
    pub interrupt_like_sleep: bool,
    pub ai_report: u32,
    pub effect_area: u32,
    pub max_targets_hit: u32,
    pub _unknown_unused: u32, // This is always 0x00000000
    pub radius: f32,
    pub arc: f32,
    pub chain_delay: f32,
    pub chain_eff: Vec<String>,
    pub chain_fork: Vec<u32>,
    pub box_offset: (f32, f32, f32),
    pub box_size: (f32, f32, f32),
    pub range: f32,
    pub range_secondary: f32,
    pub time_to_activate: f32,
    pub recharge_time: f32,
    pub activate_period: f32,
    pub endurance_cost: f32,
    pub idea_cost: f32,
    pub time_to_confirm: u32,
    pub self_confirm: u32,
    pub confirm_requires: Vec<String>,
    pub destroy_on_limit: bool,
    pub stacking_usage: bool,
    pub num_charges: u32,
    pub max_num_charges: u32,
    pub usage_time: f32,
    pub max_usage_time: f32,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub lifetime_in_game: f32,
    pub max_lifetime_in_game: f32,
    pub interrupt_time: f32,
    pub target_visibility: u32,
    pub target: u32,
    pub target_secondary: u32,
    pub ents_auto_hit: Vec<u32>,
    pub ents_affected: Vec<u32>,
    pub targets_through_vision_phase: bool,
    pub boosts_allowed: Vec<u32>,
    pub group_membership: Vec<u32>,
    pub modes_required: Vec<u32>,
    pub modes_disallowed: Vec<u32>,
    pub ai_groups: Vec<String>,
    pub redirect: Vec<PowerRedirect>,
    pub effects: Vec<Effects>,
    pub ignore_strength: bool,
    pub show_buff_icon: bool,
    pub show_in_inventory: u32,
    pub show_in_manage: bool,
    pub show_in_info: bool,
    pub deletable: bool,
    pub tradeable: bool,
    pub max_boosts: u32,
    pub do_not_save: bool,
    pub boost_ignore_effectiveness: bool,
    pub boost_always_count_for_set: bool,
    pub boost_tradeable: bool,
    pub boost_combinable: bool,
    pub boost_account_bound: bool,
    pub boost_boostable: bool,
    pub boost_use_player_level: bool,
    pub boost_catalyst_conversion: String,
    pub store_product: String,
    pub boost_license_level: u32,
    pub min_slot_level: i32,
    pub max_slot_level: u32,
    pub max_boost_level: u32,
    pub var: Vec<Var>,
    pub toggle_droppable: u32,
    pub proc_allowed: u32,
    pub strengths_disallowed: Vec<u32>,
    pub proc_main_target_only: bool,
    pub anim_main_target_only: bool,
    pub highlight_eval: Vec<String>,
    pub highlight_icon: String,
    pub highlight_ring: RGBA,
    pub travel_suppression: f32,
    pub preference_multiplier: f32,
    pub dont_set_stance: bool,
    pub point_value: f32,
    pub point_multiplier: f32,
    pub chain_into_power: String,
    pub instance_locked: bool,
    pub is_environment_hit: bool,
    pub shuffle_targets: bool,
    pub force_level_bought: i32,
    pub refreshes_on_active_player_change: bool,
    pub cancelable: bool,
    pub ignore_toggle_max_distance: bool,
    pub server_tray_priority: u32,
    pub server_tray_requires: Vec<String>,
    pub abusive_buff: bool,
    pub position_center: u32,
    pub position_distance: f32,
    pub position_height: f32,
    pub position_yaw: f32,
    pub face_target: bool,
    pub attrib_cache: Vec<u32>,
    pub visual_fx: String,
    pub attack_bits: Vec<u32>,
    pub block_bits: Vec<u32>,
    pub wind_up_bits: Vec<u32>,
    pub hit_bits: Vec<u32>,
    pub death_bits: Vec<u32>,
    pub activation_bits: Vec<u32>,
    pub deactivation_bits: Vec<u32>,
    pub initial_attack_bits: Vec<u32>,
    pub continuing_bits: Vec<u32>,
    pub conditional_bits: Vec<u32>,
    pub activation_fx: String,
    pub deactivation_fx: String,
    pub attack_fx: String,
    pub secondary_attack_fx: String,
    pub hit_fx: String,
    pub wind_up_fx: String,
    pub block_fx: String,
    pub death_fx: String,
    pub initial_attack_fx: String,
    pub continuing_fx: String,
    pub continuing_fx1: String,
    pub continuing_fx2: String,
    pub continuing_fx3: String,
    pub continuing_fx4: String,
    pub conditional_fx: String,
    pub conditional_fx1: String,
    pub conditional_fx2: String,
    pub conditional_fx3: String,
    pub conditional_fx4: String,
    pub mode_bits: Vec<u32>,
    pub frames_before_hit: u32,
    pub frames_before_secondary_hit: u32,
    pub delayed_hit: bool,
    pub attack_frames: u32,
    pub initial_frames_before_hit: u32,
    pub initial_attack_fxframe_delay: u32,
    pub projectile_speed: f32,
    pub secondary_projectile_speed: f32,
    pub initial_frames_before_block: u32,
    pub ignore_attack_time_errors: String,
    pub frames_before_block: u32,
    pub fx_important: bool,
    pub primary_tint: ConditionalRGBA,
    pub secondary_tint: ConditionalRGBA,
    pub custom_fx: Vec<CustomFx>,
}

impl Power {
    pub fn fix_strings(&mut self, pmessages: &std::collections::HashMap<String, String>) {
        // TODO: Switch to proper error handling
        self.display_name = pmessages.get(&self.display_name).unwrap().to_string();
        self.display_help = pmessages.get(&self.display_help).unwrap().to_string();
        self.display_short_help = pmessages.get(&self.display_short_help).unwrap().to_string();
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomFx {
    pub token: String,
    pub alt_theme: Vec<String>,
    pub source_file: String,
    pub category: String,
    pub display_name: String,
    pub attack_bits: Vec<u32>,
    pub block_bits: Vec<u32>,
    pub wind_up_bits: Vec<u32>,
    pub hit_bits: Vec<u32>,
    pub death_bits: Vec<u32>,
    pub activation_bits: Vec<u32>,
    pub deactivation_bits: Vec<u32>,
    pub initial_attack_bits: Vec<u32>,
    pub continuing_bits: Vec<u32>,
    pub conditional_bits: Vec<u32>,
    pub activation_fx: String,
    pub deactivation_fx: String,
    pub attack_fx: String,
    pub secondary_attack_fx: String,
    pub hit_fx: String,
    pub wind_up_fx: String,
    pub block_fx: String,
    pub death_fx: String,
    pub initial_attack_fx: String,
    pub continuing_fx: String,
    pub continuing_fx1: String,
    pub continuing_fx2: String,
    pub continuing_fx3: String,
    pub continuing_fx4: String,
    pub conditional_fx: String,
    pub conditional_fx1: String,
    pub conditional_fx2: String,
    pub conditional_fx3: String,
    pub conditional_fx4: String,
    pub mode_bits: Vec<u32>,
    pub frames_before_hit: u32,
    pub frames_before_secondary_hit: u32,
    pub delayed_hit: bool,
    pub attack_frames: u32,
    pub initial_frames_before_hit: u32,
    pub initial_attack_fxframe_delay: u32,
    pub projectile_speed: f32,
    pub secondary_projectile_speed: f32,
    pub initial_frames_before_block: u32,
    pub ignore_attack_time_errors: String,
    pub frames_before_block: u32,
    pub fx_important: bool,
    pub primary_tint: ConditionalRGBA,
    pub secondary_tint: ConditionalRGBA,
    pub palette: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RGBA(u8, u8, u8, u8);

#[derive(Debug, Deserialize, Serialize)]
pub struct ConditionalRGBA(u8, u8, u8, u8);

#[derive(Debug, Deserialize, Serialize)]
pub struct Var {
    pub index: u32,
    pub name: String,
    pub min: f32,
    pub max: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Effects {
    pub tag: Vec<String>,
    pub chance: f32,
    pub procs_per_minute: f32,
    pub delay: f32,
    pub radius_inner: f32,
    pub radius_outer: f32,
    pub requires: Vec<String>,
    pub flags: u32,
    pub eval_flags: u32,
    pub attrib_mod: Vec<AttribMod>,
    pub effect: Vec<Effects>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AttribMod {
    pub attrib: Vec<u32>,
    pub aspect: u32,
    pub application_type: u32,
    #[serde(rename = "type")]
    pub attrib_type: u32,
    pub target: u32,
    pub target_info: Vec<TargetInfo>,
    pub table: String,
    pub scale: f32,
    pub duration: f32,
    pub magnitude: f32,
    pub duration_expr: Vec<String>,
    pub magnitude_expr: Vec<String>,
    pub delay: f32,
    pub period: f32,
    pub tick_chance: f32,
    pub delayed_requires: Vec<String>,
    pub caster_stack_type: u32,
    pub stack_type: u32,
    pub stack_limit: u32,
    pub stack_key: u32,
    pub cancel_events: Vec<u32>,
    pub suppress: Vec<Suppress>,
    pub boost_mod_allowed: u32,
    pub flags: AttribModFlags,
    pub messages: Vec<AttribModMessages>,
    pub fx: Vec<AttribModFX>,
    pub param: AttribModParam,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AttribModParam {
    None,
    Costume(attrib_mod_param_structs::Costume),
    Reward(attrib_mod_param_structs::Reward),
    EntCreate(attrib_mod_param_structs::EntCreate),
    Power(attrib_mod_param_structs::Power),
    Phase(attrib_mod_param_structs::Phase),
    Teleport(attrib_mod_param_structs::Teleport),
    Behavior(attrib_mod_param_structs::Behavior),
    SZEValue(attrib_mod_param_structs::SZEValue),
    Token(attrib_mod_param_structs::Token),
    EffectFilter(attrib_mod_param_structs::EffectFilter),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AttribModMessages {
    pub display_attacker_hit: String,
    pub display_victim_hit: String,
    pub display_float: String,
    pub display_attrib_defense_float: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AttribModFX {
    pub continuing_bits: Vec<u32>,
    pub continuing_fx: String,
    pub conditional_bits: Vec<u32>,
    pub conditional_fx: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AttribModFlags(u32, u32);

#[derive(Debug, Deserialize, Serialize)]
pub struct TargetInfo {
    pub marker: Vec<String>,
    pub count: Vec<u32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Suppress {
    pub event: u32,
    pub seconds: u32,
    pub always: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Powerset {
    pub source_file: String,
    pub full_name: String,
    pub name: String,
    pub system: u32,
    pub shared: u8,
    pub display_name: String,
    pub display_help: String,
    pub display_short_help: String,
    pub icon_name: String,
    pub costume_keys: Vec<String>,
    pub costume_parts: Vec<String>,
    pub set_account_requires: String,
    pub set_account_tooltip: String,
    pub set_account_product: String,
    pub set_buy_requires: Vec<String>,
    pub set_buy_requires_failed_text: String,
    pub show_in_inventory: u32,
    pub show_in_manage: bool,
    pub show_in_info: bool,
    pub specialize_at: u32,
    pub specialize_requires: Vec<String>,
    pub powers: Vec<String>,
    pub available: Vec<u32>,
    pub aimax_level: Vec<u32>,
    pub aimin_rank_con: Vec<u32>,
    pub aimax_rank_con: Vec<u32>,
    pub min_difficulty: Vec<u32>,
    pub max_difficulty: Vec<u32>,
    pub force_level_bought: u32,
}

impl Powerset {
    pub fn fix_strings(&mut self, pmessages: &std::collections::HashMap<String, String>) {
        // TODO: Switch to proper error handling
        self.display_name = pmessages.get(&self.display_name).unwrap().to_string();
        self.display_help = pmessages.get(&self.display_help).unwrap().to_string();
        self.display_short_help = pmessages.get(&self.display_short_help).unwrap().to_string();
    }
}

pub mod attrib_mod_param_structs {
    use serde::{Deserialize, Serialize};
    #[derive(Debug, Deserialize, Serialize)]
    pub struct Costume {
        pub costume: String,
        pub priority: u32,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Reward {
        pub reward: Vec<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct EntCreate {
        pub entity_def: String,
        pub class: String,
        pub costume: String,
        pub display_name: String,
        pub priority_list: String,
        pub ai_config: String,
        pub power_category: Vec<String>,
        pub powerset: Vec<String>,
        pub power: Vec<String>,
        pub unknown1: u32,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Power {
        pub power_category: Vec<String>,
        pub powerset: Vec<String>,
        pub power: Vec<String>,
        pub count: u32,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Phase {
        pub unknown1: Vec<u32>,
        pub unknown2: Vec<u32>,
        pub unknown3: u32,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Teleport {
        pub destination: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Behavior {
        pub behavior: Vec<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct SZEValue {
        pub script_id: Vec<String>,
        pub script_value: Vec<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Token {
        pub token: Vec<String>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct EffectFilter {
        pub power_category: Vec<String>,
        pub powerset: Vec<String>,
        pub power: Vec<String>,
        pub tag: Vec<String>,
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "u32", into = "u32")]
pub enum AttackType {
    Ranged,
    Melee,
    AOE,
    Smashing,
    Lethal,
    Fire,
    Cold,
    Energy,
    NegativeEnergy,
    Psionic,
    Toxic,
    Unknown(u32),
}

impl std::convert::From<u32> for AttackType {
    fn from(num: u32) -> Self {
        match num {
            104 => AttackType::Ranged,
            108 => AttackType::Melee,
            112 => AttackType::AOE,
            116 => AttackType::Smashing,
            120 => AttackType::Lethal,
            124 => AttackType::Fire,
            128 => AttackType::Cold,
            132 => AttackType::Energy,
            136 => AttackType::NegativeEnergy,
            140 => AttackType::Psionic,
            144 => AttackType::Toxic,
            num => AttackType::Unknown(num),
        }
    }
}

impl std::convert::Into<u32> for AttackType {
    fn into(self) -> u32 {
        match self {
            AttackType::Ranged => 104,
            AttackType::Melee => 108,
            AttackType::AOE => 112,
            AttackType::Smashing => 116,
            AttackType::Lethal => 120,
            AttackType::Fire => 124,
            AttackType::Cold => 128,
            AttackType::Energy => 132,
            AttackType::NegativeEnergy => 136,
            AttackType::Psionic => 140,
            AttackType::Toxic => 144,
            AttackType::Unknown(num) => num,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BoostSet {
    pub name: String,
    pub display_name: String,
    pub group_name: String,
    pub conversion_groups: Vec<String>,
    pub powers: Vec<LString>,
    pub boost_lists: Vec<BoostList>,
    pub bonuses: Vec<BoostSetBonus>,
    pub min_level: u32,
    pub max_level: u32,
    pub store_product: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BoostList {
    pub boosts: Vec<LString>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct BoostSetBonus {
    pub display_text: String,
    pub min_boosts: u32,
    pub max_boosts: u32,
    pub requires: Vec<String>,
    pub auto_powers: Vec<LString>,
    pub bonus_power: LString,
}
