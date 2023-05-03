## bevy_health_bar3d

Health Bar plugin for Bevy 3D. Don't let the name fool you – this plugin is designed not
only for tracking health, but also for managing other values such as Mana or any attribute that can be represented
as a percentage.

## Bevy Compatibility

| Bevy Version | Crate Version |
| ------------ | ------------- |
| `0.9`        | `^1.0.0`      |

## Usage

Implement the `Percentage` trait for the component you want to track and pass the type of your component
to the plugin on instantiation:

```rust
use bevy_health_bar3d::prelude::{HealthBarPlugin, Percentage};

#[derive(Component)]
struct Health {
    max: f32,
    current: f32,
}

impl Percentage for Health {
    fn value(&self) -> f32 {
        self.current / self.max
    }
}

fn main() {
    App::new()
        .add_plugin(HealthBarPlugin::<Health>::default())
        // ... initialize multiple times to track further component types
        .add_plugin(HealthBarPlugin::<Mana>::default())
        // set a different color for the Mana bar
        .insert_resource(ColorScheme::<Mana>::new().foreground_color(ForegroundColor::Static(Color::BLUE)))
        // ... other plugins
        .run();
}
```

Spawn a mesh, the component to be tracked, and a `BarBundle` to configure the look & feel of your bar.

```rust
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere { radius, ..default() })),
            // ...
        },
        Health {
            max: 10.,
            current: 2.,
        },
        BarBundle::<Health> {
            offset: BarOffset::new(radius * 1.5),
            width: BarWidth::new(radius * 2.),
            orientation: BarOrientation::Vertical, // defaults is horizontal
            ..default()
        },
    ));
}
```

Note the generic parameter of `BarBundle`. It is used to associate the configuration with the component it is tracking
and necessary to support multiple bars per entity.

That's it! Updates to the values of your component will be automatically propagated through to the bar.

Further examples can be found [here](./examples)
