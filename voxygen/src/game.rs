// Standard
use std::{
    f32::consts::PI,
    net::ToSocketAddrs,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

// Library
use dot_vox;
use fnv::FnvBuildHasher;
use fps_counter::FPSCounter;
use glutin::ElementState;
use indexmap::IndexMap;
use parking_lot::Mutex;
use vek::*;

type FnvIndexMap<K, V> = IndexMap<K, V, FnvBuildHasher>;

// Project
use client::{self, Client, ClientEvent, PlayMode, CHUNK_SIZE};
use common::{
    get_asset_path,
    terrain::{
        self,
        chunk::{Chunk, ChunkContainer},
        Container, VolOffs,
    },
    util::manager::Manager,
};

// Local
use crate::{
    audio::frontend::AudioFrontend,
    camera::Camera,
    consts::{ConstHandle, GlobalConsts},
    get_shader_path,
    hud::{Hud, HudEvent},
    key_state::KeyState,
    keybinds::{Keybinds, VKeyCode},
    pipeline::Pipeline,
    shader::Shader,
    skybox, tonemapper, voxel,
    window::{Event, RenderWindow},
    RENDERER_INFO,
};

pub enum ChunkPayload {
    Meshes(FnvIndexMap<voxel::MaterialKind, voxel::Mesh>),
    Model {
        model: voxel::Model,
        model_consts: ConstHandle<voxel::ModelConsts>,
    },
}

pub struct Payloads {}
impl client::Payloads for Payloads {
    type Chunk = ChunkPayload;
    type Entity = ConstHandle<voxel::ModelConsts>;
    type Audio = AudioFrontend;
}

pub struct Game {
    running: AtomicBool,

    client: Manager<Client<Payloads>>,
    window: RenderWindow,

    global_consts: ConstHandle<GlobalConsts>,
    camera: Mutex<Camera>,

    key_state: Mutex<KeyState>,
    keys: Keybinds,

    skybox_pipeline: Pipeline<skybox::pipeline::Init<'static>>,
    volume_pipeline: voxel::VolumePipeline,
    tonemapper_pipeline: Pipeline<tonemapper::pipeline::Init<'static>>,

    hud: Hud,
    audio: Manager<AudioFrontend>,

    fps: FPSCounter,
    last_fps: usize,

    skybox_model: skybox::Model,
    player_model: voxel::Model,
    other_player_model: voxel::Model,
}

fn to_4x4(v: &Mat4<f32>) -> [[f32; 4]; 4] {
    let mut out = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            out[i][j] = v[(j, i)];
        }
    }
    out
}

fn gen_payload(_key: Vec3<VolOffs>, con: Arc<Mutex<Option<ChunkContainer<<Payloads as client::Payloads>::Chunk>>>>) {
    let conlock = con.lock();
    if let Some(ref con) = *conlock {
        *con.payload_mut() = Some(ChunkPayload::Meshes(match *con.data() {
            Chunk::Homo(ref homo) => voxel::Mesh::from(homo),
            Chunk::Hetero(ref hetero) => voxel::Mesh::from(hetero),
            Chunk::Rle(ref rle) => voxel::Mesh::from(rle),
            Chunk::HeteroAndRle(ref hetero, _) => voxel::Mesh::from(hetero),
        }));
    }
}

fn drop_payload(_key: Vec3<VolOffs>, _con: Arc<ChunkContainer<<Payloads as client::Payloads>::Chunk>>) {}

