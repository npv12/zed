use buffer_diff::DiffHunkStatus;
use editor::{DiffHunkDelegate, Editor, ResolvedDiffHunks};
use gpui::{AnyElement, App, Context, Entity, Window};
use project::project_settings::ProjectSettings;
use settings::Settings;
use std::ops::Range;
use ui::{Button, Tooltip, prelude::*};
use util::ResultExt;

pub(crate) struct StagedDiffDelegate;

impl DiffHunkDelegate for StagedDiffDelegate {
    fn toggle(
        &self,
        hunks: Vec<ResolvedDiffHunks>,
        editor: &mut Editor,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        self.stage_or_unstage(false, hunks, editor, window, cx);
    }

    fn stage_or_unstage(
        &self,
        stage: bool,
        hunks: Vec<ResolvedDiffHunks>,
        editor: &mut Editor,
        _window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        if stage {
            return;
        }
        let Some(project) = editor.project().cloned() else {
            return;
        };
        for hunks in hunks {
            let index_ranges = hunks
                .hunks
                .into_iter()
                .map(|hunk| hunk.buffer_range)
                .collect::<Vec<_>>();
            if index_ranges.is_empty() {
                continue;
            }
            project
                .update(cx, |project, cx| {
                    project.unstage_staged_hunks(hunks.diff, index_ranges, cx)
                })
                .log_err();
        }
    }

    fn render_hunk_controls(
        &self,
        row: u32,
        status: &DiffHunkStatus,
        hunk_range: Range<editor::Anchor>,
        _is_created_file: bool,
        line_height: Pixels,
        editor: &Entity<Editor>,
        _window: &mut Window,
        cx: &mut App,
    ) -> AnyElement {
        if !ProjectSettings::get_global(cx)
            .git
            .show_stage_restore_buttons
        {
            return gpui::Empty.into_any_element();
        }
        let hunk_range = hunk_range.start..hunk_range.start;
        h_flex()
            .h(line_height)
            .mr_1()
            .gap_1()
            .px_0p5()
            .pb_1()
            .border_x_1()
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .rounded_b_lg()
            .bg(cx.theme().colors().editor_background)
            .block_mouse_except_scroll()
            .shadow_md()
            .child(
                Button::new(("unstage", row as u64), "Unstage")
                    .alpha(if status.is_pending() { 0.66 } else { 1.0 })
                    .tooltip(Tooltip::text("Unstage Hunk"))
                    .on_click({
                        let editor = editor.clone();
                        move |_event, window, cx| {
                            editor.update(cx, |editor, cx| {
                                editor.stage_or_unstage_diff_hunks(
                                    false,
                                    vec![hunk_range.clone()],
                                    window,
                                    cx,
                                );
                            });
                        }
                    }),
            )
            .into_any_element()
    }
}
