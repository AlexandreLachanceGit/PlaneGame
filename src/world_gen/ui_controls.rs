use bevy::prelude::*;
use crate::world_gen;

pub struct UIControls;

impl Plugin for UIControls {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .add_startup_system(setup.system())
            .add_system(button_system.system());
    }
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            hovered: materials.add(Color::rgb(0.15, 0.15, 0.55).into()),
            normal: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}

fn button_system(
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut world_gen_data: ResMut<world_gen::WorldGenData>,
    //mut text_query: Query<&mut Text>,
) {
    for (interaction, mut material, _children) in interaction_query.iter_mut() {
        //let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    button_materials: Res<ButtonMaterials>,
) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(12.5), Val::Percent(25.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        border: Rect::all(Val::Px(2.0)),
                        ..Default::default()
                    },
                    material: materials.add(Color::rgb(0.65, 0.65, 0.65).into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                align_items: AlignItems::FlexEnd,
                                ..Default::default()
                            },
                            material: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                style: Style {
                                    margin: Rect::all(Val::Px(5.0)),
                                    ..Default::default()
                                },
                                text: Text::with_section(
                                    "Chunk size ",
                                    TextStyle {
                                        font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                    },
                                    Default::default(),
                                ),
                                ..Default::default()
                            });
                            parent
                                .spawn_bundle(ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(30.0), Val::Px(30.0)),
                                        // center button
                                        margin: Rect::all(Val::Px(0.0)),
                                        // horizontally center child text
                                        justify_content: JustifyContent::Center,
                                        // vertically center child text
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    material: button_materials.normal.clone(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle {
                                        text: Text::with_section(
                                            "+",
                                            TextStyle {
                                                font: asset_server
                                                    .load("fonts/FiraMono-Medium.ttf"),
                                                font_size: 30.0,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                            Default::default(),
                                        ),
                                        ..Default::default()
                                    });
                                });
                            parent
                                .spawn_bundle(ButtonBundle {
                                    style: Style {
                                        size: Size::new(Val::Px(30.0), Val::Px(30.0)),
                                        // center button
                                        margin: Rect::all(Val::Px(0.0)),
                                        // horizontally center child text
                                        justify_content: JustifyContent::SpaceEvenly,
                                        // vertically center child text
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    material: button_materials.normal.clone(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle {
                                        text: Text::with_section(
                                            "-",
                                            TextStyle {
                                                font: asset_server
                                                    .load("fonts/FiraMono-Medium.ttf"),
                                                font_size: 30.0,
                                                color: Color::rgb(0.9, 0.9, 0.9),
                                            },
                                            Default::default(),
                                        ),
                                        ..Default::default()
                                    });
                                });
                        });
                });
        });
}