impl Game {
    pub fn new<R: ToSocketAddrs>(mode: PlayMode, alias: &str, remote_addr: R, view_distance: i64) -> Game {
        let window = RenderWindow::new();
        let info = window.get_renderer_info();
        println!(
            "Graphics card info - vendor: {} model: {} OpenGL: {}",
            info.vendor, info.model, info.gl_version
        );
        *RENDERER_INFO.lock() = Some(info);

        let audio = AudioFrontend::new();

        let client = Client::new(
            mode,
            alias.to_string(),
            remote_addr,
            gen_payload,
            drop_payload,
            Manager::<AudioFrontend>::internal(&audio).clone(),
            view_distance,
        )
        .expect("Could not create new client");

        // Contruct the UI
        let _window_dims = window.get_size();

        // Create pipelines

        let volume_pipeline = voxel::VolumePipeline::new(&mut window.renderer_mut());

        let skybox_pipeline = Pipeline::new(
            window.renderer_mut().factory_mut(),
            skybox::pipeline::new(),
            &Shader::from_file(get_shader_path("skybox/skybox.vert")).expect("Could not load skybox vertex shader"),
            &Shader::from_file(get_shader_path("skybox/skybox.frag")).expect("Could not load skybox fragment shader"),
        );

        let tonemapper_pipeline = Pipeline::new(
            window.renderer_mut().factory_mut(),
            tonemapper::pipeline::new(),
            &Shader::from_file(get_shader_path("tonemapper/tonemapper.vert"))
                .expect("Could not load skybox vertex shader"),
            &Shader::from_file(get_shader_path("tonemapper/tonemapper.frag"))
                .expect("Could not load skybox fragment shader"),
        );

        let global_consts = ConstHandle::new(&mut window.renderer_mut());

        let skybox_mesh = skybox::Mesh::new_skybox();
        let skybox_model = skybox::Model::new(&mut window.renderer_mut(), &skybox_mesh);

        info!("trying to load model files");
        let vox = dot_vox::load(
            get_asset_path("voxygen/cosmetic/creature/friendly/knight.vox")
                .to_str()
                .unwrap(),
        )
        .expect("cannot find model player6.vox. Make sure to start voxygen from its folder");
        let voxmodel = voxel::vox_to_figure(vox);

        let player_meshes = voxel::Mesh::from_with_offset(&voxmodel, Vec3::new(-10.0, -4.0, 0.0), false);

        let player_model = voxel::Model::new(&mut window.renderer_mut(), &player_meshes);

        let vox = dot_vox::load(
            get_asset_path("voxygen/cosmetic/creature/friendly/knight.vox")
                .to_str()
                .unwrap(),
        )
        .expect("cannot find model player7.vox. Make sure to start voxygen from its folder");
        let voxmodel = voxel::vox_to_figure(vox);

        let other_player_meshes = voxel::Mesh::from_with_offset(&voxmodel, Vec3::new(-10.0, -4.0, 0.0), false);

        let other_player_model = voxel::Model::new(&mut window.renderer_mut(), &other_player_meshes);

        Game {
            running: AtomicBool::new(true),

            client,
            window,

            global_consts,
            camera: Mutex::new(Camera::new()),

            key_state: Mutex::new(KeyState::new()),
            keys: Keybinds::new(),

            skybox_pipeline,
            volume_pipeline,
            tonemapper_pipeline,

            hud: Hud::new(),
            audio,

            fps: FPSCounter::new(),
            last_fps: 60,

            skybox_model,
            player_model,
            other_player_model,
        }
    }

