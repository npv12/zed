use buffer_diff::DiffHunkStatus;
use editor::{DiffHunkRenderer, Editor};
use gpui::Entity;
use project::project_settings::ProjectSettings;
use settings::Settings;
use std::ops::Range;
use ui::{Tooltip, prelude::*};

pub(crate) struct UnstagedDiffHunkRenderer;

impl DiffHunkRenderer for UnstagedDiffHunkRenderer {
    fn render_hunk_controls(
        &self,
        row: u32,
        status: &DiffHunkStatus,
        hunk_range: Range<editor::Anchor>,
        is_created_file: bool,
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
        let hunk_range_for_restore = hunk_range.clone();
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
                Button::new(("stage", row as u64), "Stage")
                    .alpha(if status.is_pending() { 0.66 } else { 1.0 })
                    .tooltip(Tooltip::text("Stage Hunk"))
                    .on_click({
                        let editor = editor.clone();
                        move |_event, window, cx| {
                            editor.update(cx, |editor, cx| {
                                editor.stage_or_unstage_diff_hunks(
                                    true,
                                    vec![hunk_range.clone()],
                                    window,
                                    cx,
                                );
                            });
                        }
                    }),
            )
            .child(
                Button::new(("restore", row as u64), "Restore")
                    .tooltip(Tooltip::text("Restore Hunk"))
                    .on_click({
                        let editor = editor.clone();
                        let hunk_range = hunk_range_for_restore;
                        move |_event, window, cx| {
                            editor.update(cx, |editor, cx| {
                                let snapshot = editor.buffer().read(cx).snapshot(cx);
                                let hunks: Vec<_> = editor
                                    .diff_hunks_in_ranges(&[hunk_range.clone()], &snapshot)
                                    .collect();
                                if !hunks.is_empty() {
                                    editor.apply_restore(hunks, window, cx);
                                }
                            });
                        }
                    })
                    .disabled(is_created_file),
            )
            .into_any_element()
    }

    fn render_hunk_as_staged(&self, _status: &DiffHunkStatus, _cx: &App) -> bool {
        false
    }
}
