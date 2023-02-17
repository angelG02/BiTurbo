pub struct App {
    // pub window: Window,
    pub state: String,
    // pub events: Events,
    // pub renderer: Renderer,
    // pub input: Input,
    // pub audio: Audio,
    // pub time: Time,
    // pub assets: Assets,
    // pub ui: Ui,
    // pub scene: Scene,
    // pub camera: Camera,
    // pub physics: Physics,
    // pub network: Network,
    // pub debug: Debug,
    // pub console: Console,
    // pub profiler: Profiler,
    // pub settings: Settings,
    // pub plugins: Plugins,
}

impl App {
    pub fn new() -> Self {
        Self {
            // window: Window::new(),
            state: "new".to_string(),

        }
    }

    pub fn run(&mut self) {
        // self.window.run();        
        self.state = "running".to_string();
        while self.state == "running" {
            
            println!("App is running");
        }
    }
}