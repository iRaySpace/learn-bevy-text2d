use bevy::prelude::*;

#[derive(Component, Debug)]
struct TextMoving {
    speed: f32,
    duration: Timer,
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 36.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section("Mission completed", text_style.clone(), text_alignment),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(TextMoving {
            speed: 100.0,
            duration: Timer::from_seconds(1.0, false),
        });
}

fn text_system(mut commands: Commands, mut query: Query<(&mut TextMoving, &mut Transform, Entity)>, time: Res<Time>) {
    let mut entities_to_dispose = vec![];
    for (mut text_moving, mut transform, entity) in query.iter_mut() {
        transform.translation.y += text_moving.speed * time.delta_seconds();
        text_moving.duration.tick(time.delta());
        if text_moving.duration.finished() {
            entities_to_dispose.push(entity);
        }
    }
    for entity in entities_to_dispose.iter() {
        commands.entity(*entity).despawn();
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(startup)
        .add_system(text_system)
        .run();
}
