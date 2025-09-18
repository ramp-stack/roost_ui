pub use include_dir;
pub use include_dir::include_dir as include_assets;
pub use proc::{Component, Plugin};

use std::any::TypeId;
use std::collections::HashMap;
use std::future::Future;
use std::time::Instant;

use crate::base;
use base::{BaseAppTrait, HeadlessContext};
use base::driver::runtime::Tasks;
use base::driver::state::State;

use base::renderer::wgpu_canvas as canvas;
pub use canvas::Canvas;
use canvas::Context as CanvasContext;

use include_dir::{Dir, DirEntry};

mod events;
pub use events::{
    Events, OnEvent, Event, TickEvent,
    MouseEvent, MouseState,
    KeyboardEvent, KeyboardState,
    NamedKey, Key, SmolStr,
};

mod sizing;
pub use sizing::{Layout, SizeRequest, DefaultStack, Area};

mod drawable;
pub use drawable::{
    Component, Text, Font, Span, Cursor, CursorAction,
    Align, Image, Shape, RequestBranch, SizedBranch,
    Drawable, ShapeType, Color,
};
use drawable::_Drawable;

pub type Assets = Vec<Dir<'static>>;

pub struct Context {
    plugins: Plugins,
    assets: Assets,
    events: Events,
    base_context: base::Context<Canvas>,
}

impl Context {
    pub fn new(base_context: base::Context<Canvas>) -> Self {
        Context {
            plugins: Plugins::new(),
            assets: Assets::new(),
            events: Events::new(),
            base_context,
        }
    }

    /// Adds an event to the event queue.
    ///
    ///```rust
    /// ctx.trigger_event(MyEvent);
    ///```
    pub fn trigger_event(&mut self, event: impl Event) {
        self.events.push_back(Box::new(event));
    }

