use crate::interaction::interaction::Interaction;
use crate::interaction::interaction_kind::InteractionKind;
use crate::needs::NeedKind;
use crate::sim::sim_event::SimEvent;

pub struct InteractionResolver;

#[allow(dead_code)]
impl InteractionResolver {
    /// Resolve an interaction, returning a sorted Vec<SimEvent> for determinism.
    pub fn resolve(interaction: &Interaction) -> Vec<SimEvent> {
        let tick = interaction.tick;
        let sentiment: i32 = match interaction.kind {
            InteractionKind::Chat => 5,
            InteractionKind::Joke => 10,
            InteractionKind::Compliment => 15,
            InteractionKind::Ignore => -5,
            InteractionKind::Argue => -15,
        };

        let mut events = vec![
            SimEvent::NeedChanged {
                tick,
                agent_id: interaction.initiator,
                need_kind: NeedKind::Social,
                old_val: 0,
                new_val: sentiment.max(0) as u8,
            },
            SimEvent::NeedChanged {
                tick,
                agent_id: interaction.target,
                need_kind: NeedKind::Social,
                old_val: 0,
                new_val: sentiment.max(0) as u8,
            },
            SimEvent::InteractionOccurred {
                tick,
                interaction: interaction.clone(),
            },
        ];

        // Sort for determinism: by event discriminant then agent id
        events.sort_by_key(|e| match e {
            SimEvent::NeedChanged { agent_id, .. } => (0u8, agent_id.0),
            SimEvent::InteractionOccurred { .. } => (1u8, 0),
            _ => (2u8, 0),
        });

        events
    }
}
