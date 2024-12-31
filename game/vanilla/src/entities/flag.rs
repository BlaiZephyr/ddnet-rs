pub mod flag {
    use crate::reusable::{CloneWithCopyableElements, ReusableCore};
    use base::linked_hash_map_view::FxLinkedHashMap;
    use game_interface::{
        events::{GameFlagEventSound, GameWorldEntitySoundEvent},
        types::{
            flag::FlagType,
            game::GameTickType,
            id_types::{CharacterId, CtfFlagId},
            render::game::game_match::MatchSide,
        },
    };
    use hiarc::Hiarc;
    use math::math::{
        distance, lerp,
        vector::{ivec2, vec2},
    };
    use pool::{datatypes::PoolFxLinkedHashMap, pool::Pool, recycle::Recycle, traits::Recyclable};
    use serde::{Deserialize, Serialize};

    use crate::{
        entities::{
            character::core::character_core,
            entity::entity::{DropMode, Entity, EntityInterface, EntityTickResult},
        },
        events::events::FlagEvent,
        simulation_pipe::simulation_pipe::{
            GameWorldPendingEvents, SimulationEventWorldEntity, SimulationEventWorldEntityType,
            SimulationPipeFlag, SimulationWorldEvent, SimulationWorldEvents,
        },
        state::state::TICKS_PER_SECOND,
        world::world::GameWorld,
    };

    #[derive(Debug, Hiarc, Default, Serialize, Deserialize)]
    pub struct FlagReusableCore {}

    impl Recyclable for FlagReusableCore {
        fn new() -> Self {
            Self {}
        }

        fn reset(&mut self) {}
    }

    impl CloneWithCopyableElements for FlagReusableCore {
        fn copy_clone_from(&mut self, _other: &Self) {}
    }

    impl ReusableCore for FlagReusableCore {}

    pub type PoolFlagReusableCore = Recycle<FlagReusableCore>;

    #[derive(Debug, Hiarc, Default, Copy, Clone, Serialize, Deserialize)]
    pub struct FlagCore {
        pub pos: vec2,
        pub spawn_pos: vec2,
        pub vel: vec2,
        pub ty: FlagType,

        pub carrier: Option<CharacterId>,
        pub drop_ticks: Option<GameTickType>,

        /// If the flag is teleported, this is increased
        pub non_linear_event: u64,
    }

    #[derive(Debug, Hiarc)]
    pub struct Flag {
        pub(crate) base: Entity<CtfFlagId>,
        pub(crate) core: FlagCore,
        pub(crate) reusable_core: PoolFlagReusableCore,

        simulation_events: SimulationWorldEvents,
        game_pending_events: GameWorldPendingEvents,
    }

    impl Flag {
        pub const PHYSICAL_SIZE: f32 = 14.0;
        pub fn new(
            game_el_id: &CtfFlagId,
            pos: &vec2,
            ty: FlagType,
            pool: &FlagPool,
            game_pending_events: &GameWorldPendingEvents,
            simulation_events: &SimulationWorldEvents,
        ) -> Self {
            Self {
                base: Entity::new(game_el_id),
                core: FlagCore {
                    pos: *pos,
                    spawn_pos: *pos,
                    ty,
                    ..Default::default()
                },
                reusable_core: pool.flag_reusable_cores_pool.new(),
                game_pending_events: game_pending_events.clone(),
                simulation_events: simulation_events.clone(),
            }
        }

        pub fn lerped_pos(flag1: &Flag, flag2: &Flag, ratio: f64) -> vec2 {
            lerp(&flag1.core.pos, &flag2.core.pos, ratio as f32)
        }

        pub fn reset(&mut self, is_prediction: bool) {
            // prediction cannot move the flag so much, since that lerps weirdly.
            if !is_prediction {
                self.core.pos = self.core.spawn_pos;
                self.core.drop_ticks = None;
                self.core.carrier = None;
            }
        }

        /// check for capture
        fn check_captured(&mut self, carrier: CharacterId, other_team_flags: &Flags) {
            for other_flag in other_team_flags.values() {
                if other_flag.core.carrier.is_none()
                    && other_flag.core.pos == other_flag.core.spawn_pos
                    && distance(&self.core.pos, &other_flag.core.pos)
                        < Flag::PHYSICAL_SIZE + character_core::PHYSICAL_SIZE
                {
                    let flag_pos = self.core.pos;
                    self.reset(false);
                    self.simulation_events.push(SimulationWorldEvent::Entity(
                        SimulationEventWorldEntity {
                            ev: SimulationEventWorldEntityType::Flag {
                                id: self.base.game_element_id,
                                ev: FlagEvent::Capture {
                                    by: carrier,
                                    pos: flag_pos,
                                },
                            },
                        },
                    ));
                    self.game_pending_events.push_sound(
                        Some(carrier),
                        None,
                        GameWorldEntitySoundEvent::Flag(GameFlagEventSound::Capture),
                    );
                    self.core.non_linear_event += 1;
                }
            }
        }
    }

    impl EntityInterface<FlagCore, FlagReusableCore, SimulationPipeFlag<'_>> for Flag {
        fn pre_tick(&mut self, _pipe: &mut SimulationPipeFlag) -> EntityTickResult {
            // TODO:
            EntityTickResult::None
        }

        fn tick(&mut self, _pipe: &mut SimulationPipeFlag) -> EntityTickResult {
            // TODO:
            EntityTickResult::None
        }

        fn tick_deferred(&mut self, pipe: &mut SimulationPipeFlag) -> EntityTickResult {
            if let Some(carrier) = self.core.carrier {
                if let Some(character) = pipe.characters.characters().get(&carrier) {
                    self.core.pos = *character.pos.pos();

                    self.check_captured(carrier, pipe.other_team_flags);
                } else {
                    self.game_pending_events.push_sound(
                        Some(carrier),
                        None,
                        GameWorldEntitySoundEvent::Flag(GameFlagEventSound::Drop),
                    );
                    self.core.drop_ticks = Some(TICKS_PER_SECOND * 30);
                    self.core.carrier = None;
                }
            } else {
                if pipe.collision.is_death(self.core.pos.x, self.core.pos.y)
                    || Entity::<CtfFlagId>::outside_of_playfield(&self.core.pos, pipe.collision)
                {
                    self.reset(pipe.is_prediction);
                }

                // check if a char picked this flag up
                let intersection = GameWorld::intersect_character(
                    pipe.field,
                    pipe.characters.characters_mut(),
                    &self.core.pos,
                    Self::PHYSICAL_SIZE as i32,
                );
                if let Some(intersection) = intersection {
                    let char_in_side = intersection.core.side.is_some_and(|side| match side {
                        MatchSide::Red => matches!(self.core.ty, FlagType::Red),
                        MatchSide::Blue => matches!(self.core.ty, FlagType::Blue),
                    });
                    if char_in_side {
                        if self.core.pos != self.core.spawn_pos {
                            self.game_pending_events.push_sound(
                                Some(intersection.base.game_element_id),
                                None,
                                GameWorldEntitySoundEvent::Flag(GameFlagEventSound::Return),
                            );
                            self.reset(pipe.is_prediction);
                            self.core.non_linear_event += 1;
                        }
                    } else {
                        self.game_pending_events.push_sound(
                            Some(intersection.base.game_element_id),
                            None,
                            GameWorldEntitySoundEvent::Flag(GameFlagEventSound::Collect(
                                self.core.ty,
                            )),
                        );
                        self.core.carrier = Some(intersection.base.game_element_id);
                        self.core.drop_ticks = None;
                    }
                }

                if let Some(drop_ticks) = &mut self.core.drop_ticks {
                    match (*drop_ticks).cmp(&0) {
                        std::cmp::Ordering::Equal => {
                            self.game_pending_events.push_sound(
                                None,
                                None,
                                GameWorldEntitySoundEvent::Flag(GameFlagEventSound::Return),
                            );
                            self.reset(pipe.is_prediction);
                            self.core.non_linear_event += 1;
                        }
                        std::cmp::Ordering::Greater => {
                            *drop_ticks -= 1;

                            self.core.vel.y += pipe.collision.get_tune_at(&self.core.pos).gravity;

                            pipe.collision.move_box(
                                &mut self.core.pos,
                                &mut self.core.vel,
                                &ivec2::new(Self::PHYSICAL_SIZE as i32, Self::PHYSICAL_SIZE as i32),
                                0.5,
                            );
                        }
                        std::cmp::Ordering::Less => {
                            // ignore
                        }
                    }
                }
            }
            EntityTickResult::None
        }

        fn drop_mode(&mut self, mode: DropMode) {
            self.base.drop_mode = mode;
        }
    }

    impl Drop for Flag {
        fn drop(&mut self) {
            if matches!(self.base.drop_mode, DropMode::None) {
                self.simulation_events
                    .push_world(SimulationEventWorldEntityType::Flag {
                        id: self.base.game_element_id,
                        ev: FlagEvent::Despawn {
                            pos: self.core.pos,
                            ty: self.core.ty,
                            respawns_in_ticks: 0.into(),
                        },
                    });
            }
        }
    }

    #[derive(Debug, Hiarc, Clone)]
    pub struct FlagPool {
        pub(crate) flag_pool: Pool<PoolFlags>,
        pub(crate) flag_reusable_cores_pool: Pool<FlagReusableCore>,
    }

    pub type PoolFlags = FxLinkedHashMap<CtfFlagId, Flag>;
    pub type Flags = PoolFxLinkedHashMap<CtfFlagId, Flag>;
}
