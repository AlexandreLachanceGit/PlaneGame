use bevy::prelude::*;

pub struct Camera;

impl Plugin for Camera {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(control.system());
    }
}

struct Controllable {
    speed: f32,
}

fn setup(mut commands: Commands) {
    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(Controllable { speed: 5.0 });
}

fn control(
    time: Res<Time>,
    mut query: Query<(&Controllable, &mut Transform)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (control, mut transform) in query.iter_mut() {
        //transform.rotate(Quat::from_rotation_x(spin.speed * time.delta_seconds()));
        let mut direction = Vec3::ZERO;
        
        if keyboard_input.pressed(KeyCode::D) {
            direction.x = 1.0;
        }
        if keyboard_input.pressed(KeyCode::A) {
            direction.x = -1.0;
        }
        if keyboard_input.pressed(KeyCode::W) {
            direction.z = -1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction.z = 1.0;
        }
        if keyboard_input.pressed(KeyCode::LShift) {
            direction.y = -1.0;
        }
        if keyboard_input.pressed(KeyCode::Space) {
            direction.y = 1.0;
        }


        transform.translation += time.delta_seconds() * direction * control.speed;
    }
}
