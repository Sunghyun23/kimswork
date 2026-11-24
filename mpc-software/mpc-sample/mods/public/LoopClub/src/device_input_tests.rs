//! Command-observing audit of the actual Mod callbacks, not copied routing logic.
use super::*;
const PAGES: [Page; 16] = [
    Page::Loop,
    Page::Play,
    Page::Grid,
    Page::Mix,
    Page::KitMix,
    Page::TrackFx,
    Page::MasterFx,
    Page::Setup,
    Page::Chop,
    Page::Source,
    Page::DrumSound,
    Page::Output,
    Page::Sidechain,
    Page::Texture,
    Page::FxSettings,
    Page::Perform,
];
const CODES: [u32; 22] = [
    0, 1, 6, 7, 9, 12, 13, 14, 36, 40, 49, 53, 54, 55, 73, 74, 75, 80, 81, 82, 111, 127,
];
fn capture(f: impl FnOnce()) -> Vec<Command> {
    AUDIT_COMMANDS.with(|v| *v.borrow_mut() = Some(Vec::new()));
    f();
    AUDIT_COMMANDS.with(|v| v.borrow_mut().take().unwrap())
}
fn button(code: u32, on: bool, shift: bool) -> ButtonEvent {
    ButtonEvent {
        event: 0,
        bytes: [0; 16],
        code,
        value: if on { 127 } else { 0 },
        pressed: on,
        shift,
        erase: false,
        surface: 0,
    }
}
fn app(page: Page, kind: Kind) -> LoopClub {
    let mut a = LoopClub::default();
    a.ui.active = true;
    a.ui.page = page;
    a.snapshot.words[16] = kind as u64;
    a.snapshot.words[27] = 8000;
    a.snapshot.words[770] = 8000;
    a.snapshot.words[769] = 8000;
    a
}
#[test]
fn every_page_button_press_has_no_duplicate_engine_command_and_release_is_inert() {
    let mut cases = 0;
    for page in PAGES {
        for kind in [Kind::Audio, Kind::Synth, Kind::Drum] {
            for code in CODES {
                for shift in [false, true] {
                    for detail in [false, true] {
                        let mut a = app(page, kind);
                        a.ui.setup_timing = detail;
                        a.ui.layer_detail = detail;
                        let commands = capture(|| {
                            a.on_button(button(code, true, shift));
                        });
                        let mut unique = std::collections::HashSet::new();
                        for c in commands {
                            assert!(
                                unique.insert(format!("{c:?}")),
                                "duplicate {page:?}/{kind:?}/code{code}/shift{shift}/detail{detail}: {c:?}"
                            );
                        }
                        let state = (
                            a.ui.page,
                            a.ui.selected,
                            a.ui.synth_page,
                            a.ui.setup_timing,
                            a.ui.layer_detail,
                            a.ui.loop_mode,
                        );
                        assert!(
                            capture(|| {
                                a.on_button(button(code, false, shift));
                            })
                            .is_empty(),
                            "release command {page:?}/{code}"
                        );
                        assert_eq!(
                            state,
                            (
                                a.ui.page,
                                a.ui.selected,
                                a.ui.synth_page,
                                a.ui.setup_timing,
                                a.ui.layer_detail,
                                a.ui.loop_mode
                            )
                        );
                        cases += 1;
                    }
                }
            }
        }
    }
    assert_eq!(cases, 4224);
}
#[test]
fn browser_owns_plus_minus_modifiers_without_background_commands_on_all_pages() {
    for page in PAGES {
        for code in CODES {
            if matches!(code, 53 | 81) {
                continue;
            }
            for shift in [false, true] {
                let mut a = app(page, Kind::Synth);
                a.browser.open = true;
                a.ui.setup_timing = true;
                let before = (
                    a.ui.page,
                    a.ui.selected,
                    a.ui.setup_timing,
                    a.ui.synth_page,
                    a.ui.layer_detail,
                );
                let sent = capture(|| {
                    a.on_button(button(code, true, shift));
                    a.on_button(button(code, false, shift));
                });
                assert!(
                    sent.is_empty(),
                    "browser leaked {page:?}/{code}/shift{shift}: {sent:?}"
                );
                assert_eq!(
                    before,
                    (
                        a.ui.page,
                        a.ui.selected,
                        a.ui.setup_timing,
                        a.ui.synth_page,
                        a.ui.layer_detail
                    )
                );
            }
        }
    }
}
#[test]
fn browser_blocks_background_knobs_fader_and_pad_press_every_page() {
    for page in PAGES {
        let mut a = app(page, Kind::Synth);
        a.browser.open = true;
        for control in [16, 17, 18, 20] {
            assert!(
                capture(|| {
                    a.control_cc(
                        CcEvent {
                            event: 0,
                            control,
                            value: 100,
                            low: 0.,
                            range: 127.,
                        },
                        false,
                    );
                })
                .is_empty(),
                "{page:?}/CC{control}"
            );
        }
        for pad in 0..16 {
            assert!(
                capture(|| {
                    a.on_pad(PadEvent {
                        action: PadAction::Press,
                        input: 0,
                        source: 0,
                        pad,
                        properties: 0,
                        velocity: 0.8,
                    });
                })
                .is_empty(),
                "{page:?}/pad{pad}"
            );
        }
    }
}
#[test]
fn held_note_releases_exactly_once_after_every_navigation_destination() {
    for kind in [Kind::Synth, Kind::Drum] {
        for dest in PAGES {
            let mut a = app(Page::Play, kind);
            let event = PadEvent {
                action: PadAction::Press,
                input: 0,
                source: 0,
                pad: 3,
                properties: 0,
                velocity: 0.8,
            };
            let onset = capture(|| {
                a.on_pad(event);
            });
            assert_eq!(
                onset
                    .iter()
                    .filter(|c| matches!(c, Command::Note { on: true, .. }))
                    .count(),
                1
            );
            a.ui.navigate(dest, &a.snapshot);
            let release = capture(|| {
                a.on_pad(PadEvent {
                    action: PadAction::Release,
                    velocity: 0.,
                    ..event
                });
            });
            assert_eq!(
                release
                    .iter()
                    .filter(|c| matches!(c, Command::Note { on: false, .. }))
                    .count(),
                1,
                "{kind:?}/{dest:?}: {release:?}"
            );
            assert!(
                capture(|| {
                    a.on_pad(PadEvent {
                        action: PadAction::Release,
                        velocity: 0.,
                        ..event
                    });
                })
                .is_empty()
            );
        }
    }
}
#[test]
fn data_raw_delta_is_one_logical_selection_and_never_a_second_press_action() {
    for delta in [1, 2, 64, 128] {
        let mut a = app(Page::Loop, Kind::Synth);
        let sent = capture(|| {
            a.on_encoder(EncoderEvent { surface: 0, delta });
        });
        assert_eq!(a.ui.selected, 1);
        assert_eq!(
            sent.iter()
                .filter(|c| matches!(c, Command::Select(1)))
                .count(),
            1
        );
        assert!(
            !sent
                .iter()
                .any(|c| matches!(c, Command::LoopRecord(_) | Command::Pad(..)))
        );
    }
}