    pub fn handle_window_events(&self) {
        self.window.handle_events(|event| {
            // TODO: Experimental
            if true && self.hud.handle_event(&event, &mut self.window.renderer_mut()) {
                return true;
            }

            match event {
                Event::CloseRequest => self.running.store(false, Ordering::Relaxed),
                Event::CursorMoved { dx, dy } => {
                    if self.window.cursor_trapped().load(Ordering::Relaxed) {
                        self.camera
                            .lock()
                            .rotate_by(Vec2::new(dx as f32 * 0.002, dy as f32 * 0.002));
                    }
                },
                Event::MouseWheel { dy, .. } => {
                    self.camera.lock().zoom_by((-dy / 4.0) as f32);
                },
                Event::KeyboardInput { i, .. } => {
                    // Helper function to determine scancode equality
                    fn keypress_eq(key: &Option<VKeyCode>, input: Option<glutin::VirtualKeyCode>) -> bool {
                        if let (Some(i), Some(k)) = (input, key) {
                            k.code() == i
                        } else {
                            false
                        }
                    }

                    // Helper variables to clean up code. Add any new input modes here.
                    let general = &self.keys.general;

                    // General inputs -------------------------------------------------------------
                    if keypress_eq(&general.pause, i.virtual_keycode) {
                        // Default: Escape (free cursor)
                        self.window.untrap_cursor();
                    } else if keypress_eq(&general.use_item, i.virtual_keycode) {
                        // Default: Ctrl+Q (quit) (temporary)
                        if i.modifiers.ctrl {
                            self.running.store(false, Ordering::Relaxed);
                        }
                    } else if keypress_eq(&general.chat, i.virtual_keycode) && i.state == ElementState::Released {
                        //self.ui.borrow_mut().set_show_chat(!show_chat);
                    }

                    // TODO: Remove this check
                    if keypress_eq(&general.forward, i.virtual_keycode) {
                        self.key_state.lock().up = match i.state {
                            // Default: W (up)
                            ElementState::Pressed => true,
                            ElementState::Released => false,
                        }
                    } else if keypress_eq(&general.left, i.virtual_keycode) {
                        self.key_state.lock().left = match i.state {
                            // Default: A (left)
                            ElementState::Pressed => true,
                            ElementState::Released => false,
                        }
                    } else if keypress_eq(&general.back, i.virtual_keycode) {
                        self.key_state.lock().down = match i.state {
                            // Default: S (down)
                            ElementState::Pressed => true,
                            ElementState::Released => false,
                        }
                    } else if keypress_eq(&general.right, i.virtual_keycode) {
                        self.key_state.lock().right = match i.state {
                            // Default: D (right)
                            ElementState::Pressed => true,
                            ElementState::Released => false,
                        }
                    } else if keypress_eq(&general.jump, i.virtual_keycode) {
                        self.key_state.lock().jump = match i.state {
                            // Default: Space (fly)
                            ElementState::Pressed => true,
                            ElementState::Released => false,
                        }
                    } else if keypress_eq(&general.crouch, i.virtual_keycode) {
                        // self.key_state.lock().fall = match i.state { // Default: Shift (fall)
                        //     ElementState::Pressed => true,
                        //     ElementState::Released => false,
                        // }
                    }

                    // ----------------------------------------------------------------------------

                    // Mount inputs ---------------------------------------------------------------
                    // placeholder
                    // ----------------------------------------------------------------------------
                },
                Event::Resized { w, h } => {
                    self.camera
                        .lock()
                        .set_aspect_ratio((w.max(1) as f32) / (h.max(1) as f32));
                },
                _ => {},
            }
            false
        });

        // Calculate movement player movement vector
        let ori = *self.camera.lock().ori();
        let unit_vecs = (
            Vec2::new(ori.x.cos(), -ori.x.sin()),
            Vec2::new(ori.x.sin(), ori.x.cos()),
        );
        let dir_vec = self.key_state.lock().dir_vec();
        let mov_vec = unit_vecs.0 * dir_vec.x + unit_vecs.1 * dir_vec.y;

        // Why do we do this in Voxygen?!
        const LOOKING_VEL_FAC: f32 = 1.0;
        const LOOKING_CTRL_ACC_FAC: f32 = 1.0;
        const MIN_LOOKING: f32 = 0.5;
        const LEANING_FAC: f32 = 0.05;
        if let Some(player_entity) = self.client.player_entity() {
            let mut player_entity = player_entity.write();

            // Apply acceleration
            player_entity.ctrl_acc_mut().x = mov_vec.x;
            player_entity.ctrl_acc_mut().y = mov_vec.y;

            // Apply jumping
            player_entity.ctrl_acc_mut().z = if self.key_state.lock().jump() { 1.0 } else { 0.0 };

            let looking = (*player_entity.vel() * LOOKING_VEL_FAC
                + *player_entity.ctrl_acc_mut() * LOOKING_CTRL_ACC_FAC)
                / (LOOKING_VEL_FAC + LOOKING_CTRL_ACC_FAC);

            // Apply rotating
            if looking.magnitude() > MIN_LOOKING {
                player_entity.look_dir_mut().x = looking.x.atan2(looking.y);
            }

            // Apply leaning
            player_entity.look_dir_mut().y = Vec2::new(looking.x, looking.y).magnitude() * LEANING_FAC;
        }
    }