    pub fn get<P: Plugin + 'static>(&mut self) -> &mut P {
        self.plugins.get_mut(&TypeId::of::<P>())
            .unwrap_or_else(|| panic!("Plugin Not Configured: {:?}", std::any::type_name::<P>()))
            .downcast_mut().unwrap()
    }

    /// Returns a mutable reference to the state.
    ///```rust
    /// ctx.state().get::<MyStoredFiles>().map(|files| println!("Files Found {:?}", files));
    ///```
    pub fn state(&mut self) -> &mut State {
        self.base_context.state()
    }

    pub fn include_assets(&mut self, dir: Dir<'static>) {
        self.assets.push(dir);
    }

    pub fn add_font(&mut self, font: &[u8]) -> canvas::Font {
        self.base_context.as_mut().add_font(font)
    }

    pub fn add_image(&mut self, image: image::RgbaImage) -> canvas::Image {
        self.base_context.as_mut().add_image(image)
    }

    pub fn add_svg(&mut self, svg: &[u8], quality: f32) -> canvas::Image {
        self.base_context.as_mut().add_svg(svg, quality)
    }

    pub fn load_font(&mut self, file: &str) -> Option<canvas::Font> {
        self.load_file(file).map(|b| self.add_font(&b))
    }

    pub fn load_image(&mut self, file: &str) -> Option<canvas::Image> {
        self.load_file(file).map(|b|
            self.add_image(image::load_from_memory(&b).unwrap().into())
        )
    }

    pub fn load_file(&self, file: &str) -> Option<Vec<u8>> {
        self.assets.iter().find_map(|dir|
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

    pub fn as_canvas(&mut self) -> &mut CanvasContext {
        self.as_mut()
    }
}

impl AsMut<CanvasContext> for Context {
    fn as_mut(&mut self) -> &mut CanvasContext {self.base_context.as_mut()}
}

impl AsMut<wgpu_canvas::FontAtlas> for Context {
    fn as_mut(&mut self) -> &mut wgpu_canvas::FontAtlas {self.base_context.as_mut().as_mut()}
}

pub trait Plugin {
    fn background_tasks(_ctx: &mut HeadlessContext) -> impl Future<Output = Tasks> {
        async { vec![] }
    }
    
    fn new(
        ctx: &mut Context,
        h_ctx: &mut HeadlessContext,
    ) -> impl Future<Output = (Self, Tasks)>
    where
        Self: Sized;
}

pub type Plugins = HashMap<TypeId, Box<dyn std::any::Any>>;
pub trait App {
    fn background_tasks(_ctx: &mut HeadlessContext) -> impl Future<Output = Tasks> {
        async { vec![] }
    }

    fn plugins(
        _ctx: &mut Context,
        _h_ctx: &mut HeadlessContext,
    ) -> impl Future<Output = (Plugins, Tasks)> {
        async { (HashMap::new(), vec![]) }
    }

    fn new(ctx: &mut Context) -> impl Future<Output = Box<dyn Drawable>>;
}

pub struct ComponentApp<A: App> {
    ctx: Context,
    app: Box<dyn Drawable>,
    screen: (f32, f32),
    sized_app: SizedBranch,
    _p: std::marker::PhantomData<A>,
    time: Instant,
}

impl<A: App> BaseAppTrait<Canvas> for ComponentApp<A> {
    const LOG_LEVEL: log::Level = log::Level::Error;

    async fn background_tasks(ctx: &mut HeadlessContext) -> Tasks {
        A::background_tasks(ctx).await
    }

    async fn new(
        base_ctx: base::Context<Canvas>,
        h_ctx: &mut HeadlessContext,
        width: f32,
        height: f32,
    ) -> (Self, Tasks) {
        let mut ctx = Context::new(base_ctx);
        let (plugins, tasks) = A::plugins(&mut ctx, h_ctx).await;
        ctx.plugins = plugins;

        let mut app = A::new(&mut ctx).await;
        let size_request = _Drawable::request_size(&*app, &mut ctx);
        let screen = (width, height);
        let sized_app = app.build(&mut ctx, screen, size_request);

        (
            ComponentApp {
                ctx,
                app,
                screen,
                sized_app,
                _p: std::marker::PhantomData::<A>,
                time: Instant::now(),
            },
            tasks,
        )
    }

    fn on_event(&mut self, event: canvas::Event) {
        match event {
            canvas::Event::Resized { width, height }
            | canvas::Event::Resumed { width, height } => {
                self.screen = (width, height);
            }
            canvas::Event::Mouse { position, state } => {
                self.ctx
                    .events
                    .push_back(Box::new(MouseEvent { position: Some(position), state }));
            }
            canvas::Event::Keyboard { key, state } => {
                self.ctx
                    .events
                    .push_back(Box::new(KeyboardEvent { key, state }));
            }
            canvas::Event::Tick => {
                log::error!("last_frame: {:?}", self.time.elapsed());
                self.time = Instant::now();

                self.app.event(&mut self.ctx, self.sized_app.clone(), Box::new(TickEvent));

                while let Some(event) = self.ctx.events.pop_front() {
                    if let Some(event) = event
                        .pass(&mut self.ctx, vec![((0.0, 0.0), self.sized_app.0)])
                        .remove(0)
                    {
                        self.app
                            .event(&mut self.ctx, self.sized_app.clone(), event);
                    }
                }

                let size_request = _Drawable::request_size(&*self.app, &mut self.ctx);
                self.sized_app =
                    self.app.build(&mut self.ctx, self.screen, size_request);

                self.app.draw(
                    &mut self.ctx,
                    self.sized_app.clone(),
                    (0.0, 0.0),
                    (0.0, 0.0, self.screen.0, self.screen.1),
                );
            }
            _ => {}
        }
    }

    async fn close(self) -> base::Context<Canvas> {
        self.ctx.base_context
    }

    fn ctx(&mut self) -> &mut base::Context<Canvas> {
        &mut self.ctx.base_context
    }
}


#[macro_export]
macro_rules! create_entry_points {
    ($app:ty) => {
        create_base_entry_points!(Canvas, ComponentApp::<$app>);
    };
}

use wgpu_canvas::{ImageAtlas, FontAtlas};

use maverick_os::window::{Window, Event, Lifetime, Input};
use maverick_os::Application;


pub use wgpu_canvas::{Shape, Color, Area, Text, Span, Cursor, CursorAction, Align, Font, Image};
pub use crate::base::window::{MouseState, KeyboardState, NamedKey, SmolStr, Key};



#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Resized{width: f32, height: f32},
    Mouse{position: (f32, f32), state: MouseState},
    Keyboard{key: Key, state: KeyboardState},
    Resumed{width: f32, height: f32},
    Paused,
    Tick
}

pub struct Context{
    scale: Scale,
    image: ImageAtlas,
    font: FontAtlas,
    context: Vec<(Area, wgpu_canvas::CanvasItem)>,
    size: (f32, f32)
}
impl Context {
    pub fn add_font(&mut self, font: &[u8]) -> Font {self.font.add(font)}
    pub fn add_image(&mut self, image: image::RgbaImage) -> Image {self.image.add(image)}
    pub fn add_svg(&mut self, svg: &[u8], scale: f32) -> Image {
        let svg = std::str::from_utf8(svg).unwrap();
        let svg = nsvg::parse_str(svg, nsvg::Units::Pixel, 96.0).unwrap();
        let rgba = svg.rasterize(scale).unwrap();
        let size = rgba.dimensions();
        self.image.add(image::RgbaImage::from_raw(size.0, size.1, rgba.into_raw()).unwrap())
    }
    pub fn size(&self) -> (f32, f32) {self.size}
    pub fn draw(&mut self, area: Area, item: CanvasItem) {
        let area = Area(
            (self.scale.physical(area.0.0), self.scale.physical(area.0.1)),
            area.1.map(|(x, y, w, h)| (
                self.scale.physical(x), self.scale.physical(y),
                self.scale.physical(w), self.scale.physical(h)
            ))
        );
        self.components.push((area, item.scale(&self.scale)));
    }