#[test]
fn synth_all_subpages_and_chord_modes_have_single_soft_action_and_inert_release() {
    for page in 0..8 {
        for chord in [false, true] {
            for shift in [false, true] {
                for code in [12, 13, 14, 54, 55, 111] {
                    let mut a = app(Page::Play, Kind::Synth);
                    a.ui.synth_page = page;
                    if chord {
                        a.snapshot.words[16] |= 1 << 19;
                    }
                    a.ui.chord_modes[0] = chord;
                    let sent = capture(|| {
                        a.on_button(button(code, true, shift));
                    });
                    let mut unique = std::collections::HashSet::new();
                    for c in sent {
                        assert!(
                            unique.insert(format!("{c:?}")),
                            "synth{page}/chord{chord}/shift{shift}/{code}: {c:?}"
                        );
                    }
                    assert!(
                        capture(|| {
                            a.on_button(button(code, false, shift));
                        })
                        .is_empty()
                    );
                }
            }
        }
    }
}
#[test]
fn every_page_pad_modifiers_emit_no_duplicate_command_and_repeat_release_is_inert() {
    let mut cases = 0;
    for page in PAGES {
        for kind in [Kind::Audio, Kind::Synth, Kind::Drum] {
            for shift in [false, true] {
                for erase in [false, true] {
                    for pad in 0..16 {
                        let mut a = app(page, kind);
                        let mut sent = Vec::new();
                        a.ui.pad(pad, true, 84, shift, erase, &a.snapshot, |c| sent.push(c));
                        let mut unique = std::collections::HashSet::new();
                        for c in sent {
                            assert!(
                                unique.insert(format!("{c:?}")),
                                "{page:?}/{kind:?}/pad{pad}/shift{shift}/erase{erase}: {c:?}"
                            );
                        }
                        a.ui.pad(pad, false, 0, shift, erase, &a.snapshot, |_| {});
                        let mut again = Vec::new();
                        a.ui.pad(pad, false, 0, shift, erase, &a.snapshot, |c| again.push(c));
                        assert!(
                            again.is_empty(),
                            "repeat release {page:?}/{kind:?}/pad{pad}: {again:?}"
                        );
                        cases += 1;
                    }
                }
            }
        }
    }
    assert_eq!(cases, 3072);
}
#[test]
fn actual_button_chord_record_transport_and_navigation_are_mutually_exclusive() {
    for page in PAGES {
        let mut a = app(page, Kind::Synth);
        let sent = capture(|| {
            a.on_button(button(81, true, false));
        });
        assert_eq!(
            sent.iter().filter(|c| matches!(c, Command::Stop)).count(),
            1
        );
        assert!(
            !sent
                .iter()
                .any(|c| matches!(c, Command::LoopRecord(_) | Command::PlayAll))
        );
    }
}

