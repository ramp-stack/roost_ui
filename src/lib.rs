#![doc(html_logo_url = "https://raw.githubusercontent.com/ramp-stack/pelican_ui_std/main/logo.png")]

//! Pelican UI
//!
//! Pelican UI is a fast, cross-platform UI renderer and component system for building beautiful and consistent apps.
//!
//! Key features include:
//! - **Theme system**: Easily customize colors, fonts, and more through a central [`Theme`] object.
//! - **Simple components & layouts**: Build UIs quickly with a minimal and intuitive API.
//! - **High performance**: Optimized for speed, making it suitable for both lightweight and complex applications.
//! - **Cross-platform support**: Works seamlessly on desktop, web, and mobile platforms.
//! - **Standard components**: Access ready-to-use components via the [`pelican_ui_std`](<https://docs.rs/crate/pelican_ui_std/latest>) crate built on top of Pelican UI.
//!
//! Check out the [website](http://ramp-stack.com/pelican_ui) for more information, the [Quick Start Guide](http://ramp-stack.com/pelican_ui/getting_started) to set up your first app, and join the [community](https://discord.gg/cTRaRbUZ) if you have questions or want to share ideas.

use std::collections::BTreeMap;
use std::any::TypeId;
use std::sync::Arc;

use wgpu_canvas::{Atlas, Item as CanvasItem, Area};

use maverick_os::window::{Window, Event as WindowEvent, Lifetime};
pub use maverick_os::hardware::Context as HardwareContext;
use maverick_os::runtime::{Services, ServiceList};

pub use maverick_os::active_rusqlite;
pub use maverick_os::hardware;
pub use maverick_os::runtime;
pub use maverick_os::air;
pub use maverick_os::{MaverickOS, start as maverick_start, State};

pub use include_dir::include_dir as include_assets;

pub use pelican_ui_proc::Component;

use downcast_rs::{Downcast, impl_downcast};
use include_dir::{Dir, DirEntry};

#[cfg(target_os = "android")]
use maverick_os::AndroidApp;

mod wgpu;
use wgpu::Canvas;

pub mod events;
use events::{EventHandler, Events, Event, TickEvent};

pub mod layout;
use layout::{Scale, Scaling};

pub mod drawable;
use drawable::{Drawable, _Drawable, SizedBranch};

pub mod resources {
    pub use wgpu_canvas::{Image, Font};
}

mod theme;
pub use theme::{
    Theme,
    Illustrations,
    ColorResources,
    FontResources,
    IconResources,
    LayoutResources,
    ButtonColorScheme,
    BrandResources,
    BrandColor,
    TextColor,
    BackgroundColor,
    ButtonColors,
    OutlineColor,
    IllustrationColors,
    StatusColor,
    ShadesColor,
};

type PluginList = BTreeMap<TypeId, Box<dyn Plugin>>;

pub trait Plugin: Downcast {
    fn new(ctx: &mut Context) -> Self where Self: Sized;

    fn event(&mut self, _ctx: &mut Context, _event: &dyn Event) {}
}
impl_downcast!(Plugin); 

/// `Assets` stores all the assets required by your project, 
/// including images and fonts.
pub struct Assets {
    dirs: Vec<Dir<'static>>,
    atlas: Atlas,
}

impl Default for Assets {
    fn default() -> Self {
        Self::new()
    }
}

impl Assets {
    pub fn new() -> Self {
        Assets {
            dirs: Vec::new(),
            atlas: Atlas::default(),            
        } 
    }

    /// Returns a reference to a vector containing all included directories.
    pub fn dirs(&self) -> &Vec<Dir<'static>> {&self.dirs}
    /// Adds a font to the atlas from the provided byte slice and returns the loaded [`resources::Font`] resource.
    pub fn add_font(&mut self, font: &[u8]) -> resources::Font {self.atlas.add_font(font).unwrap()}
    /// Adds an image to the atlas from the provided [`image::RgbaImage`] and returns the loaded [`resources::Image`] resource.
    pub fn add_image(&mut self, image: image::RgbaImage) -> resources::Image {self.atlas.add_image(image)}
    /// Adds a svg image to the atlas from the provided byte slice and scale factor and returns the loaded [`resources::Image`] resource.
    pub fn add_svg(&mut self, svg: &[u8], scale: f32) -> resources::Image {
        let svg = std::str::from_utf8(svg).unwrap();
        let svg = nsvg::parse_str(svg, nsvg::Units::Pixel, 96.0).unwrap();
        let rgba = svg.rasterize(scale).unwrap();
        let size = rgba.dimensions();
        self.atlas.add_image(image::RgbaImage::from_raw(size.0, size.1, rgba.into_raw()).unwrap())
    }

    /// Loads a font from the given file path and returns an [`Option`] containing the [`resources::Font`] if successful.
    pub fn load_font(&mut self, file: &str) -> Option<resources::Font> {
        self.load_file(file).map(|b| self.add_font(&b))
    }

    /// Loads an image from the given file path and returns an [`Option`] containing the [`resources::Image`] if successful.
    pub fn load_image(&mut self, file: &str) -> Option<resources::Image> {
        self.load_file(file).map(|b|
            self.add_image(image::load_from_memory(&b).unwrap().into())
        )
    }

