use pixels::Pixels;
use wasmtime::*;
use std::{io::Read, sync::Mutex, collections::HashMap};

use crate::sdl_system::SDLSystem;

mod sdl_system;

const SCREEN_WIDTH: u32 = 1024 + 256;
const SCREEN_HEIGHT: u32 = 480;

struct McState {
    turtle_x: i32,
    turtle_y: i32,
    turtle_z: i32,
    pixels: Pixels,
    blocks: HashMap<(i32, i32, i32), i32>,
}

impl McState {
    pub fn set(&mut self, block: i32) {
        self.set_at(block, self.turtle_x, self.turtle_y, self.turtle_z);
    }

    pub fn set_at(&mut self, block: i32, x: i32, y: i32, z: i32) {
        self.blocks.insert((x, y, z), block);

        if x >= 0 && y >= 0 && z == -60 {
            let index = 4 * ((500 - x as u32) + (300 - y) as u32 * SCREEN_WIDTH);

            let pixel = &mut self.pixels.get_frame()[index as usize..][..4];
            match block {
                0  /* Air      */ => pixel.copy_from_slice(&[0x00, 0x00, 0x00, 0xFF]),
                1  /* Cobble   */ => pixel.copy_from_slice(&[0x64, 0x64, 0x64, 0xFF]),
                2  /* Granite  */ => pixel.copy_from_slice(&[0x7A, 0x55, 0x48, 0xFF]),
                3  /* Andesite */ => pixel.copy_from_slice(&[0x69, 0x69, 0x69, 0xFF]),
                4  /* Diorite  */ => pixel.copy_from_slice(&[0x9A, 0x9A, 0x9B, 0xFF]),
                5  /* Lapis    */ => pixel.copy_from_slice(&[0x00, 0x00, 0xFF, 0xFF]),
                6  /* Iron     */ => pixel.copy_from_slice(&[0xFF, 0xFF, 0xFF, 0xFF]),
                7  /* Gold     */ => pixel.copy_from_slice(&[0xBB, 0xA0, 0x37, 0xFF]),
                8  /* Diamond  */ => pixel.copy_from_slice(&[0x5A, 0xB1, 0xCA, 0xFF]),
                9  /* Redstone */ => pixel.copy_from_slice(&[0x8B, 0x13, 0x04, 0xFF]),
                10 /* Emerald  */ => pixel.copy_from_slice(&[0x00, 0xFF, 0x00, 0xFF]),
                11 /* Dirt     */ => pixel.copy_from_slice(&[0x69, 0x2D, 0x00, 0xFF]),
                12 /* Oak Log  */ => pixel.copy_from_slice(&[0x5D, 0x49, 0x2B, 0xFF]),
                13 /* Oak Leaf */ => pixel.copy_from_slice(&[0x2F, 0x47, 0x20, 0xFF]),
                14 /* Coal     */ => pixel.copy_from_slice(&[0x0D, 0x0D, 0x0D, 0xFF]),
                _ => pixel.copy_from_slice(&[0xFF, 0xFF, 0xFF, 0xFF]),
            }
        }

        if block == 9 && x == 129 && y == 1 {
            self.pixels.render().unwrap();
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}

fn main() {
    let sdl_system = SDLSystem::new();
    let pixels = Pixels::new(SCREEN_WIDTH, SCREEN_HEIGHT, pixels::SurfaceTexture::new(SCREEN_WIDTH, SCREEN_HEIGHT, &sdl_system.window)).unwrap();

    let state = Mutex::new(McState { turtle_x: 0, turtle_y: 0, turtle_z: 0, pixels, blocks: HashMap::new() });
    let state = Box::leak(Box::new(state));

    let handle = std::thread::spawn(|| {
        //let path = "/home/salix/Programming/ccleste/ccleste-mc.wasm";
        let path = "/home/salix/Programming/doomgeneric/doomgeneric/doomgeneric.wasm";
        //let path = "/home/salix/Documents/Code/wasmcraft/findplane_test/main.wasm";
        //let path = "/home/salix/Programming/rusty-nes/mnes_mc/target/wasm32-unknown-unknown/release/mnes_mc.wasm";
        //let path = "/home/salix/Programming/rusty-nes/mnes_mc/target/wasm32-unknown-unknown/release/mnes_mc.wasm";
        //let path = "./fib.wasm";
        //let path = "./fib_rs.wasm";

        //let engine = Engine::new(Config::new().debug_info(true)).unwrap();
        let engine = Engine::new(&Config::new()).unwrap();

        let mut file = std::fs::File::open(path).unwrap();
        let mut file_data = Vec::new();
        file.read_to_end(&mut file_data).unwrap();
        let file_data = file_data;

        let module = Module::new(&engine, &file_data).unwrap();
        println!("Hello, world!");

        let mut linker = Linker::new(&engine);
        linker.func_wrap("env", "memset", |mut caller: Caller<'_, u32>, dst: i32, val: i32, len: i32| -> i32 {
            let ext = caller.get_export("memory").unwrap();
            if let wasmtime::Extern::Memory(mem) = ext {
                if len < 0 {
                    panic!();
                }

                let ctx = caller.as_context_mut();
                let data = mem.data_mut(ctx);
                let data = &mut data[dst as usize..][..len as usize];
                for i in data.iter_mut() {
                    *i = val as u8;
                }
                dst
            } else {
                panic!();
            }
        }).unwrap();
        linker.func_wrap("env", "mc_putc", |caller: Caller<'_, u32>, param: i32| {
            if let Some(c) = char::from_u32(param as u32) {
                print!("{}", c);
            } else {
                println!("invalid char {}", param);
            }
        }).unwrap();
        linker.func_wrap("env", "print", |caller: Caller<'_, u32>, param: i32| {
            //println!("Printed {}", param);

            let mut l = state.lock().unwrap();

            if param == 0xDDDD {
                l.pixels.render().unwrap();
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        }).unwrap();
        /*linker.func_wrap("env", "print", |caller: Caller<'_, u32>, param: i32| {
            println!("Printing {param}");
        }).unwrap();
        linker.func_wrap("env", "sleep", |caller: Caller<'_, u32>| {
            //println!("Slept");
        }).unwrap();*/
        linker.func_wrap("env", "mc_sleep", |caller: Caller<'_, u32>| {
            //println!("Slept");
        }).unwrap();
        linker.func_wrap("env", "turtle_x", |caller: Caller<'_, u32>, param: i32| {
            //println!("Set turtle x to {param}");
            let mut l = state.lock().unwrap();
            l.turtle_x = param;
        }).unwrap();
        linker.func_wrap("env", "turtle_y", |caller: Caller<'_, u32>, param: i32| {
            //println!("Set turtle y to {param}");
            let mut l = state.lock().unwrap();
            l.turtle_y = param;
        }).unwrap();
        linker.func_wrap("env", "turtle_z", |caller: Caller<'_, u32>, param: i32| {
            //println!("Set turtle z to {param}");
            let mut l = state.lock().unwrap();
            l.turtle_z = param;
        }).unwrap();
        linker.func_wrap("env", "turtle_copy", |caller: Caller<'_, u32>| {
            let mut l = state.lock().unwrap();
            let block = l.blocks.get(&(l.turtle_x, l.turtle_y, l.turtle_z)).copied();
            if let Some(block) = block {
                l.blocks.insert((-1, -1, -1), block);
            } else {
                l.blocks.remove(&(-1, -1, -1));
            }
        }).unwrap();
        linker.func_wrap("env", "turtle_paste", |caller: Caller<'_, u32>| {
            let mut l = state.lock().unwrap();

            let param = if let Some(param) = l.blocks.get(&(-1, -1, -1)) {
                *param
            } else {
                return;
            };

            l.set(param);
        }).unwrap();
        linker.func_wrap("env", "turtle_get", |caller: Caller<'_, u32>| -> i32 {
            0
        }).unwrap();
        linker.func_wrap("env", "turtle_set", |caller: Caller<'_, u32>, param: i32| {
            //println!("Set block {param}");
            let mut l = state.lock().unwrap();
            l.set(param);

        }).unwrap();
        linker.func_wrap("env", "turtle_fill", |caller: Caller<'_, u32>, block: i32, x_span: i32, y_span: i32, z_span: i32| {
            //println!("Set block {param}");
            let mut l = state.lock().unwrap();

            assert!(x_span >= 0);
            assert!(y_span >= 0);
            assert!(z_span >= 0);

            for x_off in 0..x_span + 1 {
                for y_off in 0..y_span + 1 {
                    for z_off in 0..z_span + 1 {
                        let pos = (l.turtle_x + x_off, l.turtle_y + y_off, l.turtle_z + z_off);

                        l.set_at(block, pos.0, pos.1, pos.2);
                    }
                }
            }
        }).unwrap();
        linker.func_wrap("env", "turtle_copy_region", |caller: Caller<'_, u32>, x_span: i32, y_span: i32, z_span: i32| {
            //println!("Set block {param}");
            let mut l = state.lock().unwrap();

            assert!(x_span >= 0);
            assert!(y_span >= 0);
            assert!(z_span >= 0);

            for x_off in 0..x_span + 1 {
                for y_off in 0..y_span + 1 {
                    for z_off in 0..z_span + 1 {
                        let src_pos = (l.turtle_x + x_off, l.turtle_y + y_off, l.turtle_z + z_off);
                        let dst_pos = (x_off, y_off, -1 + z_off);

                        let block = l.blocks.get(&src_pos).copied().unwrap_or(0);

                        l.set_at(block, dst_pos.0, dst_pos.1, dst_pos.2);
                    }
                }
            }
        }).unwrap();
        linker.func_wrap("env", "turtle_paste_region_masked", |caller: Caller<'_, u32>, x_span: i32, y_span: i32, z_span: i32| {
            //println!("Set block {param}");
            let mut l = state.lock().unwrap();

            assert!(x_span >= 0);
            assert!(y_span >= 0);
            assert!(z_span >= 0);

            for x_off in 0..x_span + 1 {
                for y_off in 0..y_span + 1 {
                    for z_off in 0..z_span + 1 {
                        let src_pos = (x_off, y_off, -1 + z_off);
                        let dst_pos = (l.turtle_x + x_off, l.turtle_y + y_off, l.turtle_z + z_off);

                        let block = l.blocks.get(&src_pos).copied().unwrap_or(0);

                        if block != 0 {
                            l.set_at(block, dst_pos.0, dst_pos.1, dst_pos.2);
                        }
                    }
                }
            }
        }).unwrap();

        let mut store = Store::new(&engine, 0);
        let instance = linker.instantiate(&mut store, &module).unwrap();
        let start_func = instance.get_typed_func::<(), (), _>(&mut store, "_start").unwrap();
        start_func.call(&mut store, ()).unwrap();
    });

    let mut event_pump = sdl_system.ctx.event_pump().unwrap();

    loop {

        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => {
                    std::process::exit(0);
                }
                _ => {}
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(16));

        let l = state.lock().unwrap();
        l.pixels.render().unwrap();
    }
}