    pub fn update_chunks(&self) {
        let mut renderer = self.window.renderer_mut();
        // Find the chunk the player is in
        let player_pos = self
            .client
            .player_entity()
            .map(|p| *p.read().pos())
            .unwrap_or(Vec3::new(0.0, 0.0, 0.0));
        let player_chunk = terrain::voxabs_to_voloffs(player_pos.map(|e| e as i64), CHUNK_SIZE);
        let squared_view_distance = (self.client.view_distance() / CHUNK_SIZE.x as f32 + 1.0).powi(2) as i32; // view_distance is vox based, but its needed vol based here

        for (pos, con) in self
            .client
            .chunk_mgr()
            .pers(|chunk_offs| player_chunk.distance_squared(*chunk_offs) < squared_view_distance)
            .iter()
        {
            let trylock = &mut con.payload_try_mut(); //we try to lock it, if it is already written to we just ignore this chunk for a frame
            if let Some(ref mut lock) = trylock {
                //sometimes payload does not exist, dont render then
                if let Some(ref mut payload) = **lock {
                    if let ChunkPayload::Meshes(ref mut mesh) = payload {
                        // Calculate chunk mode matrix
                        let model_mat = Mat4::<f32>::translation_3d(pos.map2(CHUNK_SIZE, |p, s| (p * s as i32) as f32));

                        // Create set new model constants
                        let model_consts = ConstHandle::new(&mut renderer);

                        // Update chunk model constants
                        model_consts.update(
                            &mut renderer,
                            voxel::ModelConsts {
                                model_mat: to_4x4(&model_mat),
                            },
                        );

                        // Update the chunk payload
                        *payload = ChunkPayload::Model {
                            model: voxel::Model::new(&mut renderer, mesh),
                            model_consts,
                        };
                    }
                }
            }
        }
    }

    pub fn handle_client_events(&mut self) {
        let mut events = self.client.get_events();

        events.drain(..).for_each(|event| match event {
            ClientEvent::RecvChatMsg { text } => self.hud.chat_box().add_chat_msg(text),
        });
    }

    pub fn handle_hud_events(&mut self) {
        let mut events = self.hud.get_events();

        events.drain(..).for_each(|event| match event {
            HudEvent::ChatMsgSent { text } => {
                if text.len() > 0 {
                    self.client.send_chat_msg(text);
                }
            },
        });
    }

    pub fn update_entities(&self) {
        // Take the physics lock to sync client and frontend updates
        let _ = self.client.take_phys_lock();

        // Set camera focus to the player's head
        if let Some(player_entity) = self.client.player_entity() {
            let player_entity = player_entity.read();
            self.camera.lock().set_focus(Vec3::<f32>::from(
                (*player_entity.pos() + Vec3::new(0.0, 0.0, 1.75)).into_array(),
            ));
        }

        let mut renderer = self.window.renderer_mut();

        // Update each entity constbuffer
        for (_, entity) in self.client.entities().iter() {
            let mut entity = entity.write();

            // Calculate entity model matrix
            let model_mat = Mat4::<f32>::translation_3d(Vec3::from(entity.pos().into_array()))
                * Mat4::rotation_z(PI - entity.look_dir().x)
                * Mat4::rotation_x(entity.look_dir().y);

            // Update the model const buffer (its payload)
            // TODO: Put the model into the payload so we can have per-entity models!
            entity
                .payload_mut()
                .get_or_insert_with(|| ConstHandle::new(&mut renderer))
                .update(
                    &mut renderer,
                    voxel::ModelConsts {
                        model_mat: to_4x4(&model_mat),
                    },
                );
        }
    }

