#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
mod consts;


pub mod ai;
mod ai_extras;
mod bosses;
mod events;
mod items;
mod modules;


pub use ai_extras::*;
pub use bosses::*;
pub use consts::*;
pub use events::*;
pub use items::*;
pub use modules::*;

// Temporary
#[repr(C)]
pub struct BattleObject(u64);

#[repr(C)]
pub struct Fighter {
    parent: BattleObject,
    // ...
}

#[repr(C)]
pub struct Weapon {
    parent: BattleObject,
    // ...
}

#[repr(C)]
pub struct Item {
    parent: BattleObject,
    // ...
}

#[repr(C)]
pub struct BattleObjectModuleAccessor(u64);

#[repr(C)]
pub struct FighterModuleAccessor {
    parent: BattleObjectModuleAccessor,
    // ...
}

#[repr(C)]
pub struct WeaponModuleAccessor {
    parent: BattleObjectModuleAccessor,
    // ...
}

#[repr(C)]
pub struct ItemModuleAccessor {
    parent: BattleObjectModuleAccessor,
    // ...
}

#[repr(C)]
pub struct FighterAIWeapon(u64);