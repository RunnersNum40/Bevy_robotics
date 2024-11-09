use avian_motors::motor::{
    AngularVelocityTarget, MotorDerivativeGain, MotorIntegralGain, MotorMaxTorque, MotorProportionalGain,
    MotorTotalRotation, TargetRotation,
};
use bevy::prelude::*;
use bevy_fps_counter::FpsCounterPlugin;
use bevy_urdf::{Robot, RobotSpawnerPlugin};

mod camera;
mod world;

use camera::CameraPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FpsCounterPlugin,
            WorldPlugin,
            CameraPlugin,
            RobotSpawnerPlugin,
        ))
        .add_systems(Startup, spawn_test_robot)
        .add_systems(FixedUpdate, print_motor_positions)
        .run();
}

fn spawn_test_robot(mut commands: Commands) {
    let robot = Robot::new(
        "simple.urdf",
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
            scale: Vec3::ONE,
        },
    );

    commands.spawn(robot);
}

fn print_motor_positions(
    query: Query<(
        &TargetRotation,
        &MotorTotalRotation,
        &MotorProportionalGain,
        &MotorDerivativeGain,
        &MotorIntegralGain,
        &MotorMaxTorque,
    )>,
) {
    for (target, current, _, _, _, _) in query.iter() {
        println!("Target: {:.2} Current: {:.2}", target.0, current.0);
    }
    println!();
}
