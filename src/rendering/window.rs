//! This file contains all our windowing functions to simplify interfacing with the window
//! it should make it easier to access input, modify the window and access window variables
//! for the user of the library and the developer

use winit::{dpi, event_loop::{self, EventLoop}, monitor, window};

#[cfg(target_os = "linux")]
use winit::platform::unix::EventLoopExtUnix;

#[cfg(target_os = "macos")]
use winit::platform::unix::EventLoopExtUnix;

#[cfg(target_os = "windows")]
use winit::platform::windows::EventLoopExtWindows;


use winit::event::Event;

/// # Window
///
/// This struct contains information for the window used in a GUI application
///
/// It is designed to be used to abstract away from some of the low-levelness of winit
/// and create a simpler, although less powerful API to window functions
/// 
/// ## Usage
///
/// This struct should be made using a window builder
/// 
/// Once the window is build, set the event handler using `set_event_handler`
pub struct Window{
    pub window: window::Window,
    pub event_loop: Option<event_loop::EventLoop<()>>,
    pub event_callback_handler: Option<Box<dyn Fn(&Event<()>, &mut window::Window, &mut crate::rendering::Renderer) -> ()>>,
}


impl Window{
    /// The default event callback handler.
    ///
    /// You can define your own to handle events
    ///
    /// Button presses will still be automatically handled.
    pub fn default_event_callback(event: &Event<()>, _window: &mut window::Window, _renderer: &mut crate::rendering::Renderer){
        println!("Event: {:?}", event);
    }

    /// Sets the event callback handler. This cannot be changed once the GUI is running.
    pub fn set_event_handler(&mut self, event_handler: Box<dyn Fn(&Event<()>, &mut window::Window, &mut crate::rendering::Renderer) -> ()>){
        self.event_callback_handler = Some(event_handler);
    }
}

/// # WindowBuilder
/// 
/// This builds a window struct, based either on default values or
/// user defined values. Meant to simplify and abstract winit's WindowBuilder,
/// for ease of use when making GUI applications.
#[derive(Debug)]
pub struct WindowBuilder{
    resolution: (u32, u32),
    title: String,
    vsync: bool,
    screen_mode: ScreenMode,
    resizeable: bool,
    decorations: bool,
}

/// Default init for WindowBuilder
impl Default for WindowBuilder{
    fn default() -> WindowBuilder{
        Self{
            resolution: (800, 600),
            title: String::from("Rusty GUI"),
            vsync: true,
            screen_mode: ScreenMode::Windowed,
            resizeable: true,
            decorations: true,
            
        }
    }
}

/// Helpful functions to define variables for a window
impl WindowBuilder{
    /// Create a new window builder with default values
    pub fn new() -> Self{
        Self::default()
    }

    /// Set the resolution for the window (Can be any u32, but be reasonable :D)
    pub fn set_resolution(&mut self, resolution: (u32, u32)) -> &mut Self{
        self.resolution = resolution;
        self
    }

    /// Set Vsync mode (can be true or false)
    pub fn set_vsync(&mut self, vsync_enabled: bool) -> &mut Self{
        self.vsync = vsync_enabled;
        self
    }

    /// Set the title of the window - can be any type that is a String
    pub fn set_title<S: Into<String>>(&mut self, title: S) -> &mut Self{
        self.title = title.into();
        self
    }

    /// Set the fullscreen to true or false - make this enum (FULL, BORDERLESS, WINDOW)
    pub fn set_screenmode(&mut self, screen_mode: ScreenMode) -> &mut Self{
        self.screen_mode = screen_mode;
        self
    }

    /// Enable or disable decorations (the bar at the top of the window)
    pub fn set_decorations(&mut self, decorations_enabled: bool) -> &mut Self{
        self.decorations = decorations_enabled;
        self
    }

    /// Enable or disable resizing
    pub fn set_resizeable(&mut self, resizable: bool) -> &mut Self{
        self.resizeable = resizable;
        self
    }

    /// Build the window and return a Window
    pub fn build(&mut self) -> Result<Window, &'static str>{
        // Create our winit WindowBuilder
        let winit_builder = window::WindowBuilder::new();

                
        // Create an event loop
        let mut event_loop = event_loop::EventLoop::new();
        
  
        // Gather information about the monitor and video modes for fullscreen and stuff
        let mut x = 0;
        let mut monitor: Vec<monitor::MonitorHandle> = event_loop.available_monitors().filter(|_| if x == 0 { x += 1; true }else{ false }).collect();
        let monitor = monitor.swap_remove(0);
        
