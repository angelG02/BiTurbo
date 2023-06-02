pub mod app;

pub mod plugin;

pub mod prelude {
    pub use crate::app::*;
    pub use crate::plugin::*;

    pub struct AssetsManagerPlugin;

    use assets_manager::AssetCache;
    use bevy_ecs::prelude::*;
    use std::env;

    impl Plugin for AssetsManagerPlugin {
        fn build(&self, app: &mut App) {
            let asset_path = env::var("ASSETS_PATH");
            if asset_path.is_ok() {
                let cache = AssetCache::new(asset_path.unwrap()).expect("Luca is retarded");

                app.insert_resource(cache);
                app.add_systems(OnMainUpdate, (hot_reload, || {}));
            } else {
                let cache = AssetCache::new("C:\\Projects\\Hustle\\bi_turbo_v0\\sandbox\\assets")
                    .expect("Luca is retarded");

                app.insert_resource(cache);
                app.add_systems(OnMainUpdate, (hot_reload, || {}));
            }
        }
    }

    fn hot_reload(world: &mut World) {
        let mut asset_cache = world.get_resource_mut::<AssetCache>().unwrap();
        asset_cache.as_mut().hot_reload();
    }
}