    pub fn clear(&mut self, color: Color) {
        self.components.clear();
        self.components.push((Area((0.0, 0.0), None),
            wgpu_canvas::CanvasItem::Shape(Shape::Rectangle(0.0,
                (self.scale.physical(self.size.0), self.scale.physical(self.size.1))
            ), color)
        ));
    }
}

#[derive(Clone, Debug)]
pub enum CanvasItem {
    Shape(Shape, Color),
    Image(Shape, Image, Option<Color>),
    Text(Text),
}

impl CanvasItem {
    fn scale(self, scale: &Scale) -> wgpu_canvas::CanvasItem {
        match self {
            CanvasItem::Shape(shape, color) => wgpu_canvas::CanvasItem::Shape(
                Self::scale_shape(shape, scale), color
            ),
            CanvasItem::Image(shape, image, color) => wgpu_canvas::CanvasItem::Image(
                Self::scale_shape(shape, scale), image, color
            ),
            CanvasItem::Text(text) => wgpu_canvas::CanvasItem::Text(Self::scale_text(text, scale))
        }
    }

    fn scale_text(text: Text, scale: &Scale) -> Text {
        Text::new(
            text.spans.into_iter().map(|s|
                Span::new(s.text, scale.physical(s.font_size), scale.physical(s.line_height), s.font, s.color)
            ).collect(),
            text.width.map(|w| scale.physical(w)),
            text.align,
            text.cursor,
        )
    }

    fn scale_shape(shape: Shape, scale: &Scale) -> Shape {
        match shape {
            Shape::Ellipse(s, size) => Shape::Ellipse(scale.physical(s), Self::scale_size(size, scale)),
            Shape::Rectangle(s, size) => Shape::Rectangle(scale.physical(s), Self::scale_size(size, scale)),
            Shape::RoundedRectangle(s, size, r) => Shape::RoundedRectangle(
                scale.physical(s), Self::scale_size(size, scale), scale.physical(r)
            ),
        }
    }

    fn scale_size(size: (f32, f32), scale: &Scale) -> (f32, f32) {
        (scale.physical(size.0), scale.physical(size.1))
    }
}

mod wgpu;
pub use wgpu::Canvas;

impl Renderer for Canvas {
    async fn new(context: maverick_os::Context) -> Self {
        let (canvas, size) = Self::inner_new(window, width, height).await;
        let scale = Scale(scale_factor);
        let size = (scale.logical(size.0 as f32), scale.logical(size.1 as f32));
        let ctx = Context{scale, image: ImageAtlas::default(), font: FontAtlas::default(), components: Vec::new(), size};
        (canvas, ctx, size)
    }
        
    async fn on_event(&mut self, event: Event) {
        let ctx = app.ctx();
        let draw =  matches!(event, WindowEvent::Tick);
        let r_event = match event {
            WindowEvent::Resized{width, height, scale_factor} => {
                ctx.scale.0 = scale_factor;
                let size = self.resize::<W>(None, width, height);
                let size = (ctx.scale.logical(size.0 as f32), ctx.scale.logical(size.1 as f32));
                ctx.size = size;
                Event::Resized{width: size.0, height: size.1}
            },
            WindowEvent::Mouse{position, state} => {
                Event::Mouse{position: (
                    ctx.scale.logical(position.0 as f32), ctx.scale.logical(position.1 as f32)
                ), state}
            }
            WindowEvent::Keyboard{key, state} => Event::Keyboard{key, state},
            WindowEvent::Resumed{window, width, height, scale_factor} => {
                ctx.scale.0 = scale_factor;
                let size = self.resize(Some(window.into()), width, height);
                let size = (ctx.scale.logical(size.0 as f32), ctx.scale.logical(size.1 as f32));
                ctx.size = size;
                Event::Resumed{width: size.0, height: size.1}
            },
            WindowEvent::Paused => Event::Paused,
            WindowEvent::Tick => Event::Tick
        };
        app.on_event(r_event).await;
        let ctx = app.ctx();
        if draw {self.draw(&mut ctx.image, &mut ctx.font, ctx.components.drain(..).collect::<Vec<_>>());}
    }

    async fn close(self, _ctx: Self::Context) {}
}

