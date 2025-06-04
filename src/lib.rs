use std::collections::BTreeMap;
use std::sync::Arc;
use std::any::TypeId;

use wgpu_canvas::{ImageAtlas, FontAtlas};

use maverick_os::window::{Window, Event as WindowEvent, Lifetime};
use maverick_os::hardware::Context as HardwareContext;

pub use maverick_os::runtime::{BackgroundTask, Services, ServiceList, Service};
pub use maverick_os::{MaverickOS, start as maverick_start};

pub use pelican_ui_proc::Component;

use downcast_rs::{Downcast, impl_downcast};

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

pub type Plugins = BTreeMap<TypeId, Box<dyn Plugin>>;

pub trait Plugin: Downcast {
    fn new(ctx: &mut Context) -> Self where Self: Sized;

    fn event(&mut self, _ctx: &mut Context, _event: &dyn Event) {}
}
impl_downcast!(Plugin); 

pub struct Context {
    pub hardware: HardwareContext,
    plugins: Plugins,
  //assets: Assets,
    events: Events,
    image: ImageAtlas,
    font: FontAtlas,
}

impl Context {
    pub fn new(hardware: HardwareContext) -> Self {
        Context {
            hardware,
            plugins: Plugins::new(),
          //assets: Assets::new(),
            events: Events::new(),
            image: ImageAtlas::default(),            
            font: FontAtlas::default(),            
        }
    }

    pub fn add_font(&mut self, font: &[u8]) -> resources::Font {self.font.add(font)}
    pub fn add_image(&mut self, image: image::RgbaImage) -> resources::Image {self.image.add(image)}
    pub fn add_svg(&mut self, svg: &[u8], scale: f32) -> resources::Image {
        let svg = std::str::from_utf8(svg).unwrap();
        let svg = nsvg::parse_str(svg, nsvg::Units::Pixel, 96.0).unwrap();
        let rgba = svg.rasterize(scale).unwrap();
        let size = rgba.dimensions();
        self.image.add(image::RgbaImage::from_raw(size.0, size.1, rgba.into_raw()).unwrap())
    }

  //pub fn trigger_event(&mut self, event: impl Event) {
  //    self.events.push_back(Box::new(event));
  //}

    pub fn get<P: Plugin + 'static>(&mut self) -> &mut P {
        self.plugins.get_mut(&TypeId::of::<P>())
            .unwrap_or_else(|| panic!("Plugin Not Configured: {:?}", std::any::type_name::<P>()))
            .downcast_mut().unwrap()
    }

  //pub fn state(&mut self) -> &mut State {
  //    self.base_context.state()
  //}

  //pub fn include_assets(&mut self, dir: Dir<'static>) {
  //    self.assets.push(dir);
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

  //pub fn load_font(&mut self, file: &str) -> Option<canvas::Font> {
  //    self.load_file(file).map(|b| self.add_font(&b))
  //}

  //pub fn load_image(&mut self, file: &str) -> Option<canvas::Image> {
  //    self.load_file(file).map(|b|
  //        self.add_image(image::load_from_memory(&b).unwrap().into())
  //    )
  //}

  //pub fn load_file(&self, file: &str) -> Option<Vec<u8>> {
  //    self.assets.iter().find_map(|dir|
  //        dir.find(file).ok().and_then(|mut f|
  //            f.next().and_then(|f|
  //                if let DirEntry::File(f) = f {
  //                    Some(f.contents().to_vec())
  //                } else {
  //                    None
  //                }
  //            )
  //        )
  //    )
  //}

  //pub fn as_canvas(&mut self) -> &mut CanvasContext {
  //    self.as_mut()
  //}
}
impl AsMut<FontAtlas> for Context {fn as_mut(&mut self) -> &mut FontAtlas {&mut self.font}}
impl AsMut<ImageAtlas> for Context {fn as_mut(&mut self) -> &mut ImageAtlas {&mut self.image}}

pub trait Application: Services {
    fn new(ctx: &mut Context) -> impl Future<Output = Box<dyn Drawable>>;
}

pub struct PelicanEngine<A: Application> {
    _p: std::marker::PhantomData<A>,
    scale: Scale,
    canvas: Canvas,
    screen: (f32, f32),
    context: Context,
    sized_app: SizedBranch,
    application: Box<dyn Drawable>,
    event_handler: EventHandler
}
impl<A: Application> Services for PelicanEngine<A> {
    fn services() -> ServiceList {A::services()}
}
impl<A: Application> maverick_os::Application for PelicanEngine<A> {
    async fn new(context: &mut maverick_os::Context) -> Self {
        let size = context.window.size;
        let (canvas, size) = Canvas::new(context.window.handle.clone(), size.0, size.1).await;
        let scale = Scale(context.window.scale_factor);
        let screen = (scale.logical(size.0 as f32), scale.logical(size.1 as f32));
        let mut context = Context::new(context.hardware.clone());
        let mut application = A::new(&mut context).await;
        let size_request = _Drawable::request_size(&*application, &mut context);
        let sized_app = application.build(&mut context, screen, size_request);
        PelicanEngine{
            _p: std::marker::PhantomData::<A>,
            scale,
            canvas,
            screen,
            context,
            sized_app,
            application,
            event_handler: EventHandler::new()
        }
    }
        
    async fn on_event(&mut self, context: &mut maverick_os::Context, event: WindowEvent) {
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
                    let items = self.application.draw(
                        self.sized_app.clone(), (0.0, 0.0), (0.0, 0.0, self.screen.0, self.screen.1),
                    ).into_iter().map(|(a, i)| (a.scale(&self.scale), i.scale(&self.scale))).collect();
                    self.canvas.draw(&mut self.context.image, &mut self.context.font, items);
                },
                Lifetime::MemoryWarning => {},
            },
            WindowEvent::Input(input) => {if let Some(event) = self.event_handler.on_input(&self.scale, input) {self.context.events.push_back(event)}}
        }
    }
}


#[macro_export]
macro_rules! start {
    ($app:ty) => {
        maverick_start!(PelicanEngine<$app>);
    };
}