        let mut x = 0;
        let mut video_modes: Vec<monitor::VideoMode> = monitor.video_modes().filter(|_| if x == 0 { x += 1; true }else{ false }).collect();
        let video_modes = video_modes.swap_remove(0);

        // Vsync mode - refresh rate
        let _vsync_mode = match self.vsync{
            true => {
                wgpu::PresentMode::Fifo
            }
            false => {
                wgpu::PresentMode::Mailbox
            }
        };

        // Check if we're running fullscreen and/or set resolutions
        let winit_builder = match self.screen_mode{
            ScreenMode::Fullscreen => {
                winit_builder.with_fullscreen(Some(window::Fullscreen::Exclusive(video_modes)))
            }
            ScreenMode::Windowed => {
                winit_builder.with_inner_size(dpi::Size::from(dpi::LogicalSize{ width: self.resolution.0, height: self.resolution.1}))
            }
            ScreenMode::Borderless => {
                winit_builder.with_fullscreen(Some(window::Fullscreen::Borderless(Some(monitor))))
            }
        };

        
        // Build the window
        Ok(Window{
            window: winit_builder.with_resizable(self.resizeable).with_decorations(self.decorations).with_title(&self.title).build(&mut event_loop).expect("Failed to build window!"),
            event_loop: Some(event_loop),
            event_callback_handler: Some(Box::new(Window::default_event_callback)),
        })
        
    }

    pub unsafe fn build_unsafe(&mut self) -> Result<Window, &'static str>{
        // Create our winit WindowBuilder
        let winit_builder = window::WindowBuilder::new();

        let mut event_loop: EventLoop<()> = build_unsafe_event_loop(); // Build a new event loop that can run on other threads (ie, multithreading support)
        
  
        // Gather information about the monitor and video modes for fullscreen and stuff
        let mut x = 0;
        let mut monitor: Vec<monitor::MonitorHandle> = event_loop.available_monitors().filter(|_| if x == 0 { x += 1; true }else{ false }).collect();
        let monitor = monitor.swap_remove(0);
        
        let mut x = 0;
        let mut video_modes: Vec<monitor::VideoMode> = monitor.video_modes().filter(|_| if x == 0 { x += 1; true }else{ false }).collect();
        let video_modes = video_modes.swap_remove(0);

        // Vsync mode - refresh rate
        let _vsync_mode = match self.vsync{
            true => {
                wgpu::PresentMode::Fifo
            }
            false => {
                wgpu::PresentMode::Mailbox
            }
        };

        // Check if we're running fullscreen and/or set resolutions
        let winit_builder = match self.screen_mode{
            ScreenMode::Fullscreen => {
                winit_builder.with_fullscreen(Some(window::Fullscreen::Exclusive(video_modes)))
            }
            ScreenMode::Windowed => {
                winit_builder.with_inner_size(dpi::Size::from(dpi::LogicalSize{ width: self.resolution.0, height: self.resolution.1}))
            }
            ScreenMode::Borderless => {
                winit_builder.with_fullscreen(Some(window::Fullscreen::Borderless(Some(monitor))))
            }
        };

        
        // Build the window
        Ok(Window{
            window: winit_builder.with_resizable(self.resizeable).with_decorations(self.decorations).with_title(&self.title).build(&mut event_loop).expect("Failed to build window!"),
            event_loop: Some(event_loop),
            event_callback_handler: Some(Box::new(Window::default_event_callback)),
        })
        
    }
}

#[cfg(target_os = "linux")]
unsafe fn build_unsafe_event_loop() -> EventLoop<()>{
    EventLoopExtUnix::new_any_thread()
}

#[cfg(target_os = "macos")]
unsafe fn build_unsafe_event_loop() -> EventLoop<()>{
    EventLoopExtUnix::new_any_thread()
}

#[cfg(target_os = "windows")]
unsafe fn build_unsafe_event_loop() -> EventLoop<()>{
    EventLoopExtWindows::new_any_thread()
}

#[derive(Debug)]
pub enum ScreenMode{
    Fullscreen,
    Borderless,
    Windowed
}