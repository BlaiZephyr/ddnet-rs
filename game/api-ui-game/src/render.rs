use api::{GRAPHICS, IO, RUNTIME_THREAD_POOL, SOUND};
use client_containers::{
    ctf::{CtfContainer, CTF_CONTAINER_PATH},
    emoticons::{EmoticonsContainer, EMOTICONS_CONTAINER_PATH},
    entities::{EntitiesContainer, ENTITIES_CONTAINER_PATH},
    flags::{FlagsContainer, FLAGS_CONTAINER_PATH},
    freezes::{FreezeContainer, FREEZE_CONTAINER_PATH},
    game::{GameContainer, GAME_CONTAINER_PATH},
    hooks::{HookContainer, HOOK_CONTAINER_PATH},
    hud::{HudContainer, HUD_CONTAINER_PATH},
    ninja::{NinjaContainer, NINJA_CONTAINER_PATH},
    particles::{ParticlesContainer, PARTICLES_CONTAINER_PATH},
    skins::{SkinContainer, SKIN_CONTAINER_PATH},
    weapons::{WeaponContainer, WEAPON_CONTAINER_PATH},
};

/// made to be easy to use for API stuff
pub fn create_skin_container() -> SkinContainer {
    let default_skin =
        SkinContainer::load_default(&IO.with(|g| (*g).clone()), SKIN_CONTAINER_PATH.as_ref());
    let scene = SOUND.with(|g| g.scene_handle.create(Default::default()));
    SkinContainer::new(
        IO.with(|g| (*g).clone()),
        RUNTIME_THREAD_POOL.clone(),
        default_skin,
        true,
        None,
        None,
        "skin-container",
        &GRAPHICS.with(|g| (*g).clone()),
        &SOUND.with(|g| (*g).clone()),
        &scene,
        SKIN_CONTAINER_PATH.as_ref(),
    )
}

/// made to be easy to use for API stuff
pub fn create_emoticons_container() -> EmoticonsContainer {
    let default_emoticons = EmoticonsContainer::load_default(
        &IO.with(|g| (*g).clone()),
        EMOTICONS_CONTAINER_PATH.as_ref(),
    );
    let scene = SOUND.with(|g| g.scene_handle.create(Default::default()));
    EmoticonsContainer::new(
        IO.with(|g| (*g).clone()),
        RUNTIME_THREAD_POOL.clone(),
        default_emoticons,
        true,
        None,
        None,
        "emoticons-container",
        &GRAPHICS.with(|g| (*g).clone()),
        &SOUND.with(|g| (*g).clone()),
        &scene,
        EMOTICONS_CONTAINER_PATH.as_ref(),
    )
}

/// made to be easy to use for API stuff
pub fn create_weapon_container() -> WeaponContainer {
    let default_weapon =
        WeaponContainer::load_default(&IO.with(|g| (*g).clone()), WEAPON_CONTAINER_PATH.as_ref());
    let scene = SOUND.with(|g| g.scene_handle.create(Default::default()));
    WeaponContainer::new(
        IO.with(|g| (*g).clone()),
        RUNTIME_THREAD_POOL.clone(),
        default_weapon,
        true,
        None,
        None,
        "weapon-container",
        &GRAPHICS.with(|g| (*g).clone()),
        &SOUND.with(|g| (*g).clone()),
        &scene,
        WEAPON_CONTAINER_PATH.as_ref(),
    )
}

/// made to be easy to use for API stuff
pub fn create_flags_container() -> FlagsContainer {
    let default_flags =
        FlagsContainer::load_default(&IO.with(|g| (*g).clone()), FLAGS_CONTAINER_PATH.as_ref());
    let scene = SOUND.with(|g| g.scene_handle.create(Default::default()));
    FlagsContainer::new(
        IO.with(|g| (*g).clone()),
        RUNTIME_THREAD_POOL.clone(),
        default_flags,
        true,
        None,
        None,
        "flags-container",
        &GRAPHICS.with(|g| (*g).clone()),
        &SOUND.with(|g| (*g).clone()),
        &scene,
        FLAGS_CONTAINER_PATH.as_ref(),
    )
}

/// made to be easy to use for API stuff
pub fn create_hook_container() -> HookContainer {
    let default_hooks =
        HookContainer::load_default(&IO.with(|g| (*g).clone()), HOOK_CONTAINER_PATH.as_ref());
    let scene = SOUND.with(|g| g.scene_handle.create(Default::default()));
    HookContainer::new(
        IO.with(|g| (*g).clone()),
        RUNTIME_THREAD_POOL.clone(),
        default_hooks,
        true,
        None,
        None,
        "hooks-container",
        &GRAPHICS.with(|g| (*g).clone()),
        &SOUND.with(|g| (*g).clone()),
        &scene,
        HOOK_CONTAINER_PATH.as_ref(),
    )
}

