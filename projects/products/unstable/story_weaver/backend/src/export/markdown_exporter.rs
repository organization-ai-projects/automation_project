use crate::events::{StoryEvent, StoryEventKind};
use crate::report::StoryReport;
use crate::state::StateValue;

pub struct MarkdownExporter;

impl MarkdownExporter {
    pub fn export(report: &StoryReport, events: &[StoryEvent]) -> String {
        let mut md = String::new();
        md.push_str(&format!("# {}\n\n", report.title));
        md.push_str(&format!("**Seed:** {}\n\n", report.seed));
        md.push_str(&format!("**Steps:** {}\n\n", report.steps_taken));
        md.push_str("## Events\n\n");

        let mut current_step: Option<u64> = None;
        for event in events {
            if current_step != Some(event.step) {
                current_step = Some(event.step);
                md.push_str(&format!("### Step {}\n\n", event.step));
            }
            match &event.kind {
                StoryEventKind::RuleApplied { rule_id } => {
                    md.push_str(&format!("- **Rule Applied:** {}\n", rule_id));
                }
                StoryEventKind::StateChanged {
                    variable,
                    old_value,
                    new_value,
                } => {
                    md.push_str(&format!(
                        "- *{}*: {} → {}\n",
                        variable,
                        format_value(old_value),
                        format_value(new_value)
                    ));
                }
                StoryEventKind::Narration { message } => {
                    md.push_str(&format!("- {}\n", message));
                }
                StoryEventKind::NoApplicableRules => {
                    md.push_str("- *No applicable rules — story ends.*\n");
                }
            }
        }

        md.push_str("\n## Summary\n\n");
        md.push_str(&format!("- **Run Hash:** `{}`\n", report.run_hash));
        md.push_str(&format!(
            "- **Snapshot Hash:** `{}`\n",
            report.snapshot_hash
        ));
        md.push_str(&format!("- **Total Events:** {}\n", report.event_count));
        md
    }
}

fn format_value(v: &StateValue) -> String {
    match v {
        StateValue::Text(s) => format!("\"{}\"", s),
        StateValue::Number(n) => n.to_string(),
        StateValue::Flag(b) => b.to_string(),
    }
}
