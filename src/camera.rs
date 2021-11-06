use bevy::prelude::*;

pub struct Camera;

impl Plugin for Camera {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(control.system());
    }
}

struct Controllable {
    mov_speed: f32,
    rot_speed: f32,
}

fn setup(mut commands: Commands) {
    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(-20.0, 100.0, -20.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(Controllable {
            mov_speed: 5.0,
            rot_speed: 2.0,
        });
}

fn control(
    time: Res<Time>,
    mut query: Query<(&Controllable, &mut Transform)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (control, mut transform) in query.iter_mut() {
        //transform.rotate(Quat::from_rotation_x(spin.speed * time.delta_seconds()));
        let mut direction = Vec3::ZERO;
        let mut rotation = Vec4::ZERO;

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
        if keyboard_input.pressed(KeyCode::LControl) {
            direction.y = -1.0;
        }
        if keyboard_input.pressed(KeyCode::Space) {
            direction.y = 1.0;
        }
        if keyboard_input.pressed(KeyCode::LShift) {
            direction *= 3.0;
        }

        if keyboard_input.pressed(KeyCode::Left) {
            rotation.x = 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            rotation.x = -1.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            rotation.y = 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            rotation.y = -1.0;
        }

        transform.rotate(Quat::from_rotation_x(
            time.delta_seconds() * rotation.y * control.rot_speed,
        ));
        transform.rotate(Quat::from_rotation_y(
            time.delta_seconds() * rotation.x * control.rot_speed,
        ));

        transform.translation += time.delta_seconds() * direction * control.mov_speed;
    }
}