#[test]
fn loop_mute_chain_and_edit_overlays_never_fall_through_to_record_or_generation() {
    for mode in [1, 2] {
        for pad in 0..16 {
            for shift in [false, true] {
                for erase in [false, true] {
                    let mut a = app(Page::Loop, Kind::Drum);
                    a.ui.loop_mode = mode;
                    a.snapshot.words[20 + 12 * pad] = 4 * PPQ;
                    let mut sent = Vec::new();
                    a.ui.pad(pad, true, 84, shift, erase, &a.snapshot, |c| sent.push(c));
                    assert!(!sent.iter().any(|c| matches!(
                        c,
                        Command::LoopRecord(_)
                            | Command::GenerateDrumPart { .. }
                            | Command::Note { .. }
                    )));
                    if mode == 1 {
                        assert_eq!(
                            sent.iter()
                                .filter(|c| matches!(c, Command::Mute(_)))
                                .count(),
                            1
                        );
                    }
                }
            }
        }
    }
    for code in [12, 13, 14, 73, 80] {
        let mut a = app(Page::Loop, Kind::Drum);
        a.ui.loop_edit = true;
        let sent = capture(|| {
            a.on_button(button(code, true, false));
        });
        assert!(!sent.iter().any(|c| matches!(
            c,
            Command::LoopRecord(_) | Command::Note { .. } | Command::GenerateDrumPart { .. }
        )));
    }
}

#[test]
fn note_edit_overlays_change_only_notes_and_releases_never_retrigger() {
    for copy in [false, true] {
        for pad in 0..16 {
            let mut a = app(Page::DrumSound, Kind::Drum);
            a.ui.pad_notes_edit = Some(copy);
            let mut sent = Vec::new();
            a.ui.pad(pad, true, 84, false, false, &a.snapshot, |c| sent.push(c));
            assert!(sent.iter().all(|c| matches!(c, Command::PadNotes { .. })));
            if !copy {
                assert_eq!(sent.len(), 1);
            }
            a.ui.pad(pad, false, 0, false, false, &a.snapshot, |c| sent.push(c));
            assert!(
                !sent
                    .iter()
                    .any(|c| matches!(c, Command::Note { .. } | Command::GenerateDrumPart { .. }))
            );
        }
    }
}

#[test]
fn modulated_fx_settings_shape_sync_and_data_are_single_parameter_actions() {
    for code in [13, 14] {
        let mut a = app(Page::FxSettings, Kind::Synth);
        a.ui.fx_edit = Some(crate::ui::FxEdit {
            from: Page::TrackFx,
            target: 0,
            track: 0,
            slot: 0,
            effect: Effect::Tremolo,
            texture: false,
        });
        a.snapshot.words[320] = 12;
        a.snapshot.words[323] = 1 | (7 << 1);
        let sent = capture(|| {
            a.on_button(button(code, true, false));
            a.on_button(button(code, false, false));
        });
        assert_eq!(sent.len(), 1, "{code}: {sent:?}");
        assert!(
            matches!(sent[0], Command::FxValue { index: 5, .. }) && code == 13
                || matches!(sent[0], Command::FxValue { index: 3, .. }) && code == 14
        );
    }
    let mut a = app(Page::FxSettings, Kind::Synth);
    a.ui.fx_edit = Some(crate::ui::FxEdit {
        from: Page::TrackFx,
        target: 0,
        track: 0,
        slot: 0,
        effect: Effect::Tremolo,
        texture: false,
    });
    a.snapshot.words[320] = 12;
    a.snapshot.words[323] = 1 | (7 << 1);
    let sent = capture(|| {
        a.on_encoder(EncoderEvent {
            surface: 0,
            delta: 128,
        });
    });
    assert_eq!(sent.len(), 1);
    assert!(matches!(sent[0],Command::FxValue{index:4,value,..} if (value-8./15.).abs()<1e-6));
}
