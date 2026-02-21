[![Checks](https://github.com/sparten11740/bevy_health_bar3d/actions/workflows/checks.yml/badge.svg)](https://github.com/sparten11740/bevy_health_bar3d/actions/workflows/checks.yml) [![Release](https://github.com/sparten11740/bevy_health_bar3d/actions/workflows/release.yml/badge.svg)](https://github.com/sparten11740/bevy_health_bar3d/actions/workflows/release.yml)

## bevy_health_bar3d

Health Bar plugin for Bevy supporting both 2D and 3D rendering. It can be used to render a bar for any value that can be
represented as percentage. Can be freely sized, supports horizontal or vertical orientation, custom fore- and background
colors, and an optional border with configurable thickness and color. Works with split-screens or layered cameras out of
the box.

<img src="https://github.com/sparten11740/bevy_health_bar3d/assets/2863630/31c50809-30f0-45fc-8639-054db7c96429" width="300" />

## Bevy Compatibility

| Bevy Version | Crate Version |
| ------------ | ------------: |
| `0.17`       |    >= `3.7.0` |
| `0.16`       |       `3.5.0` |
| `0.15`       |       `3.4.0` |
| `0.14`       |       `3.3.0` |
| `0.13`       |       `3.2.0` |
| `0.12`       |       `2.0.0` |
| `0.11`       |       `1.2.0` |
| `0.10`       |       `1.1.0` |
| `0.9`        |       `1.0.0` |

## Usage

Implement the `Percentage` trait for the component you want to track and pass the type of your component
to the plugin on instantiation:

```rust
use bevy_health_bar3d::prelude::{HealthBarPlugin, Percentage};
use bevy::color::palettes::basic::*;

#[derive(Component, Reflect)]
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
        // add multiple times to track further component types
        .add_plugins((HealthBarPlugin::<Health>::default(), HealthBarPlugin::<Mana>::default()))
        // set a different color for the Mana bar
        .insert_resource(ColorScheme::<Mana>::new().foreground_color(ForegroundColor::Static(BLUE.into())))
        .run();
}
```

Spawn a mesh, the component to be tracked, and a `BarSettings` component to configure the look & feel of your bar.

```rust
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Sphere { radius })),
        Health {
            max: 10.,
            current: 2.,
        },
        BarSettings::<Health> {
            width: 5.,
            offset: 2.,
            orientation: BarOrientation::Vertical, // default is horizontal
            ..default()
        },
    ));
}
```

Note the generic parameter of `BarSettings`. It is used to associate the configuration with the component it is tracking
and necessary to support multiple bars per entity.

That's it! Updates to the values of your component will be automatically propagated through to the bar.

## Rendering Modes

This plugin supports both 2D sprite-based and 3D billboard-based rendering through cargo features.

### 3D Rendering (Default)

3D mode uses PBR materials with billboard shaders that automatically face the camera. This is the default mode:

```toml
[dependencies]
bevy_health_bar3d = "3.6.0"
```

Or explicitly

```toml
[dependencies]
bevy_health_bar3d = { version = "3.6.0", features = ["3d"] }
```

Use with 3D entities like meshes, and the health bar will automatically face the camera.

### 2D Rendering

2D mode uses sprite materials and shaders for 2D games:

```toml
[dependencies]
bevy_health_bar3d = { version = "3.6.0", default-features = false, features = ["2d"] }
```

Use with 2D sprites and entities. See the bar2d example for a complete demonstration.
Note: The features are mutually exclusive - choose either 2d or 3d for your project.

## Examples

Examples can be found [here](https://github.com/sparten11740/bevy_health_bar3d/tree/main/examples).
To run an example for web, first install cargo-make (`cargo install cargo-make`) and then call
`cargo make web <name-of-the-example`, such as `cargo make web dinosaurs`