/// made to be easy to use for API stuff
pub fn create_entities_container() -> EntitiesContainer {
    let default_item = EntitiesContainer::load_default(
        &IO.with(|g| (*g).clone()),
        ENTITIES_CONTAINER_PATH.as_ref(),
    );
    let scene = SOUND.with(|g| g.scene_handle.create(Default::default()));
    EntitiesContainer::new(
        IO.with(|g| (*g).clone()),
        RUNTIME_THREAD_POOL.clone(),
        default_item,
        true,
        None,
        None,
        "entities-container",
        &GRAPHICS.with(|g| (*g).clone()),
        &SOUND.with(|g| (*g).clone()),
        &scene,
        HOOK_CONTAINER_PATH.as_ref(),
    )
}

/// made to be easy to use for API stuff
pub fn create_freeze_container() -> FreezeContainer {
    let default_item =
        FreezeContainer::load_default(&IO.with(|g| (*g).clone()), FREEZE_CONTAINER_PATH.as_ref());
    let scene = SOUND.with(|g| g.scene_handle.create(Default::default()));
    FreezeContainer::new(
        IO.with(|g| (*g).clone()),
        RUNTIME_THREAD_POOL.clone(),
        default_item,
        true,
        None,
        None,
        "freeze-container",
        &GRAPHICS.with(|g| (*g).clone()),
        &SOUND.with(|g| (*g).clone()),
        &scene,
        HOOK_CONTAINER_PATH.as_ref(),
    )
}

/// made to be easy to use for API stuff
pub fn create_particles_container() -> ParticlesContainer {
    let default_item = ParticlesContainer::load_default(
        &IO.with(|g| (*g).clone()),
        PARTICLES_CONTAINER_PATH.as_ref(),
    );
    let scene = SOUND.with(|g| g.scene_handle.create(Default::default()));
    ParticlesContainer::new(
        IO.with(|g| (*g).clone()),
        RUNTIME_THREAD_POOL.clone(),
        default_item,
        true,
        None,
        None,
        "particles-container",
        &GRAPHICS.with(|g| (*g).clone()),
        &SOUND.with(|g| (*g).clone()),
        &scene,
        HOOK_CONTAINER_PATH.as_ref(),
    )
}

/// made to be easy to use for API stuff
pub fn create_ninja_container() -> NinjaContainer {
    let default_item =
        NinjaContainer::load_default(&IO.with(|g| (*g).clone()), NINJA_CONTAINER_PATH.as_ref());
    let scene = SOUND.with(|g| g.scene_handle.create(Default::default()));
    NinjaContainer::new(
        IO.with(|g| (*g).clone()),
        RUNTIME_THREAD_POOL.clone(),
        default_item,
        true,
        None,
        None,
        "ninja-container",
        &GRAPHICS.with(|g| (*g).clone()),
        &SOUND.with(|g| (*g).clone()),
        &scene,
        HOOK_CONTAINER_PATH.as_ref(),
    )
}

/// made to be easy to use for API stuff
pub fn create_game_container() -> GameContainer {
    let default_item =
        GameContainer::load_default(&IO.with(|g| (*g).clone()), GAME_CONTAINER_PATH.as_ref());
    let scene = SOUND.with(|g| g.scene_handle.create(Default::default()));
    GameContainer::new(
        IO.with(|g| (*g).clone()),
        RUNTIME_THREAD_POOL.clone(),
        default_item,
        true,
        None,
        None,
        "game-container",
        &GRAPHICS.with(|g| (*g).clone()),
        &SOUND.with(|g| (*g).clone()),
        &scene,
        HOOK_CONTAINER_PATH.as_ref(),
    )
}

/// made to be easy to use for API stuff
pub fn create_hud_container() -> HudContainer {
    let default_item =
        HudContainer::load_default(&IO.with(|g| (*g).clone()), HUD_CONTAINER_PATH.as_ref());
    let scene = SOUND.with(|g| g.scene_handle.create(Default::default()));
    HudContainer::new(
        IO.with(|g| (*g).clone()),
        RUNTIME_THREAD_POOL.clone(),
        default_item,
        true,
        None,
        None,
        "hud-container",
        &GRAPHICS.with(|g| (*g).clone()),
        &SOUND.with(|g| (*g).clone()),
        &scene,
        HOOK_CONTAINER_PATH.as_ref(),
    )
}

/// made to be easy to use for API stuff
pub fn create_ctf_container() -> CtfContainer {
    let default_item =
        CtfContainer::load_default(&IO.with(|g| (*g).clone()), CTF_CONTAINER_PATH.as_ref());
    let scene = SOUND.with(|g| g.scene_handle.create(Default::default()));
    CtfContainer::new(
        IO.with(|g| (*g).clone()),
        RUNTIME_THREAD_POOL.clone(),
        default_item,
        true,
        None,
        None,
        "ctf-container",
        &GRAPHICS.with(|g| (*g).clone()),
        &SOUND.with(|g| (*g).clone()),
        &scene,
        HOOK_CONTAINER_PATH.as_ref(),
    )
}