    pub fn render_frame(&mut self) {
        // Calculate frame constants
        let camera_mats = self.camera.lock().get_mats();
        let camera_fov = self.camera.lock().get_fov();
        // TODO: Maybe rename this to cam_pos?
        let cam_origin = self.camera.lock().get_pos(Some(&camera_mats));
        let cam_zoom = self.camera.lock().get_zoom();
        let (player_pos, player_vel, player_ori) = {
            let e = self.client.player_entity();
            if let Some(e) = e {
                let lock = e.read();
                let ld = lock.look_dir();
                (*lock.pos(), *lock.vel(), Vec3::new(ld.x, ld.y, 0.0))
            } else {
                (
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(0.0, 0.0, 0.0),
                )
            }
        };
        let play_origin = [player_pos.x, player_pos.y, player_pos.z, 1.0];
        let time = self.client.time().as_float_secs() as f32;

        // Begin rendering, don't clear the frame
        let mut renderer = self.window.renderer_mut();
        renderer.begin_frame(None);

        // Update global constants that apply to the entire frame
        self.global_consts.update(
            &mut renderer,
            GlobalConsts {
                view_mat: to_4x4(&camera_mats.0),
                proj_mat: to_4x4(&camera_mats.1),
                cam_origin: [cam_origin.x, cam_origin.y, cam_origin.z, 1.0],
                play_origin,
                view_distance: [self.client.view_distance(); 4],
                time: [time; 4],
            },
        );

        // Render the skybox
        self.skybox_model
            .render(&mut renderer, &self.skybox_pipeline, &self.global_consts);

        // Find the chunk the player is in
        let squared_view_distance = self.client.view_distance().powi(2) as f32; // view_distance is vox based, but its needed vol based here
        let cam_vec_world = camera_mats.0.inverted() * (-Vec4::unit_z());

        // Render each chunk
        for (_pos, con) in self
            .client
            .chunk_mgr()
            .pers(|chunk_offs| {
                let chunk_pos = chunk_offs.map(|e| e as f32) * CHUNK_SIZE.map(|e| e as f32);
                // This limit represents the point in the chunk that's closest to the player (0 - CHUNK_SIZE)
                let chunk_offs_limit = Vec3::clamp(player_pos - chunk_pos, Vec3::zero(), CHUNK_SIZE.map(|e| e as f32));
                // Check whether the chunk is within range of the view distance
                (chunk_pos + chunk_offs_limit).distance_squared(player_pos) < squared_view_distance &&
                // Check whether the chunk is within the frustrum of the camera (or within a certain minimum range to avoid visual artefacts)
                (Vec4::from(chunk_pos + CHUNK_SIZE.map(|e| e as f32) / 2.0 - cam_origin)
                        .normalized()
                        .dot(cam_vec_world)
                        > camera_fov.cos()
                        || (chunk_pos + CHUNK_SIZE.map(|e| e as f32) / 2.0 - cam_origin).magnitude()
                            < CHUNK_SIZE.x as f32 * 2.0)
            })
            .iter()
        {
            let trylock = &con.payload_try(); //we try to lock it, if it is already written to we just ignore this chunk for a frame
            if let Some(ref lock) = trylock {
                if let Some(ref payload) = **lock {
                    if let ChunkPayload::Model {
                        ref model,
                        ref model_consts,
                    } = payload
                    {
                        self.volume_pipeline
                            .draw_model(&model, model_consts, &self.global_consts);
                    }
                }
            }
        }

        // Render each entity
        for (&uid, entity) in self.client.entities().iter() {
            // Choose the correct model for the entity
            let model = match self.client.player().entity_uid {
                Some(player_uid) if uid == player_uid => {
                    if cam_zoom == 0.0 {
                        continue;
                    }
                    &self.player_model
                },
                _ => &self.other_player_model,
            };

            if let Some(ref model_consts) = entity.read().payload() {
                self.volume_pipeline
                    .draw_model(&model, model_consts, &self.global_consts);
            }
        }

        // flush voxel pipeline draws
        self.volume_pipeline.flush(&mut renderer);

        //update audio
        self.audio
            .set_pos(player_pos, player_vel, camera_mats.0 * camera_mats.1);

        tonemapper::render(&mut renderer, &self.tonemapper_pipeline, &self.global_consts);

        use crate::{get_build_time, get_git_hash};

        // TODO: Use a HudEvent to pass this in!
        self.hud
            .debug_box()
            .version_label
            .set_text(format!("Version: {}", env!("CARGO_PKG_VERSION")));
        self.hud
            .debug_box()
            .githash_label
            .set_text(format!("Git hash: {}", &get_git_hash().get(..8).unwrap_or("<none>")));
        self.hud
            .debug_box()
            .buildtime_label
            .set_text(format!("Build time: {}", get_build_time()));
        self.hud
            .debug_box()
            .fps_label
            .set_text(format!("FPS: {}", self.last_fps));

        let pos_text = self
            .client
            .player_entity()
            .map(|p| format!("Pos: {}", p.read().pos().map(|e| e as i64)))
            .unwrap_or("Unknown position".to_string());
        self.hud.debug_box().pos_label.set_text(pos_text);

        self.hud.render(&mut renderer);

        self.window.swap_buffers();
        renderer.end_frame();

        self.last_fps = self.fps.tick();
    }

    pub fn run(&mut self) {
        while self.running.load(Ordering::Relaxed) {
            self.handle_window_events();
            self.handle_hud_events();
            self.handle_client_events();
            self.update_chunks();
            self.update_entities();

            self.render_frame();
        }
    }
}