    /// Loads the contents of the specified file from the search directories, returning its bytes if found.
    pub fn load_file(&self, file: &str) -> Option<Vec<u8>> {
        self.dirs.iter().find_map(|dir|
            dir.find(file).ok().and_then(|mut f|
                f.next().and_then(|f|
                    if let DirEntry::File(f) = f {
                        Some(f.contents().to_vec())
                    } else {
                        None
                    }
                )
            )
        )
    }

    /// Adds a directory to the list of asset search paths.
    pub fn include_assets(&mut self, dir: Dir<'static>) {
        self.dirs.push(dir);
    }
}

pub struct PluginGuard<'a, P: Plugin>(Option<P>, &'a mut Context);
impl<'a, P: Plugin> PluginGuard<'a, P> {
    pub fn get(&mut self) -> (&mut P, &mut Context) {
        (self.0.as_mut().unwrap(), &mut *self.1)
    }
    pub fn run<T>(&mut self, clo: impl FnOnce(&mut P, &mut Context) -> T) -> T {
        clo(self.0.as_mut().unwrap(), self.1)
    }
}
impl<'a, P: Plugin> Drop for PluginGuard<'a, P> {
    fn drop(&mut self) {
        self.1.plugins.insert(TypeId::of::<P>(), Box::new(self.0.take().unwrap()));
    }
}

/// `Context` holds the app context, including hardware, runtime, assets, theme, plugins, events, and state.
pub struct Context {
    pub hardware: HardwareContext,
    pub runtime: runtime::Context,
    pub assets: Assets,
    pub theme: Theme,
    plugins: PluginList,
    events: Events,
    state: Option<State>
}

impl Context {
    /// Creates a new `Context` instance and loads the default Pelican UI assets.
    pub fn new(hardware: HardwareContext, runtime: runtime::Context, state: Option<State>) -> Self {
        let mut assets = Assets::new();
        assets.include_assets(include_assets!("./resources"));
        Context {
            hardware,
            runtime,
            theme: Theme::default(&mut assets),
            assets,  
            plugins: PluginList::new(),
            events: Events::new(),    
            state
        }
    }

    /// Adds an [`Event`] to the context's event queue to be triggered.
    pub fn trigger_event(&mut self, event: impl Event) {
        self.events.push_back(Box::new(event));
    }

    pub fn get<P: Plugin + 'static>(&mut self) -> PluginGuard<'_, P> {
        PluginGuard(Some(*self.plugins.remove(&TypeId::of::<P>())
            .unwrap_or_else(|| panic!("Plugin Not Configured: {:?}", std::any::type_name::<P>()))
            .downcast().ok().unwrap()), self)
    }

    /// Returns a mutable reference to the [`State`]
    pub fn state(&mut self) -> &mut State {
        self.state.as_mut().unwrap()
    }

  //pub fn state(&mut self) -> &mut State {
  //    self.base_context.state()
  //}


  //pub fn add_font(&mut self, font: &[u8]) -> canvas::Font {
  //    self.base_context.as_mut().add_font(font)
  //}

  //pub fn add_image(&mut self, image: image::RgbaImage) -> canvas::Image {
  //    self.base_context.as_mut().add_image(image)
  //}

  //pub fn add_svg(&mut self, svg: &[u8], quality: f32) -> canvas::Image {
  //    self.base_context.as_mut().add_svg(svg, quality)
  //}



    // pub fn as_canvas(&mut self) -> &mut FontAtlas {
    //     self.as_mut()
    // }
}

/// A trait for registering plugins.
///
/// Implementors should return a collection of plugins to be stored in the [`Context`].
///
/// # Example
/// ```
/// struct MyPlugin;
/// impl Plugin for MyPlugin { /* ... */ }
///
/// struct MyApp;
/// impl Plugins for MyApp {
///     fn plugins(ctx: &mut Context) -> Vec<Box<dyn Plugin>> {
///         vec![Box::new(MyPlugin)]
///     }
/// }
/// ```
pub trait Plugins {
    /// Returns a list of plugins for the application.
    fn plugins(ctx: &mut Context) -> Vec<Box<dyn Plugin>>;
}

/// Allow [`Context`] to provide mutable access to the [`Atlas`].
impl AsMut<Atlas> for Context {fn as_mut(&mut self) -> &mut Atlas {&mut self.assets.atlas}}

/// The core application trait.
///
/// An `Application` provides services, registers plugins, and defines
/// the entrypoint for creating the root [`Drawable`] of the app.
///
/// # Example
/// ```
/// struct MyApp;
/// impl Services for MyApp {
///     fn services() -> ServiceList { ServiceList::new() }
/// }
///
/// impl Plugins for MyApp {
///     fn plugins(ctx: &mut Context) -> Vec<Box<dyn Plugin>> { vec![] }
/// }
///
/// impl Application for MyApp {
///     async fn new(ctx: &mut Context) -> Box<dyn Drawable> {
///         Box::new(MyRootDrawable::new())
///     }
/// }
/// ```
pub trait Application: Services + Plugins {
    fn new(ctx: &mut Context) -> impl Future<Output = Box<dyn Drawable>>;
}

