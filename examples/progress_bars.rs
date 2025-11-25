//! Progress Bars Example - Demonstrates different progress bar styles
//!
//! Run with: cargo run --example progress_bars --features progress

use bevy::prelude::*;
use bevy_ui_builders::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(UiBuilderPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, animate_progress)
        .run();
}

#[derive(Component)]
struct AnimatedProgress {
    speed: f32,
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2d);

    // Entity to add AnimatedProgress component later
    let mut animated_bar = None;

    // Root node
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(40.0)),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(30.0),
            ..default()
        })
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Progress Bar Showcase"),
                TextFont {
                    font_size: 36.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Progress bars container
            parent
                .spawn(Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(25.0),
                    width: Val::Px(400.0),
                    ..default()
                })
                .with_children(|container| {
                    // Default style
                    create_progress_section(container, "Default Style", |section| {
                        ProgressBarBuilder::new(0.65)
                            .with_label()
                            .build(section);
                    });

                    // Thin style
                    create_progress_section(container, "Thin Style", |section| {
                        ProgressBarBuilder::new(0.45)
                            .style(ProgressBarStyle::Thin)
                            .with_label()
                            .build(section);
                    });

                    // Thick style
                    create_progress_section(container, "Thick Style", |section| {
                        ProgressBarBuilder::new(0.80)
                            .style(ProgressBarStyle::Thick)
                            .with_label()
                            .build(section);
                    });

                    // Segmented style
                    create_progress_section(container, "Segmented Style", |section| {
                        ProgressBarBuilder::new(0.70)
                            .style(ProgressBarStyle::Segmented)
                            .with_label()
                            .build(section);
                    });

                    // Without label
                    create_progress_section(container, "Without Label", |section| {
                        progress(0.55).build(section);
                    });

                    // Animated progress
                    create_progress_section(container, "Animated Progress", |section| {
                        let bar = ProgressBarBuilder::new(0.0)
                            .with_label()
                            .animated()
                            .build(section);

                        animated_bar = Some(bar);
                    });

                    // Multiple progress bars (stacked)
                    create_progress_section(container, "Stacked Progress", |section| {
                        section
                            .spawn(Node {
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(5.0),
                                ..default()
                            })
                            .with_children(|stack| {
                                progress(0.9)
                                    .style(ProgressBarStyle::Thin)
                                    .build(stack);

                                progress(0.6)
                                    .style(ProgressBarStyle::Thin)
                                    .build(stack);

                                progress(0.3)
                                    .style(ProgressBarStyle::Thin)
                                    .build(stack);
                            });
                    });
                });
        });

    // Add AnimatedProgress component to the animated bar
    if let Some(bar) = animated_bar {
        commands.entity(bar).insert(AnimatedProgress { speed: 0.3 });
    }
}

fn create_progress_section<F>(parent: &mut ChildSpawnerCommands, label: &str, content: F)
where
    F: FnOnce(&mut ChildSpawnerCommands),
{
    parent
        .spawn(Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            ..default()
        })
        .with_children(|section| {
            // Label
            section.spawn((
                Text::new(label),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
            ));

            // Progress bar
            content(section);
        });
}

fn animate_progress(
    mut query: Query<&mut ProgressBar, With<AnimatedProgress>>,
    animated_query: Query<&AnimatedProgress>,
    time: Res<Time>,
) {
    for (mut progress_bar, animated) in query.iter_mut().zip(animated_query.iter()) {
        progress_bar.value += animated.speed * time.delta_secs();
        if progress_bar.value > 1.0 {
            progress_bar.value = 0.0;
        }
    }
}