/// Provide [`Services`] for [`PelicanEngine`] by deferring to the application type.
impl<A: Application> Services for PelicanEngine<A> {
    fn services() -> ServiceList { A::services() }
}


/// The main engine type.
///
/// `PelicanEngine` wires together the [`Application`], windowing system,
/// plugin management, drawing, and event handling.
pub struct PelicanEngine<A: Application> {
    _p: std::marker::PhantomData<A>,
    scale: Scale,
    canvas: Canvas,
    screen: (f32, f32),
    context: Context,
    sized_app: SizedBranch,
    application: Box<dyn Drawable>,
    event_handler: EventHandler,
    items: Vec<(Area, CanvasItem)>,
}

impl<A: Application> maverick_os::Application for PelicanEngine<A> {
    /// Initializes the engine with the given MaverickOS context.
    ///
    /// - Creates a canvas bound to the OS window.
    /// - Constructs an application via [`Application::new`].
    /// - Registers plugins returned by [`Application::plugins`].
    async fn new(ctx: &mut maverick_os::Context) -> Self {
        ctx.hardware.register_notifs();
        let size = ctx.window.size;
        let (canvas, size) = Canvas::new(ctx.window.handle.clone(), size.0, size.1).await;
        let scale = Scale(ctx.window.scale_factor);
        let screen = (scale.logical(size.0 as f32), scale.logical(size.1 as f32));
        let mut context = Context::new(ctx.hardware.clone(), ctx.runtime.clone(), ctx.state.take());
        let plugins = A::plugins(&mut context);
        context.plugins = plugins.into_iter().map(|p| ((*p).type_id(), p)).collect();
        let mut application = A::new(&mut context).await;
        let size_request = _Drawable::request_size(&*application, &mut context);
        let sized_app = application.build(&mut context, screen, size_request);
        ctx.state = context.state.take();
        PelicanEngine{
            _p: std::marker::PhantomData::<A>,
            scale,
            canvas,
            screen,
            context,
            sized_app,
            application,
            event_handler: EventHandler::new(),
            items: Vec::new()
        }
    }
        
    async fn on_event(&mut self, context: &mut maverick_os::Context, event: WindowEvent) {
        self.context.state = context.state.take();
        match event {
            WindowEvent::Lifetime(lifetime) => match lifetime {
                Lifetime::Resized => {
                    self.scale.0 = context.window.scale_factor;
                    let size = context.window.size;
                    let size = self.canvas.resize::<Arc<Window>>(None, size.0, size.1);
                    let size = (self.scale.logical(size.0 as f32), self.scale.logical(size.1 as f32));
                    self.screen = size;
                },
                Lifetime::Resumed => {
                    let _ = self.items.drain(..);
                    self.scale.0 = context.window.scale_factor;
                    let size = context.window.size;
                    let size = self.canvas.resize(Some(context.window.handle.clone()), size.0, size.1);
                    let size = (self.scale.logical(size.0 as f32), self.scale.logical(size.1 as f32));
                    self.screen = size;
                },
                Lifetime::Paused => {},
                Lifetime::Close => {},
                Lifetime::Draw => {//Size before events because the events are given between
                                   //resizing

                    let result = self.event_handler.on_input(&self.scale, maverick_os::window::Input::Tick);
                    if let Some(event) = result {
                        self.context.events.push_back(event);
                    }
                    self.application.event(&mut self.context, self.sized_app.clone(), Box::new(TickEvent));

                    while let Some(event) = self.context.events.pop_front() {
                        if let Some(event) = event
                            .pass(&mut self.context, vec![((0.0, 0.0), self.sized_app.0)])
                            .remove(0)
                        {
                            for id in self.context.plugins.keys().copied().collect::<Vec<_>>() {
                                let mut plugin = self.context.plugins.remove(&id).unwrap();
                                plugin.event(&mut self.context, &*event);    
                                self.context.plugins.insert(id, plugin);
                            }
                            self.application.event(&mut self.context, self.sized_app.clone(), event);
                        }
                    }

                    let size_request = _Drawable::request_size(&*self.application, &mut self.context);
                    self.sized_app = self.application.build(&mut self.context, self.screen, size_request);
                    let drawn = self.application.draw(self.sized_app.clone(), (0.0, 0.0), (0.0, 0.0, self.screen.0, self.screen.1));
                    let items: Vec<_> = drawn.into_iter().map(|(a, i)| (a.scale(&self.scale), i.scale(&self.scale))).collect();
                    if self.items != items {
                        self.items = items.clone();
                        self.canvas.draw(&mut self.context.assets.atlas, items);
                    }
                },
                Lifetime::MemoryWarning => {},
            },
            WindowEvent::Input(input) => {if let Some(event) = self.event_handler.on_input(&self.scale, input) {self.context.events.push_back(event)}}
        }
        context.state = self.context.state.take();
    }
}




#[macro_export]
macro_rules! start {
    ($app:ty) => {
        maverick_start!(PelicanEngine<$app>);
    };
}
